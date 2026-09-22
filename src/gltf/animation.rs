use super::*;
use crate::animation::{Clip, Interpolation, Property, Track};
#[derive(Clone)]
struct NodeData {
    name: String,
    matrix: Matrix4,
    children: Vec<usize>,
    skin: Option<usize>,
    weights: Vec<f64>,
    instances: Vec<crate::scene::Instance>,
}
#[derive(Clone)]
struct SkinData {
    joints: Vec<usize>,
    inverse: Vec<Matrix4>,
}
#[derive(Clone)]
struct Channel {
    node: usize,
    property: Property,
    times: Vec<f64>,
    values: Vec<Vec<f64>>,
    interpolation: Interpolation,
}
#[derive(Clone)]
pub struct AnimatedGltf {
    pub bounds: Box3,
    pub triangles: usize,
    imported: ImportedGltf,
    nodes: Vec<NodeData>,
    skins: Vec<SkinData>,
    clips: Vec<(String, Vec<Channel>)>,
    roots: Vec<usize>,
}
pub struct GltfInstance {
    pub roots: Vec<Object3D>,
    pub meshes: Vec<Object3D>,
    pub nodes: Vec<Object3D>,
    pub clips: Vec<Arc<Clip>>,
}
pub fn import_animated(
    asset: &gltf::Gltf,
    buffers: &[Vec<u8>],
    images: &[Vec<u8>],
) -> Result<AnimatedGltf> {
    let decoded = images
        .iter()
        .map(|bytes| Texture::from_image(bytes))
        .collect::<Result<Vec<_>>>()?;
    import_animated_decoded(asset, buffers, &decoded)
}
pub fn import_animated_decoded(
    asset: &gltf::Gltf,
    buffers: &[Vec<u8>],
    images: &[Texture],
) -> Result<AnimatedGltf> {
    let imported = super::import_internal(asset, buffers, images, true)?;
    let nodes = asset
        .nodes()
        .map(|node| -> Result<NodeData> {
            Ok(NodeData {
                instances: read_instances(&node, asset, buffers)?,
                name: node.name().unwrap_or("").into(),
                matrix: super::local_transform(node.transform()),
                children: node.children().map(|n| n.index()).collect(),
                skin: node.skin().map(|s| s.index()),
                weights: node
                    .weights()
                    .or_else(|| node.mesh().and_then(|m| m.weights()))
                    .map(|w| w.iter().map(|v| *v as f64).collect())
                    .unwrap_or_else(|| {
                        vec![
                            0.0;
                            node.mesh()
                                .and_then(|m| m.primitives().next())
                                .map_or(0, |p| p.morph_targets().count())
                        ]
                    }),
            })
        })
        .collect::<Result<Vec<_>>>()?;
    let skins = asset
        .skins()
        .map(|skin| {
            let joints = skin.joints().map(|j| j.index()).collect::<Vec<_>>();
            let inverse = skin
                .reader(|b| buffers.get(b.index()).map(Vec::as_slice))
                .read_inverse_bind_matrices()
                .map(|m| {
                    m.map(|m| Matrix4::from_cols_array_2d(&m.map(|v| v.map(f64::from))))
                        .collect()
                })
                .unwrap_or_else(|| vec![Matrix4::IDENTITY; joints.len()]);
            SkinData { joints, inverse }
        })
        .collect::<Vec<_>>();
    if skins.iter().any(|s| s.inverse.len() != s.joints.len()) {
        return Err(Error::Invalid("glTF inverse bind matrix count"));
    }
    let mut clips = Vec::new();
    for animation in asset.animations() {
        let mut channels = Vec::new();
        for channel in animation.channels() {
            let node = channel.target().node().index();
            let reader = channel.reader(|b| buffers.get(b.index()).map(Vec::as_slice));
            let times = reader
                .read_inputs()
                .ok_or(Error::Invalid("glTF animation input"))?
                .map(f64::from)
                .collect::<Vec<_>>();
            let interpolation = match channel.sampler().interpolation() {
                gltf::animation::Interpolation::Step => Interpolation::Step,
                gltf::animation::Interpolation::Linear => Interpolation::Linear,
                gltf::animation::Interpolation::CubicSpline => Interpolation::CubicSpline,
            };
            use gltf::animation::util::ReadOutputs;
            let (property, values) = match reader
                .read_outputs()
                .ok_or(Error::Invalid("glTF animation output"))?
            {
                ReadOutputs::Translations(v) => (
                    Property::Position,
                    v.map(|v| v.map(f64::from).to_vec()).collect(),
                ),
                ReadOutputs::Rotations(v) => (
                    Property::Rotation,
                    v.into_f32().map(|v| v.map(f64::from).to_vec()).collect(),
                ),
                ReadOutputs::Scales(v) => (
                    Property::Scale,
                    v.map(|v| v.map(f64::from).to_vec()).collect(),
                ),
                ReadOutputs::MorphTargetWeights(v) => {
                    let width = nodes[node].weights.len();
                    if width == 0 {
                        return Err(Error::Invalid("glTF animation morph width"));
                    }
                    let flat = v.into_f32().map(f64::from).collect::<Vec<_>>();
                    if !flat.len().is_multiple_of(width) {
                        return Err(Error::Invalid("glTF animation morph output"));
                    }
                    (
                        Property::Weights,
                        flat.chunks_exact(width).map(<[f64]>::to_vec).collect(),
                    )
                }
            };
            channels.push(Channel {
                node,
                property,
                times,
                values,
                interpolation,
            });
        }
        clips.push((animation.name().unwrap_or("").into(), channels));
    }
    let roots = asset
        .default_scene()
        .or_else(|| asset.scenes().next())
        .ok_or(Error::Invalid("glTF scene"))?
        .nodes()
        .map(|n| n.index())
        .collect();
    Ok(AnimatedGltf {
        bounds: imported.bounds,
        triangles: imported.triangles,
        imported,
        nodes,
        skins,
        clips,
        roots,
    })
}
impl AnimatedGltf {
    pub fn instantiate(self, scene: &mut Scene) -> Result<GltfInstance> {
        // Validate keyframe layouts before changing the caller's scene.
        let mut validation_scene = Scene::new();
        let dummy = validation_scene.insert(NodeKind::Group);
        for (_, channels) in &self.clips {
            for c in channels {
                Track {
                    target: dummy,
                    property: c.property,
                    times: c.times.clone(),
                    values: c.values.clone(),
                    interpolation: c.interpolation,
                }
                .validate()?;
            }
        }
        let nodes = self
            .nodes
            .iter()
            .map(|data| {
                let handle = scene.insert(NodeKind::Group);
                let node = scene.get_mut(handle)?;
                node.name = data.name.clone();
                let (scale, rotation, position) = data.matrix.to_scale_rotation_translation();
                node.position = position;
                node.quaternion = rotation;
                node.scale = scale;
                node.matrix = data.matrix;
                node.matrix_auto_update = false;
                node.matrix_world_needs_update = true;
                node.morph_weights = data.weights.clone();
                Ok(handle)
            })
            .collect::<Result<Vec<_>>>()?;
        for (i, data) in self.nodes.iter().enumerate() {
            for &child in &data.children {
                scene.add(nodes[i], nodes[child])?;
            }
        }
        let mut meshes = Vec::new();
        let mut mesh_bindings = vec![Vec::new(); nodes.len()];
        for ((name, _, mesh), parent) in self
            .imported
            .meshes
            .into_iter()
            .zip(self.imported.mesh_nodes)
        {
            let handle = scene.insert(NodeKind::Mesh(mesh));
            let node = scene.get_mut(handle)?;
            node.name = name;
            node.morph_weights = self.nodes[parent].weights.clone();
            node.instances = self.nodes[parent].instances.clone();
            if let Some(index) = self.nodes[parent].skin {
                let skin = &self.skins[index];
                node.skin = Some(crate::deformation::Skin {
                    joints: skin.joints.iter().map(|&j| nodes[j]).collect(),
                    inverse_bind_matrices: skin.inverse.clone(),
                });
            }
            scene.add(nodes[parent], handle)?;
            meshes.push(handle);
            mesh_bindings[parent].push(handle);
        }
        let mut clips = Vec::new();
        for (name, channels) in self.clips {
            let mut tracks = Vec::new();
            for channel in channels {
                let targets = if channel.property == Property::Weights {
                    mesh_bindings[channel.node].clone()
                } else {
                    vec![nodes[channel.node]]
                };
                for target in targets {
                    tracks.push(Track {
                        target,
                        property: channel.property,
                        times: channel.times.clone(),
                        values: channel.values.clone(),
                        interpolation: channel.interpolation,
                    });
                }
            }
            clips.push(Arc::new(Clip { name, tracks }));
        }
        Ok(GltfInstance {
            roots: self.roots.into_iter().map(|i| nodes[i]).collect(),
            nodes,
            meshes,
            clips,
        })
    }
}

