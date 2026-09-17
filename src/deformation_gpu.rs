//! Persistent skin/morph inputs. Per-frame uploads contain only bone matrices and weights.
use crate::{Error, Result, geometry::BufferGeometry, scene::*};
use std::{
    collections::HashMap,
    sync::{Arc, Weak},
};
use wgpu::util::DeviceExt;
#[derive(Clone)]
pub(crate) struct Bindings {
    pub data: wgpu::Buffer,
    pub pose: wgpu::Buffer,
    pub info: wgpu::Buffer,
}
struct Geometry {
    owner: Weak<BufferGeometry>,
    versions: Vec<u64>,
    data: wgpu::Buffer,
    targets: usize,
    max_joint: Option<usize>,
    layout: [u32; 3],
}
struct Pose {
    geometry: usize,
    binding: Bindings,
    last: Vec<f32>,
    header: [u32; 8],
}
#[derive(Default)]
pub(crate) struct Cache {
    geometry: HashMap<usize, Geometry>,
    poses: HashMap<Object3D, Pose>,
    pub static_bytes: u64,
    pub pose_bytes: u64,
}
pub(crate) fn layout_entries() -> Vec<wgpu::BindGroupLayoutEntry> {
    (21..=23)
        .map(|binding| wgpu::BindGroupLayoutEntry {
            binding,
            visibility: wgpu::ShaderStages::VERTEX,
            ty: wgpu::BindingType::Buffer {
                ty: if binding == 23 {
                    wgpu::BufferBindingType::Uniform
                } else {
                    wgpu::BufferBindingType::Storage { read_only: true }
                },
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        })
        .collect()
}
fn buffer(device: &wgpu::Device, bytes: &[u8], uniform: bool) -> wgpu::Buffer {
    device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("skin/morph input"),
        contents: bytes,
        usage: wgpu::BufferUsages::COPY_DST
            | if uniform {
                wgpu::BufferUsages::UNIFORM
            } else {
                wgpu::BufferUsages::STORAGE
            },
    })
}
impl Cache {
    pub fn prune(&mut self, scene: &Scene) {
        self.geometry.retain(|_, g| g.owner.strong_count() > 0);
        self.poses
            .retain(|h, p| scene.get(*h).is_ok() && self.geometry.contains_key(&p.geometry));
    }
    pub fn get(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        scene: &Scene,
        handle: Object3D,
    ) -> Result<Bindings> {
        let node = scene.get(handle)?;
        let source = node
            .geometry()
            .ok_or(Error::Invalid("deformation geometry"))?;
        let key = Arc::as_ptr(source) as usize;
        let versions = source
            .attributes
            .values()
            .chain(source.morph_attributes.values().flatten())
            .map(|a| a.version())
            .collect::<Vec<_>>();
        let stale = self
            .geometry
            .get(&key)
            .is_none_or(|g| g.owner.strong_count() == 0 || g.versions != versions);
        if stale {
            let count = source.vertex_count();
            let targets = source
                .morph_attributes
                .values()
                .map(Vec::len)
                .max()
                .unwrap_or(0);
            let skinned = source.attributes.contains_key("skinIndex")
                || source.attributes.contains_key("skinWeight");
            let mask = ["position", "normal", "color", "tangent"]
                .iter()
                .enumerate()
                .fold(0u32, |mask, (i, name)| {
                    mask | if source
                        .morph_attributes
                        .get(*name)
                        .is_some_and(|v| !v.is_empty())
                    {
                        1 << i
                    } else {
                        0
                    }
                });
            let stride = mask.count_ones() as usize * 4;
            let skin_size = if skinned {
                count
                    .checked_mul(8)
                    .ok_or(Error::Invalid("skin capacity"))?
            } else {
                0
            };
            let size = count
                .checked_mul(targets)
                .and_then(|v| v.checked_mul(stride))
                .and_then(|v| v.checked_add(skin_size))
                .ok_or(Error::Invalid("deformation capacity"))?
                .max(4);
            if size as u64 * 4 > device.limits().max_storage_buffer_binding_size as u64 {
                return Err(Error::Invalid("GPU deformation storage limit"));
            }
            let mut data = vec![0.0f32; size];
            let mut max_joint = None;
            if source.attributes.contains_key("skinIndex")
                || source.attributes.contains_key("skinWeight")
            {
                let joints = source
                    .attributes
                    .get("skinIndex")
                    .ok_or(Error::Invalid("skinIndex attribute"))?;
                let weights = source
                    .attributes
                    .get("skinWeight")
                    .ok_or(Error::Invalid("skinWeight attribute"))?;
                if joints.item_size() != 4
                    || weights.item_size() != 4
                    || joints.count() != count
                    || weights.count() != count
                {
                    return Err(Error::Invalid("skin attribute dimensions"));
                }
                for i in 0..count {
                    for c in 0..4 {
                        let joint = joints.get_component(i, c)?;
                        let weight = weights.get_component(i, c)?;
                        if !joint.is_finite()
                            || joint < 0.0
                            || joint.fract() != 0.0
                            || !weight.is_finite()
                            || weight < 0.0
                        {
                            return Err(Error::Invalid("skin influence"));
                        }
                        if weight > 0.0 {
                            max_joint = Some(max_joint.unwrap_or(0).max(joint as usize));
                        }
                        data[i * 8 + c] = joint as f32;
                        data[i * 8 + 4 + c] = weight as f32;
                    }
                }
            }
            for (attribute, name) in ["position", "normal", "color", "tangent"]
                .into_iter()
                .enumerate()
            {
                if let Some(morphs) = source.morph_attributes.get(name) {
                    let offset = (mask & ((1u32 << attribute) - 1)).count_ones() as usize * 4;
                    let base = source
                        .attributes
                        .get(name)
                        .ok_or(Error::Invalid("morph base"))?;
                    for (target, morph) in morphs.iter().enumerate() {
                        let width = if name == "tangent" {
                            3
                        } else {
                            base.item_size()
                        };
                        if morph.count() != count || morph.item_size() != width || width > 4 {
                            return Err(Error::Invalid("morph attribute dimensions"));
                        }
                        for i in 0..count {
                            for c in 0..width {
                                data[skin_size + (target * count + i) * stride + offset + c] =
                                    (morph.get_component(i, c)?
                                        - if source.morph_targets_relative {
                                            0.0
                                        } else {
                                            base.get_component(i, c)?
                                        }) as f32;
                            }
                        }
                    }
                }
            }
            self.static_bytes += data.len() as u64 * 4;
            self.geometry.insert(
                key,
                Geometry {
                    owner: Arc::downgrade(source),
                    versions,
                    data: buffer(device, bytemuck::cast_slice(&data), false),
                    targets,
                    max_joint,
                    layout: [skin_size as u32, stride as u32, mask],
                },
            );
            self.poses.retain(|_, p| p.geometry != key);
        }
        let geometry = &self.geometry[&key];
        if node.morph_weights.len() > geometry.targets
            || node.morph_weights.iter().any(|v| !v.is_finite())
        {
            return Err(Error::Invalid("morph weight count/value"));
        }
        let mut values = Vec::new();
        let mut joints = 0;
        if let Some(skin) = &node.skin {
            if skin.joints.len() != skin.inverse_bind_matrices.len()
                || node.matrix_world.determinant() == 0.0
                || geometry.max_joint.is_some_and(|i| i >= skin.joints.len())
            {
                return Err(Error::Invalid("skin joints/bind matrices"));
            }
            joints = skin.joints.len();
            let inverse = node.matrix_world.inverse();
            for (&joint, bind) in skin.joints.iter().zip(&skin.inverse_bind_matrices) {
                let m = inverse * scene.get(joint)?.matrix_world * *bind;
                if !m.is_finite() {
                    return Err(Error::Invalid("skin matrix"));
                }
                values.extend(m.as_mat4().to_cols_array());
            }
        }
        for i in 0..geometry.targets {
            values.push(node.morph_weights.get(i).copied().unwrap_or(0.0) as f32);
        }
        if values.is_empty() {
            values.resize(4, 0.0);
        }
        if values.len() as u64 * 4 > device.limits().max_storage_buffer_binding_size as u64 {
            return Err(Error::Invalid("skin palette storage limit"));
        }
        let header = [
            source.vertex_count() as u32,
            joints as u32,
            geometry.targets as u32,
            if matches!(node.kind, NodeKind::Points(_)) {
                6
            } else {
                1
            },
            geometry.layout[0],
            geometry.layout[1],
            geometry.layout[2],
            0,
        ];
        let pose = self.poses.entry(handle).or_insert_with(|| Pose {
            geometry: key,
            binding: Bindings {
                data: geometry.data.clone(),
                pose: buffer(device, bytemuck::cast_slice(&values), false),
                info: buffer(device, bytemuck::cast_slice(&header), true),
            },
            last: Vec::new(),
            header,
        });
        if pose.geometry != key || pose.binding.pose.size() != values.len() as u64 * 4 {
            *pose = Pose {
                geometry: key,
                binding: Bindings {
                    data: geometry.data.clone(),
                    pose: buffer(device, bytemuck::cast_slice(&values), false),
                    info: buffer(device, bytemuck::cast_slice(&header), true),
                },
                last: Vec::new(),
                header,
            };
        }
        if pose.last != values {
            queue.write_buffer(&pose.binding.pose, 0, bytemuck::cast_slice(&values));
            self.pose_bytes += values.len() as u64 * 4;
            pose.last = values;
        }
        if pose.header != header {
            queue.write_buffer(&pose.binding.info, 0, bytemuck::cast_slice(&header));
            pose.header = header;
        }
        Ok(pose.binding.clone())
    }
}
