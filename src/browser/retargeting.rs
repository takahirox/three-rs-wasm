//! webgpu_animation_retargeting: Michelle's SambaDance retargeted onto the
//! Soldier with SkeletonUtils.retargetClip, both dancing over a reflector
//! floor under a light-speed background. The retargeted clip is baked once on
//! the CPU, as the page does: per frame of the source clip the source pose is
//! sampled, the target skeleton is reset to its bind pose and each mapped
//! bone takes the source bone's world rotation ( with the page's local
//! offsets ) and the hip its scaled position, following three.js's matrix
//! semantics, including the world matrices the loader left stale. Both
//! characters are skinned on the GPU from their mixers' bone matrices; the
//! reflector renders the mirrored view into a target the floor samples.
//! Every stage runs the WGSL three.js r186 generates for the page (in
//! `retargeting/`; the output is the compute_cloth modules, byte-identical).
use super::controls_attributes::{Controls, camera_state};
use super::deferred::{Draw, sampled_pipeline, set};
use super::gltf_viewer::load_asset;
use super::lights_projector::{m3, m4, pack};
use super::pmrem_cube_uv::bind;
use super::retro::{mipmapped, uniform};
use super::shadowmap_opacity::Mipmaps;
use super::three_mixer::ThreeMixer;
use crate::animation::{Clip, Interpolation, Property, Track};
use crate::{
    Error, Result, camera::*, geometry::*, math::*, render_target::*, renderer::*, scene::*,
};
use std::collections::HashMap;
use std::f64::consts::PI;
use std::sync::Arc;
use wgpu::util::DeviceExt;

