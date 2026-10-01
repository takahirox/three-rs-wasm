//! webgpu_skinning_instancing_individual: thirty dancing Michelles, each with
//! its own animation time, body proportions and belly morph, skinned in one
//! compute pass and drawn as one instanced mesh. Per frame, as the page does,
//! the mixer is set to each instance's time on the CPU, the proportion bones
//! are rescaled, and the skeleton's bone matrices and the instance's foot-
//! grounded matrix are stored; a compute kernel then skins every instance's
//! vertices ( with the belly morph ) into a storage buffer the vertex stage
//! reads. A hemisphere light, the SunLight addon's two fitted cascades ( cast
//! onto a shadow-catcher ground ), a rim light and the RoomEnvironment PMREM
//! light them over a screen-space gradient, with 4× MSAA and Neutral tone
//! mapping. Every stage runs the WGSL three.js r186 generates for the page (in
//! `skinning_instances/`, the compute kernel without its unused subgroup
//! built-in; the output is the compute_cloth modules, byte-identical).
use super::controls_attributes::{Controls, camera_state};
use super::deferred::{Draw, sampled_pipeline, set};
use super::fog_volume::{Cascade, cascades};
use super::gltf_viewer::load_asset;
use super::lights_projector::{m3, m4, pack};
use super::pmrem_cube_uv::bind;
use super::retro::{mipmapped, uniform};
use super::shadowmap_opacity::Mipmaps;
use super::three_mixer::ThreeMixer;
use crate::{Error, Result, camera::*, math::*, render_target::*, renderer::*, scene::*};
use std::f64::consts::PI;
use wgpu::util::DeviceExt;

