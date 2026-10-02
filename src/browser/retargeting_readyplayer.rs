//! webgpu_animation_retargeting_readyplayer: the Mixamo character's dance
//! ( mixamo.fbx ) retargeted onto the readyplayer.me avatar with
//! SkeletonUtils.retargetClip, both dancing over a reflector floor under the
//! hemisphere and two directional lights. The FBX is the pinned FBXLoader's
//! parse, baked by `tools/tsl/prepare-mixamo.mjs` with the world matrices the
//! loader left; the retargeted clip is baked once on the CPU, as the page
//! does, with retargeting's simulation of three.js's matrix semantics. Every
//! skinned mesh ( the avatar's ten, the character's two ) is skinned on the
//! GPU from its skeleton's bone matrices; the opaque meshes draw in r186's
//! order ( their bounding-sphere centers' clip depth ) after the frustum test
//! with the spheres SkinnedMesh computes at the first culling; the reflector
//! renders the mirrored view into the target the floor samples. Every stage
//! runs the WGSL three.js r186 generates for the page ( in
//! `retargeting_readyplayer/`; the background vertex and output modules are
//! the skinning_instances and compute_cloth ones, byte-identical ).
use super::controls_attributes::{Controls, camera_state};
use super::gltf_viewer::{fetch, load_asset};
use super::lights_projector::{m3, m4, pack};
use super::retargeting::{
    BakedBone, Sim, SimNode, linear, reflector_camera, retarget_clip, skinned_sphere,
};
use super::retro::mipmapped;
use super::shadowmap_opacity::Mipmaps;
use super::three_mixer::ThreeMixer;
use super::wgsl_bind::{Spec, groups};
use crate::animation::{Clip, Interpolation, Property, Track};
use crate::{
    Error, Result, camera::*, geometry::*, math::*, render_target::*, renderer::*, scene::*,
};
use std::collections::HashMap;
use std::f64::consts::PI;
use std::sync::Arc;
use wgpu::util::DeviceExt;