fn read_instances(
    node: &gltf::Node<'_>,
    asset: &gltf::Gltf,
    buffers: &[Vec<u8>],
) -> Result<Vec<crate::scene::Instance>> {
    let Some(extension) = node.extension_value("EXT_mesh_gpu_instancing") else {
        return Ok(Vec::new());
    };
    if node.skin().is_some() {
        return Err(Error::Invalid("glTF skinned instancing"));
    }
    let attributes = extension
        .get("attributes")
        .and_then(serde_json::Value::as_object)
        .ok_or(Error::Invalid("glTF instance attributes"))?;
    let mut translations = None;
    let mut rotations = None;
    let mut scales = None;
    let mut count = None;
    for (name, index) in attributes {
        if !matches!(name.as_str(), "TRANSLATION" | "ROTATION" | "SCALE") {
            continue;
        }
        let accessor = asset
            .accessors()
            .nth(
                index
                    .as_u64()
                    .ok_or(Error::Invalid("glTF instance accessor"))? as usize,
            )
            .ok_or(Error::Invalid("glTF instance accessor"))?;
        if accessor.data_type() != gltf::accessor::DataType::F32
            || accessor.count() == 0
            || count.is_some_and(|n| n != accessor.count())
        {
            return Err(Error::Invalid("glTF instance accessor layout"));
        }
        count = Some(accessor.count());
        let data = |b: gltf::Buffer<'_>| buffers.get(b.index()).map(Vec::as_slice);
        if name == "ROTATION" {
            if accessor.dimensions() != gltf::accessor::Dimensions::Vec4 {
                return Err(Error::Invalid("glTF instance rotation"));
            }
            rotations = Some(
                gltf::accessor::Iter::<[f32; 4]>::new(accessor, data)
                    .ok_or(Error::Invalid("glTF instance data"))?
                    .collect::<Vec<_>>(),
            );
        } else {
            if accessor.dimensions() != gltf::accessor::Dimensions::Vec3 {
                return Err(Error::Invalid("glTF instance vector"));
            }
            let values = gltf::accessor::Iter::<[f32; 3]>::new(accessor, data)
                .ok_or(Error::Invalid("glTF instance data"))?
                .collect::<Vec<_>>();
            if name == "TRANSLATION" {
                translations = Some(values)
            } else {
                scales = Some(values)
            }
        }
    }
    let count = count.ok_or(Error::Invalid("glTF instance transform"))?;
    (0..count)
        .map(|i| {
            let position = Vector3::from_array(
                translations
                    .as_ref()
                    .map_or([0.0; 3], |v| v[i])
                    .map(f64::from),
            );
            let scale =
                Vector3::from_array(scales.as_ref().map_or([1.0; 3], |v| v[i]).map(f64::from));
            let q = rotations
                .as_ref()
                .map_or([0.0, 0.0, 0.0, 1.0], |v| v[i])
                .map(f64::from);
            let rotation = Quaternion::from_array(q);
            if !rotation.is_finite()
                || rotation.length_squared() < 1e-20
                || !position.is_finite()
                || !scale.is_finite()
                || scale.min_element() <= 0.0
            {
                return Err(Error::Invalid("glTF instance transform"));
            }
            Ok(crate::scene::Instance {
                matrix: Matrix4::from_scale_rotation_translation(scale, rotation, position),
                color: Color::WHITE,
            })
        })
        .collect()
}