const HALF: wgpu::TextureFormat = wgpu::TextureFormat::Rgba16Float;
const SRGB: wgpu::TextureFormat = wgpu::TextureFormat::Rgba8UnormSrgb;
const BYTE: wgpu::TextureFormat = wgpu::TextureFormat::Rgba8Unorm;
const DEPTH: wgpu::TextureFormat = wgpu::TextureFormat::Depth24Plus;
macro_rules! wgsl {
    ($name:literal) => {
        include_str!(concat!("retargeting/", $name, ".wgsl"))
    };
}
const OUTPUT_VS: &str = include_str!("compute_cloth/output_vs.wgsl");
const OUTPUT_FS: &str = include_str!("compute_cloth/output_fs.wgsl");
/// PropertyBinding.sanitizeNodeName: GLTFLoader's node names.
pub(super) fn sanitize(name: &str) -> String {
    name.chars()
        .filter(|c| !"[]\\.:/".contains(*c))
        .map(|c| if c.is_whitespace() { '_' } else { c })
        .collect()
}
pub(super) fn linear(hex: u32, intensity: f64) -> Vec<f64> {
    [16, 8, 0]
        .map(|shift| {
            let c = ((hex >> shift) & 255) as f64 / 255.;
            let l = if c < 0.04045 {
                c * 0.0773993808
            } else {
                (c * 0.9478672986 + 0.0521327014).powf(2.4)
            };
            l * intensity
        })
        .to_vec()
}
/// An Object3D of the retargeting simulation: three.js's transform state,
/// with world matrices that only change when updateMatrixWorld runs.
#[derive(Clone)]
pub(super) struct SimNode {
    pub(super) name: String,
    pub(super) parent: Option<usize>,
    pub(super) children: Vec<usize>,
    pub(super) position: Vector3,
    pub(super) quaternion: Quaternion,
    pub(super) scale: Vector3,
    pub(super) matrix: Matrix4,
    pub(super) world: Matrix4,
    pub(super) needs_update: bool,
}
pub(super) struct Sim {
    pub(super) nodes: Vec<SimNode>,
}
impl Sim {
    /// The glTF node tree as GLTFLoader leaves it: locals from the TRS and
    /// worlds from the load-time scene.updateMatrixWorld() at the origin.
    pub(super) fn from_gltf(asset: &gltf::Gltf) -> Result<(Self, Vec<usize>)> {
        let mut nodes: Vec<SimNode> = asset
            .nodes()
            .map(|n| {
                let (t, r, s) = n.transform().decomposed();
                SimNode {
                    name: sanitize(n.name().unwrap_or("")),
                    parent: None,
                    children: n.children().map(|c| c.index()).collect(),
                    position: Vector3::new(t[0] as f64, t[1] as f64, t[2] as f64),
                    quaternion: Quaternion::from_xyzw(
                        r[0] as f64,
                        r[1] as f64,
                        r[2] as f64,
                        r[3] as f64,
                    ),
                    scale: Vector3::new(s[0] as f64, s[1] as f64, s[2] as f64),
                    matrix: Matrix4::IDENTITY,
                    world: Matrix4::IDENTITY,
                    needs_update: false,
                }
            })
            .collect();
        for i in 0..nodes.len() {
            for c in nodes[i].children.clone() {
                nodes[c].parent = Some(i);
            }
        }
        let roots: Vec<usize> = asset
            .default_scene()
            .or_else(|| asset.scenes().next())
            .ok_or(Error::Invalid("glTF scene"))?
            .nodes()
            .map(|n| n.index())
            .collect();
        let mut sim = Self { nodes };
        for &root in &roots {
            sim.update_world(root, true);
        }
        Ok((sim, roots))
    }
    fn update_matrix(&mut self, i: usize) {
        let n = &mut self.nodes[i];
        n.matrix = Matrix4::from_scale_rotation_translation(n.scale, n.quaternion, n.position);
        n.needs_update = true;
    }
    /// updateMatrixWorld( force ).
    fn update_world(&mut self, i: usize, mut force: bool) {
        self.update_matrix(i);
        if self.nodes[i].needs_update || force {
            let parent = self.nodes[i].parent.map(|p| self.nodes[p].world);
            let n = &mut self.nodes[i];
            n.world = parent.map_or(n.matrix, |p| p * n.matrix);
            n.needs_update = false;
            force = true;
        }
        for c in self.nodes[i].children.clone() {
            self.update_world(c, force);
        }
    }
    fn decompose(&mut self, i: usize, m: Matrix4) {
        let (s, q, t) = m.to_scale_rotation_translation();
        let n = &mut self.nodes[i];
        n.matrix = m;
        n.position = t;
        n.quaternion = q;
        n.scale = s;
    }
}
/// A retargeted bone's keys: the hip's positions, if any, and the rotations.
pub(super) type BoneKeys = (Option<Vec<[f32; 3]>>, Vec<[f32; 4]>);
/// A baked bone: its node, keys and Float32 times.
pub(super) type BakedBone = (usize, Option<Vec<[f32; 3]>>, Vec<[f32; 4]>, Vec<f32>);
/// SkeletonUtils.retargetClip( target, source skeleton, clip, options ) for
/// the page's options: the bones' bind pose from `inverses`, the source
/// pose sampled by `sample` per frame.
#[allow(clippy::too_many_arguments)]
pub(super) fn retarget_clip(
    target: &mut Sim,
    mesh: usize,
    bones: &[usize],
    inverses: &[Matrix4],
    source: &mut Sim,
    source_bones: &[usize],
    names: &HashMap<String, String>,
    offsets: &HashMap<String, Matrix4>,
    scale: f64,
    duration: f64,
    fps: f64,
    sample: &mut dyn FnMut(&mut Sim, f64) -> Result<()>,
) -> Result<Vec<BakedBone>> {
    let num_frames = (duration * (fps / 1000.) * 1000.).round() as usize;
    let delta = duration / (num_frames as f64 - 1.);
    let hip = "mixamorigHips";
    let find_source = |source: &Sim, name: &str| {
        source_bones
            .iter()
            .copied()
            .find(|&b| source.nodes[b].name == name)
    };
    let mut datas: Vec<Option<BoneKeys>> = vec![None; bones.len()];
    let mut times = vec![];
    sample(source, 0.)?;
    for frame in 0..num_frames {
        let time = frame as f64 * delta;
        // retarget( target, source, options )
        // skeleton.pose(): worlds from the inverse bind matrices.
        for (k, &b) in bones.iter().enumerate() {
            target.nodes[b].world = inverses[k].inverse();
        }
        for &b in bones {
            let world = target.nodes[b].world;
            let m = match target.nodes[b].parent {
                Some(p) if bones.contains(&p) => target.nodes[p].world.inverse() * world,
                _ => world,
            };
            target.decompose(b, m);
        }
        let positions: Vec<Vector3> = bones.iter().map(|&b| target.nodes[b].position).collect();
        // preserveBoneMatrix: the mesh's world reset to the identity.
        target.update_world(mesh, false);
        target.nodes[mesh].world = Matrix4::IDENTITY;
        for c in target.nodes[mesh].children.clone() {
            target.update_world(c, true);
        }
        for &b in bones {
            let name = names.get(&target.nodes[b].name).cloned();
            let mut global = target.nodes[b].world;
            if let Some(source_bone) = name.as_deref().and_then(|n| find_source(source, n)) {
                source.update_world(source_bone, false);
                let relative = source.nodes[source_bone].world;
                let (s, _, _) = relative.to_scale_rotation_translation();
                let unscaled = relative * Matrix4::from_scale(Vector3::ONE / s);
                let rotation = Quaternion::from_mat4(&unscaled);
                global = Matrix4::from_quat(rotation);
                if let Some(offset) = offsets.get(&target.nodes[b].name) {
                    global *= *offset;
                }
                global.w_axis = relative.w_axis;
            }
            if name.as_deref() == Some(hip) {
                global.w_axis.x *= scale;
                global.w_axis.y *= scale;
                global.w_axis.z *= scale;
            }
            let m = match target.nodes[b].parent {
                Some(p) => target.nodes[p].world.inverse() * global,
                None => global,
            };
            target.decompose(b, m);
            target.update_world(b, false);
        }
        // preserveBonePositions: every bone but the hip keeps its pose position.
        for (k, &b) in bones.iter().enumerate() {
            if names.get(&target.nodes[b].name).map(String::as_str) != Some(hip) {
                target.nodes[b].position = positions[k];
            }
        }
        target.update_world(mesh, true);
        times.push(time as f32);
        for (k, &b) in bones.iter().enumerate() {
            let Some(name) = names.get(&target.nodes[b].name) else {
                continue;
            };
            if find_source(source, name).is_none() {
                continue;
            }
            let data = datas[k]
                .get_or_insert_with(|| (if name == hip { Some(vec![]) } else { None }, vec![]));
            let n = &target.nodes[b];
            if let Some(p) = &mut data.0 {
                p.push(n.position.to_array().map(|v| v as f32));
            }
            data.1.push(n.quaternion.to_array().map(|v| v as f32));
        }
        // The last mixer step stops short of the clip's end.
        let step = if frame + 2 == num_frames {
            delta - 0.0000001
        } else {
            delta
        };
        sample(source, step)?;
    }
    Ok(datas
        .into_iter()
        .enumerate()
        .filter_map(|(k, d)| d.map(|(p, q)| (bones[k], p, q, times.clone())))
        .collect())
}
/// The reflector's virtual camera and oblique projection for the camera's
/// world matrix and projection ( the mirror is the plane y = 0 facing up ):
/// its view and projection.
pub(super) fn reflector_camera(world: Matrix4, projection: Matrix4) -> (Matrix4, Matrix4) {
    // The reflector's virtual camera and oblique projection ( the
    // mirror is the plane y = 0 facing up ).
    let normal = Vector3::Y;
    let camera_position = world.w_axis.truncate();
    let reflect = |v: Vector3| v - normal * (2. * v.dot(normal));
    let mirror_view = -reflect(-camera_position);
    let rotation = Matrix4::from_mat3(glam::DMat3::from_mat4(world));
    let look_at = rotation.transform_vector3(Vector3::new(0., 0., -1.)) + camera_position;
    let target = -reflect(-look_at);
    let up = reflect(rotation.transform_vector3(Vector3::Y));
    let virtual_world = {
        // Object3D.lookAt for a camera: −z towards the target.
        let z = (mirror_view - target).normalize();
        let mut x = up.cross(z);
        if x.length_squared() == 0. {
            x = Vector3::X;
        }
        let x = x.normalize();
        let y = z.cross(x);
        Matrix4::from_cols(
            x.extend(0.),
            y.extend(0.),
            z.extend(0.),
            mirror_view.extend(1.),
        )
    };
    let virtual_view = virtual_world.inverse();
    let mut virtual_projection = projection;
    {
        // Plane.setFromNormalAndCoplanarPoint( n, 0 ) into view space.
        let plane_normal = virtual_view.transform_vector3(normal).normalize();
        let point = virtual_view.transform_point3(Vector3::ZERO);
        let constant = -point.dot(plane_normal);
        let mut clip = plane_normal.extend(constant);
        let e = virtual_projection.to_cols_array();
        let q = Vector4::new(
            (clip.x.signum() + e[8]) / e[0],
            (clip.y.signum() + e[9]) / e[5],
            -1.,
            (1. + e[10]) / e[14],
        );
        clip *= 1. / clip.dot(q);
        let mut e = e;
        e[2] = clip.x;
        e[6] = clip.y;
        e[10] = clip.z;
        e[14] = clip.w;
        virtual_projection = Matrix4::from_cols_array(&e);
    }
    (virtual_view, virtual_projection)
}
/// A skinned mesh: its buffers, skeleton, material maps and the sphere
/// SkinnedMesh.computeBoundingSphere takes at the first culling.
struct Skinned {
    buffers: [wgpu::Buffer; 5],
    index: wgpu::Buffer,
    count: u32,
    node: Object3D,
    bones: Vec<(Object3D, Matrix4)>,
    bone_buffer: wgpu::Buffer,
    object: wgpu::Buffer,
    positions: Vec<f32>,
    joints: Vec<u32>,
    weights: Vec<f32>,
    sphere: Option<Sphere>,
}
impl Skinned {
    fn draw(&self, pass: &mut wgpu::RenderPass) {
        for (slot, b) in self.buffers.iter().enumerate() {
            pass.set_vertex_buffer(slot as u32, b.slice(..));
        }
        pass.set_index_buffer(self.index.slice(..), wgpu::IndexFormat::Uint32);
        pass.draw_indexed(0..self.count, 0, 0..1);
    }
    /// getVertexPosition for every vertex into Sphere.expandByPoint.
    fn bounding_sphere(&self, s: &Scene) -> Result<Sphere> {
        let world = s.get(self.node)?.matrix_world;
        let matrices: Vec<Matrix4> = self
            .bones
            .iter()
            .map(|(h, inverse)| Ok(s.get(*h)?.matrix_world * *inverse))
            .collect::<Result<_>>()?;
        skinned_sphere(
            &self.positions,
            &self.joints,
            &self.weights,
            &matrices,
            world.inverse(),
        )
    }
}
/// SkinnedMesh.computeBoundingSphere: every vertex skinned by the bone
/// matrices and brought back by the bind matrix inverse, into
/// Sphere.expandByPoint.
pub(super) fn skinned_sphere(
    positions: &[f32],
    joints: &[u32],
    weights: &[f32],
    matrices: &[Matrix4],
    bind_inverse: Matrix4,
) -> Result<Sphere> {
    let mut sphere: Option<Sphere> = None;
    for i in 0..positions.len() / 3 {
        let base = Vector3::new(
            positions[i * 3] as f64,
            positions[i * 3 + 1] as f64,
            positions[i * 3 + 2] as f64,
        );
        let mut p = Vector3::ZERO;
        for k in 0..4 {
            let w = weights[i * 4 + k] as f64;
            if w != 0. {
                p += matrices[joints[i * 4 + k] as usize].transform_point3(base) * w;
            }
        }
        let p = bind_inverse.transform_point3(p);
        match &mut sphere {
            None => {
                sphere = Some(Sphere {
                    center: p,
                    radius: 0.,
                })
            }
            Some(s) => {
                let v = p - s.center;
                let length_sq = v.length_squared();
                if length_sq > s.radius * s.radius {
                    let length = length_sq.sqrt();
                    let delta = (length - s.radius) * 0.5;
                    s.center += v * (delta / length);
                    s.radius += delta;
                }
            }
        }
    }
    sphere.ok_or(Error::Invalid("skinned mesh vertices"))
}
struct Targets {
    width: u32,
    height: u32,
    format: wgpu::TextureFormat,
    samples: u32,
    msaa: Option<(wgpu::TextureView, wgpu::TextureView)>,
    color: wgpu::TextureView,
    depth: wgpu::TextureView,
    reflection: (wgpu::TextureView, wgpu::TextureView),
    /// Per pass ( main, reflection ): background, Michelle, Soldier, visor.
    draws: [[Draw; 4]; 2],
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
    meshes: [Skinned; 3],
    maps: [Vec<wgpu::TextureView>; 2],
    background: (wgpu::Buffer, wgpu::Buffer, wgpu::Buffer, u32),
    floor: (wgpu::Buffer, wgpu::Buffer, u32),
    renders: [[wgpu::Buffer; 4]; 2],
    background_object: wgpu::Buffer,
    floor_render: wgpu::Buffer,
    floor_object: wgpu::Buffer,
    output_render: wgpu::Buffer,
    output_object: wgpu::Buffer,
    quad: wgpu::Buffer,
    clamp: wgpu::Sampler,
    repeat: wgpu::Sampler,
    targets: Option<Targets>,
}
/// A skinned glTF mesh's attributes, with GLTFLoader's normalized weights.
#[allow(clippy::type_complexity)]
fn skinned_attributes(
    primitive: &gltf::Primitive,
    buffers: &[Vec<u8>],
) -> Result<(Vec<f32>, Vec<u32>, Vec<f32>, Vec<f32>, Vec<f32>, Vec<u32>)> {
    let reader = primitive.reader(|b| buffers.get(b.index()).map(Vec::as_slice));
    let positions = reader
        .read_positions()
        .ok_or(Error::Invalid("skinned positions"))?
        .flatten()
        .collect();
    let joints = reader
        .read_joints(0)
        .ok_or(Error::Invalid("skinned joints"))?
        .into_u16()
        .flat_map(|j| j.map(u32::from))
        .collect();
    let weights = reader
        .read_weights(0)
        .ok_or(Error::Invalid("skinned weights"))?
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
    let normals = reader
        .read_normals()
        .ok_or(Error::Invalid("skinned normals"))?
        .flatten()
        .collect();
    let uv = reader
        .read_tex_coords(0)
        .ok_or(Error::Invalid("skinned uv"))?
        .into_f32()
        .flatten()
        .collect();
    let index = reader
        .read_indices()
        .ok_or(Error::Invalid("skinned index"))?
        .into_u32()
        .collect();
    Ok((positions, joints, weights, normals, uv, index))
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
        s.get_mut(c)?.position = Vector3::new(0., 1., 4.);
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
        let (michelle, michelle_buffers, michelle_images) =
            load_asset("/web/gallery/assets/tsl-viewport/models/gltf/Michelle.glb").await?;
        let (soldier, soldier_buffers, soldier_images) =
            load_asset("/web/gallery/assets/spot-skinning/Soldier.glb").await?;
        let mut mipmaps = Mipmaps::new(r);
        // Michelle's maps: color, metalness / roughness, specular color, normal.
        let m_material = michelle
            .meshes()
            .next()
            .and_then(|m| m.primitives().next())
            .ok_or(Error::Invalid("Michelle mesh"))?
            .material();
        let pbr = m_material.pbr_metallic_roughness();
        let src = |t: Option<gltf::Texture>| -> Result<usize> {
            t.map(|t| t.source().index()).ok_or(Error::Invalid("map"))
        };
        let m_image = |i: usize| {
            michelle_images
                .get(i)
                .ok_or(Error::Invalid("Michelle image"))
        };
        let michelle_maps = vec![
            mipmapped(
                r,
                &mut mipmaps,
                m_image(src(pbr.base_color_texture().map(|t| t.texture()))?)?,
                SRGB,
            ),
            mipmapped(
                r,
                &mut mipmaps,
                m_image(src(pbr.metallic_roughness_texture().map(|t| t.texture()))?)?,
                BYTE,
            ),
            mipmapped(
                r,
                &mut mipmaps,
                m_image(src(m_material
                    .specular()
                    .and_then(|s| s.specular_color_texture())
                    .map(|t| t.texture()))?)?,
                SRGB,
            ),
            mipmapped(
                r,
                &mut mipmaps,
                m_image(src(m_material.normal_texture().map(|t| t.texture()))?)?,
                BYTE,
            ),
        ];
        let s_material = soldier
            .meshes()
            .next()
            .and_then(|m| m.primitives().next())
            .ok_or(Error::Invalid("Soldier mesh"))?
            .material();
        let s_image = |i: usize| soldier_images.get(i).ok_or(Error::Invalid("Soldier image"));
        let soldier_maps = vec![
            mipmapped(
                r,
                &mut mipmaps,
                s_image(src(s_material
                    .pbr_metallic_roughness()
                    .base_color_texture()
                    .map(|t| t.texture()))?)?,
                SRGB,
            ),
            mipmapped(
                r,
                &mut mipmaps,
                s_image(src(s_material.normal_texture().map(|t| t.texture()))?)?,
                BYTE,
            ),
        ];
        // The two characters in the scene, with the page's placement.
        let michelle_instance =
            crate::gltf::import_animated_decoded(&michelle, &michelle_buffers, &michelle_images)?
                .instantiate(s)?;
        let soldier_instance =
            crate::gltf::import_animated_decoded(&soldier, &soldier_buffers, &soldier_images)?
                .instantiate(s)?;
        let michelle_root = s.insert(NodeKind::Group);
        for &h in &michelle_instance.roots {
            s.add(michelle_root, h)?;
        }
        let soldier_root = s.insert(NodeKind::Group);
        for &h in &soldier_instance.roots {
            s.add(soldier_root, h)?;
        }
        {
            let n = s.get_mut(michelle_root)?;
            n.position.x = -0.8;
            n.quaternion = Quaternion::from_rotation_y(PI / 2.);
        }
        {
            let n = s.get_mut(soldier_root)?;
            n.position = Vector3::new(0.7, 0., -0.1);
            n.scale = Vector3::splat(0.01);
            n.quaternion = Quaternion::from_rotation_y(-PI / 2.);
        }
        // The source and the main mixer: SambaDance on Michelle.
        let clip = michelle_instance.clips[0].clone();
        let mut source_mixer = ThreeMixer::new();
        let action = source_mixer.clip_action(s, clip.clone(), false)?;
        source_mixer.play(s, action)?;
        // retargetClip on simulations of both node trees, the source pose
        // sampled by a mixer of its own on the scene's Michelle.
        let (mut target_sim, _) = Sim::from_gltf(&soldier)?;
        let (mut source_sim, _) = Sim::from_gltf(&michelle)?;
        let skin = soldier
            .skins()
            .next()
            .ok_or(Error::Invalid("Soldier skin"))?;
        let target_bones: Vec<usize> = skin.joints().map(|j| j.index()).collect();
        let inverses: Vec<Matrix4> = skin
            .reader(|b| soldier_buffers.get(b.index()).map(Vec::as_slice))
            .read_inverse_bind_matrices()
            .ok_or(Error::Invalid("Soldier inverse bind matrices"))?
            .map(|m| Matrix4::from_cols_array(&std::array::from_fn(|i| m[i / 4][i % 4] as f64)))
            .collect();
        let mesh_index = soldier
            .nodes()
            .find(|n| n.name() == Some("vanguard_Mesh"))
            .ok_or(Error::Invalid("Soldier mesh node"))?
            .index();
        let michelle_skin = michelle
            .skins()
            .next()
            .ok_or(Error::Invalid("Michelle skin"))?;
        let michelle_joints: Vec<usize> = michelle_skin.joints().map(|j| j.index()).collect();
        // SkeletonHelper's bone list: the bones in depth-first order.
        let mut source_bones = vec![];
        fn walk(n: gltf::Node, joints: &[usize], out: &mut Vec<usize>) {
            if joints.contains(&n.index()) {
                out.push(n.index());
            }
            for c in n.children() {
                walk(c, joints, out);
            }
        }
        for n in michelle
            .default_scene()
            .or_else(|| michelle.scenes().next())
            .ok_or(Error::Invalid("Michelle scene"))?
            .nodes()
        {
            walk(n, &michelle_joints, &mut source_bones);
        }
        let mapped = [
            "Hips",
            "Spine",
            "Spine2",
            "Head",
            "LeftShoulder",
            "RightShoulder",
            "LeftArm",
            "RightArm",
            "LeftForeArm",
            "RightForeArm",
            "LeftHand",
            "RightHand",
            "LeftUpLeg",
            "RightUpLeg",
            "LeftLeg",
            "RightLeg",
            "LeftFoot",
            "RightFoot",
            "LeftToeBase",
            "RightToeBase",
        ];
        let names: HashMap<String, String> = mapped
            .iter()
            .map(|n| (format!("mixamorig{n}"), format!("mixamorig{n}")))
            .collect();
        let deg = |d: f64| d * PI / 180.;
        let cw45 = Matrix4::from_rotation_y(deg(45.));
        let ccw180 = Matrix4::from_rotation_y(deg(-180.));
        let cw180 = Matrix4::from_rotation_y(deg(180.));
        // makeRotationFromEuler( 45°, 180°, 0° ), XYZ.
        let foot = Matrix4::from_quat(Quaternion::from_euler(
            glam::EulerRot::XYZ,
            deg(45.),
            deg(180.),
            0.,
        ));
        let offsets: HashMap<String, Matrix4> = [
            ("LeftShoulder", cw45),
            ("RightShoulder", ccw180),
            ("LeftArm", cw45),
            ("RightArm", ccw180),
            ("LeftForeArm", cw45),
            ("RightForeArm", ccw180),
            ("LeftHand", cw45),
            ("RightHand", ccw180),
            ("LeftUpLeg", cw180),
            ("RightUpLeg", cw180),
            ("LeftLeg", cw180),
            ("RightLeg", cw180),
            ("LeftFoot", foot),
            ("RightFoot", foot),
            ("LeftToeBase", cw180),
            ("RightToeBase", cw180),
        ]
        .into_iter()
        .map(|(n, m)| (format!("mixamorig{n}"), m))
        .collect();
        let duration = clip.duration();
        let frames = clip.tracks.iter().map(|t| t.times.len()).max().unwrap_or(1);
        let fps = frames as f64 / duration;
        // The retarget mixer drives the scene's Michelle; its pose is
        // copied into the source simulation.
        let mut retarget_mixer = ThreeMixer::new();
        let retarget_action = retarget_mixer.clip_action(s, clip.clone(), false)?;
        retarget_mixer.play(s, retarget_action)?;
        let joint_handles: Vec<(usize, Object3D)> = michelle_joints
            .iter()
            .map(|&j| (j, michelle_instance.nodes[j]))
            .collect();
        let scene = &mut *s;
        let mut sample = |sim: &mut Sim, dt: f64| -> Result<()> {
            let s = &mut *scene;
            retarget_mixer.update(s, dt)?;
            for &(j, h) in &joint_handles {
                let n = s.get(h)?;
                sim.nodes[j].position = n.position;
                sim.nodes[j].quaternion = n.quaternion;
                sim.nodes[j].scale = n.scale;
            }
            Ok(())
        };
        let baked = retarget_clip(
            &mut target_sim,
            mesh_index,
            &target_bones,
            &inverses,
            &mut source_sim,
            &source_bones,
            &names,
            &offsets,
            100.,
            duration,
            fps,
            &mut sample,
        )?;
        // uncacheAction restores the source bones; the target keeps the
        // last retargeted state.
        for &(j, h) in &joint_handles {
            let gltf_node = michelle
                .nodes()
                .nth(j)
                .ok_or(Error::Invalid("Michelle node"))?;
            let (t, q, sc) = gltf_node.transform().decomposed();
            let n = s.get_mut(h)?;
            n.position = Vector3::new(t[0] as f64, t[1] as f64, t[2] as f64);
            n.quaternion =
                Quaternion::from_xyzw(q[0] as f64, q[1] as f64, q[2] as f64, q[3] as f64);
            n.scale = Vector3::new(sc[0] as f64, sc[1] as f64, sc[2] as f64);
        }
        for &b in &target_bones {
            let h = soldier_instance.nodes[b];
            let sim = &target_sim.nodes[b];
            let n = s.get_mut(h)?;
            n.position = sim.position;
            n.quaternion = sim.quaternion;
            n.scale = sim.scale;
        }
        let mut tracks = vec![];
        for (b, positions, quaternions, times) in baked {
            let target = soldier_instance.nodes[b];
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
        // The skinned meshes: Michelle's, the Soldier's body and visor.
        let skinned = |asset: &gltf::Gltf,
                       buffers: &[Vec<u8>],
                       instance: &crate::gltf::GltfInstance,
                       name: &str,
                       fs: &str|
         -> Result<Skinned> {
            let node = asset
                .nodes()
                .find(|n| n.mesh().is_some() && n.name().is_some_and(|n| n.contains(name)))
                .ok_or(Error::Invalid("skinned node"))?;
            let primitive = node
                .mesh()
                .and_then(|m| m.primitives().next())
                .ok_or(Error::Invalid("skinned primitive"))?;
            let (positions, joints, weights, normals, uv, index) =
                skinned_attributes(&primitive, buffers)?;
            let skin = node.skin().ok_or(Error::Invalid("skin"))?;
            let inverses: Vec<Matrix4> = skin
                .reader(|b| buffers.get(b.index()).map(Vec::as_slice))
                .read_inverse_bind_matrices()
                .ok_or(Error::Invalid("inverse bind matrices"))?
                .map(|m| Matrix4::from_cols_array(&std::array::from_fn(|i| m[i / 4][i % 4] as f64)))
                .collect();
            let bones: Vec<(Object3D, Matrix4)> = skin
                .joints()
                .map(|j| instance.nodes[j.index()])
                .zip(inverses)
                .collect();
            Ok(Skinned {
                buffers: [
                    init(
                        "skinned positions",
                        bytemuck::cast_slice(&positions),
                        vertex,
                    ),
                    init("skinned joints", bytemuck::cast_slice(&joints), vertex),
                    init("skinned weights", bytemuck::cast_slice(&weights), vertex),
                    init("skinned normals", bytemuck::cast_slice(&normals), vertex),
                    init("skinned uv", bytemuck::cast_slice(&uv), vertex),
                ],
                index: init("skinned index", bytemuck::cast_slice(&index), index_usage),
                count: index.len() as u32,
                node: instance.nodes[node.index()],
                bone_buffer: r.device.create_buffer(&wgpu::BufferDescriptor {
                    label: Some("skinned bones"),
                    size: (bones.len() * 64) as u64,
                    usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                    mapped_at_creation: false,
                }),
                bones,
                object: uniform(r, "skinned object", fs, "objectStruct")?,
                positions,
                joints,
                weights,
                sphere: None,
            })
        };
        let meshes = [
            skinned(
                &michelle,
                &michelle_buffers,
                &michelle_instance,
                "Ch03",
                wgsl!("michelle_fs"),
            )?,
            skinned(
                &soldier,
                &soldier_buffers,
                &soldier_instance,
                "vanguard_Mesh",
                wgsl!("soldier_fs"),
            )?,
            skinned(
                &soldier,
                &soldier_buffers,
                &soldier_instance,
                "vanguard_visor",
                wgsl!("soldier_fs"),
            )?,
        ];
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
        let render = |fs: &str| uniform(r, "retargeting render", fs, "renderStruct");
        let pass_renders = || -> Result<[wgpu::Buffer; 4]> {
            Ok([
                render(wgsl!("background_fs"))?,
                render(wgsl!("michelle_fs"))?,
                render(wgsl!("soldier_fs"))?,
                render(wgsl!("soldier_fs"))?,
            ])
        };
        let sampler = |address| {
            r.device.create_sampler(&wgpu::SamplerDescriptor {
                address_mode_u: address,
                address_mode_v: address,
                mag_filter: wgpu::FilterMode::Linear,
                min_filter: wgpu::FilterMode::Linear,
                mipmap_filter: if address == wgpu::AddressMode::Repeat {
                    wgpu::FilterMode::Linear
                } else {
                    wgpu::FilterMode::Nearest
                },
                ..Default::default()
            })
        };
        Ok(Self {
            controls,
            time: 0.,
            last: 0.,
            pending: true,
            source_mixer,
            target_mixer,
            meshes,
            maps: [michelle_maps, soldier_maps],
            background,
            floor,
            renders: [pass_renders()?, pass_renders()?],
            background_object: uniform(
                r,
                "retargeting background",
                wgsl!("background_fs"),
                "objectStruct",
            )?,
            floor_render: uniform(r, "retargeting floor", wgsl!("floor_fs"), "renderStruct")?,
            floor_object: uniform(r, "retargeting floor", wgsl!("floor_fs"), "objectStruct")?,
            output_render: uniform(r, "retargeting output", OUTPUT_FS, "renderStruct")?,
            output_object: uniform(r, "retargeting output", OUTPUT_VS, "objectStruct")?,
            quad: init(
                "retargeting quad",
                bytemuck::cast_slice(&[-1f32, 3., 0., -1., -1., 0., 3., -1., 0.]),
                vertex,
            ),
            clamp: sampler(wgpu::AddressMode::ClampToEdge),
            repeat: sampler(wgpu::AddressMode::Repeat),
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
                    label: Some("retargeting target"),
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
        let two = |p: &wgpu::RenderPipeline,
                   render: &wgpu::Buffer,
                   object: &[(u32, wgpu::BindingResource)]|
         -> Draw {
            (
                p.clone(),
                vec![
                    bind(
                        r,
                        p.get_bind_group_layout(0),
                        &[(0, render.as_entire_binding())],
                    ),
                    bind(r, p.get_bind_group_layout(1), object),
                ],
            )
        };
        let bg_attrs = [
            wgpu::vertex_attr_array![0 => Float32x3],
            wgpu::vertex_attr_array![1 => Float32x3],
        ];
        let bg_layouts = [0, 1].map(|i| wgpu::VertexBufferLayout {
            array_stride: 12,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &bg_attrs[i],
        });
        let skin_attrs = [
            wgpu::vertex_attr_array![0 => Float32x3],
            wgpu::vertex_attr_array![1 => Uint32x4],
            wgpu::vertex_attr_array![2 => Float32x4],
            wgpu::vertex_attr_array![3 => Float32x3],
            wgpu::vertex_attr_array![4 => Float32x2],
        ];
        let skin_layouts = [(0, 12), (1, 16), (2, 16), (3, 12), (4, 8)].map(|(i, stride)| {
            wgpu::VertexBufferLayout {
                array_stride: stride,
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes: &skin_attrs[i],
            }
        });
        let build = |pass: usize, pass_samples: u32| -> [Draw; 4] {
            let bg = sampled_pipeline(
                r,
                "retargeting background",
                (wgsl!("background_vs"), wgsl!("background_fs")),
                &bg_layouts,
                &[HALF],
                Some((wgpu::CompareFunction::Always, false)),
                (true, false),
                (pass_samples, wgpu::PrimitiveTopology::TriangleList),
            );
            let michelle = no_cull(
                r,
                (wgsl!("michelle_vs"), wgsl!("michelle_fs")),
                &skin_layouts,
                pass_samples,
            );
            let soldier = sampled_pipeline(
                r,
                "retargeting soldier",
                (wgsl!("soldier_vs"), wgsl!("soldier_fs")),
                &skin_layouts,
                &[HALF],
                Some((wgpu::CompareFunction::LessEqual, true)),
                (false, false),
                (pass_samples, wgpu::PrimitiveTopology::TriangleList),
            );
            let visor = sampled_pipeline(
                r,
                "retargeting visor",
                (wgsl!("visor_vs"), wgsl!("soldier_fs")),
                &skin_layouts,
                &[HALF],
                Some((wgpu::CompareFunction::LessEqual, true)),
                (false, false),
                (pass_samples, wgpu::PrimitiveTopology::TriangleList),
            );
            let m = &self.maps[0];
            let sm = &self.maps[1];
            let soldier_entries = |k: usize| {
                vec![
                    (0, self.meshes[k].object.as_entire_binding()),
                    (1, sampler(&self.repeat)),
                    (2, tex(&sm[0])),
                    (3, sampler(&self.clamp)),
                    (4, tex(&r.dfg)),
                    (5, sampler(&self.repeat)),
                    (6, tex(&sm[1])),
                    (7, self.meshes[k].bone_buffer.as_entire_binding()),
                ]
            };
            [
                two(
                    &bg,
                    &self.renders[pass][0],
                    &[(0, self.background_object.as_entire_binding())],
                ),
                two(
                    &michelle,
                    &self.renders[pass][1],
                    &[
                        (0, self.meshes[0].object.as_entire_binding()),
                        (1, sampler(&self.repeat)),
                        (2, tex(&m[0])),
                        (3, sampler(&self.repeat)),
                        (4, tex(&m[1])),
                        (5, sampler(&self.repeat)),
                        (6, tex(&m[2])),
                        (7, sampler(&self.clamp)),
                        (8, tex(&r.dfg)),
                        (9, sampler(&self.repeat)),
                        (10, tex(&m[3])),
                        (11, self.meshes[0].bone_buffer.as_entire_binding()),
                    ],
                ),
                two(&soldier, &self.renders[pass][2], &soldier_entries(1)),
                two(&visor, &self.renders[pass][3], &soldier_entries(2)),
            ]
        };
        let draws = [build(0, samples), build(1, 1)];
        let floor_attrs = [wgpu::vertex_attr_array![0 => Float32x3]];
        let floor_layout = [wgpu::VertexBufferLayout {
            array_stride: 12,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &floor_attrs[0],
        }];
        let floor_pipeline = sampled_pipeline(
            r,
            "retargeting floor",
            (wgsl!("floor_vs"), wgsl!("floor_fs")),
            &floor_layout,
            &[HALF],
            Some((wgpu::CompareFunction::LessEqual, true)),
            (false, true),
            (samples, wgpu::PrimitiveTopology::TriangleList),
        );
        let floor = two(
            &floor_pipeline,
            &self.floor_render,
            &[
                (0, sampler(&self.clamp)),
                (1, tex(&reflection.0)),
                (2, self.floor_object.as_entire_binding()),
            ],
        );
        let format = out.options.format;
        let output_pipeline = sampled_pipeline(
            r,
            "retargeting output",
            (OUTPUT_VS, OUTPUT_FS),
            &floor_layout,
            &[format],
            None,
            (false, false),
            (1, wgpu::PrimitiveTopology::TriangleList),
        );
        let output = two(
            &output_pipeline,
            &self.output_render,
            &[
                (0, sampler(&self.clamp)),
                (1, tex(&color)),
                (2, self.output_object.as_entire_binding()),
            ],
        );
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
            floor,
            output,
            screen,
        });
        Ok(())
    }
    fn present(&self, encoder: &mut wgpu::CommandEncoder, t: &Targets) {
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("retargeting output"),
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
            .ok_or(Error::Invalid("retargeting targets"))?;
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
        let identity3 = m3(Matrix4::IDENTITY);
        let (sky, ground_color, sun) = (
            linear(0xe9c0a5, 5.),
            linear(0x0175ad, 5.),
            linear(0xfff9ea, 4.),
        );
        for (pass, (p, v)) in [(projection, view), (virtual_projection, virtual_view)]
            .into_iter()
            .enumerate()
        {
            let camera_values = [
                ("cameraProjectionMatrix", m4(p)),
                ("cameraViewMatrix", m4(v)),
            ];
            let mut bg = vec![
                ("nodeUniform0", vec![self.time]),
                ("nodeUniform4", vec![1.]),
            ];
            bg.extend(camera_values.clone());
            bg.push(("nodeUniform1", size.clone()));
            write(
                &self.renders[pass][0],
                wgsl!("background_fs"),
                "renderStruct",
                &bg,
            )?;
            let mut lit = camera_values.to_vec();
            lit.extend([
                ("nodeUniform27", sky.clone()),
                ("nodeUniform29", vec![0., 1., 0.]),
                ("nodeUniform26", ground_color.clone()),
                ("nodeUniform32", sun.clone()),
                ("nodeUniform30", vec![2., 5., 2.]),
                ("nodeUniform31", vec![0.; 3]),
            ]);
            write(
                &self.renders[pass][1],
                wgsl!("michelle_fs"),
                "renderStruct",
                &lit,
            )?;
            let mut lit = camera_values.to_vec();
            lit.extend([
                ("nodeUniform19", sky.clone()),
                ("nodeUniform21", vec![0., 1., 0.]),
                ("nodeUniform18", ground_color.clone()),
                ("nodeUniform24", sun.clone()),
                ("nodeUniform22", vec![2., 5., 2.]),
                ("nodeUniform23", vec![0.; 3]),
            ]);
            write(
                &self.renders[pass][2],
                wgsl!("soldier_fs"),
                "renderStruct",
                &lit,
            )?;
            write(
                &self.renders[pass][3],
                wgsl!("soldier_fs"),
                "renderStruct",
                &lit,
            )?;
        }
        write(
            &self.background_object,
            wgsl!("background_fs"),
            "objectStruct",
            &[
                ("nodeUniform3", identity3.clone()),
                ("nodeUniform5", vec![1.]),
                ("nodeUniform7", m4(Matrix4::IDENTITY)),
            ],
        )?;
        for (k, mesh) in self.meshes.iter_mut().enumerate() {
            let mesh_world = s.get(mesh.node)?.matrix_world;
            let bones: Vec<f32> = mesh
                .bones
                .iter()
                .map(|(h, inverse)| Ok(s.get(*h)?.matrix_world * *inverse))
                .collect::<Result<Vec<Matrix4>>>()?
                .iter()
                .flat_map(|m| m.to_cols_array().map(|v| v as f32))
                .collect();
            r.queue
                .write_buffer(&mesh.bone_buffer, 0, bytemuck::cast_slice(&bones));
            if mesh.sphere.is_none() {
                mesh.sphere = Some(mesh.bounding_sphere(s)?);
            }
            let normal_matrix = m3(mesh_world.inverse().transpose());
            if k == 0 {
                write(
                    &mesh.object,
                    wgsl!("michelle_fs"),
                    "objectStruct",
                    &[
                        ("nodeUniform0", m4(mesh_world.inverse())),
                        ("nodeUniform2", m4(Matrix4::IDENTITY)),
                        ("nodeUniform3", vec![1.; 3]),
                        ("nodeUniform5", identity3.clone()),
                        ("nodeUniform6", vec![1.]),
                        ("nodeUniform7", vec![0.5]),
                        ("nodeUniform9", identity3.clone()),
                        ("nodeUniform10", vec![1.]),
                        ("nodeUniform11", identity3.clone()),
                        ("nodeUniform13", normal_matrix),
                        ("nodeUniform14", vec![1.45]),
                        ("nodeUniform15", vec![1.; 3]),
                        ("nodeUniform17", identity3.clone()),
                        ("nodeUniform18", vec![1.]),
                        ("nodeUniform19", vec![0.; 3]),
                        ("nodeUniform20", vec![1.]),
                        ("nodeUniform22", m4(mesh_world)),
                        ("nodeUniform24", identity3.clone()),
                        ("nodeUniform25", vec![1., -1.]),
                    ],
                )?;
            } else {
                write(
                    &mesh.object,
                    wgsl!("soldier_fs"),
                    "objectStruct",
                    &[
                        ("nodeUniform0", m4(mesh_world.inverse())),
                        ("nodeUniform2", m4(Matrix4::IDENTITY)),
                        ("nodeUniform3", vec![0.800000011920929; 3]),
                        ("nodeUniform5", identity3.clone()),
                        ("nodeUniform6", vec![1.]),
                        ("nodeUniform7", vec![0.]),
                        ("nodeUniform8", vec![1.]),
                        ("nodeUniform10", normal_matrix),
                        ("nodeUniform11", vec![0.; 3]),
                        ("nodeUniform12", vec![1.]),
                        ("nodeUniform14", m4(mesh_world)),
                        ("nodeUniform16", identity3.clone()),
                        ("nodeUniform17", vec![1., -1.]),
                    ],
                )?;
            }
        }
        write(
            &self.floor_render,
            wgsl!("floor_fs"),
            "renderStruct",
            &[
                ("cameraProjectionMatrix", m4(projection)),
                ("cameraViewMatrix", m4(view)),
                ("nodeUniform1", size.clone()),
            ],
        )?;
        write(
            &self.floor_object,
            wgsl!("floor_fs"),
            "objectStruct",
            &[
                ("nodeUniform2", vec![0.2]),
                ("nodeUniform5", m4(Matrix4::IDENTITY)),
            ],
        )?;
        write(
            &self.output_render,
            OUTPUT_FS,
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
            OUTPUT_VS,
            "objectStruct",
            &[("nodeUniform5", m4(Matrix4::IDENTITY))],
        )?;
        // Frustum culling with the skinned meshes' first-render spheres.
        let visible = |p: Matrix4, v: Matrix4| -> Result<[bool; 3]> {
            let frustum = Frustum::from_projection(p * v);
            let mut out = [false; 3];
            for (k, mesh) in self.meshes.iter().enumerate() {
                let sphere = mesh.sphere.ok_or(Error::Invalid("skinned sphere"))?;
                let m = s.get(mesh.node)?.matrix_world;
                let (scale, _, _) = m.to_scale_rotation_translation();
                out[k] = frustum.intersects_sphere(Sphere {
                    center: m.transform_point3(sphere.center),
                    radius: sphere.radius * scale.abs().max_element(),
                });
            }
            Ok(out)
        };
        let main_visible = visible(projection, view)?;
        let mirror_visible = visible(virtual_projection, virtual_view)?;
        let mut encoder = r.device.create_command_encoder(&Default::default());
        let scene_pass = |encoder: &mut wgpu::CommandEncoder,
                          target: &wgpu::TextureView,
                          resolve: Option<&wgpu::TextureView>,
                          depth: &wgpu::TextureView,
                          draws: &[Draw; 4],
                          visible: [bool; 3],
                          floor: Option<&Draw>| {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("retargeting scene"),
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
            for k in 0..3 {
                if visible[k] {
                    set(&mut pass, &draws[k + 1]);
                    self.meshes[k].draw(&mut pass);
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
            mirror_visible,
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
            main_visible,
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
    /// The helpers' visibility: the skeleton helpers are not drawn.
    pub fn parameter(&mut self, _index: usize, _value: f32) -> Result<()> {
        Err(Error::Invalid("retargeting parameter"))
    }
    pub fn seek(&mut self, t: f64) {
        self.time = t;
        self.pending = true;
    }
}
/// A pipeline without culling for the double-sided Michelle.
fn no_cull(
    r: &Renderer,
    (vs, fs): (&str, &str),
    buffers: &[wgpu::VertexBufferLayout],
    samples: u32,
) -> wgpu::RenderPipeline {
    let module = |source: &str| {
        r.device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("retargeting Michelle"),
            source: wgpu::ShaderSource::Wgsl(source.into()),
        })
    };
    r.device
        .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("retargeting Michelle"),
            layout: None,
            vertex: wgpu::VertexState {
                module: &module(vs),
                entry_point: Some("main"),
                compilation_options: Default::default(),
                buffers,
            },
            fragment: Some(wgpu::FragmentState {
                module: &module(fs),
                entry_point: Some("main"),
                compilation_options: Default::default(),
                targets: &[Some(HALF.into())],
            }),
            primitive: Default::default(),
            depth_stencil: Some(wgpu::DepthStencilState {
                format: DEPTH,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::LessEqual,
                stencil: Default::default(),
                bias: Default::default(),
            }),
            multisample: wgpu::MultisampleState {
                count: samples,
                ..Default::default()
            },
            multiview: None,
            cache: None,
        })
}
