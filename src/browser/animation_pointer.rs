//! webgl_loader_gltf_animation_pointer: Khronos's DragonAttenuation ( the
//! Stanford dragon in a transmissive, iridescent, attenuating volume
//! material on Adobe's cloth backdrop ) with a KHR_animation_pointer clip
//! that animates the dragon's material, its node and the cloth texture's
//! transform, under RoomEnvironment with damped OrbitControls. As on the
//! page, the Draco and KTX2 file loads from Needle Cloud.
//!
//! GLTFAnimationPointerExtension turns each pointer into a three.js track:
//! node translation and rotation become node channels, the material values
//! become keyframe tracks on the material ( baseColorFactor into color and
//! opacity, alphaCutoff into alphaTest, the texture transform into the map's
//! repeat and offset ), interpolated linearly as LinearInterpolant does.
use super::controls_attributes::{Controls, camera_state};
use super::gltf_viewer::{fetch, load_asset_bytes};
use crate::animation::AnimationMixer;
use crate::{Error, Result, camera::*, material::*, math::*, renderer::*, scene::*};
use serde_json::{Value, json};
use std::borrow::Cow;
use std::f64::consts::PI;
use std::sync::Arc;

const URL: &str = "https://cloud.needle.tools/-/assets/Z23hmXB27L6Db-optimized/file";