const HALF: wgpu::TextureFormat = wgpu::TextureFormat::Rgba16Float;
const BYTE: wgpu::TextureFormat = wgpu::TextureFormat::Rgba8Unorm;
const SRGB: wgpu::TextureFormat = wgpu::TextureFormat::Rgba8UnormSrgb;
const DEPTH: wgpu::TextureFormat = wgpu::TextureFormat::Depth24Plus;
const INSTANCES: usize = 30;
const MAP: u32 = 2048;
const SUN: [f64; 3] = [-4., 10., 8.];
macro_rules! wgsl {
    ($name:literal) => {
        include_str!(concat!("skinning_instances/", $name, ".wgsl"))
    };
}
const OUTPUT_VS: &str = include_str!("compute_cloth/output_vs.wgsl");
const OUTPUT_FS: &str = include_str!("compute_cloth/output_fs.wgsl");
fn linear(hex: u32, intensity: f64) -> Vec<f64> {
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
/// A body type: width, height, depth, belly, head scale, belly width, chest
/// width, arm length and leg length.
const BODY_TYPES: [[f64; 9]; 5] = [
    [0.92, 1.09, 0.92, 0., 0.96, 0.98, 0.98, 1.03, 1.04],
    [1.0, 1.05, 0.99, 0., 0.98, 1.0, 1.03, 1.02, 1.02],
    [1.0, 1.0, 1.0, 0., 1.0, 1.0, 1.0, 1.0, 1.0],
    [1.08, 0.9, 1.1, 0.75, 1.06, 1.12, 1.0, 0.9, 0.9],
    [1.2, 0.8, 1.22, 1.6, 1.12, 1.24, 1.04, 0.72, 0.7],
];
/// One instance: its body type, size, ring position and time offset.
struct Variation {
    body: [f64; 9],
    size: f64,
    x: f64,
    y: f64,
    offset: f64,
}
struct Targets {
    width: u32,
    height: u32,
    format: wgpu::TextureFormat,
    samples: u32,
    msaa: Option<(wgpu::TextureView, wgpu::TextureView)>,
    color: wgpu::TextureView,
    depth: wgpu::TextureView,
    background: Draw,
    michelle: Draw,
    ground: Draw,
    output: Draw,
    screen: RenderTarget,
}
pub(super) struct Demo {
    controls: Controls,
    time: f64,
    pending: bool,
    mixer: ThreeMixer,
    skinned: Object3D,
    parent: Object3D,
    bones: Vec<(Object3D, Matrix4)>,
    /// The head, belly, chest, arms ( 4 ) and legs ( 4 ) and their scales.
    proportion: Vec<(Object3D, Vector3)>,
    feet: [Object3D; 2],
    ground_foot: f64,
    variations: Vec<Variation>,
    vertex_count: u32,
    /// Position, uv, index, count and normal ( the main pass binds all three ).
    michelle: (wgpu::Buffer, wgpu::Buffer, wgpu::Buffer, u32, wgpu::Buffer),
    background_mesh: (wgpu::Buffer, wgpu::Buffer, u32),
    ground_mesh: (wgpu::Buffer, wgpu::Buffer, wgpu::Buffer),
    bone_buffer: wgpu::Buffer,
    instance_buffer: wgpu::Buffer,
    skinning: (wgpu::ComputePipeline, wgpu::BindGroup),
    compute_object: wgpu::Buffer,
    shadow: (wgpu::TextureView, wgpu::TextureView),
    shadow_draws: [Draw; 2],
    shadow_renders: [wgpu::Buffer; 2],
    shadow_object: wgpu::Buffer,
    michelle_render: wgpu::Buffer,
    michelle_object: wgpu::Buffer,
    background_render: wgpu::Buffer,
    background_object: wgpu::Buffer,
    ground_render: wgpu::Buffer,
    ground_object: wgpu::Buffer,
    output_render: wgpu::Buffer,
    output_object: wgpu::Buffer,
    maps: [wgpu::TextureView; 3],
    vertices: wgpu::Buffer,
    pmrem: wgpu::TextureView,
    quad_uv: wgpu::Buffer,
    clamp: wgpu::Sampler,
    repeat: wgpu::Sampler,
    compare: wgpu::Sampler,
    targets: Option<Targets>,
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, _id: u32, r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 45.,
            near: 0.01,
            far: 60.,
            aspect,
            ..Default::default()
        }));
        s.get_mut(c)?.position = Vector3::new(4.6, 3.3, 6.2);
        let mut controls = Controls::new(Some(0.05), (3., 25.), PI / 2., true);
        controls.set_target(Vector3::new(0., 0.75, 0.));
        controls.update(s, c)?;
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
        let storage = wgpu::BufferUsages::STORAGE;
        let (asset, buffers, images) =
            load_asset("/web/gallery/assets/tsl-viewport/models/gltf/Michelle.glb").await?;
        let primitive = asset
            .meshes()
            .next()
            .and_then(|m| m.primitives().next())
            .ok_or(Error::Invalid("Michelle mesh"))?;
        let reader = primitive.reader(|b| buffers.get(b.index()).map(Vec::as_slice));
        let positions: Vec<f32> = reader
            .read_positions()
            .ok_or(Error::Invalid("Michelle positions"))?
            .flatten()
            .collect();
        let normals: Vec<f32> = reader
            .read_normals()
            .ok_or(Error::Invalid("Michelle normals"))?
            .flatten()
            .collect();
        let uv: Vec<f32> = reader
            .read_tex_coords(0)
            .ok_or(Error::Invalid("Michelle uv"))?
            .into_f32()
            .flatten()
            .collect();
        let joints: Vec<u32> = reader
            .read_joints(0)
            .ok_or(Error::Invalid("Michelle joints"))?
            .into_u16()
            .flat_map(|j| j.map(u32::from))
            .collect();
        // normalizeSkinWeights, as GLTFLoader applies it.
        let weights: Vec<f32> = reader
            .read_weights(0)
            .ok_or(Error::Invalid("Michelle weights"))?
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
        let index: Vec<u32> = reader
            .read_indices()
            .ok_or(Error::Invalid("Michelle index"))?
            .into_u32()
            .collect();
        let vertex_count = (positions.len() / 3) as u32;
        // createSourceVertexAttribute: position, normal and the belly offset
        // ( x and z scaled by a band around y = 0.94 ), as vec4s.
        let mut source = Vec::with_capacity(vertex_count as usize * 12);
        for i in 0..vertex_count as usize {
            let (x, y, z) = (positions[i * 3], positions[i * 3 + 1], positions[i * 3 + 2]);
            let amount = (1. - (y as f64 - 0.94).abs() / 0.3).max(0.) * 0.8;
            source.extend([x, y, z, 0.]);
            source.extend([normals[i * 3], normals[i * 3 + 1], normals[i * 3 + 2], 0.]);
            source.extend([
                (x as f64 * amount) as f32,
                0.,
                (z as f64 * amount) as f32,
                0.,
            ]);
        }
        let skin = asset
            .skins()
            .next()
            .ok_or(Error::Invalid("Michelle skin"))?;
        let inverses: Vec<Matrix4> = skin
            .reader(|b| buffers.get(b.index()).map(Vec::as_slice))
            .read_inverse_bind_matrices()
            .ok_or(Error::Invalid("Michelle inverse bind matrices"))?
            .map(|m| Matrix4::from_cols_array(&std::array::from_fn(|i| m[i / 4][i % 4] as f64)))
            .collect();
        let material = primitive.material();
        let pbr = material.pbr_metallic_roughness();
        let source_image = |t: Option<gltf::Texture>| -> Result<usize> {
            t.map(|t| t.source().index())
                .ok_or(Error::Invalid("Michelle map"))
        };
        let base = source_image(pbr.base_color_texture().map(|t| t.texture()))?;
        let orm = source_image(pbr.metallic_roughness_texture().map(|t| t.texture()))?;
        let specular = material
            .specular()
            .and_then(|s| s.specular_color_texture())
            .map(|t| t.texture().source().index())
            .ok_or(Error::Invalid("Michelle specular map"))?;
        let mut mipmaps = Mipmaps::new(r);
        let image = |i: usize| images.get(i).ok_or(Error::Invalid("Michelle image"));
        let maps = [
            mipmapped(r, &mut mipmaps, image(base)?, SRGB),
            mipmapped(r, &mut mipmaps, image(orm)?, BYTE),
            mipmapped(r, &mut mipmaps, image(specular)?, SRGB),
        ];
        let instance =
            crate::gltf::import_animated_decoded(&asset, &buffers, &images)?.instantiate(s)?;
        let root = s.insert(NodeKind::Group);
        for &h in &instance.roots {
            s.add(root, h)?;
        }
        let skinned = *instance
            .meshes
            .first()
            .ok_or(Error::Invalid("Michelle skinned mesh"))?;
        let parent = s
            .get(skinned)?
            .parent()
            .ok_or(Error::Invalid("Michelle mesh parent"))?;
        let joints_nodes: Vec<Object3D> =
            skin.joints().map(|j| instance.nodes[j.index()]).collect();
        let bones: Vec<(Object3D, Matrix4)> = joints_nodes.iter().copied().zip(inverses).collect();
        let bone = |name: &str| -> Result<Object3D> {
            for &h in &joints_nodes {
                if s.get(h)?.name.ends_with(name) {
                    return Ok(h);
                }
            }
            Err(Error::Invalid("Michelle bone"))
        };
        let proportion = [
            "Head",
            "Spine",
            "Spine2",
            "LeftArm",
            "RightArm",
            "LeftForeArm",
            "RightForeArm",
            "LeftUpLeg",
            "RightUpLeg",
            "LeftLeg",
            "RightLeg",
        ]
        .iter()
        .map(|n| -> Result<(Object3D, Vector3)> {
            let h = bone(n)?;
            Ok((h, s.get(h)?.scale))
        })
        .collect::<Result<Vec<_>>>()?;
        let feet = [bone("LeftFoot")?, bone("RightFoot")?];
        let mut mixer = ThreeMixer::new();
        let clip = instance.clips[0].clone();
        let duration = clip.duration();
        let action = mixer.clip_action(s, clip, false)?;
        mixer.play(s, action)?;
        let variations: Vec<Variation> = (0..INSTANCES)
            .map(|i| {
                let size = if i % 2 == 0 {
                    if i % 4 == 0 { 0.68 } else { 0.78 }
                } else {
                    1.
                };
                let inner = i < 10;
                let (ring, count) = if inner { (i, 10) } else { (i - 10, 20) };
                let angle = ring as f64 / count as f64 * PI * 2.;
                let radius = if inner { 175. } else { 340. };
                Variation {
                    body: BODY_TYPES[i % BODY_TYPES.len()],
                    size,
                    x: angle.sin() * radius,
                    y: angle.cos() * radius,
                    offset: duration * i as f64 / INSTANCES as f64,
                }
            })
            .collect();
        let belly: Vec<f32> = variations.iter().map(|v| v.body[3] as f32).collect();
        s.update()?;
        let ground_foot = foot(s, feet, skinned)?;
        let pmrem = super::room_environment::environment(r)?
            .gpu
            .as_ref()
            .ok_or(Error::Invalid("room environment"))?
            .view
            .clone();
        let bone_buffer = r.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Michelle bone matrices"),
            size: (INSTANCES * bones.len() * 64) as u64,
            usage: storage | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let instance_buffer = r.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Michelle instance matrices"),
            size: (INSTANCES * 64) as u64,
            usage: storage | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let vertices = r.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Michelle skinned vertices"),
            size: INSTANCES as u64 * vertex_count as u64 * 2 * 16,
            usage: storage,
            mapped_at_creation: false,
        });
        let compute_object = uniform(r, "Michelle skinning", wgsl!("skinning_cs"), "objectStruct")?;
        let module = r.device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Michelle skinning"),
            source: wgpu::ShaderSource::Wgsl(wgsl!("skinning_cs").into()),
        });
        let pipeline = r
            .device
            .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                label: Some("Michelle skinning"),
                layout: None,
                module: &module,
                entry_point: Some("main"),
                compilation_options: Default::default(),
                cache: None,
            });
        let skin_indices = init(
            "Michelle skin index",
            bytemuck::cast_slice(&joints),
            storage,
        );
        let skin_weights = init(
            "Michelle skin weight",
            bytemuck::cast_slice(&weights),
            storage,
        );
        let source_vertices = init("Michelle source", bytemuck::cast_slice(&source), storage);
        let belly_weights = init("Michelle belly", bytemuck::cast_slice(&belly), storage);
        let skinning_group = bind(
            r,
            pipeline.get_bind_group_layout(0),
            &[
                (0, vertices.as_entire_binding()),
                (1, instance_buffer.as_entire_binding()),
                (2, compute_object.as_entire_binding()),
                (3, bone_buffer.as_entire_binding()),
                (4, skin_indices.as_entire_binding()),
                (5, skin_weights.as_entire_binding()),
                (6, source_vertices.as_entire_binding()),
                (7, belly_weights.as_entire_binding()),
            ],
        );
        let michelle = (
            init(
                "Michelle positions",
                bytemuck::cast_slice(&positions),
                vertex,
            ),
            init("Michelle uv", bytemuck::cast_slice(&uv), vertex),
            init("Michelle index", bytemuck::cast_slice(&index), index_usage),
            index.len() as u32,
            init("Michelle normals", bytemuck::cast_slice(&normals), vertex),
        );
        // The background's SphereGeometry( 1, 32, 32 ) ( positions only ).
        let sphere = crate::geometry::SphereGeometry::build(1., 32, 32)?;
        let sphere_positions: Vec<f32> = {
            let a = sphere
                .attributes
                .get("position")
                .ok_or(Error::Invalid("background positions"))?;
            (0..a.count())
                .flat_map(|i| (0..3).map(move |k| (i, k)))
                .map(|(i, k)| a.get_component(i, k).map(|v| v as f32))
                .collect::<Result<_>>()?
        };
        let sphere_index = sphere
            .index
            .clone()
            .ok_or(Error::Invalid("background index"))?;
        let background_mesh = (
            init(
                "background",
                bytemuck::cast_slice(&sphere_positions),
                vertex,
            ),
            init(
                "background index",
                bytemuck::cast_slice(&sphere_index),
                index_usage,
            ),
            sphere_index.len() as u32,
        );
        // PlaneGeometry( 400, 400 ): normal and position.
        let ground_mesh = (
            init(
                "ground normals",
                bytemuck::cast_slice(&[0f32, 0., 1., 0., 0., 1., 0., 0., 1., 0., 0., 1.]),
                vertex,
            ),
            init(
                "ground positions",
                bytemuck::cast_slice(&[
                    -200f32, 200., 0., 200., 200., 0., -200., -200., 0., 200., -200., 0.,
                ]),
                vertex,
            ),
            init(
                "ground index",
                bytemuck::cast_slice(&[0u32, 2, 1, 2, 3, 1]),
                index_usage,
            ),
        );
        let texture = |size: (u32, u32), format| {
            r.device
                .create_texture(&wgpu::TextureDescriptor {
                    label: Some("skinning shadow"),
                    size: wgpu::Extent3d {
                        width: size.0,
                        height: size.1,
                        depth_or_array_layers: 1,
                    },
                    mip_level_count: 1,
                    sample_count: 1,
                    dimension: wgpu::TextureDimension::D2,
                    format,
                    usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                        | wgpu::TextureUsages::TEXTURE_BINDING,
                    view_formats: &[],
                })
                .create_view(&Default::default())
        };
        let shadow = (
            texture((MAP * 2, MAP), BYTE),
            texture((MAP * 2, MAP), DEPTH),
        );
        let repeat = r.device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::Repeat,
            address_mode_v: wgpu::AddressMode::Repeat,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        let clamp = r.device.create_sampler(&wgpu::SamplerDescriptor {
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        let compare = r.device.create_sampler(&wgpu::SamplerDescriptor {
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            compare: Some(wgpu::CompareFunction::LessEqual),
            ..Default::default()
        });
        let skin_attrs = [
            wgpu::vertex_attr_array![0 => Float32x3],
            wgpu::vertex_attr_array![1 => Float32x2],
        ];
        let skin_layouts = [(0, 12), (1, 8)].map(|(i, stride)| wgpu::VertexBufferLayout {
            array_stride: stride,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &skin_attrs[i],
        });
        // The instanced mesh casts shadows without culling ( double-sided ).
        let shadow_pipeline = no_cull(
            r,
            (wgsl!("shadow_vs"), wgsl!("shadow_fs")),
            &skin_layouts,
            BYTE,
            1,
        );
        let shadow_renders = [
            uniform(r, "skinning shadow", wgsl!("shadow_vs"), "renderStruct")?,
            uniform(r, "skinning shadow", wgsl!("shadow_vs"), "renderStruct")?,
        ];
        let shadow_object = uniform(r, "skinning shadow", wgsl!("shadow_vs"), "objectStruct")?;
        let shadow_draws = shadow_renders.each_ref().map(|render| {
            (
                shadow_pipeline.clone(),
                vec![
                    bind(
                        r,
                        shadow_pipeline.get_bind_group_layout(0),
                        &[(0, render.as_entire_binding())],
                    ),
                    bind(
                        r,
                        shadow_pipeline.get_bind_group_layout(1),
                        &[
                            (0, wgpu::BindingResource::Sampler(&repeat)),
                            (1, wgpu::BindingResource::TextureView(&maps[0])),
                            (2, shadow_object.as_entire_binding()),
                            (3, vertices.as_entire_binding()),
                        ],
                    ),
                ],
            )
        });
        Ok(Self {
            controls,
            time: 0.,
            pending: true,
            mixer,
            skinned,
            parent,
            bones,
            proportion,
            feet,
            ground_foot,
            variations,
            vertex_count,
            michelle,
            background_mesh,
            ground_mesh,
            bone_buffer,
            instance_buffer,
            skinning: (pipeline, skinning_group),
            compute_object,
            shadow,
            shadow_draws,
            shadow_renders,
            shadow_object,
            michelle_render: uniform(r, "skinning Michelle", wgsl!("michelle_fs"), "renderStruct")?,
            michelle_object: uniform(r, "skinning Michelle", wgsl!("michelle_fs"), "objectStruct")?,
            background_render: uniform(
                r,
                "skinning background",
                wgsl!("background_fs"),
                "renderStruct",
            )?,
            background_object: uniform(
                r,
                "skinning background",
                wgsl!("background_fs"),
                "objectStruct",
            )?,
            ground_render: uniform(r, "skinning ground", wgsl!("ground_fs"), "renderStruct")?,
            ground_object: uniform(r, "skinning ground", wgsl!("ground_fs"), "objectStruct")?,
            output_render: uniform(r, "skinning output", OUTPUT_FS, "renderStruct")?,
            output_object: uniform(r, "skinning output", OUTPUT_VS, "objectStruct")?,
            maps,
            vertices,
            pmrem,
            // QuadMesh's triangle ( positions ) for the output pass.
            quad_uv: init(
                "skinning quad",
                bytemuck::cast_slice(&[-1f32, 3., 0., -1., -1., 0., 3., -1., 0.]),
                vertex,
            ),
            clamp,
            repeat,
            compare,
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
                    label: Some("skinning target"),
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
        let position_attrs = [wgpu::vertex_attr_array![0 => Float32x3]];
        let position_layout = [wgpu::VertexBufferLayout {
            array_stride: 12,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &position_attrs[0],
        }];
        let background_pipeline = sampled_pipeline(
            r,
            "skinning background",
            (wgsl!("background_vs"), wgsl!("background_fs")),
            &position_layout,
            &[HALF],
            Some((wgpu::CompareFunction::Always, false)),
            (true, false),
            (samples, wgpu::PrimitiveTopology::TriangleList),
        );
        let background = two(
            &background_pipeline,
            &self.background_render,
            &[(0, self.background_object.as_entire_binding())],
        );
        let skin_attrs = [
            wgpu::vertex_attr_array![0 => Float32x3],
            wgpu::vertex_attr_array![1 => Float32x2],
        ];
        let skin_layouts = [(0, 12), (1, 8)].map(|(i, stride)| wgpu::VertexBufferLayout {
            array_stride: stride,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &skin_attrs[i],
        });
        let normal_attrs = [wgpu::vertex_attr_array![2 => Float32x3]];
        let main_layouts = [
            skin_layouts[0].clone(),
            skin_layouts[1].clone(),
            wgpu::VertexBufferLayout {
                array_stride: 12,
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes: &normal_attrs[0],
            },
        ];
        let michelle_pipeline = no_cull(
            r,
            (wgsl!("michelle_vs"), wgsl!("michelle_fs")),
            &main_layouts,
            HALF,
            samples,
        );
        let michelle = two(
            &michelle_pipeline,
            &self.michelle_render,
            &[
                (0, self.michelle_object.as_entire_binding()),
                (1, sampler(&self.repeat)),
                (2, tex(&self.maps[0])),
                (3, sampler(&self.repeat)),
                (4, tex(&self.maps[1])),
                (5, sampler(&self.repeat)),
                (6, tex(&self.maps[2])),
                (7, sampler(&self.clamp)),
                (8, tex(&r.dfg)),
                (9, sampler(&self.clamp)),
                (10, tex(&self.pmrem)),
                (11, self.vertices.as_entire_binding()),
            ],
        );
        let ground_attrs = [
            wgpu::vertex_attr_array![0 => Float32x3],
            wgpu::vertex_attr_array![1 => Float32x3],
        ];
        let ground_layouts = [0, 1].map(|i| wgpu::VertexBufferLayout {
            array_stride: 12,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &ground_attrs[i],
        });
        // ShadowMaterial: transparent, blended over the scene.
        let ground_pipeline = sampled_pipeline(
            r,
            "skinning ground",
            (wgsl!("ground_vs"), wgsl!("ground_fs")),
            &ground_layouts,
            &[HALF],
            Some((wgpu::CompareFunction::LessEqual, true)),
            (false, true),
            (samples, wgpu::PrimitiveTopology::TriangleList),
        );
        let ground = two(
            &ground_pipeline,
            &self.ground_render,
            &[
                (0, self.ground_object.as_entire_binding()),
                (1, sampler(&self.compare)),
                (2, tex(&self.shadow.1)),
            ],
        );
        let quad_attrs = [wgpu::vertex_attr_array![0 => Float32x3]];
        let quad = [wgpu::VertexBufferLayout {
            array_stride: 12,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &quad_attrs[0],
        }];
        let format = out.options.format;
        let output_pipeline = sampled_pipeline(
            r,
            "skinning output",
            (OUTPUT_VS, OUTPUT_FS),
            &quad,
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
            background,
            michelle,
            ground,
            output,
            screen,
        });
        Ok(())
    }
    fn present(&self, encoder: &mut wgpu::CommandEncoder, t: &Targets) {
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("skinning output"),
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
        pass.set_vertex_buffer(0, self.quad_uv.slice(..));
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
            .ok_or(Error::Invalid("skinning targets"))?;
        if !std::mem::take(&mut self.pending) {
            let mut encoder = r.device.create_command_encoder(&Default::default());
            self.present(&mut encoder, t);
            r.queue.submit([encoder.finish()]);
            return Ok(true);
        }
        // animate(): the controls, then each instance's pose, proportions,
        // bone matrices and grounded instance matrix.
        self.controls.frame_update(s, c)?;
        let elapsed = self.time;
        let mut bone_data: Vec<f32> = Vec::with_capacity(INSTANCES * self.bones.len() * 16);
        let mut instance_data: Vec<f32> = Vec::with_capacity(INSTANCES * 16);
        for v in &self.variations {
            self.mixer.set_time(s, elapsed + v.offset)?;
            let [width, height, depth, _, head, belly, chest, arms, legs] = v.body;
            for (i, (h, base)) in self.proportion.iter().enumerate() {
                let mut scale = *base;
                match i {
                    0 => scale *= head,
                    1 => {
                        scale.x *= belly;
                        scale.z *= belly;
                    }
                    2 => {
                        scale.x *= chest;
                        scale.z *= chest;
                    }
                    3..=6 => scale.y *= arms,
                    _ => scale.y *= legs,
                }
                s.get_mut(*h)?.scale = scale;
            }
            s.update()?;
            for (h, inverse) in &self.bones {
                let m = s.get(*h)?.matrix_world * *inverse;
                bone_data.extend(m.to_cols_array().map(|x| x as f32));
            }
            let foot = foot(s, self.feet, self.skinned)?;
            let m = Matrix4::from_scale_rotation_translation(
                Vector3::new(width, depth, height) * v.size,
                Quaternion::IDENTITY,
                Vector3::new(v.x, v.y, self.ground_foot - foot * height * v.size),
            );
            instance_data.extend(m.to_cols_array().map(|x| x as f32));
        }
        r.queue
            .write_buffer(&self.bone_buffer, 0, bytemuck::cast_slice(&bone_data));
        r.queue.write_buffer(
            &self.instance_buffer,
            0,
            bytemuck::cast_slice(&instance_data),
        );
        let (camera, world) = s.camera(c)?;
        let (projection, view) = (camera.projection_matrix()?, world.inverse());
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
        let mesh_world = s.get(self.skinned)?.matrix_world;
        let parent_world = s.get(self.parent)?.matrix_world;
        write(
            &self.compute_object,
            wgsl!("skinning_cs"),
            "objectStruct",
            &[
                ("nodeUniform2", m4(mesh_world.inverse())),
                ("nodeUniform6", m4(Matrix4::IDENTITY)),
                (
                    "nodeUniform9",
                    vec![(INSTANCES as u32 * self.vertex_count) as f64],
                ),
            ],
        )?;
        let cascades: [Cascade; 2] =
            cascades(Vector3::from_array(SUN), world, projection, 0.01, 60., 30.);
        for (i, cascade) in cascades.iter().enumerate() {
            write(
                &self.shadow_renders[i],
                wgsl!("shadow_vs"),
                "renderStruct",
                &[
                    ("cameraProjectionMatrix", m4(cascade.projection)),
                    ("cameraViewMatrix", m4(cascade.view)),
                ],
            )?;
        }
        let identity3 = m3(Matrix4::IDENTITY);
        write(
            &self.shadow_object,
            wgsl!("shadow_vs"),
            "objectStruct",
            &[
                ("nodeUniform2", identity3.clone()),
                ("nodeUniform3", vec![1.]),
                ("nodeUniform6", m4(parent_world)),
            ],
        )?;
        let camera_values = vec![
            ("cameraProjectionMatrix", m4(projection)),
            ("cameraViewMatrix", m4(view)),
        ];
        let (sky, ground_color) = (linear(0xffffff, 1.2), linear(0x939b9e, 1.2));
        let mut michelle_values = camera_values.clone();
        michelle_values.extend([
            ("nodeUniform22", sky.clone()),
            ("nodeUniform24", vec![0., 1., 0.]),
            ("nodeUniform21", ground_color.clone()),
            ("nodeUniform26", linear(0xfffbf4, 2.8)),
            ("nodeUniform29", linear(0xe8f4ff, 0.85)),
            ("nodeUniform25", SUN.to_vec()),
            ("nodeUniform27", vec![7., 5., -5.]),
            ("nodeUniform28", vec![0.; 3]),
            ("cameraWorldMatrix", m4(world)),
        ]);
        write(
            &self.michelle_render,
            wgsl!("michelle_fs"),
            "renderStruct",
            &michelle_values,
        )?;
        write(
            &self.michelle_object,
            wgsl!("michelle_fs"),
            "objectStruct",
            &[
                ("nodeUniform1", vec![1.; 3]),
                ("nodeUniform3", identity3.clone()),
                ("nodeUniform4", vec![1.]),
                ("nodeUniform5", vec![0.5]),
                ("nodeUniform7", identity3.clone()),
                ("nodeUniform8", vec![1.]),
                ("nodeUniform9", identity3.clone()),
                ("nodeUniform11", m3(parent_world.inverse().transpose())),
                ("nodeUniform12", vec![1.45]),
                ("nodeUniform13", vec![1.; 3]),
                ("nodeUniform15", identity3.clone()),
                ("nodeUniform16", vec![1.]),
                ("nodeUniform17", vec![0.; 3]),
                ("nodeUniform18", vec![1.]),
                ("nodeUniform20", m4(parent_world)),
                ("nodeUniform30", vec![8.]),
                ("nodeUniform31", m4(Matrix4::IDENTITY)),
                ("nodeUniform33", vec![1. / 768.]),
                ("nodeUniform34", vec![1. / 1024.]),
                ("nodeUniform36", vec![1.]),
            ],
        )?;
        let mut background_values = vec![("nodeUniform1", vec![1.])];
        background_values.extend(camera_values.clone());
        background_values.push(("nodeUniform0", vec![t.width as f64, t.height as f64]));
        write(
            &self.background_render,
            wgsl!("background_fs"),
            "renderStruct",
            &background_values,
        )?;
        write(
            &self.background_object,
            wgsl!("background_fs"),
            "objectStruct",
            &[
                ("nodeUniform2", vec![1.]),
                ("nodeUniform5", m4(Matrix4::IDENTITY)),
            ],
        )?;
        let size = vec![(MAP * 2) as f64, MAP as f64];
        let mut ground_values = camera_values.clone();
        ground_values.extend([
            ("nodeUniform3", sky),
            ("nodeUniform7", vec![0., 1., 0.]),
            ("nodeUniform2", ground_color),
            ("nodeUniform11", m4(cascades[1].matrix)),
            ("nodeUniform10", cascades[1].data.to_vec()),
            ("nodeUniform17", m4(cascades[0].matrix)),
            ("nodeUniform16", cascades[0].data.to_vec()),
            ("nodeUniform9", vec![0.]),
            ("nodeUniform12", vec![0.]),
            ("nodeUniform14", vec![1.]),
            ("nodeUniform15", size.clone()),
            ("nodeUniform18", vec![0.]),
            ("nodeUniform19", vec![1.]),
            ("nodeUniform20", size),
            ("nodeUniform21", vec![1.]),
        ]);
        write(
            &self.ground_render,
            wgsl!("ground_fs"),
            "renderStruct",
            &ground_values,
        )?;
        let ground_model = Matrix4::from_rotation_x(-PI / 2.);
        write(
            &self.ground_object,
            wgsl!("ground_fs"),
            "objectStruct",
            &[
                ("nodeUniform0", linear(0x5d6568, 1.)),
                ("nodeUniform1", vec![0.3]),
                ("nodeUniform5", m3(ground_model.inverse().transpose())),
                ("nodeUniform8", m4(ground_model)),
            ],
        )?;
        write(
            &self.output_render,
            OUTPUT_FS,
            "renderStruct",
            &[
                // QuadMesh's orthographic camera ( −1 … 1, near 0, far 1 ).
                (
                    "cameraProjectionMatrix",
                    vec![
                        1., 0., 0., 0., 0., 1., 0., 0., 0., 0., -1., 0., 0., 0., 0., 1.,
                    ],
                ),
                ("cameraViewMatrix", m4(Matrix4::IDENTITY)),
                ("nodeUniform1", vec![t.width as f64, t.height as f64]),
                ("nodeUniform2", vec![0.96]),
            ],
        )?;
        write(
            &self.output_object,
            OUTPUT_VS,
            "objectStruct",
            &[("nodeUniform5", m4(Matrix4::IDENTITY))],
        )?;
        let mut encoder = r.device.create_command_encoder(&Default::default());
        {
            let mut pass = encoder.begin_compute_pass(&Default::default());
            pass.set_pipeline(&self.skinning.0);
            pass.set_bind_group(0, &self.skinning.1, &[]);
            pass.dispatch_workgroups((INSTANCES as u32 * self.vertex_count).div_ceil(64), 1, 1);
        }
        for (i, cascade) in cascades.iter().enumerate() {
            let clear = i == 0;
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("skinning shadow"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &self.shadow.0,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: if clear {
                            wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT)
                        } else {
                            wgpu::LoadOp::Load
                        },
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.shadow.1,
                    depth_ops: Some(wgpu::Operations {
                        load: if clear {
                            wgpu::LoadOp::Clear(1.)
                        } else {
                            wgpu::LoadOp::Load
                        },
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                ..Default::default()
            });
            let [x, y, w, h] = cascade.viewport;
            pass.set_viewport(x, y, w, h, 0., 1.);
            set(&mut pass, &self.shadow_draws[i]);
            pass.set_vertex_buffer(0, self.michelle.0.slice(..));
            pass.set_vertex_buffer(1, self.michelle.1.slice(..));
            pass.set_index_buffer(self.michelle.2.slice(..), wgpu::IndexFormat::Uint32);
            pass.draw_indexed(0..self.michelle.3, 0, 0..INSTANCES as u32);
        }
        {
            let (target, resolve, depth) = match &t.msaa {
                Some((color, depth)) => (color, Some(&t.color), depth),
                None => (&t.color, None, &t.depth),
            };
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("skinning scene"),
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
            set(&mut pass, &t.background);
            pass.set_vertex_buffer(0, self.background_mesh.0.slice(..));
            pass.set_index_buffer(self.background_mesh.1.slice(..), wgpu::IndexFormat::Uint32);
            pass.draw_indexed(0..self.background_mesh.2, 0, 0..1);
            set(&mut pass, &t.michelle);
            pass.set_vertex_buffer(0, self.michelle.0.slice(..));
            pass.set_vertex_buffer(1, self.michelle.1.slice(..));
            pass.set_vertex_buffer(2, self.michelle.4.slice(..));
            pass.set_index_buffer(self.michelle.2.slice(..), wgpu::IndexFormat::Uint32);
            pass.draw_indexed(0..self.michelle.3, 0, 0..INSTANCES as u32);
            set(&mut pass, &t.ground);
            pass.set_vertex_buffer(0, self.ground_mesh.0.slice(..));
            pass.set_vertex_buffer(1, self.ground_mesh.1.slice(..));
            pass.set_index_buffer(self.ground_mesh.2.slice(..), wgpu::IndexFormat::Uint32);
            pass.draw_indexed(0..6, 0, 0..1);
        }
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
        Err(Error::Invalid("skinning instancing parameter"))
    }
    pub fn seek(&mut self, t: f64) {
        self.time = t;
        self.pending = true;
    }
}
/// getFootCoordinate(): the higher foot's z in the reference mesh's space.
fn foot(s: &Scene, feet: [Object3D; 2], mesh: Object3D) -> Result<f64> {
    let to_local = s.get(mesh)?.matrix_world.inverse();
    let mut z = f64::NEG_INFINITY;
    for f in feet {
        let p = s.get(f)?.matrix_world.w_axis.truncate();
        z = z.max(to_local.transform_point3(p).z);
    }
    Ok(z)
}
/// A pipeline without culling for the double-sided instanced mesh.
fn no_cull(
    r: &Renderer,
    (vs, fs): (&str, &str),
    buffers: &[wgpu::VertexBufferLayout],
    format: wgpu::TextureFormat,
    samples: u32,
) -> wgpu::RenderPipeline {
    let module = |source: &str| {
        r.device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("skinning instances"),
            source: wgpu::ShaderSource::Wgsl(source.into()),
        })
    };
    r.device
        .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("skinning instances"),
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
                targets: &[Some(format.into())],
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
