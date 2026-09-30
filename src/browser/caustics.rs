//! webgpu_caustics: a turning glass duck (double-sided transmissive
//! MeshPhysicalMaterial, from the Draco duck.glb) on a hardwood floor under a
//! spot light whose half-float shadow map also records the caster's color.
//! The duck's castShadowNode refracts the view through its normals into the
//! Caustic_Free texture, so its shadow is a bright caustic; the glass plane
//! alternative casts its colors.png through 80 % opacity. Each frame draws
//! the shadow (both faces), the floor, then the transmissive object's back
//! and front faces, each refracting a fresh mipmapped copy of the frame.
//! Every stage runs the WGSL three.js r186 generates for the page (in
//! `caustics/`).
use super::controls_attributes::{Controls, camera_state};
use super::gltf_viewer::{decode_texture_image, fetch, load_asset};
use super::lights_projector::{m3, m4, pack};
use super::shadowmap_opacity::{Mipmaps, mip_count};
use crate::{
    Error, Result, camera::*, geometry::*, math::*, render_target::*, renderer::*, scene::*,
};
use std::f64::consts::PI;
use wgpu::util::DeviceExt;

const HALF: wgpu::TextureFormat = wgpu::TextureFormat::Rgba16Float;
const DEPTH: wgpu::TextureFormat = wgpu::TextureFormat::Depth24Plus;
const SHADOW_SIZE: u32 = 1024;
macro_rules! wgsl {
    ($name:literal) => {
        include_str!(concat!("caustics/", $name, ".wgsl"))
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
/// A uniform buffer sized for a struct of the generated WGSL.
fn sized(r: &Renderer, label: &str, source: &str, name: &str) -> Result<wgpu::Buffer> {
    Ok(uniform(r, label, pack(source, name, &[])?.len() as u64))
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
fn texture(
    r: &Renderer,
    size: (u32, u32),
    format: wgpu::TextureFormat,
    samples: u32,
    mips: u32,
    usage: wgpu::TextureUsages,
) -> wgpu::Texture {
    r.device.create_texture(&wgpu::TextureDescriptor {
        label: Some("caustics texture"),
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
fn buffers_of(
    attributes: &[[wgpu::VertexAttribute; 1]; 2],
    strides: [u64; 2],
) -> [wgpu::VertexBufferLayout<'_>; 2] {
    [0, 1].map(|i| wgpu::VertexBufferLayout {
        array_stride: strides[i],
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: &attributes[i],
    })
}
fn uvs(m: &Mesh) -> Result<&wgpu::Buffer> {
    m.uvs.as_ref().ok_or(Error::Invalid("plane uvs"))
}
/// Vertex buffers of a mesh, its index and bounding sphere.
struct Mesh {
    positions: wgpu::Buffer,
    normals: wgpu::Buffer,
    uvs: Option<wgpu::Buffer>,
    index: wgpu::Buffer,
    count: u32,
    center: Vector3,
    radius: f64,
}
/// Scene-pass pipelines for one target size and sample count.
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
    /// Duck and glass: back and front pipelines and their bind groups.
    duck: [(wgpu::RenderPipeline, wgpu::BindGroup, wgpu::BindGroup); 2],
    glass: [(wgpu::RenderPipeline, wgpu::BindGroup, wgpu::BindGroup); 2],
    output: (wgpu::RenderPipeline, [wgpu::BindGroup; 2]),
}
pub(super) struct Demo {
    controls: Controls,
    time: f64,
    last: f64,
    rotation: f64,
    /// caustic occlusion, material color, model (0 duck, 1 glass).
    params: [f64; 3],
    duck: Mesh,
    floor: Mesh,
    glass: Mesh,
    /// Caustic_Free, hardwood2_diffuse and colors.png, mipmapped.
    maps: [wgpu::TextureView; 3],
    shadow_color: wgpu::TextureView,
    shadow_depth: wgpu::TextureView,
    /// Duck front and back, glass front and back.
    shadow_pipelines: [wgpu::RenderPipeline; 4],
    shadow_binds: [(wgpu::BindGroup, wgpu::BindGroup); 4],
    shadow_render: wgpu::Buffer,
    caustic_object: wgpu::Buffer,
    glass_shadow_object: wgpu::Buffer,
    floor_render: wgpu::Buffer,
    floor_object: wgpu::Buffer,
    duck_render: wgpu::Buffer,
    duck_object: wgpu::Buffer,
    glass_render: wgpu::Buffer,
    glass_object: wgpu::Buffer,
    output_render: wgpu::Buffer,
    output_object: wgpu::Buffer,
    quad: wgpu::Buffer,
    repeat: wgpu::Sampler,
    clamp: wgpu::Sampler,
    linear: wgpu::Sampler,
    compare: wgpu::Sampler,
    mipmaps: Mipmaps,
    targets: Option<Targets>,
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, _id: u32, r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 25.,
            near: 0.025,
            far: 5.,
            aspect,
            ..Default::default()
        }));
        s.get_mut(c)?.position = Vector3::new(-0.5, 0.35, 0.2);
        let mut controls = Controls::new(None, (0., 3.), PI / 2., true);
        controls.set_target(Vector3::ZERO);
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
        let mesh = |positions: &[f32], normals: &[f32], uvs: Option<&[f32]>, index: &[u32]| {
            let points: Vec<Vector3> = positions
                .chunks(3)
                .map(|p| Vector3::new(p[0] as f64, p[1] as f64, p[2] as f64))
                .collect();
            let (lo, hi) = points.iter().fold(
                (Vector3::splat(f64::MAX), Vector3::splat(f64::MIN)),
                |(lo, hi), p| (lo.min(*p), hi.max(*p)),
            );
            let center = (lo + hi) / 2.;
            Mesh {
                positions: init(
                    "caustics positions",
                    bytemuck::cast_slice(positions),
                    vertex,
                ),
                normals: init("caustics normals", bytemuck::cast_slice(normals), vertex),
                uvs: uvs.map(|uvs| init("caustics uvs", bytemuck::cast_slice(uvs), vertex)),
                index: init(
                    "caustics index",
                    bytemuck::cast_slice(index),
                    wgpu::BufferUsages::INDEX,
                ),
                count: index.len() as u32,
                center,
                radius: points
                    .iter()
                    .map(|p| (*p - center).length())
                    .fold(0., f64::max),
            }
        };
        let (asset, buffers, _) = load_asset("/web/gallery/assets/gltf/duck.glb").await?;
        let primitive = asset
            .meshes()
            .next()
            .and_then(|m| m.primitives().next())
            .ok_or(Error::Invalid("duck mesh"))?;
        let reader = primitive.reader(|b| buffers.get(b.index()).map(Vec::as_slice));
        let positions: Vec<f32> = reader
            .read_positions()
            .ok_or(Error::Invalid("duck positions"))?
            .flatten()
            .collect();
        let normals: Vec<f32> = reader
            .read_normals()
            .ok_or(Error::Invalid("duck normals"))?
            .flatten()
            .collect();
        let index: Vec<u32> = reader
            .read_indices()
            .ok_or(Error::Invalid("duck index"))?
            .into_u32()
            .collect();
        let duck = mesh(&positions, &normals, None, &index);
        let plane = |size: f64| -> Result<Mesh> {
            let g = PlaneGeometry::build(size, size, 1, 1)?;
            let f = |name: &str, n: usize| -> Result<Vec<f32>> {
                let a = g
                    .attributes
                    .get(name)
                    .ok_or(Error::Invalid("plane attribute"))?;
                (0..a.count())
                    .flat_map(|i| (0..n).map(move |k| (i, k)))
                    .map(|(i, k)| a.get_component(i, k).map(|v| v as f32))
                    .collect()
            };
            let index = g.index.clone().ok_or(Error::Invalid("plane index"))?;
            Ok(mesh(
                &f("position", 3)?,
                &f("normal", 3)?,
                Some(&f("uv", 2)?),
                &index,
            ))
        };
        let (floor, glass) = (plane(2.)?, plane(0.2)?);
        // TextureLoader images: flipped on upload, mipmapped on the GPU.
        let mut mipmaps = Mipmaps::new(r);
        let mut encoder = r.device.create_command_encoder(&Default::default());
        let mut maps = vec![];
        for (path, format) in [
            (
                "opengameart/Caustic_Free.jpg",
                wgpu::TextureFormat::Rgba8UnormSrgb,
            ),
            ("hardwood2_diffuse.jpg", wgpu::TextureFormat::Rgba8Unorm),
            (
                "spot-skinning/colors.png",
                wgpu::TextureFormat::Rgba8UnormSrgb,
            ),
        ] {
            let image =
                decode_texture_image(&fetch(&format!("/web/gallery/assets/{path}")).await?).await?;
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
            let row = image.width as usize * 4;
            let flipped: Vec<u8> = image.rgba.chunks(row).rev().flatten().copied().collect();
            r.queue.write_texture(
                t.as_image_copy(),
                &flipped,
                wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(row as u32),
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
        let maps: [wgpu::TextureView; 3] = maps
            .try_into()
            .map_err(|_| Error::Invalid("caustics textures"))?;
        let sampler = |address, mipmap| {
            r.device.create_sampler(&wgpu::SamplerDescriptor {
                address_mode_u: address,
                address_mode_v: address,
                mag_filter: wgpu::FilterMode::Linear,
                min_filter: wgpu::FilterMode::Linear,
                mipmap_filter: mipmap,
                ..Default::default()
            })
        };
        let repeat = sampler(wgpu::AddressMode::Repeat, wgpu::FilterMode::Linear);
        let clamp = sampler(wgpu::AddressMode::ClampToEdge, wgpu::FilterMode::Linear);
        let linear = sampler(wgpu::AddressMode::ClampToEdge, wgpu::FilterMode::Nearest);
        let module = |label, source: &str| {
            r.device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some(label),
                source: wgpu::ShaderSource::Wgsl(source.into()),
            })
        };
        // The shadow pipelines: the duck's position/normal and the glass's
        // uv/position, front faces then back faces.
        let duck_buffers = [
            wgpu::vertex_attr_array![0 => Float32x3],
            wgpu::vertex_attr_array![1 => Float32x3],
        ];
        let glass_buffers = [
            wgpu::vertex_attr_array![0 => Float32x2],
            wgpu::vertex_attr_array![1 => Float32x3],
        ];
        let shadow_pipeline =
            |vs: &str, fs: &str, buffers: &[wgpu::VertexBufferLayout], cw: bool| {
                r.device
                    .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                        label: Some("caustics shadow"),
                        layout: None,
                        vertex: wgpu::VertexState {
                            module: &module("shadow", vs),
                            entry_point: Some("main"),
                            compilation_options: Default::default(),
                            buffers,
                        },
                        fragment: Some(wgpu::FragmentState {
                            module: &module("shadow", fs),
                            entry_point: Some("main"),
                            compilation_options: Default::default(),
                            targets: &[Some(HALF.into())],
                        }),
                        primitive: wgpu::PrimitiveState {
                            front_face: if cw {
                                wgpu::FrontFace::Cw
                            } else {
                                wgpu::FrontFace::Ccw
                            },
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
        let duck_layout = buffers_of(&duck_buffers, [12, 12]);
        let glass_layout = buffers_of(&glass_buffers, [8, 12]);
        let shadow_pipelines = [
            shadow_pipeline(
                wgsl!("caustic_vs"),
                wgsl!("caustic_fs"),
                &duck_layout,
                false,
            ),
            shadow_pipeline(
                wgsl!("caustic_vs"),
                wgsl!("caustic_back_fs"),
                &duck_layout,
                true,
            ),
            shadow_pipeline(
                wgsl!("glass_shadow_vs"),
                wgsl!("glass_shadow_fs"),
                &glass_layout,
                false,
            ),
            shadow_pipeline(
                wgsl!("glass_shadow_vs"),
                wgsl!("glass_shadow_fs"),
                &glass_layout,
                true,
            ),
        ];
        let shadow_render = uniform(r, "shadow render", 128);
        let caustic_object = sized(r, "caustic object", wgsl!("caustic_fs"), "objectStruct")?;
        let glass_shadow_object = sized(
            r,
            "glass shadow object",
            wgsl!("glass_shadow_fs"),
            "objectStruct",
        )?;
        let shadow_binds = [0, 1, 2, 3].map(|i| {
            let (map, object) = if i < 2 {
                (&maps[0], &caustic_object)
            } else {
                (&maps[2], &glass_shadow_object)
            };
            (
                bind(
                    r,
                    shadow_pipelines[i].get_bind_group_layout(0),
                    &[(0, shadow_render.as_entire_binding())],
                ),
                bind(
                    r,
                    shadow_pipelines[i].get_bind_group_layout(1),
                    &[
                        (0, wgpu::BindingResource::Sampler(&repeat)),
                        (1, wgpu::BindingResource::TextureView(map)),
                        (2, object.as_entire_binding()),
                    ],
                ),
            )
        });
        let attachment =
            wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING;
        let shadow_size = (SHADOW_SIZE, SHADOW_SIZE);
        Ok(Self {
            controls,
            time: 0.,
            last: 0.,
            rotation: 0.,
            params: [20., 0xffd700 as f64, 0.],
            duck,
            floor,
            glass,
            maps,
            shadow_color: view(&texture(r, shadow_size, HALF, 1, 1, attachment)),
            shadow_depth: view(&texture(r, shadow_size, DEPTH, 1, 1, attachment)),
            shadow_pipelines,
            shadow_binds,
            shadow_render,
            caustic_object,
            glass_shadow_object,
            floor_render: sized(r, "floor render", wgsl!("floor_fs"), "renderStruct")?,
            floor_object: sized(r, "floor object", wgsl!("floor_fs"), "objectStruct")?,
            duck_render: sized(r, "duck render", wgsl!("duck_fs"), "renderStruct")?,
            duck_object: sized(r, "duck object", wgsl!("duck_fs"), "objectStruct")?,
            glass_render: sized(r, "glass render", wgsl!("glass_fs"), "renderStruct")?,
            glass_object: sized(r, "glass object", wgsl!("glass_fs"), "objectStruct")?,
            output_render: uniform(r, "output render", 144),
            output_object: uniform(r, "output object", 64),
            quad: init(
                "quad",
                bytemuck::cast_slice(&[-1f32, 3., 0., -1., -1., 0., 3., -1., 0.]),
                vertex,
            ),
            repeat,
            clamp,
            linear,
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
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, dt: f64, animate: bool) -> Result<()> {
        if animate {
            self.time += dt;
        }
        Ok(())
    }
    /// animate(): the duck turns 0.01 rad per frame (60 fps steps), then
    /// controls.update().
    pub fn prepare(&mut self, s: &mut Scene, c: Object3D, _r: &Renderer) -> Result<()> {
        self.rotation -= 0.01 * (self.time - self.last) * 60.;
        self.last = self.time;
        self.controls.frame_update(s, c)
    }
    fn resize(&mut self, r: &Renderer, out: &RenderTarget) -> Result<()> {
        let size = (out.width, out.height);
        let samples = out.options.samples.max(1);
        let attachment = wgpu::TextureUsages::RENDER_ATTACHMENT;
        let sampled =
            attachment | wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_SRC;
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
        let blend = wgpu::BlendState {
            color: wgpu::BlendComponent {
                src_factor: wgpu::BlendFactor::SrcAlpha,
                dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                operation: wgpu::BlendOperation::Add,
            },
            alpha: wgpu::BlendComponent {
                src_factor: wgpu::BlendFactor::One,
                dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                operation: wgpu::BlendOperation::Add,
            },
        };
        let pipeline = |label,
                        vs: &str,
                        fs: &str,
                        buffers: &[wgpu::VertexBufferLayout],
                        format,
                        scene: bool,
                        cw: bool,
                        blended: bool| {
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
                        targets: &[Some(wgpu::ColorTargetState {
                            format,
                            blend: blended.then_some(blend),
                            write_mask: wgpu::ColorWrites::ALL,
                        })],
                    }),
                    primitive: wgpu::PrimitiveState {
                        front_face: if cw {
                            wgpu::FrontFace::Cw
                        } else {
                            wgpu::FrontFace::Ccw
                        },
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
        let v2 = [wgpu::VertexAttribute {
            format: wgpu::VertexFormat::Float32x2,
            offset: 0,
            shader_location: 0,
        }];
        let v3 = |location| {
            [wgpu::VertexAttribute {
                format: wgpu::VertexFormat::Float32x3,
                offset: 0,
                shader_location: location,
            }]
        };
        let (n1, p2, n0, p1, p0) = (v3(1), v3(2), v3(0), v3(1), v3(0));
        let buffer = |attributes, stride| wgpu::VertexBufferLayout {
            array_stride: stride,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes,
        };
        // uv, normal, position for the planes; normal, position for the duck.
        let planes = [buffer(&v2[..], 8), buffer(&n1[..], 12), buffer(&p2[..], 12)];
        let duck = [buffer(&n0[..], 12), buffer(&p1[..], 12)];
        let sampler = wgpu::BindingResource::Sampler;
        let tex = wgpu::BindingResource::TextureView;
        let floor = pipeline(
            "caustics floor",
            wgsl!("floor_vs"),
            wgsl!("floor_fs"),
            &planes,
            HALF,
            true,
            false,
            false,
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
                    (1, sampler(&self.repeat)),
                    (2, tex(&self.maps[1])),
                    (3, sampler(&self.linear)),
                    (4, tex(&r.dfg)),
                    (5, sampler(&self.linear)),
                    (6, tex(&self.shadow_color)),
                    (7, sampler(&self.compare)),
                    (8, tex(&self.shadow_depth)),
                ],
            ),
        );
        let duck_pipelines = [
            pipeline(
                "caustics duck back",
                wgsl!("duck_vs"),
                wgsl!("duck_back_fs"),
                &duck,
                HALF,
                true,
                true,
                true,
            ),
            pipeline(
                "caustics duck",
                wgsl!("duck_vs"),
                wgsl!("duck_fs"),
                &duck,
                HALF,
                true,
                false,
                true,
            ),
        ];
        let duck = duck_pipelines.map(|p| {
            let render = bind(
                r,
                p.get_bind_group_layout(0),
                &[(0, self.duck_render.as_entire_binding())],
            );
            let object = bind(
                r,
                p.get_bind_group_layout(1),
                &[
                    (0, self.duck_object.as_entire_binding()),
                    (1, sampler(&self.linear)),
                    (2, tex(&r.dfg)),
                    (3, sampler(&self.clamp)),
                    (4, tex(&transmission_view)),
                ],
            );
            (p, render, object)
        });
        let glass_pipelines = [
            pipeline(
                "caustics glass back",
                wgsl!("glass_vs"),
                wgsl!("glass_back_fs"),
                &planes,
                HALF,
                true,
                true,
                true,
            ),
            pipeline(
                "caustics glass",
                wgsl!("glass_vs"),
                wgsl!("glass_fs"),
                &planes,
                HALF,
                true,
                false,
                true,
            ),
        ];
        let glass = glass_pipelines.map(|p| {
            let render = bind(
                r,
                p.get_bind_group_layout(0),
                &[(0, self.glass_render.as_entire_binding())],
            );
            let object = bind(
                r,
                p.get_bind_group_layout(1),
                &[
                    (0, self.glass_object.as_entire_binding()),
                    (1, sampler(&self.repeat)),
                    (2, tex(&self.maps[2])),
                    (3, sampler(&self.linear)),
                    (4, tex(&r.dfg)),
                    (5, sampler(&self.clamp)),
                    (6, tex(&transmission_view)),
                ],
            );
            (p, render, object)
        });
        let output = pipeline(
            "render output",
            include_str!("lights_dynamic/output_vs.wgsl"),
            include_str!("lights_dynamic/output_fs.wgsl"),
            &[buffer(&p0[..], 12)],
            out.options.format,
            false,
            false,
            false,
        );
        let scene_view = match (&resolve, &single) {
            (Some(t), _) | (None, Some(t)) => view(t),
            _ => return Err(Error::Invalid("caustics target")),
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
        let mut data: Vec<f32> = [Matrix4::IDENTITY; 2]
            .iter()
            .flat_map(|m| m.to_cols_array().map(|v| v as f32))
            .collect();
        data.extend([size.0 as f32, size.1 as f32, 0., 0.]);
        r.queue
            .write_buffer(&self.output_render, 0, bytemuck::cast_slice(&data));
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
            duck,
            glass,
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
        let glass_mode = self.params[2].round() as usize == 1;
        // The spot light at (0.2, 0.3, 0.2) aimed at the origin; its shadow
        // camera spans 2 × angle, near 0.1, far 1.
        let spot = Vector3::new(0.2, 0.3, 0.2);
        let (near, far) = (0.1, 1.);
        let top = near * (PI / 6.).tan();
        let shadow_projection = Matrix4::from_cols_array(&[
            near / top,
            0.,
            0.,
            0.,
            0.,
            near / top,
            0.,
            0.,
            0.,
            0.,
            -far / (far - near),
            -1.,
            0.,
            0.,
            -far * near / (far - near),
            0.,
        ]);
        let shadow_view = Matrix4::look_at_rh(spot, Vector3::ZERO, Vector3::Y);
        let bias = Matrix4::from_cols_array(&[
            0.5, 0., 0., 0., 0., 0.5, 0., 0., 0., 0., 1., 0., 0.5, 0.5, 0., 1.,
        ]);
        let size = self
            .targets
            .as_ref()
            .map_or((1, 1), |t| (t.width, t.height));
        let spot_view = view.transform_point3(spot).to_array().to_vec();
        let color = Color::from_hex(self.params[1] as u32).0.to_array().to_vec();
        let duck_model =
            Matrix4::from_scale(Vector3::splat(0.5)) * Matrix4::from_rotation_y(self.rotation);
        let floor_model = Matrix4::from_rotation_x(-PI / 2.);
        let glass_model = Matrix4::from_translation(Vector3::new(0., 0.1, 0.));
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
        let camera_values = || {
            vec![
                ("cameraProjectionMatrix", m4(projection)),
                ("cameraViewMatrix", m4(view)),
                ("cameraPosition", camera_position.to_array().to_vec()),
            ]
        };
        // SpotLightNode: cone and penumbra cosines, cutoff, decay, color,
        // view position, world position and target.
        let spot_values = |names: [&'static str; 8]| {
            vec![
                (names[0], vec![(PI / 6.).cos()]),
                (names[1], vec![1.]),
                (names[2], vec![0.]),
                (names[3], vec![2.]),
                (names[4], vec![1.; 3]),
                (names[5], spot_view.clone()),
                (names[6], spot.to_array().to_vec()),
                (names[7], vec![0.; 3]),
            ]
        };
        let mut floor_render = camera_values();
        floor_render.extend(spot_values([
            "nodeUniform23",
            "nodeUniform24",
            "nodeUniform27",
            "nodeUniform28",
            "nodeUniform13",
            "nodeUniform12",
            "nodeUniform25",
            "nodeUniform26",
        ]));
        floor_render.extend([
            ("nodeUniform15", m4(bias * shadow_projection * shadow_view)),
            ("nodeUniform17", vec![0.]),
            ("nodeUniform18", vec![0.]),
            ("nodeUniform20", vec![1.]),
            ("nodeUniform21", vec![SHADOW_SIZE as f64; 2]),
            ("nodeUniform22", vec![0.95]),
        ]);
        write(
            &self.floor_render,
            wgsl!("floor_fs"),
            "renderStruct",
            &floor_render,
        )?;
        let uv_repeat = Matrix4::from_scale(Vector3::new(10., 10., 1.));
        write(
            &self.floor_object,
            wgsl!("floor_fs"),
            "objectStruct",
            &[
                (
                    "nodeUniform0",
                    Color::from_hex(0x999999).0.to_array().to_vec(),
                ),
                ("nodeUniform2", m3(uv_repeat)),
                ("nodeUniform3", vec![1.]),
                ("nodeUniform4", vec![0.]),
                ("nodeUniform5", vec![1.]),
                ("nodeUniform7", m3(floor_model.inverse().transpose())),
                ("nodeUniform9", vec![1.]),
                ("nodeUniform11", m4(floor_model)),
            ],
        )?;
        let viewport = vec![size.0 as f64, size.1 as f64];
        let mut duck_render = camera_values();
        duck_render.extend(spot_values([
            "nodeUniform25",
            "nodeUniform26",
            "nodeUniform29",
            "nodeUniform30",
            "nodeUniform24",
            "nodeUniform23",
            "nodeUniform27",
            "nodeUniform28",
        ]));
        duck_render.push(("nodeUniform21", viewport.clone()));
        write(
            &self.duck_render,
            wgsl!("duck_fs"),
            "renderStruct",
            &duck_render,
        )?;
        write(
            &self.duck_object,
            wgsl!("duck_fs"),
            "objectStruct",
            &[
                ("nodeUniform0", color.clone()),
                ("nodeUniform1", vec![1.]),
                ("nodeUniform2", vec![0.]),
                ("nodeUniform3", vec![0.1]),
                ("nodeUniform5", m3(duck_model.inverse().transpose())),
                ("nodeUniform6", vec![1.5]),
                ("nodeUniform7", vec![1.; 3]),
                ("nodeUniform8", vec![1.]),
                ("nodeUniform9", vec![1.]),
                ("nodeUniform10", vec![0.25]),
                ("nodeUniform11", vec![f64::INFINITY]),
                ("nodeUniform12", vec![1.; 3]),
                ("nodeUniform14", vec![1.]),
                ("nodeUniform17", m4(duck_model)),
                ("nodeUniform19", m4(duck_model)),
            ],
        )?;
        let mut glass_render = camera_values();
        glass_render.extend(spot_values([
            "nodeUniform27",
            "nodeUniform28",
            "nodeUniform31",
            "nodeUniform32",
            "nodeUniform26",
            "nodeUniform25",
            "nodeUniform29",
            "nodeUniform30",
        ]));
        glass_render.push(("nodeUniform23", viewport));
        write(
            &self.glass_render,
            wgsl!("glass_fs"),
            "renderStruct",
            &glass_render,
        )?;
        write(
            &self.glass_object,
            wgsl!("glass_fs"),
            "objectStruct",
            &[
                ("nodeUniform0", vec![1.; 3]),
                ("nodeUniform2", m3(Matrix4::IDENTITY)),
                ("nodeUniform3", vec![1.]),
                ("nodeUniform4", vec![0.]),
                ("nodeUniform5", vec![0.1]),
                ("nodeUniform7", m3(glass_model.inverse().transpose())),
                ("nodeUniform8", vec![1.5]),
                ("nodeUniform9", vec![1.; 3]),
                ("nodeUniform10", vec![1.]),
                ("nodeUniform11", vec![1.]),
                ("nodeUniform12", vec![0.]),
                ("nodeUniform13", vec![f64::INFINITY]),
                ("nodeUniform14", vec![1.; 3]),
                ("nodeUniform16", vec![1.]),
                ("nodeUniform19", m4(glass_model)),
                ("nodeUniform21", m4(glass_model)),
            ],
        )?;
        let mut shadow_camera: Vec<f32> =
            shadow_projection.to_cols_array().map(|v| v as f32).to_vec();
        shadow_camera.extend(shadow_view.to_cols_array().map(|v| v as f32));
        r.queue
            .write_buffer(&self.shadow_render, 0, bytemuck::cast_slice(&shadow_camera));
        write(
            &self.caustic_object,
            wgsl!("caustic_fs"),
            "objectStruct",
            &[
                ("nodeUniform2", m4(duck_model)),
                ("nodeUniform3", m3(duck_model.inverse().transpose())),
                ("nodeUniform4", vec![self.params[0]]),
                ("nodeUniform5", color),
                ("nodeUniform6", vec![1.]),
            ],
        )?;
        write(
            &self.glass_shadow_object,
            wgsl!("glass_shadow_fs"),
            "objectStruct",
            &[
                ("nodeUniform1", m3(Matrix4::IDENTITY)),
                ("nodeUniform2", m3(Matrix4::IDENTITY)),
                ("nodeUniform3", vec![1.]),
                ("nodeUniform6", m4(glass_model)),
            ],
        )?;
        let t = self
            .targets
            .as_ref()
            .ok_or(Error::Invalid("caustics targets"))?;
        let mut encoder = r.device.create_command_encoder(&Default::default());
        let (object, model) = if glass_mode {
            (&self.glass, glass_model)
        } else {
            (&self.duck, duck_model)
        };
        let draw = |pass: &mut wgpu::RenderPass, m: &Mesh, slots: &[&wgpu::Buffer]| {
            for (slot, buffer) in slots.iter().enumerate() {
                pass.set_vertex_buffer(slot as u32, buffer.slice(..));
            }
            pass.set_index_buffer(m.index.slice(..), wgpu::IndexFormat::Uint32);
            pass.draw_indexed(0..m.count, 0, 0..1);
        };
        // The caster's shadow, front faces then back faces.
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("caustics shadow"),
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
            let first = if glass_mode { 2 } else { 0 };
            for i in first..first + 2 {
                pass.set_pipeline(&self.shadow_pipelines[i]);
                pass.set_bind_group(0, &self.shadow_binds[i].0, &[]);
                pass.set_bind_group(1, &self.shadow_binds[i].1, &[]);
                if glass_mode {
                    draw(&mut pass, object, &[uvs(object)?, &object.positions]);
                } else {
                    draw(&mut pass, object, &[&object.positions, &object.normals]);
                }
            }
        }
        let screen = projection * view;
        let frustum = Frustum::from_projection(screen);
        let visible = |model: Matrix4, m: &Mesh, scale: f64| {
            frustum.intersects_sphere(Sphere {
                center: model.transform_point3(m.center),
                radius: m.radius * scale,
            })
        };
        let scene_pass = |encoder: &mut wgpu::CommandEncoder, load| {
            encoder
                .begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("caustics scene"),
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
        {
            let mut pass = scene_pass(&mut encoder, wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT));
            if visible(floor_model, &self.floor, 1.) {
                pass.set_pipeline(&t.floor.0);
                pass.set_bind_group(0, &t.floor.1, &[]);
                pass.set_bind_group(1, &t.floor.2, &[]);
                let m = &self.floor;
                draw(&mut pass, m, &[uvs(m)?, &m.normals, &m.positions]);
            }
        }
        let source = t
            .resolve
            .as_ref()
            .or(t.single.as_ref())
            .ok_or(Error::Invalid("caustics target"))?;
        // The transmissive object: back faces, then front faces, each over a
        // fresh mipmapped copy of the frame.
        let scale = if glass_mode { 1. } else { 0.5 };
        if visible(model, object, scale) {
            let faces = if glass_mode { &t.glass } else { &t.duck };
            for (pipeline, render, bind) in faces {
                encoder.copy_texture_to_texture(
                    source.as_image_copy(),
                    t.transmission.as_image_copy(),
                    source.size(),
                );
                self.mipmaps
                    .encode(&mut encoder, HALF, &t.transmission_levels);
                let mut pass = scene_pass(&mut encoder, wgpu::LoadOp::Load);
                pass.set_pipeline(pipeline);
                pass.set_bind_group(0, render, &[]);
                pass.set_bind_group(1, bind, &[]);
                if glass_mode {
                    draw(
                        &mut pass,
                        object,
                        &[uvs(object)?, &object.normals, &object.positions],
                    );
                } else {
                    draw(&mut pass, object, &[&object.normals, &object.positions]);
                }
            }
        }
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("caustics output"),
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
    /// caustic occlusion, material color and model.
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        *self
            .params
            .get_mut(index)
            .ok_or(Error::Invalid("caustics parameter"))? = value as f64;
        Ok(())
    }
    pub fn seek(&mut self, t: f64) {
        self.time = t;
    }
}