const HALF: wgpu::TextureFormat = wgpu::TextureFormat::Rgba16Float;
const DEPTH: wgpu::TextureFormat = wgpu::TextureFormat::Depth24Plus;
macro_rules! wgsl {
    ($name:literal) => {
        include_str!(concat!("retargeting_readyplayer/", $name, ".wgsl"))
    };
}
const BACKGROUND: (&str, &str) = (
    include_str!("skinning_instances/background_vs.wgsl"),
    wgsl!("background_fs"),
);
const FLOOR: (&str, &str) = (wgsl!("floor_vs"), wgsl!("floor_fs"));
const OUTPUT: (&str, &str) = (
    include_str!("compute_cloth/output_vs.wgsl"),
    include_str!("compute_cloth/output_fs.wgsl"),
);
/// The materials' generated variants.
#[derive(Clone, Copy, PartialEq)]
enum Kind {
    /// Morphed, normal-mapped without tangents ( the beard ).
    Beard,
    /// Morphed ( the eyes, head and teeth ).
    Morphed,
    /// Metal-roughness and normal maps ( the headwear and outfit ).
    Mapped,
    /// Normal-mapped with tangents ( the body ).
    Body,
    /// The FBX character's Phong materials.
    Phong,
}
/// A skinned mesh and its resources.
struct Mesh {
    kind: Kind,
    shaders: (&'static str, &'static str),
    double_sided: bool,
    /// Location, buffer and format of each vertex input.
    attributes: Vec<(u32, wgpu::Buffer, wgpu::VertexFormat)>,
    index: Option<wgpu::Buffer>,
    count: u32,
    node: Object3D,
    bones: Vec<(Object3D, Matrix4)>,
    bone_buffer: wgpu::Buffer,
    /// The morph influences and the morph target texture.
    morph: Option<(wgpu::Buffer, wgpu::TextureView)>,
    /// Texture names with their view and sampler indices.
    maps: Vec<(&'static str, usize, usize)>,
    object: wgpu::Buffer,
    /// Color, metalness, roughness ( or Phong's shininess ), normal scale y,
    /// Phong specular.
    material: ([f64; 3], f64, f64, f64, [f64; 3]),
    positions: Vec<f32>,
    joints: Vec<u32>,
    weights: Vec<f32>,
    /// The geometry's bounding sphere center ( the sort key's origin ).
    center: Vector3,
    sphere: Option<Sphere>,
}
type Draw = (wgpu::RenderPipeline, Vec<wgpu::BindGroup>);
struct Targets {
    width: u32,
    height: u32,
    format: wgpu::TextureFormat,
    samples: u32,
    msaa: Option<(wgpu::TextureView, wgpu::TextureView)>,
    color: wgpu::TextureView,
    depth: wgpu::TextureView,
    reflection: (wgpu::TextureView, wgpu::TextureView),
    /// Per pass ( main, reflection ): the background, then each mesh.
    draws: [Vec<Draw>; 2],
    floor: Draw,
    output: Draw,
    screen: RenderTarget,
}
pub(super) struct Demo {
    controls: Controls,
    time: f64,
    last: f64,
    pending: bool,
    source_mixer: ThreeMixer,
    target_mixer: ThreeMixer,
    meshes: Vec<Mesh>,
    views: Vec<wgpu::TextureView>,
    samplers: Vec<wgpu::Sampler>,
    background: (wgpu::Buffer, wgpu::Buffer, wgpu::Buffer, u32),
    floor: (wgpu::Buffer, wgpu::Buffer, u32),
    /// Per pass: the background's and the lit materials' render structs.
    renders: [[wgpu::Buffer; 2]; 2],
    background_object: wgpu::Buffer,
    floor_render: wgpu::Buffer,
    floor_object: wgpu::Buffer,
    output_render: wgpu::Buffer,
    output_object: wgpu::Buffer,
    quad: wgpu::Buffer,
    clamp: wgpu::Sampler,
    targets: Option<Targets>,
}
/// The baked FBX: its JSON index and binary data.
struct Baked {
    json: serde_json::Value,
    bin: Vec<u8>,
}
impl Baked {
    fn floats(&self, v: &serde_json::Value) -> Result<Vec<f32>> {
        let offset = v["offset"].as_u64().ok_or(Error::Invalid("baked offset"))? as usize;
        let length = v["length"].as_u64().ok_or(Error::Invalid("baked length"))? as usize;
        let bytes = self
            .bin
            .get(offset..offset + length * 4)
            .ok_or(Error::Invalid("baked range"))?;
        Ok(bytes
            .as_chunks::<4>()
            .0
            .iter()
            .map(|&c| f32::from_le_bytes(c))
            .collect())
    }
    fn u16s(&self, v: &serde_json::Value) -> Result<Vec<u32>> {
        let offset = v["offset"].as_u64().ok_or(Error::Invalid("baked offset"))? as usize;
        let length = v["length"].as_u64().ok_or(Error::Invalid("baked length"))? as usize;
        let bytes = self
            .bin
            .get(offset..offset + length * 2)
            .ok_or(Error::Invalid("baked range"))?;
        Ok(bytes
            .as_chunks::<2>()
            .0
            .iter()
            .map(|&c| u32::from(u16::from_le_bytes(c)))
            .collect())
    }
}
fn numbers(v: &serde_json::Value) -> Vec<f64> {
    v.as_array()
        .map(|a| a.iter().filter_map(serde_json::Value::as_f64).collect())
        .unwrap_or_default()
}
fn matrix(v: &serde_json::Value) -> Matrix4 {
    let n = numbers(v);
    Matrix4::from_cols_array(&std::array::from_fn(|i| n.get(i).copied().unwrap_or(0.)))
}
/// The vertex inputs of a generated vertex module's main: ( location, name ).
fn inputs(vs: &str) -> Vec<(u32, String)> {
    let Some(start) = vs.find("fn main(") else {
        return vec![];
    };
    let end = vs[start..].find("->").map_or(vs.len(), |e| start + e);
    let mut out = vec![];
    let mut rest = &vs[start..end];
    while let Some(at) = rest.find("@location(") {
        rest = &rest[at + "@location(".len()..];
        let Some(close) = rest.find(')') else {
            break;
        };
        let location: u32 = rest[..close].trim().parse().unwrap_or(0);
        let name: String = rest[close + 1..]
            .trim_start()
            .chars()
            .take_while(|c| c.is_alphanumeric() || *c == '_')
            .collect();
        out.push((location, name));
    }
    out
}
/// BufferGeometry.computeBoundingSphere's center: the box center of the
/// positions, expanded by the relative morph targets' extents.
fn geometry_center(positions: &[f32], morphs: &[Vec<f32>]) -> Vector3 {
    let bounds = |p: &[f32]| {
        p.chunks(3).fold(
            (
                Vector3::splat(f64::INFINITY),
                Vector3::splat(f64::NEG_INFINITY),
            ),
            |(lo, hi), v| {
                let v = Vector3::new(v[0] as f64, v[1] as f64, v[2] as f64);
                (lo.min(v), hi.max(v))
            },
        )
    };
    let (mut lo, mut hi) = bounds(positions);
    for m in morphs {
        let (mlo, mhi) = bounds(m);
        for p in [lo + mlo, hi + mhi] {
            lo = lo.min(p);
            hi = hi.max(p);
        }
    }
    (lo + hi) * 0.5
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, _id: u32, r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 40.,
            near: 0.25,
            far: 50.,
            aspect,
            ..Default::default()
        }));
        s.get_mut(c)?.position = Vector3::new(0., 3., 5.);
        let mut controls = Controls::new(None, (3., 12.), PI / 2., true);
        controls.set_target(Vector3::new(0., 1., 0.));
        let init = |label, data: &[u8], usage| {
            r.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some(label),
                    contents: data,
                    usage,
                })
        };
        let vertex = wgpu::BufferUsages::VERTEX;
        let index_usage = wgpu::BufferUsages::INDEX;
        let uniform_usage = wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST;
        // The source: the baked FBX scene graph, at the page's placement.
        let baked = Baked {
            json: serde_json::from_slice(
                &fetch("/web/gallery/assets/retargeting-readyplayer/mixamo.json").await?,
            )
            .map_err(|e| Error::Asset(e.to_string()))?,
            bin: fetch("/web/gallery/assets/retargeting-readyplayer/mixamo.bin").await?,
        };
        let baked_nodes = baked.json["nodes"]
            .as_array()
            .ok_or(Error::Invalid("baked nodes"))?;
        let mut fbx_handles = vec![];
        let mut sim_nodes = vec![];
        for n in baked_nodes {
            let h = s.insert(NodeKind::Group);
            let p = numbers(&n["position"]);
            let q = numbers(&n["quaternion"]);
            let sc = numbers(&n["scale"]);
            let node = s.get_mut(h)?;
            node.position = Vector3::new(p[0], p[1], p[2]);
            node.quaternion = Quaternion::from_xyzw(q[0], q[1], q[2], q[3]);
            node.scale = Vector3::new(sc[0], sc[1], sc[2]);
            let parent = n["parent"].as_u64().map(|p| p as usize);
            if let Some(p) = parent {
                s.add(fbx_handles[p], h)?;
            }
            fbx_handles.push(h);
            sim_nodes.push(SimNode {
                name: n["name"].as_str().unwrap_or("").to_string(),
                parent,
                children: vec![],
                position: Vector3::new(p[0], p[1], p[2]),
                quaternion: Quaternion::from_xyzw(q[0], q[1], q[2], q[3]),
                scale: Vector3::new(sc[0], sc[1], sc[2]),
                matrix: Matrix4::IDENTITY,
                world: matrix(&n["matrixWorld"]),
                needs_update: false,
            });
        }
        for i in 0..sim_nodes.len() {
            if let Some(p) = sim_nodes[i].parent {
                sim_nodes[p].children.push(i);
            }
        }
        // sourceModel.position.x −= 0.9 and the centimeter scale.
        {
            let root = s.get_mut(fbx_handles[0])?;
            root.position.x -= 0.9;
            root.scale = Vector3::splat(0.01);
        }
        sim_nodes[0].position.x -= 0.9;
        sim_nodes[0].scale = Vector3::splat(0.01);
        let mut source_sim = Sim { nodes: sim_nodes };
        // The source clip, bound to the FBX scene graph by name.
        // PropertyBinding's depth-first search: the first node of each name
        // ( the FBX nests each bone over a namesake ).
        let mut by_name: HashMap<String, usize> = HashMap::new();
        for (i, n) in source_sim.nodes.iter().enumerate() {
            by_name.entry(n.name.clone()).or_insert(i);
        }
        let mut tracks = vec![];
        for t in baked.json["clip"]["tracks"]
            .as_array()
            .ok_or(Error::Invalid("baked tracks"))?
        {
            let name = t["name"].as_str().unwrap_or("");
            let (node, property) = name.rsplit_once('.').ok_or(Error::Invalid("track name"))?;
            let target = fbx_handles[*by_name.get(node).ok_or(Error::Invalid("track node"))?];
            let (property, size) = match property {
                "position" => (Property::Position, 3),
                "quaternion" => (Property::Rotation, 4),
                "scale" => (Property::Scale, 3),
                _ => return Err(Error::Invalid("track property")),
            };
            let times: Vec<f64> = baked
                .floats(&t["times"])?
                .iter()
                .map(|&v| v as f64)
                .collect();
            let values: Vec<Vec<f64>> = baked
                .floats(&t["values"])?
                .chunks(size)
                .map(|v| v.iter().map(|&x| x as f64).collect())
                .collect();
            tracks.push(Track {
                target,
                property,
                times,
                values,
                interpolation: Interpolation::Linear,
            });
        }
        let clip = Arc::new(Clip {
            name: baked.json["clip"]["name"]
                .as_str()
                .unwrap_or("")
                .to_string(),
            tracks,
        });
        let mut source_mixer = ThreeMixer::new();
        let action = source_mixer.clip_action(s, clip.clone(), false)?;
        source_mixer.play(s, action)?;
        // The target: readyplayer.me, at the page's placement.
        let (asset, buffers, images) =
            load_asset("/web/gallery/assets/gltf/readyplayer.me.glb").await?;
        let instance =
            crate::gltf::import_animated_decoded(&asset, &buffers, &images)?.instantiate(s)?;
        let target_root = s.insert(NodeKind::Group);
        for &h in &instance.roots {
            s.add(target_root, h)?;
        }
        s.get_mut(target_root)?.position.x += 0.9;
        let (mut target_sim, roots) = Sim::from_gltf(&asset)?;
        // targetModel.scene.children[ 0 ].children[ 1 ]: the skinned mesh
        // whose skeleton the clip retargets to.
        let armature = *roots.first().ok_or(Error::Invalid("target root"))?;
        let skin_node = *target_sim.nodes[armature]
            .children
            .get(1)
            .ok_or(Error::Invalid("target skin"))?;
        let skin = asset
            .nodes()
            .nth(skin_node)
            .and_then(|n| n.skin())
            .ok_or(Error::Invalid("target skin"))?;
        let target_bones: Vec<usize> = skin.joints().map(|j| j.index()).collect();
        let inverses: Vec<Matrix4> = skin
            .reader(|b| buffers.get(b.index()).map(Vec::as_slice))
            .read_inverse_bind_matrices()
            .ok_or(Error::Invalid("inverse bind matrices"))?
            .map(|m| Matrix4::from_cols_array(&std::array::from_fn(|i| m[i / 4][i % 4] as f64)))
            .collect();
        // getBoneName: 'mixamorig' + bone.name.
        let names: HashMap<String, String> = target_bones
            .iter()
            .map(|&b| {
                let n = target_sim.nodes[b].name.clone();
                (n.clone(), format!("mixamorig{n}"))
            })
            .collect();
        // SkeletonHelper( sourceModel ).bones: the bones in depth-first order.
        let mut source_bones = vec![];
        fn walk(nodes: &[serde_json::Value], i: usize, sim: &Sim, out: &mut Vec<usize>) {
            if nodes[i]["type"].as_str() == Some("Bone") {
                out.push(i);
            }
            for &c in &sim.nodes[i].children {
                walk(nodes, c, sim, out);
            }
        }
        walk(baked_nodes, 0, &source_sim, &mut source_bones);
        let duration = clip.duration();
        let frames = clip.tracks.iter().map(|t| t.times.len()).max().unwrap_or(1);
        let fps = frames as f64 / duration;
        let mut retarget_mixer = ThreeMixer::new();
        let retarget_action = retarget_mixer.clip_action(s, clip.clone(), false)?;
        retarget_mixer.play(s, retarget_action)?;
        let bone_handles: Vec<(usize, Object3D)> =
            source_bones.iter().map(|&b| (b, fbx_handles[b])).collect();
        let scene = &mut *s;
        let mut sample = |sim: &mut Sim, dt: f64| -> Result<()> {
            let s = &mut *scene;
            retarget_mixer.update(s, dt)?;
            for &(j, h) in &bone_handles {
                let n = s.get(h)?;
                sim.nodes[j].position = n.position;
                sim.nodes[j].quaternion = n.quaternion;
                sim.nodes[j].scale = n.scale;
            }
            Ok(())
        };
        let baked_clip: Vec<BakedBone> = retarget_clip(
            &mut target_sim,
            skin_node,
            &target_bones,
            &inverses,
            &mut source_sim,
            &source_bones,
            &names,
            &HashMap::new(),
            0.01,
            duration,
            fps,
            &mut sample,
        )?;
        // uncacheAction restores the source bones; the target keeps the
        // last retargeted state.
        for &(j, h) in &bone_handles {
            let n = &baked_nodes[j];
            let p = numbers(&n["position"]);
            let q = numbers(&n["quaternion"]);
            let sc = numbers(&n["scale"]);
            let node = s.get_mut(h)?;
            node.position = Vector3::new(p[0], p[1], p[2]);
            node.quaternion = Quaternion::from_xyzw(q[0], q[1], q[2], q[3]);
            node.scale = Vector3::new(sc[0], sc[1], sc[2]);
        }
        for &b in &target_bones {
            let h = instance.nodes[b];
            let sim = &target_sim.nodes[b];
            let n = s.get_mut(h)?;
            n.position = sim.position;
            n.quaternion = sim.quaternion;
            n.scale = sim.scale;
        }
        let mut tracks = vec![];
        for (b, positions, quaternions, times) in baked_clip {
            let target = instance.nodes[b];
            let times: Vec<f64> = times.iter().map(|&t| t as f64).collect();
            if let Some(p) = positions {
                tracks.push(Track {
                    target,
                    property: Property::Position,
                    times: times.clone(),
                    values: p.iter().map(|v| v.map(f64::from).to_vec()).collect(),
                    interpolation: Interpolation::Linear,
                });
            }
            tracks.push(Track {
                target,
                property: Property::Rotation,
                times,
                values: quaternions
                    .iter()
                    .map(|v| v.map(f64::from).to_vec())
                    .collect(),
                interpolation: Interpolation::Linear,
            });
        }
        let retargeted = Arc::new(Clip {
            name: clip.name.clone(),
            tracks,
        });
        let mut target_mixer = ThreeMixer::new();
        let target_action = target_mixer.clip_action(s, retargeted, false)?;
        target_mixer.play(s, target_action)?;
        controls.update(s, c)?;
        // The avatar's textures and samplers, by glTF index.
        let mut mipmaps = Mipmaps::new(r);
        let mut views = vec![];
        let mut texture_index = HashMap::new();
        let mut samplers: Vec<wgpu::Sampler> = vec![];
        let mut sampler_index: HashMap<(bool, bool), usize> = HashMap::new();
        let mut texture = |t: gltf::Texture, srgb: bool| -> Result<(usize, usize)> {
            let key = (t.index(), srgb);
            let view = if let Some(&v) = texture_index.get(&key) {
                v
            } else {
                let image = images
                    .get(t.source().index())
                    .ok_or(Error::Invalid("avatar image"))?;
                let format = if srgb {
                    wgpu::TextureFormat::Rgba8UnormSrgb
                } else {
                    wgpu::TextureFormat::Rgba8Unorm
                };
                views.push(mipmapped(r, &mut mipmaps, image, format));
                texture_index.insert(key, views.len() - 1);
                views.len() - 1
            };
            let wrap =
                |w: gltf::texture::WrappingMode| w != gltf::texture::WrappingMode::ClampToEdge;
            let sk = (wrap(t.sampler().wrap_s()), wrap(t.sampler().wrap_t()));
            let sampler = *sampler_index.entry(sk).or_insert_with(|| {
                let mode = |repeat: bool| {
                    if repeat {
                        wgpu::AddressMode::Repeat
                    } else {
                        wgpu::AddressMode::ClampToEdge
                    }
                };
                samplers.push(r.device.create_sampler(&wgpu::SamplerDescriptor {
                    address_mode_u: mode(sk.0),
                    address_mode_v: mode(sk.1),
                    mag_filter: wgpu::FilterMode::Linear,
                    min_filter: wgpu::FilterMode::Linear,
                    mipmap_filter: wgpu::FilterMode::Linear,
                    ..Default::default()
                }));
                samplers.len() - 1
            });
            Ok((view, sampler))
        };
        let mut meshes = vec![];
        let bone_buffer = |count: usize| {
            r.device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("skeleton"),
                size: (count * 64) as u64,
                usage: uniform_usage,
                mapped_at_creation: false,
            })
        };
        for node in asset.nodes() {
            let (Some(mesh), Some(skin)) = (node.mesh(), node.skin()) else {
                continue;
            };
            let primitive = mesh
                .primitives()
                .next()
                .ok_or(Error::Invalid("avatar primitive"))?;
            let reader = primitive.reader(|b| buffers.get(b.index()).map(Vec::as_slice));
            let positions: Vec<f32> = reader
                .read_positions()
                .ok_or(Error::Invalid("avatar positions"))?
                .flatten()
                .collect();
            let joints: Vec<u32> = reader
                .read_joints(0)
                .ok_or(Error::Invalid("avatar joints"))?
                .into_u16()
                .flat_map(|j| j.map(u32::from))
                .collect();
            // normalizeSkinWeights().
            let weights: Vec<f32> = reader
                .read_weights(0)
                .ok_or(Error::Invalid("avatar weights"))?
                .into_f32()
                .flat_map(|w| {
                    let w = w.map(f64::from);
                    let scale = 1. / w.iter().map(|v| v.abs()).sum::<f64>();
                    if scale.is_finite() {
                        w.map(|v| (v * scale) as f32)
                    } else {
                        [1., 0., 0., 0.]
                    }
                })
                .collect();
            let normals: Vec<f32> = reader
                .read_normals()
                .ok_or(Error::Invalid("avatar normals"))?
                .flatten()
                .collect();
            let uvs: Vec<f32> = reader
                .read_tex_coords(0)
                .ok_or(Error::Invalid("avatar uv"))?
                .into_f32()
                .flatten()
                .collect();
            let tangents: Option<Vec<f32>> = reader.read_tangents().map(|t| t.flatten().collect());
            let index: Vec<u32> = reader
                .read_indices()
                .ok_or(Error::Invalid("avatar index"))?
                .into_u32()
                .collect();
            let morphs: Vec<Vec<f32>> = reader
                .read_morph_targets()
                .map(|(p, _, _)| p.map(|p| p.flatten().collect()).unwrap_or_default())
                .collect();
            let material = primitive.material();
            let pbr = material.pbr_metallic_roughness();
            let name = mesh.name().unwrap_or("");
            let has_normal_map = material.normal_texture().is_some();
            let kind = match (morphs.is_empty(), has_normal_map, tangents.is_some()) {
                (false, true, _) => Kind::Beard,
                (false, false, _) => Kind::Morphed,
                (true, true, false) => Kind::Mapped,
                (true, true, true) if pbr.metallic_roughness_texture().is_some() => Kind::Mapped,
                _ => Kind::Body,
            };
            let shaders = match (kind, name) {
                (Kind::Beard, _) => (wgsl!("beard_vs"), wgsl!("beard_fs")),
                (Kind::Morphed, "Wolf3D_Teeth") => (wgsl!("teeth_vs"), wgsl!("teeth_fs")),
                (Kind::Morphed, "Wolf3D_Head") => (wgsl!("head_vs"), wgsl!("lit_fs")),
                (Kind::Morphed, _) => (wgsl!("eye_vs"), wgsl!("lit_fs")),
                (Kind::Mapped, _) if tangents.is_none() => {
                    (wgsl!("headwear_vs"), wgsl!("headwear_fs"))
                }
                (Kind::Mapped, _) => (wgsl!("outfit_vs"), wgsl!("outfit_fs")),
                _ => (wgsl!("body_vs"), wgsl!("body_fs")),
            };
            let mut attributes = vec![];
            for (location, input) in inputs(shaders.0) {
                let (data, format): (Vec<u8>, _) = match input.as_str() {
                    "position" => (
                        bytemuck::cast_slice(&positions).to_vec(),
                        wgpu::VertexFormat::Float32x3,
                    ),
                    "normal" => (
                        bytemuck::cast_slice(&normals).to_vec(),
                        wgpu::VertexFormat::Float32x3,
                    ),
                    "uv" => (
                        bytemuck::cast_slice(&uvs).to_vec(),
                        wgpu::VertexFormat::Float32x2,
                    ),
                    "tangent" => (
                        bytemuck::cast_slice(
                            tangents
                                .as_deref()
                                .ok_or(Error::Invalid("avatar tangents"))?,
                        )
                        .to_vec(),
                        wgpu::VertexFormat::Float32x4,
                    ),
                    "skinIndex" => (
                        bytemuck::cast_slice(&joints).to_vec(),
                        wgpu::VertexFormat::Uint32x4,
                    ),
                    "skinWeight" => (
                        bytemuck::cast_slice(&weights).to_vec(),
                        wgpu::VertexFormat::Float32x4,
                    ),
                    _ => return Err(Error::Invalid("avatar vertex input")),
                };
                attributes.push((location, init("avatar attribute", &data, vertex), format));
            }
            let morph = (!morphs.is_empty()).then(|| {
                let count = positions.len() / 3;
                let texels: Vec<f32> = morphs
                    .iter()
                    .flat_map(|m| m.chunks(3).flat_map(|p| [p[0], p[1], p[2], 0.]))
                    .collect();
                let texture = r.device.create_texture_with_data(
                    &r.queue,
                    &wgpu::TextureDescriptor {
                        label: Some("morph targets"),
                        size: wgpu::Extent3d {
                            width: count as u32,
                            height: 1,
                            depth_or_array_layers: morphs.len() as u32,
                        },
                        mip_level_count: 1,
                        sample_count: 1,
                        dimension: wgpu::TextureDimension::D2,
                        format: wgpu::TextureFormat::Rgba32Float,
                        usage: wgpu::TextureUsages::TEXTURE_BINDING,
                        view_formats: &[],
                    },
                    wgpu::util::TextureDataOrder::LayerMajor,
                    bytemuck::cast_slice(&texels),
                );
                (
                    init("morph influences", &[0; 32], uniform_usage),
                    texture.create_view(&wgpu::TextureViewDescriptor {
                        dimension: Some(wgpu::TextureViewDimension::D2Array),
                        ..Default::default()
                    }),
                )
            });
            let base = texture(
                pbr.base_color_texture()
                    .ok_or(Error::Invalid("avatar map"))?
                    .texture(),
                true,
            )?;
            let mut maps = vec![];
            match kind {
                Kind::Beard => {
                    maps.push(("nodeUniform7", base.0, base.1));
                    let n = texture(
                        material
                            .normal_texture()
                            .ok_or(Error::Invalid("normal map"))?
                            .texture(),
                        false,
                    )?;
                    maps.push(("nodeUniform18", n.0, n.1));
                }
                Kind::Morphed => maps.push(("nodeUniform7", base.0, base.1)),
                Kind::Mapped => {
                    maps.push(("nodeUniform4", base.0, base.1));
                    let mr = texture(
                        pbr.metallic_roughness_texture()
                            .ok_or(Error::Invalid("metal-roughness map"))?
                            .texture(),
                        false,
                    )?;
                    maps.push(("nodeUniform8", mr.0, mr.1));
                    let n = texture(
                        material
                            .normal_texture()
                            .ok_or(Error::Invalid("normal map"))?
                            .texture(),
                        false,
                    )?;
                    maps.push(("nodeUniform18", n.0, n.1));
                }
                _ => {
                    maps.push(("nodeUniform4", base.0, base.1));
                    let n = texture(
                        material
                            .normal_texture()
                            .ok_or(Error::Invalid("normal map"))?
                            .texture(),
                        false,
                    )?;
                    maps.push(("nodeUniform15", n.0, n.1));
                }
            }
            let bones: Vec<(Object3D, Matrix4)> = skin
                .joints()
                .map(|j| instance.nodes[j.index()])
                .zip(inverses.iter().copied())
                .collect();
            let object = super::retro::uniform(r, "avatar object", shaders.1, "objectStruct")
                .or_else(|_| {
                    super::retro::uniform(r, "avatar object", shaders.0, "objectStruct")
                })?;
            let color = pbr.base_color_factor();
            meshes.push(Mesh {
                kind,
                shaders,
                double_sided: material.double_sided(),
                attributes,
                count: index.len() as u32,
                index: Some(init(
                    "avatar index",
                    bytemuck::cast_slice(&index),
                    index_usage,
                )),
                node: instance.nodes[node.index()],
                bone_buffer: bone_buffer(bones.len()),
                bones,
                morph,
                maps,
                object,
                material: (
                    [color[0] as f64, color[1] as f64, color[2] as f64],
                    pbr.metallic_factor() as f64,
                    pbr.roughness_factor() as f64,
                    // GLTFLoader flips the normal scale's y without tangents.
                    if tangents.is_some() { 1. } else { -1. },
                    [0.; 3],
                ),
                center: geometry_center(&positions, &morphs),
                positions,
                joints,
                weights,
                sphere: None,
            });
        }
        // The FBX character's two skinned meshes.
        for m in baked.json["meshes"]
            .as_array()
            .ok_or(Error::Invalid("baked meshes"))?
        {
            let a = &m["attributes"];
            let positions = baked.floats(&a["position"])?;
            let joints = baked.u16s(&a["skinIndex"])?;
            let weights = baked.floats(&a["skinWeight"])?;
            let normals = baked.floats(&a["normal"])?;
            let node = m["node"]
                .as_u64()
                .ok_or(Error::Invalid("baked mesh node"))? as usize;
            let surface = baked_nodes[node]["name"].as_str() == Some("Beta_Surface");
            let shaders = if surface {
                (wgsl!("surface_vs"), wgsl!("phong_fs"))
            } else {
                (wgsl!("joints_vs"), wgsl!("phong_fs"))
            };
            let mut attributes = vec![];
            for (location, input) in inputs(shaders.0) {
                let (data, format): (Vec<u8>, _) = match input.as_str() {
                    "position" => (
                        bytemuck::cast_slice(&positions).to_vec(),
                        wgpu::VertexFormat::Float32x3,
                    ),
                    "normal" => (
                        bytemuck::cast_slice(&normals).to_vec(),
                        wgpu::VertexFormat::Float32x3,
                    ),
                    "skinIndex" => (
                        bytemuck::cast_slice(&joints).to_vec(),
                        wgpu::VertexFormat::Uint32x4,
                    ),
                    "skinWeight" => (
                        bytemuck::cast_slice(&weights).to_vec(),
                        wgpu::VertexFormat::Float32x4,
                    ),
                    _ => return Err(Error::Invalid("character vertex input")),
                };
                attributes.push((location, init("character attribute", &data, vertex), format));
            }
            let bones: Vec<(Object3D, Matrix4)> = m["bones"]
                .as_array()
                .ok_or(Error::Invalid("baked bones"))?
                .iter()
                .zip(
                    m["boneInverses"]
                        .as_array()
                        .ok_or(Error::Invalid("baked inverses"))?,
                )
                .map(|(b, inverse)| {
                    Ok((
                        fbx_handles[b.as_u64().ok_or(Error::Invalid("baked bone"))? as usize],
                        matrix(inverse),
                    ))
                })
                .collect::<Result<_>>()?;
            let material = &m["material"];
            let color = numbers(&material["color"]);
            let specular = numbers(&material["specular"]);
            meshes.push(Mesh {
                kind: Kind::Phong,
                shaders,
                double_sided: false,
                attributes,
                index: None,
                count: (positions.len() / 3) as u32,
                node: fbx_handles[node],
                bone_buffer: bone_buffer(bones.len()),
                bones,
                morph: None,
                maps: vec![],
                object: super::retro::uniform(r, "character object", shaders.1, "objectStruct")?,
                material: (
                    [color[0], color[1], color[2]],
                    0.,
                    material["shininess"].as_f64().unwrap_or(30.),
                    1.,
                    [specular[0], specular[1], specular[2]],
                ),
                center: geometry_center(&positions, &[]),
                positions,
                joints,
                weights,
                sphere: None,
            });
        }
        let read = |g: &BufferGeometry, name: &str| -> Result<Vec<f32>> {
            let a = g.attributes.get(name).ok_or(Error::Invalid("attribute"))?;
            (0..a.count())
                .flat_map(|i| (0..3).map(move |k| (i, k)))
                .map(|(i, k)| a.get_component(i, k).map(|v| v as f32))
                .collect()
        };
        let sphere = SphereGeometry::build(1., 32, 32)?;
        let sphere_index = sphere.index.clone().ok_or(Error::Invalid("sphere index"))?;
        let background = (
            init(
                "background normals",
                bytemuck::cast_slice(&read(&sphere, "normal")?),
                vertex,
            ),
            init(
                "background positions",
                bytemuck::cast_slice(&read(&sphere, "position")?),
                vertex,
            ),
            init(
                "background index",
                bytemuck::cast_slice(&sphere_index),
                index_usage,
            ),
            sphere_index.len() as u32,
        );
        let floor_box = BoxGeometry::build(50., 0.001, 50.)?;
        let floor_index = floor_box
            .index
            .clone()
            .ok_or(Error::Invalid("floor index"))?;
        let floor = (
            init(
                "floor positions",
                bytemuck::cast_slice(&read(&floor_box, "position")?),
                vertex,
            ),
            init(
                "floor index",
                bytemuck::cast_slice(&floor_index),
                index_usage,
            ),
            floor_index.len() as u32,
        );
        let uniform = |label, source, name| super::retro::uniform(r, label, source, name);
        let pass_renders = || -> Result<[wgpu::Buffer; 2]> {
            Ok([
                uniform("background render", BACKGROUND.1, "renderStruct")?,
                uniform("lit render", wgsl!("beard_fs"), "renderStruct")?,
            ])
        };
        Ok(Self {
            controls,
            time: 0.,
            last: 0.,
            pending: true,
            source_mixer,
            target_mixer,
            meshes,
            views,
            samplers,
            background,
            floor,
            renders: [pass_renders()?, pass_renders()?],
            background_object: uniform("background object", BACKGROUND.1, "objectStruct")?,
            floor_render: uniform("floor render", FLOOR.1, "renderStruct")?,
            floor_object: uniform("floor object", FLOOR.1, "objectStruct")?,
            output_render: uniform("output render", OUTPUT.1, "renderStruct")?,
            output_object: uniform("output object", OUTPUT.0, "objectStruct")?,
            quad: init(
                "output quad",
                bytemuck::cast_slice(&[-1f32, 3., 0., -1., -1., 0., 3., -1., 0.]),
                vertex,
            ),
            clamp: r.device.create_sampler(&wgpu::SamplerDescriptor {
                mag_filter: wgpu::FilterMode::Linear,
                min_filter: wgpu::FilterMode::Linear,
                ..Default::default()
            }),
            targets: None,
        })
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, dt: f64, animate: bool) -> Result<()> {
        if animate {
            self.time += dt;
            self.pending = true;
        }
        Ok(())
    }
    pub fn prepare(&mut self, _s: &mut Scene, _c: Object3D, _r: &Renderer) -> Result<()> {
        Ok(())
    }
    fn resize(&mut self, r: &Renderer, out: &RenderTarget, samples: u32) -> Result<()> {
        let (width, height) = (out.width, out.height);
        let texture = |format, samples| {
            r.device
                .create_texture(&wgpu::TextureDescriptor {
                    label: Some("readyplayer target"),
                    size: wgpu::Extent3d {
                        width,
                        height,
                        depth_or_array_layers: 1,
                    },
                    mip_level_count: 1,
                    sample_count: samples,
                    dimension: wgpu::TextureDimension::D2,
                    format,
                    usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                        | wgpu::TextureUsages::TEXTURE_BINDING,
                    view_formats: &[],
                })
                .create_view(&Default::default())
        };
        let color = texture(HALF, 1);
        let depth = texture(DEPTH, samples);
        let msaa = (samples > 1).then(|| (texture(HALF, samples), texture(DEPTH, samples)));
        let reflection = (texture(HALF, 1), texture(DEPTH, 1));
        let tex = wgpu::BindingResource::TextureView;
        let sampler = wgpu::BindingResource::Sampler;
        let bg_attrs = [
            wgpu::vertex_attr_array![0 => Float32x3],
            wgpu::vertex_attr_array![1 => Float32x3],
        ];
        let bg_layouts = [0, 1].map(|i| wgpu::VertexBufferLayout {
            array_stride: 12,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &bg_attrs[i],
        });
        let mut draws: [Vec<Draw>; 2] = [vec![], vec![]];
        for (pass, pass_samples) in [(0, samples), (1, 1)] {
            let render = &self.renders[pass];
            let bg = Spec {
                label: "readyplayer background",
                shaders: BACKGROUND,
                buffers: &bg_layouts,
                format: HALF,
                blend: false,
                depth: Some((DEPTH, wgpu::CompareFunction::Always, false, (0, 0.))),
                cw: true,
                cull: Some(wgpu::Face::Back),
                samples: pass_samples,
            }
            .build(r);
            let bg_groups = groups(
                r,
                |g| bg.get_bind_group_layout(g),
                &[BACKGROUND.0, BACKGROUND.1],
                |name| match name {
                    "render" => Some(render[0].as_entire_binding()),
                    "object" => Some(self.background_object.as_entire_binding()),
                    _ => None,
                },
            )?;
            draws[pass].push((bg, bg_groups));
            for mesh in &self.meshes {
                let attrs: Vec<[wgpu::VertexAttribute; 1]> = mesh
                    .attributes
                    .iter()
                    .map(|(location, _, format)| {
                        [wgpu::VertexAttribute {
                            format: *format,
                            offset: 0,
                            shader_location: *location,
                        }]
                    })
                    .collect();
                let layouts: Vec<wgpu::VertexBufferLayout> = mesh
                    .attributes
                    .iter()
                    .zip(&attrs)
                    .map(|((_, _, format), a)| wgpu::VertexBufferLayout {
                        array_stride: format.size(),
                        step_mode: wgpu::VertexStepMode::Vertex,
                        attributes: a,
                    })
                    .collect();
                let pipeline = Spec {
                    label: "readyplayer mesh",
                    shaders: mesh.shaders,
                    buffers: &layouts,
                    format: HALF,
                    blend: false,
                    depth: Some((DEPTH, wgpu::CompareFunction::LessEqual, true, (0, 0.))),
                    cw: false,
                    cull: (!mesh.double_sided).then_some(wgpu::Face::Back),
                    samples: pass_samples,
                }
                .build(r);
                let bind = groups(
                    r,
                    |g| pipeline.get_bind_group_layout(g),
                    &[mesh.shaders.0, mesh.shaders.1],
                    |name| {
                        if name == "render" {
                            return Some(render[1].as_entire_binding());
                        }
                        if name == "object" {
                            return Some(mesh.object.as_entire_binding());
                        }
                        if name.starts_with("NodeBuffer_") {
                            // The bone matrices or the morph influences, by type.
                            let decl = format!("struct {name}Struct {{");
                            let vs = mesh.shaders.0;
                            let bones = vs.find(&decl).is_some_and(|at| {
                                let rest = &vs[at..];
                                rest[..rest.find("};").unwrap_or(rest.len())].contains("mat4x4")
                            });
                            return if bones {
                                Some(mesh.bone_buffer.as_entire_binding())
                            } else {
                                mesh.morph.as_ref().map(|(b, _)| b.as_entire_binding())
                            };
                        }
                        if name == "nodeUniform2" {
                            return mesh.morph.as_ref().map(|(_, v)| tex(v));
                        }
                        let (base, is_sampler) = match name.strip_suffix("_sampler") {
                            Some(b) => (b, true),
                            None => (name, false),
                        };
                        if let Some((_, view, s)) = mesh.maps.iter().find(|(n, _, _)| *n == base) {
                            return Some(if is_sampler {
                                sampler(&self.samplers[*s])
                            } else {
                                tex(&self.views[*view])
                            });
                        }
                        // The remaining texture is the DFG table.
                        Some(if is_sampler {
                            sampler(&self.clamp)
                        } else {
                            tex(&r.dfg)
                        })
                    },
                )?;
                draws[pass].push((pipeline, bind));
            }
        }
        let floor_attrs = [wgpu::vertex_attr_array![0 => Float32x3]];
        let floor_layout = [wgpu::VertexBufferLayout {
            array_stride: 12,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &floor_attrs[0],
        }];
        let floor_pipeline = Spec {
            label: "readyplayer floor",
            shaders: FLOOR,
            buffers: &floor_layout,
            format: HALF,
            blend: true,
            depth: Some((DEPTH, wgpu::CompareFunction::LessEqual, true, (0, 0.))),
            cw: false,
            cull: Some(wgpu::Face::Back),
            samples,
        }
        .build(r);
        let floor_groups = groups(
            r,
            |g| floor_pipeline.get_bind_group_layout(g),
            &[FLOOR.0, FLOOR.1],
            |name| match name {
                "render" => Some(self.floor_render.as_entire_binding()),
                "object" => Some(self.floor_object.as_entire_binding()),
                n if n.ends_with("_sampler") => Some(sampler(&self.clamp)),
                _ => Some(tex(&reflection.0)),
            },
        )?;
        let format = out.options.format;
        let output_pipeline = Spec {
            label: "readyplayer output",
            shaders: OUTPUT,
            buffers: &floor_layout,
            format,
            blend: false,
            depth: None,
            cw: false,
            cull: Some(wgpu::Face::Back),
            samples: 1,
        }
        .build(r);
        let output_groups = groups(
            r,
            |g| output_pipeline.get_bind_group_layout(g),
            &[OUTPUT.0, OUTPUT.1],
            |name| match name {
                "render" => Some(self.output_render.as_entire_binding()),
                "object" => Some(self.output_object.as_entire_binding()),
                n if n.ends_with("_sampler") => Some(sampler(&self.clamp)),
                _ => Some(tex(&color)),
            },
        )?;
        let screen = RenderTarget::with_options(
            &r.device,
            width,
            height,
            RenderTargetOptions {
                samples: 0,
                depth_buffer: false,
                ..out.options.clone()
            },
        )?;
        self.targets = Some(Targets {
            width,
            height,
            format,
            samples,
            msaa,
            color,
            depth,
            reflection,
            draws,
            floor: (floor_pipeline, floor_groups),
            output: (output_pipeline, output_groups),
            screen,
        });
        Ok(())
    }
    fn present(&self, encoder: &mut wgpu::CommandEncoder, t: &Targets) {
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("readyplayer output"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &t.screen.view,
                depth_slice: None,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                    store: wgpu::StoreOp::Store,
                },
            })],
            ..Default::default()
        });
        set(&mut pass, &t.output);
        pass.set_vertex_buffer(0, self.quad.slice(..));
        pass.draw(0..3, 0..1);
    }
    /// A mesh's object struct for its world matrix.
    fn object_values(mesh: &Mesh, world: Matrix4) -> Vec<(&'static str, Vec<f64>)> {
        let id3 = m3(Matrix4::IDENTITY);
        let (color, metalness, roughness, normal_y, specular) = mesh.material;
        let normal = m3(world.inverse().transpose());
        let bind_inverse = m4(world.inverse());
        let identity = m4(Matrix4::IDENTITY);
        match mesh.kind {
            Kind::Beard | Kind::Morphed => {
                let mut v = vec![
                    ("nodeUniform0", vec![1.]),
                    ("nodeUniform3", bind_inverse),
                    ("nodeUniform5", identity),
                    ("nodeUniform6", color.to_vec()),
                    ("nodeUniform8", id3.clone()),
                    ("nodeUniform9", vec![1.]),
                    ("nodeUniform10", vec![metalness]),
                    ("nodeUniform11", vec![roughness]),
                    ("nodeUniform13", normal),
                    ("nodeUniform14", vec![0.; 3]),
                    ("nodeUniform15", vec![1.]),
                    ("nodeUniform17", m4(world)),
                ];
                if mesh.kind == Kind::Beard {
                    v.push(("nodeUniform19", id3));
                    v.push(("nodeUniform20", vec![1., normal_y]));
                }
                v
            }
            Kind::Mapped => vec![
                ("nodeUniform0", bind_inverse),
                ("nodeUniform2", identity),
                ("nodeUniform3", color.to_vec()),
                ("nodeUniform5", id3.clone()),
                ("nodeUniform6", vec![1.]),
                ("nodeUniform7", vec![metalness]),
                ("nodeUniform9", id3.clone()),
                ("nodeUniform10", vec![roughness]),
                ("nodeUniform11", id3.clone()),
                ("nodeUniform13", normal),
                ("nodeUniform14", vec![0.; 3]),
                ("nodeUniform15", vec![1.]),
                ("nodeUniform17", m4(world)),
                ("nodeUniform19", id3),
                ("nodeUniform20", vec![1., normal_y]),
            ],
            Kind::Body => vec![
                ("nodeUniform0", bind_inverse),
                ("nodeUniform2", identity),
                ("nodeUniform3", color.to_vec()),
                ("nodeUniform5", id3.clone()),
                ("nodeUniform6", vec![1.]),
                ("nodeUniform7", vec![metalness]),
                ("nodeUniform8", vec![roughness]),
                ("nodeUniform10", normal),
                ("nodeUniform11", vec![0.; 3]),
                ("nodeUniform12", vec![1.]),
                ("nodeUniform14", m4(world)),
                ("nodeUniform16", id3),
                ("nodeUniform17", vec![1., normal_y]),
            ],
            Kind::Phong => vec![
                ("nodeUniform0", bind_inverse),
                ("nodeUniform2", identity),
                ("nodeUniform3", color.to_vec()),
                ("nodeUniform4", vec![1.]),
                ("nodeUniform5", vec![roughness]),
                ("nodeUniform6", specular.to_vec()),
                ("nodeUniform7", vec![0.; 3]),
                ("nodeUniform8", vec![1.]),
                ("nodeUniform12", normal),
                ("nodeUniform18", m4(world)),
            ],
        }
    }
    pub fn render(
        &mut self,
        r: &Renderer,
        s: &mut Scene,
        c: Object3D,
        out: &RenderTarget,
    ) -> Result<bool> {
        let samples = if out.options.samples > 1 { 4 } else { 1 };
        if self.targets.as_ref().is_none_or(|t| {
            t.width != out.width
                || t.height != out.height
                || t.format != out.options.format
                || t.samples != samples
        }) {
            self.resize(r, out, samples)?;
        }
        let t = self
            .targets
            .as_ref()
            .ok_or(Error::Invalid("readyplayer targets"))?;
        if !std::mem::take(&mut self.pending) {
            let mut encoder = r.device.create_command_encoder(&Default::default());
            self.present(&mut encoder, t);
            r.queue.submit([encoder.finish()]);
            return Ok(true);
        }
        // animate(): both mixers by the timer's delta, then the controls.
        let delta = (self.time - self.last).max(0.);
        self.last = self.time;
        self.source_mixer.update(s, delta)?;
        self.target_mixer.update(s, delta)?;
        self.controls.frame_update(s, c)?;
        s.update()?;
        let (camera, world) = s.camera(c)?;
        let (projection, view) = (camera.projection_matrix()?, world.inverse());
        let (virtual_view, virtual_projection) = reflector_camera(world, projection);
        let write = |buffer: &wgpu::Buffer,
                     source: &str,
                     name: &str,
                     values: &[(&str, Vec<f64>)]|
         -> Result<()> {
            let values: Vec<(&str, &[f64])> = values.iter().map(|(n, v)| (*n, &v[..])).collect();
            r.queue
                .write_buffer(buffer, 0, &pack(source, name, &values)?);
            Ok(())
        };
        let size = vec![t.width as f64, t.height as f64];
        for (pass, (p, v)) in [(projection, view), (virtual_projection, virtual_view)]
            .into_iter()
            .enumerate()
        {
            let camera_values = [
                ("cameraProjectionMatrix", m4(p)),
                ("cameraViewMatrix", m4(v)),
            ];
            let mut bg = vec![("nodeUniform1", vec![1.]), ("nodeUniform0", size.clone())];
            bg.extend(camera_values.clone());
            write(&self.renders[pass][0], BACKGROUND.1, "renderStruct", &bg)?;
            // The hemisphere ( sky, up, ground ), the back light and the key
            // light ( color, position, target ).
            let mut lit = camera_values.to_vec();
            lit.extend([
                ("nodeUniform22", linear(0x311649, 10.)),
                ("nodeUniform24", vec![0., 1., 0.]),
                ("nodeUniform21", linear(0x0c5d68, 10.)),
                ("nodeUniform27", linear(0xffffff, 10.)),
                ("nodeUniform30", linear(0xfff9ea, 4.)),
                ("nodeUniform25", vec![0., 5., -5.]),
                ("nodeUniform26", vec![0.; 3]),
                ("nodeUniform28", vec![3., 5., 3.]),
                ("nodeUniform29", vec![0.; 3]),
            ]);
            write(
                &self.renders[pass][1],
                wgsl!("beard_fs"),
                "renderStruct",
                &lit,
            )?;
        }
        write(
            &self.background_object,
            BACKGROUND.1,
            "objectStruct",
            &[
                ("nodeUniform2", vec![1.]),
                ("nodeUniform5", m4(Matrix4::IDENTITY)),
            ],
        )?;
        for mesh in self.meshes.iter_mut() {
            let mesh_world = s.get(mesh.node)?.matrix_world;
            let matrices: Vec<Matrix4> = mesh
                .bones
                .iter()
                .map(|(h, inverse)| Ok(s.get(*h)?.matrix_world * *inverse))
                .collect::<Result<_>>()?;
            let bones: Vec<f32> = matrices
                .iter()
                .flat_map(|m| m.to_cols_array().map(|v| v as f32))
                .collect();
            r.queue
                .write_buffer(&mesh.bone_buffer, 0, bytemuck::cast_slice(&bones));
            if mesh.sphere.is_none() {
                mesh.sphere = Some(skinned_sphere(
                    &mesh.positions,
                    &mesh.joints,
                    &mesh.weights,
                    &matrices,
                    mesh_world.inverse(),
                )?);
            }
            let source = if mesh.shaders.1.contains("struct objectStruct {") {
                mesh.shaders.1
            } else {
                mesh.shaders.0
            };
            write(
                &mesh.object,
                source,
                "objectStruct",
                &Self::object_values(mesh, mesh_world),
            )?;
        }
        write(
            &self.floor_render,
            FLOOR.1,
            "renderStruct",
            &[
                ("cameraProjectionMatrix", m4(projection)),
                ("cameraViewMatrix", m4(view)),
                ("nodeUniform1", size.clone()),
            ],
        )?;
        write(
            &self.floor_object,
            FLOOR.1,
            "objectStruct",
            &[
                ("nodeUniform2", m4(Matrix4::IDENTITY)),
                ("nodeUniform3", vec![0.2]),
            ],
        )?;
        write(
            &self.output_render,
            OUTPUT.1,
            "renderStruct",
            &[
                (
                    "cameraProjectionMatrix",
                    vec![
                        1., 0., 0., 0., 0., 1., 0., 0., 0., 0., -1., 0., 0., 0., 0., 1.,
                    ],
                ),
                ("cameraViewMatrix", m4(Matrix4::IDENTITY)),
                ("nodeUniform1", size),
                ("nodeUniform2", vec![1.]),
            ],
        )?;
        write(
            &self.output_object,
            OUTPUT.0,
            "objectStruct",
            &[("nodeUniform5", m4(Matrix4::IDENTITY))],
        )?;
        // Frustum culling with the first-render spheres, then r186's opaque
        // order: the geometry sphere centers' clip-space z, ties by creation.
        let order = |p: Matrix4, v: Matrix4| -> Result<Vec<usize>> {
            let frustum = Frustum::from_projection(p * v);
            let mut items = vec![];
            for (k, mesh) in self.meshes.iter().enumerate() {
                let sphere = mesh.sphere.ok_or(Error::Invalid("skinned sphere"))?;
                let m = s.get(mesh.node)?.matrix_world;
                let (scale, _, _) = m.to_scale_rotation_translation();
                if frustum.intersects_sphere(Sphere {
                    center: m.transform_point3(sphere.center),
                    radius: sphere.radius * scale.abs().max_element(),
                }) {
                    let z = (p * v * m * mesh.center.extend(1.)).z;
                    items.push((z, k));
                }
            }
            items.sort_by(|a, b| a.0.total_cmp(&b.0).then(a.1.cmp(&b.1)));
            Ok(items.into_iter().map(|(_, k)| k).collect())
        };
        let main_order = order(projection, view)?;
        let mirror_order = order(virtual_projection, virtual_view)?;
        let mut encoder = r.device.create_command_encoder(&Default::default());
        let scene_pass = |encoder: &mut wgpu::CommandEncoder,
                          target: &wgpu::TextureView,
                          resolve: Option<&wgpu::TextureView>,
                          depth: &wgpu::TextureView,
                          draws: &[Draw],
                          order: &[usize],
                          floor: Option<&Draw>| {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("readyplayer scene"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: target,
                    depth_slice: None,
                    resolve_target: resolve,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: depth,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                ..Default::default()
            });
            set(&mut pass, &draws[0]);
            pass.set_vertex_buffer(0, self.background.0.slice(..));
            pass.set_vertex_buffer(1, self.background.1.slice(..));
            pass.set_index_buffer(self.background.2.slice(..), wgpu::IndexFormat::Uint32);
            pass.draw_indexed(0..self.background.3, 0, 0..1);
            for &k in order {
                let mesh = &self.meshes[k];
                set(&mut pass, &draws[k + 1]);
                for (slot, (_, buffer, _)) in mesh.attributes.iter().enumerate() {
                    pass.set_vertex_buffer(slot as u32, buffer.slice(..));
                }
                match &mesh.index {
                    Some(index) => {
                        pass.set_index_buffer(index.slice(..), wgpu::IndexFormat::Uint32);
                        pass.draw_indexed(0..mesh.count, 0, 0..1);
                    }
                    None => pass.draw(0..mesh.count, 0..1),
                }
            }
            if let Some(floor) = floor {
                set(&mut pass, floor);
                pass.set_vertex_buffer(0, self.floor.0.slice(..));
                pass.set_index_buffer(self.floor.1.slice(..), wgpu::IndexFormat::Uint32);
                pass.draw_indexed(0..self.floor.2, 0, 0..1);
            }
        };
        // The reflector renders first ( without the floor ), then the view.
        scene_pass(
            &mut encoder,
            &t.reflection.0,
            None,
            &t.reflection.1,
            &t.draws[1],
            &mirror_order,
            None,
        );
        let (target, resolve, depth) = match &t.msaa {
            Some((color, depth)) => (color, Some(&t.color), depth),
            None => (&t.color, None, &t.depth),
        };
        scene_pass(
            &mut encoder,
            target,
            resolve,
            depth,
            &t.draws[0],
            &main_order,
            Some(&t.floor),
        );
        self.present(&mut encoder, t);
        r.queue.submit([encoder.finish()]);
        Ok(true)
    }
    pub fn output(&self) -> Option<&RenderTarget> {
        self.targets.as_ref().map(|t| &t.screen)
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
        wheel: f64,
        pan: bool,
        height: f64,
    ) -> Result<()> {
        let camera = camera_state(s, c)?;
        if wheel != 0. {
            self.controls.dolly(wheel, &camera, Vector2::ZERO);
        } else if pan {
            self.controls.pan(&camera, dx, dy, height);
        } else {
            self.controls.rotate(dx, dy, height);
        }
        Ok(())
    }
    pub fn parameter(&mut self, _index: usize, _value: f32) -> Result<()> {
        Err(Error::Invalid("readyplayer parameter"))
    }
    pub fn seek(&mut self, t: f64) {
        self.time = t;
        self.pending = true;
    }
}
fn set(pass: &mut wgpu::RenderPass, (pipeline, groups): &Draw) {
    pass.set_pipeline(pipeline);
    for (i, g) in groups.iter().enumerate() {
        pass.set_bind_group(i as u32, g, &[]);
    }
}