/// A material track: the pointer's target, its key times and values.
struct Track {
    material: usize,
    property: String,
    times: Vec<f64>,
    values: Vec<f64>,
    size: usize,
}
impl Track {
    /// LinearInterpolant: the first or last key outside the range.
    fn sample(&self, t: f64) -> Vec<f64> {
        let n = self.times.len();
        let at = |i: usize| self.values[i * self.size..(i + 1) * self.size].to_vec();
        if t <= self.times[0] {
            return at(0);
        }
        if t >= self.times[n - 1] {
            return at(n - 1);
        }
        let k = self.times.partition_point(|&x| x <= t).max(1);
        let (t0, t1) = (self.times[k - 1], self.times[k]);
        let a = (t - t0) / (t1 - t0);
        let (v0, v1) = (at(k - 1), at(k));
        v0.iter()
            .zip(&v1)
            .map(|(p, q)| p * (1. - a) + q * a)
            .collect()
    }
}
/// The pointer channels out of the document: node translation, rotation and
/// scale become ordinary channels; the rest are returned with their sampler.
/// A pointer and its animation sampler.
type Pointer = (String, Value);
fn split_pointers(bytes: &[u8]) -> Result<(Vec<u8>, Vec<Pointer>)> {
    let glb = gltf::binary::Glb::from_slice(bytes).map_err(|e| Error::Asset(e.to_string()))?;
    let mut document: Value =
        serde_json::from_slice(&glb.json).map_err(|e| Error::Asset(e.to_string()))?;
    let mut pointers = vec![];
    if let Some(animations) = document["animations"].as_array_mut() {
        for animation in animations {
            let samplers = animation["samplers"].clone();
            let Some(channels) = animation["channels"].as_array_mut() else {
                continue;
            };
            let mut kept = vec![];
            for mut channel in channels.drain(..) {
                let Some(pointer) =
                    channel["target"]["extensions"]["KHR_animation_pointer"]["pointer"]
                        .as_str()
                        .map(str::to_owned)
                else {
                    kept.push(channel);
                    continue;
                };
                let parts: Vec<&str> = pointer.trim_start_matches('/').split('/').collect();
                match parts.as_slice() {
                    ["nodes", node, path @ ("translation" | "rotation" | "scale")] => {
                        let node: u64 = node.parse().map_err(|_| Error::Invalid("pointer node"))?;
                        channel["target"] = json!({ "node": node, "path": path });
                        kept.push(channel);
                    }
                    _ => {
                        let sampler =
                            samplers[channel["sampler"].as_u64().unwrap_or(0) as usize].clone();
                        pointers.push((pointer, sampler));
                    }
                }
            }
            *channels = kept;
        }
    }
    for key in ["extensionsUsed", "extensionsRequired"] {
        if let Some(list) = document[key].as_array_mut() {
            list.retain(|v| v.as_str() != Some("KHR_animation_pointer"));
        }
    }
    let json = serde_json::to_vec(&document).map_err(|e| Error::Asset(e.to_string()))?;
    let out = gltf::binary::Glb {
        header: glb.header,
        json: Cow::Owned(json),
        bin: glb.bin.clone(),
    }
    .to_vec()
    .map_err(|e| Error::Asset(e.to_string()))?;
    Ok((out, pointers))
}
/// A float accessor's values, as GLTFParser reads them.
fn accessor(asset: &gltf::Gltf, buffers: &[Vec<u8>], index: usize) -> Result<(Vec<f64>, usize)> {
    let a = asset
        .accessors()
        .nth(index)
        .ok_or(Error::Invalid("pointer accessor"))?;
    if a.data_type() != gltf::accessor::DataType::F32 || a.normalized() || a.sparse().is_some() {
        return Err(Error::Invalid("pointer accessor type"));
    }
    let size = a.dimensions().multiplicity();
    let view = a.view().ok_or(Error::Invalid("pointer accessor view"))?;
    let data = &buffers[view.buffer().index()];
    let stride = view.stride().unwrap_or(size * 4);
    let start = view.offset() + a.offset();
    let mut values = Vec::with_capacity(a.count() * size);
    for i in 0..a.count() {
        for c in 0..size {
            let o = start + i * stride + c * 4;
            let bytes = data
                .get(o..o + 4)
                .ok_or(Error::Invalid("pointer accessor range"))?;
            values.push(f64::from(f32::from_le_bytes(bytes.try_into().unwrap())));
        }
    }
    Ok((values, size))
}
pub(super) struct Demo {
    controls: Controls,
    mixer: AnimationMixer,
    tracks: Vec<Track>,
    /// The meshes of the backdrop and the dragon ( glTF materials 0 and 1 ).
    meshes: [Object3D; 2],
    duration: f64,
    time: f64,
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, _id: u32, r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 40.,
            near: 0.2,
            far: 100.,
            aspect,
            ..Default::default()
        }));
        s.get_mut(c)?.position = Vector3::new(-3., 2., 6.);
        s.background = Color::from_hex(0xbfe3dd);
        s.environment = Some(super::room_environment::environment(r)?);
        let mut controls = Controls::new(Some(0.05), (0., f64::INFINITY), PI, true);
        controls.set_target(Vector3::new(0., 0.5, 0.));
        controls.update(s, c)?;
        let (bytes, pointers) = split_pointers(&fetch(URL).await?)?;
        let (asset, buffers, images) =
            load_asset_bytes(&bytes, "https://cloud.needle.tools/-/assets").await?;
        let mut tracks = vec![];
        for (pointer, sampler) in pointers {
            if sampler["interpolation"].as_str().unwrap_or("LINEAR") != "LINEAR" {
                return Err(Error::Invalid("pointer interpolation"));
            }
            let (times, _) = accessor(
                &asset,
                &buffers,
                sampler["input"].as_u64().unwrap_or(0) as usize,
            )?;
            let (values, size) = accessor(
                &asset,
                &buffers,
                sampler["output"].as_u64().unwrap_or(0) as usize,
            )?;
            let parts: Vec<&str> = pointer.trim_start_matches('/').splitn(3, '/').collect();
            let ["materials", material, property] = parts.as_slice() else {
                return Err(Error::Invalid("pointer target"));
            };
            tracks.push(Track {
                material: material
                    .parse()
                    .map_err(|_| Error::Invalid("pointer material"))?,
                property: (*property).to_owned(),
                times,
                values,
                size,
            });
        }
        let instance =
            crate::gltf::import_animated_decoded(&asset, &buffers, &images)?.instantiate(s)?;
        // The meshes under the Cloth Backdrop ( 1 ) and Dragon ( 2 ) nodes.
        let mesh_under = |s: &Scene, node: usize| -> Result<Object3D> {
            let n = instance.nodes[node];
            if matches!(s.get(n)?.kind, NodeKind::Mesh(_)) {
                return Ok(n);
            }
            s.get(n)?
                .children()
                .iter()
                .copied()
                .find(|&h| matches!(s.get(h).map(|n| &n.kind), Ok(NodeKind::Mesh(_))))
                .ok_or(Error::Invalid("pointer mesh"))
        };
        let meshes = [mesh_under(s, 1)?, mesh_under(s, 2)?];
        let mut mixer = AnimationMixer::default();
        let clip = instance
            .clips
            .first()
            .cloned()
            .ok_or(Error::Invalid("pointer clip"))?;
        mixer.play(clip)?;
        // AnimationClip.resetDuration(): the latest key of every track.
        let duration = tracks
            .iter()
            .filter_map(|t| t.times.last().copied())
            .fold(0., f64::max);
        let mut d = Self {
            controls,
            mixer,
            tracks,
            meshes,
            duration,
            time: 0.,
        };
        d.apply(s)?;
        Ok(d)
    }
    /// The material tracks at the action's time: LoopRepeat over the clip.
    fn apply(&mut self, s: &mut Scene) -> Result<()> {
        let t = if self.duration > 0. {
            self.time.rem_euclid(self.duration)
        } else {
            0.
        };
        for track in &self.tracks {
            let v = track.sample(t);
            let NodeKind::Mesh(m) = &mut s.get_mut(self.meshes[track.material.min(1)])?.kind else {
                continue;
            };
            let material = Arc::make_mut(&mut m.materials[0]);
            match track.property.as_str() {
                "pbrMetallicRoughness/baseColorFactor" => {
                    let p = material.properties_mut();
                    // ColorKeyframeTrack sets r, g, b; the opacity track takes alpha.
                    p.color = Color(Vector3::new(v[0], v[1], v[2]));
                    p.opacity = v[3];
                }
                "alphaCutoff" => material.properties_mut().alpha_test = v[0],
                "pbrMetallicRoughness/baseColorTexture/extensions/KHR_texture_transform/scale" => {
                    if let Some(map) = &mut material.properties_mut().map {
                        let map = Arc::make_mut(map);
                        map.repeat = Vector2::new(v[0], v[1]);
                        // Texture.updateMatrix(): setUvTransform from the animated values.
                        map.matrix = None;
                    }
                }
                "pbrMetallicRoughness/baseColorTexture/extensions/KHR_texture_transform/offset" => {
                    if let Some(map) = &mut material.properties_mut().map {
                        let map = Arc::make_mut(map);
                        map.offset = Vector2::new(v[0], v[1]);
                        // Texture.updateMatrix(): setUvTransform from the animated values.
                        map.matrix = None;
                    }
                }
                property => {
                    let Material::Physical(p) = material else {
                        return Err(Error::Invalid("pointer physical material"));
                    };
                    match property {
                        "pbrMetallicRoughness/metallicFactor" => p.base.metalness = v[0],
                        "pbrMetallicRoughness/roughnessFactor" => p.base.roughness = v[0],
                        "extensions/KHR_materials_volume/thicknessFactor" => p.thickness = v[0],
                        "extensions/KHR_materials_volume/attenuationDistance" => {
                            p.attenuation_distance = v[0]
                        }
                        "extensions/KHR_materials_volume/attenuationColor" => {
                            p.attenuation_color = Color(Vector3::new(v[0], v[1], v[2]))
                        }
                        "extensions/KHR_materials_iridescence/iridescenceFactor" => {
                            p.iridescence = v[0]
                        }
                        "extensions/KHR_materials_iridescence/iridescenceIor" => {
                            p.iridescence_ior = v[0]
                        }
                        "extensions/KHR_materials_transmission/transmissionFactor" => {
                            p.transmission = v[0]
                        }
                        _ => return Err(Error::Invalid("pointer property")),
                    }
                }
            }
        }
        Ok(())
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, dt: f64, animate: bool) -> Result<()> {
        if animate {
            self.time += dt;
        }
        Ok(())
    }
    /// animate(): mixer.update( delta ), then controls.update().
    pub fn prepare(&mut self, s: &mut Scene, c: Object3D, _r: &Renderer) -> Result<()> {
        for a in &mut self.mixer.actions {
            a.time = self.time;
        }
        self.mixer.update(s, 0.)?;
        self.apply(s)?;
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
        wheel: f64,
        pan: bool,
        height: f64,
    ) -> Result<()> {
        let camera = camera_state(s, c)?;
        if wheel != 0. {
            self.controls.dolly(wheel, &camera, Vector2::ZERO);
        } else if !pan {
            // enablePan = false.
            self.controls.rotate(dx, dy, height);
        }
        Ok(())
    }
    pub fn parameter(&mut self, _index: usize, _value: f32) -> Result<()> {
        Err(Error::Invalid("animation_pointer parameter"))
    }
    pub fn seek(&mut self, t: f64) {
        self.time = t;
    }
}
