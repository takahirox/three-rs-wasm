//! webgpu_shadowmap_opacity: two transmissive dragons (KHR_materials_volume
//! with a thickness map and yellow or red attenuation) over a textured cloth
//! backdrop, lit by an ambient light and a directional light whose shadow map
//! also records the casters' colors (shadowMap.transmitted), so the dragons
//! throw tinted shadows. The shadow is rendered once (autoUpdate off). Each
//! frame draws the backdrop, copies the resolved frame into a mipmapped
//! texture, draws the dragons refracting it through their volumes, and
//! applies AgX tone mapping (exposure 1.5). Every stage runs the WGSL
//! three.js r186 generates for the page (in `shadowmap_opacity/`).
use super::controls_attributes::{Controls, camera_state};
use super::gltf_viewer::load_asset;
use super::lights_projector::{m3, m4, pack};
use crate::{Error, Result, camera::*, math::*, render_target::*, renderer::*, scene::*};
use std::f64::consts::PI;
use wgpu::util::DeviceExt;

const HALF: wgpu::TextureFormat = wgpu::TextureFormat::Rgba16Float;
const DEPTH: wgpu::TextureFormat = wgpu::TextureFormat::Depth24Plus;
const SHADOW_SIZE: u32 = 2048;
macro_rules! wgsl {
    ($name:literal) => {
        include_str!(concat!("shadowmap_opacity/", $name, ".wgsl"))
    };
}
fn uniform(r: &Renderer, label: &str, size: u64) -> wgpu::Buffer {
    r.device.create_buffer(&wgpu::BufferDescriptor {
        label: Some(label),
        size,
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    })
}
fn bind(
    r: &Renderer,
    layout: wgpu::BindGroupLayout,
    entries: &[(u32, wgpu::BindingResource)],
) -> wgpu::BindGroup {
    r.device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: None,
        layout: &layout,
        entries: &entries
            .iter()
            .map(|(binding, resource)| wgpu::BindGroupEntry {
                binding: *binding,
                resource: resource.clone(),
            })
            .collect::<Vec<_>>(),
    })
}
#[allow(clippy::too_many_arguments)]
fn texture(
    r: &Renderer,
    size: (u32, u32),
    format: wgpu::TextureFormat,
    samples: u32,
    mips: u32,
    usage: wgpu::TextureUsages,
) -> wgpu::Texture {
    r.device.create_texture(&wgpu::TextureDescriptor {
        label: Some("opacity texture"),
        size: wgpu::Extent3d {
            width: size.0,
            height: size.1,
            depth_or_array_layers: 1,
        },
        mip_level_count: mips,
        sample_count: samples,
        dimension: wgpu::TextureDimension::D2,
        format,
        usage,
        view_formats: &[],
    })
}
fn view(t: &wgpu::Texture) -> wgpu::TextureView {
    t.create_view(&Default::default())
}
pub(super) fn mip_count(size: (u32, u32)) -> u32 {
    32 - size.0.max(size.1).leading_zeros()
}
/// WebGPUTexturePassUtils.generateMipmaps: each level is drawn from the
/// previous one through the linear-minifying sampler.
pub(super) struct Mipmaps {
    pipelines: Vec<(wgpu::TextureFormat, wgpu::RenderPipeline)>,
    sampler: wgpu::Sampler,
    flip: wgpu::Buffer,
}
impl Mipmaps {
    pub(super) fn new(r: &Renderer) -> Self {
        let flip = uniform(r, "mipmap flipY", 16);
        Self {
            pipelines: vec![],
            sampler: r.device.create_sampler(&wgpu::SamplerDescriptor {
                min_filter: wgpu::FilterMode::Linear,
                ..Default::default()
            }),
            flip,
        }
    }
    fn pipeline(&mut self, r: &Renderer, format: wgpu::TextureFormat) -> usize {
        if let Some(i) = self.pipelines.iter().position(|(f, _)| *f == format) {
            return i;
        }
        let module = r.device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("mipmap"),
            source: wgpu::ShaderSource::Wgsl(wgsl!("mipmap").into()),
        });
        let pipeline = r
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("mipmap"),
                layout: None,
                vertex: wgpu::VertexState {
                    module: &module,
                    entry_point: Some("mainVS"),
                    compilation_options: Default::default(),
                    buffers: &[],
                },
                fragment: Some(wgpu::FragmentState {
                    module: &module,
                    entry_point: Some("main_2d"),
                    compilation_options: Default::default(),
                    targets: &[Some(format.into())],
                }),
                primitive: Default::default(),
                depth_stencil: None,
                multisample: Default::default(),
                multiview: None,
                cache: None,
            });
        self.pipelines.push((format, pipeline));
        self.pipelines.len() - 1
    }
    /// Bind groups drawing each level from the one above.
    pub(super) fn levels(
        &mut self,
        r: &Renderer,
        t: &wgpu::Texture,
    ) -> Vec<(wgpu::TextureView, wgpu::BindGroup)> {
        let i = self.pipeline(r, t.format());
        let level = |mip| {
            t.create_view(&wgpu::TextureViewDescriptor {
                base_mip_level: mip,
                mip_level_count: Some(1),
                ..Default::default()
            })
        };
        (1..t.mip_level_count())
            .map(|mip| {
                let source = level(mip - 1);
                let bind = bind(
                    r,
                    self.pipelines[i].1.get_bind_group_layout(0),
                    &[
                        (0, wgpu::BindingResource::Sampler(&self.sampler)),
                        (1, wgpu::BindingResource::TextureView(&source)),
                        (2, self.flip.as_entire_binding()),
                    ],
                );
                (level(mip), bind)
            })
            .collect()
    }
    pub(super) fn encode(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        format: wgpu::TextureFormat,
        levels: &[(wgpu::TextureView, wgpu::BindGroup)],
    ) {
        let Some((_, pipeline)) = self.pipelines.iter().find(|(f, _)| *f == format) else {
            return;
        };
        for (target, bind) in levels {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("mipmap"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: target,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                ..Default::default()
            });
            pass.set_pipeline(pipeline);
            pass.set_bind_group(0, bind, &[]);
            pass.draw(0..3, 0..1);
        }
    }
}
/// A glTF mesh: vertex buffers, index, bounds and placement.
struct Mesh {
    positions: wgpu::Buffer,
    normals: wgpu::Buffer,
    uvs: wgpu::Buffer,
    index: wgpu::Buffer,
    count: u32,
    center: Vector3,
    radius: f64,
    model: Matrix4,
}
struct Targets {
    width: u32,
    height: u32,
    format: wgpu::TextureFormat,
    samples: u32,
    color: wgpu::TextureView,
    resolve: Option<wgpu::Texture>,
    resolve_view: Option<wgpu::TextureView>,
    single: Option<wgpu::Texture>,
    depth: wgpu::TextureView,
    transmission: wgpu::Texture,
    transmission_levels: Vec<(wgpu::TextureView, wgpu::BindGroup)>,
    screen: RenderTarget,
    floor: (wgpu::RenderPipeline, wgpu::BindGroup, wgpu::BindGroup),
    dragon: (wgpu::RenderPipeline, wgpu::BindGroup, [wgpu::BindGroup; 2]),
    output: (wgpu::RenderPipeline, [wgpu::BindGroup; 2]),
}
pub(super) struct Demo {
    controls: Controls,
    floor: Mesh,
    dragon: Mesh,
    /// The dragons' model matrices and attenuation colors.
    dragons: [(Matrix4, [f64; 3]); 2],
    material: DragonMaterial,
    floor_roughness: f64,
    shadow_pending: bool,
    shadow_projection: Matrix4,
    shadow_view: Matrix4,
    shadow_color: wgpu::TextureView,
    shadow_depth: wgpu::TextureView,
    shadow_pipelines: [wgpu::RenderPipeline; 2],
    shadow_binds: [(wgpu::BindGroup, wgpu::BindGroup); 2],
    shadow_render: wgpu::Buffer,
    shadow_objects: [wgpu::Buffer; 2],
    floor_render: wgpu::Buffer,
    floor_object: wgpu::Buffer,
    dragon_render: wgpu::Buffer,
    dragon_objects: [wgpu::Buffer; 2],
    output_render: wgpu::Buffer,
    output_object: wgpu::Buffer,
    quad: wgpu::Buffer,
    /// The backdrop's color map and the dragons' thickness map (mipmapped).
    maps: [wgpu::TextureView; 2],
    map_sampler: wgpu::Sampler,
    linear: wgpu::Sampler,
    compare: wgpu::Sampler,
    mipmaps: Mipmaps,
    targets: Option<Targets>,
}
/// MeshPhysicalMaterial from KHR_materials_transmission and _volume.
struct DragonMaterial {
    roughness: f64,
    metalness: f64,
    transmission: f64,
    thickness: f64,
    attenuation_distance: f64,
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, _id: u32, r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 45.,
            near: 0.1,
            far: 40.,
            aspect,
            ..Default::default()
        }));
        s.get_mut(c)?.position = Vector3::new(-4., 2., 6.);
        let mut controls = Controls::new(None, (0.1, 10.), PI, true);
        controls.set_target(Vector3::ZERO);
        controls.update(s, c)?;
        let (asset, buffers, images) =
            load_asset("/web/gallery/assets/tsl-next/models/gltf/DragonAttenuation.glb").await?;
        let init = |label, data: &[u8], usage| {
            r.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some(label),
                    contents: data,
                    usage,
                })
        };
        let nodes: Vec<_> = asset.nodes().collect();
        let mesh = |node: &gltf::Node, model: Matrix4| -> Result<Mesh> {
            let primitive = node
                .mesh()
                .and_then(|m| m.primitives().next())
                .ok_or(Error::Invalid("glTF mesh"))?;
            let reader = primitive.reader(|b| buffers.get(b.index()).map(Vec::as_slice));
            let positions: Vec<[f32; 3]> = reader
                .read_positions()
                .ok_or(Error::Invalid("glTF positions"))?
                .collect();
            let normals: Vec<[f32; 3]> = reader
                .read_normals()
                .ok_or(Error::Invalid("glTF normals"))?
                .collect();
            let uvs: Vec<[f32; 2]> = reader
                .read_tex_coords(0)
                .ok_or(Error::Invalid("glTF uvs"))?
                .into_f32()
                .collect();
            let index: Vec<u32> = reader
                .read_indices()
                .ok_or(Error::Invalid("glTF index"))?
                .into_u32()
                .collect();
            // computeBoundingSphere: the box center and the farthest vertex.
            let (mut lo, mut hi) = (Vector3::splat(f64::MAX), Vector3::splat(f64::MIN));
            for p in &positions {
                let p = Vector3::new(p[0] as f64, p[1] as f64, p[2] as f64);
                lo = lo.min(p);
                hi = hi.max(p);
            }
            let center = (lo + hi) / 2.;
            let radius = positions
                .iter()
                .map(|p| (Vector3::new(p[0] as f64, p[1] as f64, p[2] as f64) - center).length())
                .fold(0., f64::max);
            Ok(Mesh {
                positions: init(
                    "opacity positions",
                    bytemuck::cast_slice(&positions),
                    wgpu::BufferUsages::VERTEX,
                ),
                normals: init(
                    "opacity normals",
                    bytemuck::cast_slice(&normals),
                    wgpu::BufferUsages::VERTEX,
                ),
                uvs: init(
                    "opacity uvs",
                    bytemuck::cast_slice(&uvs),
                    wgpu::BufferUsages::VERTEX,
                ),
                index: init(
                    "opacity index",
                    bytemuck::cast_slice(&index),
                    wgpu::BufferUsages::INDEX,
                ),
                count: index.len() as u32,
                center,
                radius,
                model,
            })
        };
        // gltf.scene at (0, 0, −0.5); the backdrop widened by 4 in x and y;
        // the dragons moved to (−1.5, −0.8, 1) and 4 to its right.
        let scene = Matrix4::from_translation(Vector3::new(0., 0., -0.5));
        let (t, q, sc) = nodes[0].transform().decomposed();
        let floor_model = scene
            * Matrix4::from_scale_rotation_translation(
                Vector3::new(sc[0] as f64 + 4., sc[1] as f64 + 4., sc[2] as f64),
                Quaternion::from_xyzw(q[0] as f64, q[1] as f64, q[2] as f64, q[3] as f64),
                Vector3::new(t[0] as f64, t[1] as f64, t[2] as f64),
            );
        let (_, q, sc) = nodes[1].transform().decomposed();
        let dragon_at = |x: f64| {
            scene
                * Matrix4::from_scale_rotation_translation(
                    Vector3::new(sc[0] as f64, sc[1] as f64, sc[2] as f64),
                    Quaternion::from_xyzw(q[0] as f64, q[1] as f64, q[2] as f64, q[3] as f64),
                    Vector3::new(x, -0.8, 1.),
                )
        };
        let floor = mesh(&nodes[0], floor_model)?;
        let dragon = mesh(&nodes[1], dragon_at(-1.5))?;
        let material = nodes[1]
            .mesh()
            .and_then(|m| m.primitives().next())
            .map(|p| p.material())
            .ok_or(Error::Invalid("dragon material"))?;
        let volume = material.extension_value("KHR_materials_volume");
        let number = |v: Option<&serde_json::Value>, key: &str, default: f64| {
            v.and_then(|v| v.get(key))
                .and_then(|v| v.as_f64())
                .unwrap_or(default)
        };
        let attenuation = volume
            .and_then(|v| v.get("attenuationColor"))
            .and_then(|v| v.as_array())
            .map(|a| {
                let c: Vec<f64> = a.iter().filter_map(|v| v.as_f64()).collect();
                [c[0], c[1], c[2]]
            })
            .unwrap_or([1.; 3]);
        let pbr = material.pbr_metallic_roughness();
        let dragon_material = DragonMaterial {
            roughness: pbr.roughness_factor() as f64,
            metalness: pbr.metallic_factor() as f64,
            transmission: number(
                material.extension_value("KHR_materials_transmission"),
                "transmissionFactor",
                0.,
            ),
            thickness: number(volume, "thicknessFactor", 0.),
            attenuation_distance: number(volume, "attenuationDistance", f64::INFINITY),
        };
        let floor_roughness = nodes[0]
            .mesh()
            .and_then(|m| m.primitives().next())
            .map_or(1., |p| {
                p.material().pbr_metallic_roughness().roughness_factor() as f64
            });
        // The backdrop's sRGB color map and the linear thickness map, with
        // mipmaps generated on the GPU.
        let mut mipmaps = Mipmaps::new(r);
        let mut encoder = r.device.create_command_encoder(&Default::default());
        let mut maps = vec![];
        for (image, format) in images.iter().zip([
            wgpu::TextureFormat::Rgba8UnormSrgb,
            wgpu::TextureFormat::Rgba8Unorm,
        ]) {
            let size = (image.width, image.height);
            let t = texture(
                r,
                size,
                format,
                1,
                mip_count(size),
                wgpu::TextureUsages::TEXTURE_BINDING
                    | wgpu::TextureUsages::COPY_DST
                    | wgpu::TextureUsages::RENDER_ATTACHMENT,
            );
            r.queue.write_texture(
                t.as_image_copy(),
                &image.rgba,
                wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(image.width * 4),
                    rows_per_image: Some(image.height),
                },
                wgpu::Extent3d {
                    width: image.width,
                    height: image.height,
                    depth_or_array_layers: 1,
                },
            );
            let levels = mipmaps.levels(r, &t);
            mipmaps.encode(&mut encoder, format, &levels);
            maps.push(view(&t));
        }
        r.queue.submit([encoder.finish()]);
        let maps: [wgpu::TextureView; 2] = maps
            .try_into()
            .map_err(|_| Error::Invalid("dragon textures"))?;
        let module = |label, source: &str| {
            r.device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some(label),
                source: wgpu::ShaderSource::Wgsl(source.into()),
            })
        };
        let position = wgpu::vertex_attr_array![0 => Float32x3];
        let shadow_pipeline = |fs: &str| {
            r.device
                .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    label: Some("opacity shadow"),
                    layout: None,
                    vertex: wgpu::VertexState {
                        module: &module("shadow", include_str!("shadowmap_vsm/depth_vs.wgsl")),
                        entry_point: Some("main"),
                        compilation_options: Default::default(),
                        buffers: &[wgpu::VertexBufferLayout {
                            array_stride: 12,
                            step_mode: wgpu::VertexStepMode::Vertex,
                            attributes: &position,
                        }],
                    },
                    fragment: Some(wgpu::FragmentState {
                        module: &module("shadow", fs),
                        entry_point: Some("main"),
                        compilation_options: Default::default(),
                        targets: &[Some(wgpu::TextureFormat::Rgba8Unorm.into())],
                    }),
                    // As the page's shadow pipelines: clockwise front faces.
                    primitive: wgpu::PrimitiveState {
                        front_face: wgpu::FrontFace::Cw,
                        cull_mode: Some(wgpu::Face::Back),
                        ..Default::default()
                    },
                    depth_stencil: Some(wgpu::DepthStencilState {
                        format: DEPTH,
                        depth_write_enabled: true,
                        depth_compare: wgpu::CompareFunction::LessEqual,
                        stencil: Default::default(),
                        bias: Default::default(),
                    }),
                    multisample: Default::default(),
                    multiview: None,
                    cache: None,
                })
        };
        let shadow_pipelines = [
            shadow_pipeline(wgsl!("shadow_fs")),
            shadow_pipeline(wgsl!("shadow_red_fs")),
        ];
        let shadow_render = uniform(r, "shadow render", 128);
        let shadow_objects = [0; 2].map(|_| uniform(r, "shadow object", 80));
        let shadow_binds = [0, 1].map(|i| {
            (
                bind(
                    r,
                    shadow_pipelines[i].get_bind_group_layout(0),
                    &[(0, shadow_render.as_entire_binding())],
                ),
                bind(
                    r,
                    shadow_pipelines[i].get_bind_group_layout(1),
                    &[(0, shadow_objects[i].as_entire_binding())],
                ),
            )
        });
        let attachment =
            wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING;
        let shadow_size = (SHADOW_SIZE, SHADOW_SIZE);
        // The directional shadow camera: orthographic ±5, near 0.1, far 50.
        let (near, far) = (0.1, 50.);
        let shadow_projection = Matrix4::from_cols_array(&[
            0.2,
            0.,
            0.,
            0.,
            0.,
            0.2,
            0.,
            0.,
            0.,
            0.,
            -1. / (far - near),
            0.,
            0.,
            0.,
            -near / (far - near),
            1.,
        ]);
        let shadow_view = Matrix4::look_at_rh(Vector3::new(3., 5., 17.), Vector3::ZERO, Vector3::Y);
        let linear = |mipmap| {
            r.device.create_sampler(&wgpu::SamplerDescriptor {
                mag_filter: wgpu::FilterMode::Linear,
                min_filter: wgpu::FilterMode::Linear,
                mipmap_filter: mipmap,
                ..Default::default()
            })
        };
        Ok(Self {
            controls,
            floor,
            dragons: [
                (dragon_at(-1.5), attenuation),
                (dragon_at(2.5), [1., 0., 0.]),
            ],
            dragon,
            material: dragon_material,
            floor_roughness,
            shadow_pending: true,
            shadow_projection,
            shadow_view,
            shadow_color: view(&texture(
                r,
                shadow_size,
                wgpu::TextureFormat::Rgba8Unorm,
                1,
                1,
                attachment,
            )),
            shadow_depth: view(&texture(r, shadow_size, DEPTH, 1, 1, attachment)),
            shadow_pipelines,
            shadow_binds,
            shadow_render,
            shadow_objects,
            floor_render: uniform(
                r,
                "floor render",
                pack(wgsl!("floor_fs"), "renderStruct", &[])?.len() as u64,
            ),
            floor_object: uniform(
                r,
                "floor object",
                pack(wgsl!("floor_fs"), "objectStruct", &[])?.len() as u64,
            ),
            dragon_render: uniform(
                r,
                "dragon render",
                pack(wgsl!("dragon_fs"), "renderStruct", &[])?.len() as u64,
            ),
            dragon_objects: [0; 2].map(|_| {
                uniform(
                    r,
                    "dragon object",
                    pack(wgsl!("dragon_fs"), "objectStruct", &[]).map_or(0, |v| v.len() as u64),
                )
            }),
            output_render: uniform(r, "output render", 144),
            output_object: uniform(r, "output object", 64),
            quad: init(
                "quad",
                bytemuck::cast_slice(&[-1f32, 3., 0., -1., -1., 0., 3., -1., 0.]),
                wgpu::BufferUsages::VERTEX,
            ),
            maps,
            map_sampler: linear(wgpu::FilterMode::Linear),
            linear: linear(wgpu::FilterMode::Nearest),
            compare: r.device.create_sampler(&wgpu::SamplerDescriptor {
                mag_filter: wgpu::FilterMode::Linear,
                min_filter: wgpu::FilterMode::Linear,
                compare: Some(wgpu::CompareFunction::LessEqual),
                ..Default::default()
            }),
            mipmaps,
            targets: None,
        })
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, _dt: f64, _animate: bool) -> Result<()> {
        Ok(())
    }
    pub fn prepare(&mut self, s: &mut Scene, c: Object3D, _r: &Renderer) -> Result<()> {
        self.controls.frame_update(s, c)
    }
    fn resize(&mut self, r: &Renderer, out: &RenderTarget) -> Result<()> {
        let size = (out.width, out.height);
        let samples = out.options.samples.max(1);
        let attachment = wgpu::TextureUsages::RENDER_ATTACHMENT;
        let sampled =
            attachment | wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_SRC;
        // The scene target: multisampled with a resolve texture, or single.
        let (color, resolve, single) = if samples > 1 {
            (
                view(&texture(r, size, HALF, samples, 1, attachment)),
                Some(texture(r, size, HALF, 1, 1, sampled)),
                None,
            )
        } else {
            let t = texture(r, size, HALF, 1, 1, sampled);
            (view(&t), None, Some(t))
        };
        let depth = view(&texture(r, size, DEPTH, samples, 1, attachment));
        let transmission = texture(
            r,
            size,
            HALF,
            1,
            mip_count(size),
            wgpu::TextureUsages::TEXTURE_BINDING
                | wgpu::TextureUsages::COPY_DST
                | wgpu::TextureUsages::RENDER_ATTACHMENT,
        );
        let transmission_levels = self.mipmaps.levels(r, &transmission);
        let transmission_view = view(&transmission);
        let module = |label, source: &str| {
            r.device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some(label),
                source: wgpu::ShaderSource::Wgsl(source.into()),
            })
        };
        let pipeline = |label,
                        vs: &str,
                        fs: &str,
                        buffers: &[wgpu::VertexBufferLayout],
                        format,
                        scene: bool| {
            r.device
                .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    label: Some(label),
                    layout: None,
                    vertex: wgpu::VertexState {
                        module: &module(label, vs),
                        entry_point: Some("main"),
                        compilation_options: Default::default(),
                        buffers,
                    },
                    fragment: Some(wgpu::FragmentState {
                        module: &module(label, fs),
                        entry_point: Some("main"),
                        compilation_options: Default::default(),
                        targets: &[Some(wgpu::ColorTargetState::from(format))],
                    }),
                    primitive: wgpu::PrimitiveState {
                        cull_mode: Some(wgpu::Face::Back),
                        ..Default::default()
                    },
                    depth_stencil: scene.then(|| wgpu::DepthStencilState {
                        format: DEPTH,
                        depth_write_enabled: true,
                        depth_compare: wgpu::CompareFunction::LessEqual,
                        stencil: Default::default(),
                        bias: Default::default(),
                    }),
                    multisample: wgpu::MultisampleState {
                        count: if scene { samples } else { 1 },
                        ..Default::default()
                    },
                    multiview: None,
                    cache: None,
                })
        };
        let vertex = |location, format, stride| {
            (
                [wgpu::VertexAttribute {
                    format,
                    offset: 0,
                    shader_location: location,
                }],
                stride,
            )
        };
        let v3 = wgpu::VertexFormat::Float32x3;
        let v2 = wgpu::VertexFormat::Float32x2;
        // The backdrop reads uv, normal, position; the dragons normal, uv,
        // position.
        let floor_attributes = [vertex(0, v2, 8), vertex(1, v3, 12), vertex(2, v3, 12)];
        let dragon_attributes = [vertex(0, v3, 12), vertex(1, v2, 8), vertex(2, v3, 12)];
        fn layouts(
            attributes: &[([wgpu::VertexAttribute; 1], u64); 3],
        ) -> [wgpu::VertexBufferLayout<'_>; 3] {
            attributes
                .each_ref()
                .map(|(a, stride)| wgpu::VertexBufferLayout {
                    array_stride: *stride,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: a,
                })
        }
        let sampler = |s| wgpu::BindingResource::Sampler(s);
        let tex = |v| wgpu::BindingResource::TextureView(v);
        let floor = pipeline(
            "opacity floor",
            wgsl!("floor_vs"),
            wgsl!("floor_fs"),
            &layouts(&floor_attributes),
            HALF,
            true,
        );
        let floor_binds = (
            bind(
                r,
                floor.get_bind_group_layout(0),
                &[(0, self.floor_render.as_entire_binding())],
            ),
            bind(
                r,
                floor.get_bind_group_layout(1),
                &[
                    (0, self.floor_object.as_entire_binding()),
                    (1, sampler(&self.map_sampler)),
                    (2, tex(&self.maps[0])),
                    (3, sampler(&self.linear)),
                    (4, tex(&r.dfg)),
                    (5, sampler(&self.linear)),
                    (6, tex(&self.shadow_color)),
                    (7, sampler(&self.compare)),
                    (8, tex(&self.shadow_depth)),
                ],
            ),
        );
        let dragon = pipeline(
            "opacity dragon",
            wgsl!("dragon_vs"),
            wgsl!("dragon_fs"),
            &layouts(&dragon_attributes),
            HALF,
            true,
        );
        let dragon_render = bind(
            r,
            dragon.get_bind_group_layout(0),
            &[(0, self.dragon_render.as_entire_binding())],
        );
        let dragon_objects = [0, 1].map(|i| {
            bind(
                r,
                dragon.get_bind_group_layout(1),
                &[
                    (0, self.dragon_objects[i].as_entire_binding()),
                    (1, sampler(&self.map_sampler)),
                    (2, tex(&self.maps[1])),
                    (3, sampler(&self.linear)),
                    (4, tex(&r.dfg)),
                    (5, sampler(&self.map_sampler)),
                    (6, tex(&transmission_view)),
                    (7, sampler(&self.linear)),
                    (8, tex(&self.shadow_color)),
                    (9, sampler(&self.compare)),
                    (10, tex(&self.shadow_depth)),
                ],
            )
        });
        let quad = [vertex(0, v3, 12)];
        let output = pipeline(
            "render output",
            include_str!("lights_projector/output_vs.wgsl"),
            wgsl!("output_fs"),
            &[wgpu::VertexBufferLayout {
                array_stride: 12,
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes: &quad[0].0,
            }],
            out.options.format,
            false,
        );
        let scene_view = match (&resolve, &single) {
            (Some(t), _) | (None, Some(t)) => view(t),
            _ => return Err(Error::Invalid("opacity target")),
        };
        let output_binds = [
            bind(
                r,
                output.get_bind_group_layout(0),
                &[(0, self.output_render.as_entire_binding())],
            ),
            bind(
                r,
                output.get_bind_group_layout(1),
                &[
                    (0, sampler(&self.linear)),
                    (1, tex(&scene_view)),
                    (2, self.output_object.as_entire_binding()),
                ],
            ),
        ];
        r.queue.write_buffer(
            &self.output_render,
            0,
            &pack(
                wgsl!("output_fs"),
                "renderStruct",
                &[
                    ("cameraProjectionMatrix", &m4(Matrix4::IDENTITY)),
                    ("cameraViewMatrix", &m4(Matrix4::IDENTITY)),
                    ("nodeUniform1", &[size.0 as f64, size.1 as f64]),
                    ("nodeUniform2", &[1.5]),
                ],
            )?,
        );
        r.queue.write_buffer(
            &self.output_object,
            0,
            bytemuck::cast_slice(&Matrix4::IDENTITY.to_cols_array().map(|v| v as f32)),
        );
        let screen = RenderTarget::with_options(
            &r.device,
            size.0,
            size.1,
            RenderTargetOptions {
                samples: 0,
                depth_buffer: false,
                ..out.options.clone()
            },
        )?;
        self.targets = Some(Targets {
            width: size.0,
            height: size.1,
            format: out.options.format,
            samples,
            color,
            resolve_view: resolve.as_ref().map(view),
            resolve,
            single,
            depth,
            transmission,
            transmission_levels,
            screen,
            floor: (floor, floor_binds.0, floor_binds.1),
            dragon: (dragon, dragon_render, dragon_objects),
            output: (output, output_binds),
        });
        Ok(())
    }
    pub fn render(
        &mut self,
        r: &Renderer,
        s: &mut Scene,
        c: Object3D,
        out: &RenderTarget,
    ) -> Result<bool> {
        if self.targets.as_ref().is_none_or(|t| {
            t.width != out.width
                || t.height != out.height
                || t.format != out.options.format
                || t.samples != out.options.samples.max(1)
        }) {
            self.resize(r, out)?;
        }
        s.update()?;
        let (camera, world) = s.camera(c)?;
        let (projection, view) = (camera.projection_matrix()?, world.inverse());
        let camera_position = world.transform_point3(Vector3::ZERO);
        let bias = Matrix4::from_cols_array(&[
            0.5, 0., 0., 0., 0., 0.5, 0., 0., 0., 0., 1., 0., 0.5, 0.5, 0., 1.,
        ]);
        let shadow_matrix = bias * self.shadow_projection * self.shadow_view;
        let light_color = Color::from_hex(0x6666ff).0.to_array().map(|v| v * 10.);
        let light = [3., 5., 17.];
        let size = self
            .targets
            .as_ref()
            .map_or((1, 1), |t| (t.width, t.height));
        let values: Vec<(&str, Vec<f64>)> = vec![
            ("cameraProjectionMatrix", m4(projection)),
            ("cameraViewMatrix", m4(view)),
            ("cameraPosition", camera_position.to_array().to_vec()),
        ];
        let lights = |ambient: &str, color: &str, position: &str, target: &str| {
            vec![
                (ambient.to_string(), vec![0.5; 3]),
                (color.to_string(), light_color.to_vec()),
                (position.to_string(), light.to_vec()),
                (target.to_string(), vec![0.; 3]),
            ]
        };
        let shadow =
            |matrix: &str, normal: &str, bias: &str, radius: &str, map: &str, intensity: &str| {
                vec![
                    (matrix.to_string(), m4(shadow_matrix)),
                    (normal.to_string(), vec![0.]),
                    (bias.to_string(), vec![0.]),
                    (radius.to_string(), vec![4.]),
                    (map.to_string(), vec![SHADOW_SIZE as f64; 2]),
                    (intensity.to_string(), vec![1.]),
                ]
            };
        let write = |buffer: &wgpu::Buffer,
                     source: &str,
                     name: &str,
                     extra: Vec<(String, Vec<f64>)>|
         -> Result<()> {
            let mut all: Vec<(&str, &[f64])> = values.iter().map(|(n, v)| (*n, &v[..])).collect();
            all.extend(extra.iter().map(|(n, v)| (n.as_str(), &v[..])));
            r.queue.write_buffer(buffer, 0, &pack(source, name, &all)?);
            Ok(())
        };
        let mut floor_render = lights(
            "nodeUniform12",
            "nodeUniform16",
            "nodeUniform14",
            "nodeUniform15",
        );
        floor_render.extend(shadow(
            "nodeUniform18",
            "nodeUniform19",
            "nodeUniform20",
            "nodeUniform22",
            "nodeUniform23",
            "nodeUniform24",
        ));
        write(
            &self.floor_render,
            wgsl!("floor_fs"),
            "renderStruct",
            floor_render,
        )?;
        let mut dragon_render = lights(
            "nodeUniform25",
            "nodeUniform28",
            "nodeUniform26",
            "nodeUniform27",
        );
        dragon_render.extend(shadow(
            "nodeUniform30",
            "nodeUniform31",
            "nodeUniform32",
            "nodeUniform34",
            "nodeUniform35",
            "nodeUniform36",
        ));
        dragon_render.push((
            "nodeUniform23".to_string(),
            vec![size.0 as f64, size.1 as f64],
        ));
        write(
            &self.dragon_render,
            wgsl!("dragon_fs"),
            "renderStruct",
            dragon_render,
        )?;
        let floor = &self.floor;
        write(
            &self.floor_object,
            wgsl!("floor_fs"),
            "objectStruct",
            vec![
                ("nodeUniform0".into(), vec![1.; 3]),
                ("nodeUniform2".into(), m3(Matrix4::IDENTITY)),
                ("nodeUniform3".into(), vec![1.]),
                ("nodeUniform4".into(), vec![0.]),
                ("nodeUniform5".into(), vec![self.floor_roughness]),
                ("nodeUniform7".into(), m3(floor.model.inverse().transpose())),
                ("nodeUniform9".into(), vec![1.]),
                ("nodeUniform11".into(), m4(floor.model)),
            ],
        )?;
        let m = &self.material;
        for (i, (model, attenuation)) in self.dragons.iter().enumerate() {
            write(
                &self.dragon_objects[i],
                wgsl!("dragon_fs"),
                "objectStruct",
                vec![
                    ("nodeUniform0".into(), vec![1.; 3]),
                    ("nodeUniform1".into(), vec![1.]),
                    ("nodeUniform2".into(), vec![m.metalness]),
                    ("nodeUniform3".into(), vec![m.roughness]),
                    ("nodeUniform5".into(), m3(model.inverse().transpose())),
                    ("nodeUniform6".into(), vec![1.5]),
                    ("nodeUniform7".into(), vec![1.; 3]),
                    ("nodeUniform8".into(), vec![1.]),
                    ("nodeUniform9".into(), vec![m.transmission]),
                    ("nodeUniform10".into(), vec![m.thickness]),
                    ("nodeUniform12".into(), m3(Matrix4::IDENTITY)),
                    ("nodeUniform13".into(), vec![m.attenuation_distance]),
                    ("nodeUniform14".into(), attenuation.to_vec()),
                    ("nodeUniform16".into(), vec![1.]),
                    ("nodeUniform19".into(), m4(*model)),
                    ("nodeUniform21".into(), m4(*model)),
                ],
            )?;
        }
        let mut encoder = r.device.create_command_encoder(&Default::default());
        // The shadow (map and caster colors) renders once.
        if std::mem::take(&mut self.shadow_pending) {
            let mut camera: Vec<f32> = self
                .shadow_projection
                .to_cols_array()
                .map(|v| v as f32)
                .to_vec();
            camera.extend(self.shadow_view.to_cols_array().map(|v| v as f32));
            r.queue
                .write_buffer(&self.shadow_render, 0, bytemuck::cast_slice(&camera));
            for (i, (model, _)) in self.dragons.iter().enumerate() {
                let mut object = vec![1f32, 0., 0., 0.];
                object.extend(model.to_cols_array().map(|v| v as f32));
                r.queue
                    .write_buffer(&self.shadow_objects[i], 0, bytemuck::cast_slice(&object));
            }
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("opacity shadow"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &self.shadow_color,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.shadow_depth,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                ..Default::default()
            });
            for i in 0..2 {
                pass.set_pipeline(&self.shadow_pipelines[i]);
                pass.set_bind_group(0, &self.shadow_binds[i].0, &[]);
                pass.set_bind_group(1, &self.shadow_binds[i].1, &[]);
                pass.set_vertex_buffer(0, self.dragon.positions.slice(..));
                pass.set_index_buffer(self.dragon.index.slice(..), wgpu::IndexFormat::Uint32);
                pass.draw_indexed(0..self.dragon.count, 0, 0..1);
            }
        }
        let t = self
            .targets
            .as_ref()
            .ok_or(Error::Invalid("opacity targets"))?;
        let screen = projection * view;
        let frustum = Frustum::from_projection(screen);
        let visible = |model: &Matrix4, m: &Mesh| {
            let scale = model
                .x_axis
                .truncate()
                .length()
                .max(model.y_axis.truncate().length())
                .max(model.z_axis.truncate().length());
            let center = model.transform_point3(m.center);
            frustum
                .intersects_sphere(Sphere {
                    center,
                    radius: m.radius * scale,
                })
                .then(|| screen.project_point3(center).z)
        };
        let background = Color::from_hex(0x9e9eff).0;
        let draw = |pass: &mut wgpu::RenderPass, m: &Mesh, slots: [&wgpu::Buffer; 3]| {
            for (slot, buffer) in slots.iter().enumerate() {
                pass.set_vertex_buffer(slot as u32, buffer.slice(..));
            }
            pass.set_index_buffer(m.index.slice(..), wgpu::IndexFormat::Uint32);
            pass.draw_indexed(0..m.count, 0, 0..1);
        };
        let scene_pass = |encoder: &mut wgpu::CommandEncoder, load| {
            encoder
                .begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("opacity scene"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &t.color,
                        depth_slice: None,
                        resolve_target: t.resolve_view.as_ref(),
                        ops: wgpu::Operations {
                            load,
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                    depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                        view: &t.depth,
                        depth_ops: Some(wgpu::Operations {
                            load: if matches!(load, wgpu::LoadOp::Load) {
                                wgpu::LoadOp::Load
                            } else {
                                wgpu::LoadOp::Clear(1.)
                            },
                            store: wgpu::StoreOp::Store,
                        }),
                        stencil_ops: None,
                    }),
                    ..Default::default()
                })
                .forget_lifetime()
        };
        // Opaque: the backdrop.
        {
            let mut pass = scene_pass(
                &mut encoder,
                wgpu::LoadOp::Clear(wgpu::Color {
                    r: background.x,
                    g: background.y,
                    b: background.z,
                    a: 1.,
                }),
            );
            if visible(&self.floor.model, &self.floor).is_some() {
                pass.set_pipeline(&t.floor.0);
                pass.set_bind_group(0, &t.floor.1, &[]);
                pass.set_bind_group(1, &t.floor.2, &[]);
                let m = &self.floor;
                draw(&mut pass, m, [&m.uvs, &m.normals, &m.positions]);
            }
        }
        // The resolved frame, copied and mipmapped for the transmission.
        let source = t
            .resolve
            .as_ref()
            .or(t.single.as_ref())
            .ok_or(Error::Invalid("opacity target"))?;
        encoder.copy_texture_to_texture(
            source.as_image_copy(),
            t.transmission.as_image_copy(),
            source.size(),
        );
        self.mipmaps
            .encode(&mut encoder, HALF, &t.transmission_levels);
        // Transmissive: the dragons, front to back.
        let mut dragons: Vec<(f64, usize)> = self
            .dragons
            .iter()
            .enumerate()
            .filter_map(|(i, (model, _))| visible(model, &self.dragon).map(|z| (z, i)))
            .collect();
        dragons.sort_by(|a, b| {
            a.0.partial_cmp(&b.0)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then(a.1.cmp(&b.1))
        });
        {
            let mut pass = scene_pass(&mut encoder, wgpu::LoadOp::Load);
            pass.set_pipeline(&t.dragon.0);
            pass.set_bind_group(0, &t.dragon.1, &[]);
            for &(_, i) in &dragons {
                pass.set_bind_group(1, &t.dragon.2[i], &[]);
                let m = &self.dragon;
                draw(&mut pass, m, [&m.normals, &m.uvs, &m.positions]);
            }
        }
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("opacity output"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &t.screen.view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                ..Default::default()
            });
            pass.set_pipeline(&t.output.0);
            pass.set_bind_group(0, &t.output.1[0], &[]);
            pass.set_bind_group(1, &t.output.1[1], &[]);
            pass.set_vertex_buffer(0, self.quad.slice(..));
            pass.draw(0..3, 0..1);
        }
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
        self.controls.update(s, c)
    }
    pub fn parameter(&mut self, _index: usize, _value: f32) -> Result<()> {
        Err(Error::Invalid("opacity parameter"))
    }
    pub fn seek(&mut self, _t: f64) {}
}
