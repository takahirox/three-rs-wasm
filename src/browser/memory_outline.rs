//! webgpu_test_memory: every frame builds a new sphere with random segment
//! counts and a new 256 × 256 canvas texture of a random color (mipmapped
//! on the GPU), lit by a shadow-casting directional light over a Lambert
//! plane, and drops the previous ones, as the page disposes them. The
//! post-processing switch adds OutlineNode around the current sphere: the
//! non-selected depth, the selected mask, the half-resolution downsample and
//! edge detection, the half- and quarter-resolution blurs and the composite,
//! added to the scene. Every stage runs the WGSL three.js r186 generates for
//! the page (in `memory_outline/`).
use super::lights_projector::{m3, m4, pack};
use super::shadowmap_csm::look_rotation;
use super::shadowmap_opacity::{Mipmaps, mip_count};
use crate::{
    Error, Result, camera::*, geometry::*, math::*, render_target::*, renderer::*, scene::*,
};
use std::f64::consts::PI;
use wgpu::util::DeviceExt;

const HALF: wgpu::TextureFormat = wgpu::TextureFormat::Rgba16Float;
const DEPTH: wgpu::TextureFormat = wgpu::TextureFormat::Depth24Plus;
const LDR: wgpu::TextureFormat = wgpu::TextureFormat::Rgba8Unorm;
const SHADOW: u32 = 512;
macro_rules! wgsl {
    ($name:literal) => {
        include_str!(concat!("memory_outline/", $name, ".wgsl"))
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
fn texture(r: &Renderer, size: (u32, u32), format: wgpu::TextureFormat) -> wgpu::TextureView {
    r.device
        .create_texture(&wgpu::TextureDescriptor {
            label: Some("memory target"),
            size: wgpu::Extent3d {
                width: size.0,
                height: size.1,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        })
        .create_view(&Default::default())
}
#[allow(clippy::too_many_arguments)]
fn pipeline(
    r: &Renderer,
    label: &str,
    vs: &str,
    fs: &str,
    buffers: &[wgpu::VertexBufferLayout],
    format: wgpu::TextureFormat,
    depth: Option<wgpu::TextureFormat>,
    cw: bool,
) -> wgpu::RenderPipeline {
    let module = |source: &str| {
        r.device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some(label),
            source: wgpu::ShaderSource::Wgsl(source.into()),
        })
    };
    r.device
        .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some(label),
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
            primitive: wgpu::PrimitiveState {
                front_face: if cw {
                    wgpu::FrontFace::Cw
                } else {
                    wgpu::FrontFace::Ccw
                },
                cull_mode: Some(wgpu::Face::Back),
                ..Default::default()
            },
            depth_stencil: depth.map(|format| wgpu::DepthStencilState {
                format,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::LessEqual,
                stencil: Default::default(),
                bias: Default::default(),
            }),
            multisample: Default::default(),
            multiview: None,
            cache: None,
        })
}
fn target(
    view: &wgpu::TextureView,
    clear: wgpu::Color,
) -> Option<wgpu::RenderPassColorAttachment<'_>> {
    Some(wgpu::RenderPassColorAttachment {
        view,
        depth_slice: None,
        resolve_target: None,
        ops: wgpu::Operations {
            load: wgpu::LoadOp::Clear(clear),
            store: wgpu::StoreOp::Store,
        },
    })
}
fn depth_attachment(
    view: &wgpu::TextureView,
) -> Option<wgpu::RenderPassDepthStencilAttachment<'_>> {
    Some(wgpu::RenderPassDepthStencilAttachment {
        view,
        depth_ops: Some(wgpu::Operations {
            load: wgpu::LoadOp::Clear(1.),
            store: wgpu::StoreOp::Store,
        }),
        stencil_ops: None,
    })
}
/// This frame's sphere: geometry, canvas texture and the bind groups that
/// read them (dropped with it, as the page disposes them).
struct Ball {
    positions: wgpu::Buffer,
    normals: wgpu::Buffer,
    uvs: wgpu::Buffer,
    index: wgpu::Buffer,
    count: u32,
    shadow: wgpu::BindGroup,
    map: wgpu::TextureView,
    /// The Lambert and outline mask object groups (rebuilt with the targets).
    groups: Option<(wgpu::BindGroup, wgpu::BindGroup)>,
}
/// Pipelines and targets for one drawing-buffer size.
struct Targets {
    width: u32,
    height: u32,
    format: wgpu::TextureFormat,
    color: wgpu::TextureView,
    depth: wgpu::TextureView,
    screen: RenderTarget,
    plane: (wgpu::RenderPipeline, [wgpu::BindGroup; 2]),
    sphere: (wgpu::RenderPipeline, wgpu::BindGroup),
    output: (wgpu::RenderPipeline, [wgpu::BindGroup; 2]),
    /// OutlineNode: depth (depth32float), mask, downsample, edge 1, blur 1,
    /// blur 2 and edge 2 targets, and the composite.
    outline_depth: (wgpu::TextureView, wgpu::TextureView),
    mask: (wgpu::TextureView, wgpu::TextureView),
    half: [wgpu::TextureView; 3],
    quarter: [wgpu::TextureView; 2],
    composite: wgpu::TextureView,
    depth_pass: (wgpu::RenderPipeline, wgpu::BindGroup, wgpu::BindGroup),
    mask_pass: (wgpu::RenderPipeline, wgpu::BindGroup),
    downsample: (wgpu::RenderPipeline, wgpu::BindGroup),
    edge: (wgpu::RenderPipeline, wgpu::BindGroup),
    /// Half-resolution blur x and y, quarter-resolution blur x and y.
    blurs: [(wgpu::RenderPipeline, wgpu::BindGroup); 4],
    composite_pass: (wgpu::RenderPipeline, wgpu::BindGroup),
    pp_output: (wgpu::RenderPipeline, wgpu::BindGroup),
}
pub(super) struct Demo {
    random: u32,
    pending: bool,
    /// cast shadow, post-processing, outline, AO, creating meshes.
    params: [bool; 5],
    light: Vector3,
    plane: (wgpu::Buffer, wgpu::Buffer, wgpu::Buffer),
    sphere: Option<Ball>,
    shadow_pipeline: wgpu::RenderPipeline,
    shadow_render: (wgpu::Buffer, wgpu::BindGroup),
    shadow_map: (wgpu::TextureView, wgpu::TextureView),
    plane_render: wgpu::Buffer,
    plane_object: wgpu::Buffer,
    sphere_render: wgpu::Buffer,
    sphere_object: wgpu::Buffer,
    shadow_object: wgpu::Buffer,
    output_render: wgpu::Buffer,
    output_object: wgpu::Buffer,
    depth_render: wgpu::Buffer,
    depth_object: wgpu::Buffer,
    mask_render: wgpu::Buffer,
    mask_object: wgpu::Buffer,
    identity: wgpu::Buffer,
    blur_objects: [wgpu::Buffer; 4],
    edge_object: wgpu::Buffer,
    composite_object: wgpu::Buffer,
    quad: wgpu::Buffer,
    quad_uv: wgpu::Buffer,
    linear: wgpu::Sampler,
    mipmapped: wgpu::Sampler,
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
            fov: 60.,
            near: 1.,
            far: 10000.,
            aspect,
            ..Default::default()
        }));
        s.get_mut(c)?.position = Vector3::new(0., 0., 200.);
        let init = |label, data: &[u8], usage| {
            r.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some(label),
                    contents: data,
                    usage,
                })
        };
        let plane = PlaneGeometry::build(1000., 1000., 1, 1)?;
        let f = |name: &str| -> Result<Vec<f32>> {
            let a = plane
                .attributes
                .get(name)
                .ok_or(Error::Invalid("plane attribute"))?;
            (0..a.count())
                .flat_map(|i| (0..3).map(move |k| (i, k)))
                .map(|(i, k)| a.get_component(i, k).map(|v| v as f32))
                .collect()
        };
        let index = plane.index.clone().ok_or(Error::Invalid("plane index"))?;
        let vertex = wgpu::BufferUsages::VERTEX;
        let uv_position = [
            wgpu::vertex_attr_array![0 => Float32x2],
            wgpu::vertex_attr_array![1 => Float32x3],
        ];
        let shadow_pipeline = pipeline(
            r,
            "memory shadow",
            wgsl!("shadow_vs"),
            wgsl!("shadow_fs"),
            &[
                wgpu::VertexBufferLayout {
                    array_stride: 8,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &uv_position[0],
                },
                wgpu::VertexBufferLayout {
                    array_stride: 12,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &uv_position[1],
                },
            ],
            LDR,
            Some(DEPTH),
            true,
        );
        let shadow_render_buffer = uniform(r, "shadow camera", 128);
        let shadow_render = bind(
            r,
            shadow_pipeline.get_bind_group_layout(0),
            &[(0, shadow_render_buffer.as_entire_binding())],
        );
        let depth_texture = r.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("memory shadow"),
            size: wgpu::Extent3d {
                width: SHADOW,
                height: SHADOW,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: DEPTH,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });
        let identity = uniform(r, "uv transforms", 192);
        let mut identities = vec![];
        for _ in 0..4 {
            identities.extend([1f32, 0., 0., 0., 0., 1., 0., 0., 0., 0., 1., 0.]);
        }
        r.queue
            .write_buffer(&identity, 0, bytemuck::cast_slice(&identities));
        Ok(Self {
            random: 186,
            pending: false,
            params: [true, false, true, true, true],
            light: Vector3::new(0., 100., 0.),
            plane: (
                init("plane normals", bytemuck::cast_slice(&f("normal")?), vertex),
                init(
                    "plane positions",
                    bytemuck::cast_slice(&f("position")?),
                    vertex,
                ),
                init(
                    "plane index",
                    bytemuck::cast_slice(&index),
                    wgpu::BufferUsages::INDEX,
                ),
            ),
            sphere: None,
            shadow_pipeline,
            shadow_render: (shadow_render_buffer, shadow_render),
            shadow_map: (
                texture(r, (SHADOW, SHADOW), LDR),
                depth_texture.create_view(&Default::default()),
            ),
            plane_render: sized(r, "plane render", wgsl!("plane_fs"), "renderStruct")?,
            plane_object: sized(r, "plane object", wgsl!("plane_fs"), "objectStruct")?,
            sphere_render: sized(r, "sphere render", wgsl!("sphere_fs"), "renderStruct")?,
            sphere_object: sized(r, "sphere object", wgsl!("sphere_fs"), "objectStruct")?,
            shadow_object: sized(r, "shadow object", wgsl!("shadow_fs"), "objectStruct")?,
            output_render: uniform(r, "output render", 144),
            output_object: uniform(r, "output object", 64),
            depth_render: uniform(r, "outline depth render", 128),
            depth_object: sized(
                r,
                "outline depth object",
                wgsl!("outline_depth_fs"),
                "objectStruct",
            )?,
            mask_render: sized(r, "mask render", wgsl!("mask_fs"), "renderStruct")?,
            mask_object: sized(r, "mask object", wgsl!("mask_fs"), "objectStruct")?,
            identity,
            blur_objects: [
                sized(r, "outline blur", wgsl!("blur_fs"), "objectStruct")?,
                sized(r, "outline blur", wgsl!("blur_fs"), "objectStruct")?,
                sized(r, "outline blur", wgsl!("blur_fs"), "objectStruct")?,
                sized(r, "outline blur", wgsl!("blur_fs"), "objectStruct")?,
            ],
            edge_object: uniform(r, "edge object", 192),
            composite_object: uniform(r, "composite object", 144),
            quad: init(
                "quad",
                bytemuck::cast_slice(&[-1f32, 3., 0., -1., -1., 0., 3., -1., 0.]),
                vertex,
            ),
            quad_uv: init(
                "quad uv",
                bytemuck::cast_slice(&[0f32, -1., 0., 1., 2., 1.]),
                vertex,
            ),
            linear: r.device.create_sampler(&wgpu::SamplerDescriptor {
                mag_filter: wgpu::FilterMode::Linear,
                min_filter: wgpu::FilterMode::Linear,
                ..Default::default()
            }),
            mipmapped: r.device.create_sampler(&wgpu::SamplerDescriptor {
                mag_filter: wgpu::FilterMode::Linear,
                min_filter: wgpu::FilterMode::Linear,
                mipmap_filter: wgpu::FilterMode::Linear,
                ..Default::default()
            }),
            compare: r.device.create_sampler(&wgpu::SamplerDescriptor {
                mag_filter: wgpu::FilterMode::Linear,
                min_filter: wgpu::FilterMode::Linear,
                compare: Some(wgpu::CompareFunction::LessEqual),
                ..Default::default()
            }),
            mipmaps: Mipmaps::new(r),
            targets: None,
        })
    }
    /// The fixture's seeded Math.random.
    fn random(&mut self) -> f64 {
        self.random = self.random.wrapping_mul(1664525).wrapping_add(1013904223);
        self.random as f64 / 4294967296.
    }
    /// animate(): a new SphereGeometry( 50, random × 64, random × 32 ) and a
    /// new CanvasTexture filled with rgb( ⌊random × 256⌋, … ), the previous
    /// sphere disposed.
    fn new_sphere(&mut self, r: &Renderer) -> Result<()> {
        let width = (self.random() * 64.).floor().max(3.) as u32;
        let height = (self.random() * 32.).floor().max(2.) as u32;
        let rgb = [0; 3].map(|_| (self.random() * 256.).floor() as u8);
        let g = SphereGeometry::build(50., width, height)?;
        let read = |name: &str, n: usize| -> Result<Vec<f32>> {
            let a = g
                .attributes
                .get(name)
                .ok_or(Error::Invalid("sphere attribute"))?;
            (0..a.count())
                .flat_map(|i| (0..n).map(move |k| (i, k)))
                .map(|(i, k)| a.get_component(i, k).map(|v| v as f32))
                .collect()
        };
        let index = g.index.clone().ok_or(Error::Invalid("sphere index"))?;
        let init = |label, data: &[u8], usage| {
            r.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some(label),
                    contents: data,
                    usage,
                })
        };
        let vertex = wgpu::BufferUsages::VERTEX;
        let size = (256, 256);
        let canvas = r.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("canvas texture"),
            size: wgpu::Extent3d {
                width: size.0,
                height: size.1,
                depth_or_array_layers: 1,
            },
            mip_level_count: mip_count(size),
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: LDR,
            usage: wgpu::TextureUsages::TEXTURE_BINDING
                | wgpu::TextureUsages::COPY_DST
                | wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });
        let pixels: Vec<u8> = (0..size.0 * size.1)
            .flat_map(|_| [rgb[0], rgb[1], rgb[2], 255])
            .collect();
        r.queue.write_texture(
            canvas.as_image_copy(),
            &pixels,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(size.0 * 4),
                rows_per_image: Some(size.1),
            },
            wgpu::Extent3d {
                width: size.0,
                height: size.1,
                depth_or_array_layers: 1,
            },
        );
        let mut encoder = r.device.create_command_encoder(&Default::default());
        let levels = self.mipmaps.levels(r, &canvas);
        self.mipmaps.encode(&mut encoder, LDR, &levels);
        r.queue.submit([encoder.finish()]);
        let map = canvas.create_view(&Default::default());
        let tex = wgpu::BindingResource::TextureView(&map);
        let shadow = bind(
            r,
            self.shadow_pipeline.get_bind_group_layout(1),
            &[
                (0, wgpu::BindingResource::Sampler(&self.mipmapped)),
                (1, tex),
                (2, self.shadow_object.as_entire_binding()),
            ],
        );
        self.sphere = Some(Ball {
            positions: init(
                "sphere positions",
                bytemuck::cast_slice(&read("position", 3)?),
                vertex,
            ),
            normals: init(
                "sphere normals",
                bytemuck::cast_slice(&read("normal", 3)?),
                vertex,
            ),
            uvs: init("sphere uvs", bytemuck::cast_slice(&read("uv", 2)?), vertex),
            index: init(
                "sphere index",
                bytemuck::cast_slice(&index),
                wgpu::BufferUsages::INDEX,
            ),
            count: index.len() as u32,
            shadow,
            groups: None,
            map,
        });
        self.rebind(r);
        Ok(())
    }
    /// The sphere's groups that read the size-dependent pipelines and targets.
    fn rebind(&mut self, r: &Renderer) {
        let (Some(t), Some(ball)) = (self.targets.as_ref(), self.sphere.as_mut()) else {
            return;
        };
        let lambert = bind(
            r,
            t.sphere.0.get_bind_group_layout(1),
            &[
                (0, self.sphere_object.as_entire_binding()),
                (1, wgpu::BindingResource::Sampler(&self.mipmapped)),
                (2, wgpu::BindingResource::TextureView(&ball.map)),
            ],
        );
        let outline = bind(
            r,
            t.mask_pass.0.get_bind_group_layout(1),
            &[
                (0, self.mask_object.as_entire_binding()),
                (1, wgpu::BindingResource::TextureView(&t.outline_depth.1)),
            ],
        );
        ball.groups = Some((lambert, outline));
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, _dt: f64, animate: bool) -> Result<()> {
        if animate {
            self.pending = true;
        }
        Ok(())
    }
    pub fn prepare(&mut self, _s: &mut Scene, _c: Object3D, _r: &Renderer) -> Result<()> {
        Ok(())
    }
    fn resize(&mut self, r: &Renderer, out: &RenderTarget) -> Result<()> {
        let size = (out.width, out.height);
        let half = (
            ((size.0 as f64 / 2.).round() as u32).max(1),
            ((size.1 as f64 / 2.).round() as u32).max(1),
        );
        let quarter = (
            ((half.0 as f64 / 2.).round() as u32).max(1),
            ((half.1 as f64 / 2.).round() as u32).max(1),
        );
        let attributes = [
            wgpu::vertex_attr_array![0 => Float32x3],
            wgpu::vertex_attr_array![1 => Float32x3],
            wgpu::vertex_attr_array![0 => Float32x2],
            wgpu::vertex_attr_array![1 => Float32x3],
            wgpu::vertex_attr_array![2 => Float32x3],
        ];
        let layout = |i: usize, stride| wgpu::VertexBufferLayout {
            array_stride: stride,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &attributes[i],
        };
        let tex = wgpu::BindingResource::TextureView;
        let sampler = wgpu::BindingResource::Sampler;
        let color = texture(r, size, HALF);
        let depth = texture(r, size, DEPTH);
        let plane = pipeline(
            r,
            "memory plane",
            wgsl!("plane_vs"),
            wgsl!("plane_fs"),
            &[layout(0, 12), layout(1, 12)],
            HALF,
            Some(DEPTH),
            false,
        );
        let plane_binds = [
            bind(
                r,
                plane.get_bind_group_layout(0),
                &[(0, self.plane_render.as_entire_binding())],
            ),
            bind(
                r,
                plane.get_bind_group_layout(1),
                &[
                    (0, self.plane_object.as_entire_binding()),
                    (1, sampler(&self.compare)),
                    (2, tex(&self.shadow_map.1)),
                ],
            ),
        ];
        let sphere = pipeline(
            r,
            "memory sphere",
            wgsl!("sphere_vs"),
            wgsl!("sphere_fs"),
            &[layout(2, 8), layout(3, 12), layout(4, 12)],
            HALF,
            Some(DEPTH),
            false,
        );
        let sphere_render = bind(
            r,
            sphere.get_bind_group_layout(0),
            &[(0, self.sphere_render.as_entire_binding())],
        );
        let output = pipeline(
            r,
            "render output",
            include_str!("lights_dynamic/output_vs.wgsl"),
            include_str!("lights_dynamic/output_fs.wgsl"),
            &[layout(0, 12)],
            out.options.format,
            None,
            false,
        );
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
                    (1, tex(&color)),
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
        // OutlineNode's targets and passes.
        let outline_depth = (
            texture(r, size, LDR),
            texture(r, size, wgpu::TextureFormat::Depth32Float),
        );
        let mask = (texture(r, size, LDR), texture(r, size, DEPTH));
        let halves = [0; 3].map(|_| texture(r, half, LDR));
        let quarters = [0; 2].map(|_| texture(r, quarter, LDR));
        let composite = texture(r, size, LDR);
        let depth_pass = pipeline(
            r,
            "outline depth",
            wgsl!("outline_depth_vs"),
            wgsl!("outline_depth_fs"),
            &[layout(0, 12)],
            LDR,
            Some(wgpu::TextureFormat::Depth32Float),
            false,
        );
        let depth_render = bind(
            r,
            depth_pass.get_bind_group_layout(0),
            &[(0, self.depth_render.as_entire_binding())],
        );
        let depth_object = bind(
            r,
            depth_pass.get_bind_group_layout(1),
            &[(0, self.depth_object.as_entire_binding())],
        );
        let mask_pass = pipeline(
            r,
            "outline mask",
            wgsl!("mask_vs"),
            wgsl!("mask_fs"),
            &[layout(0, 12)],
            LDR,
            Some(DEPTH),
            false,
        );
        let mask_render = bind(
            r,
            mask_pass.get_bind_group_layout(0),
            &[(0, self.mask_render.as_entire_binding())],
        );
        let quad = [layout(2, 8)];
        let downsample = pipeline(
            r,
            "outline downsample",
            wgsl!("downsample_vs"),
            wgsl!("downsample_fs"),
            &quad,
            LDR,
            None,
            false,
        );
        let downsample_bind = bind(
            r,
            downsample.get_bind_group_layout(0),
            &[
                (0, sampler(&self.linear)),
                (1, tex(&mask.0)),
                (2, self.identity.as_entire_binding()),
            ],
        );
        let edge = pipeline(
            r,
            "outline edges",
            wgsl!("quad_vs"),
            wgsl!("edge_fs"),
            &quad,
            LDR,
            None,
            false,
        );
        let edge_bind = bind(
            r,
            edge.get_bind_group_layout(0),
            &[
                (0, sampler(&self.linear)),
                (1, tex(&halves[0])),
                (2, self.edge_object.as_entire_binding()),
            ],
        );
        // Blur sources: edge 1 → blur 1 → edge 1 (half), then edge 1 → blur 2
        // → edge 2 (quarter); every blur takes its texel size from the
        // downsampled mask, as the page's does.
        let sources = [&halves[1], &halves[2], &halves[1], &quarters[0]];
        let blurs = [0, 1, 2, 3].map(|i| {
            let fs = if i < 2 {
                wgsl!("blur_fs")
            } else {
                wgsl!("blur_half_fs")
            };
            let p = pipeline(
                r,
                "outline blur",
                wgsl!("quad_vs"),
                fs,
                &quad,
                LDR,
                None,
                false,
            );
            let group = bind(
                r,
                p.get_bind_group_layout(0),
                &[
                    // Read for its size only: the auto layout drops its sampler.
                    (1, tex(&halves[0])),
                    (2, sampler(&self.linear)),
                    (3, tex(sources[i])),
                    (4, self.blur_objects[i].as_entire_binding()),
                ],
            );
            (p, group)
        });
        let composite_pass = pipeline(
            r,
            "outline composite",
            wgsl!("composite_vs"),
            wgsl!("composite_fs"),
            &quad,
            LDR,
            None,
            false,
        );
        let composite_bind = bind(
            r,
            composite_pass.get_bind_group_layout(0),
            &[
                (0, sampler(&self.linear)),
                (1, tex(&mask.0)),
                (2, self.composite_object.as_entire_binding()),
                (3, sampler(&self.linear)),
                (4, tex(&halves[1])),
                (5, sampler(&self.linear)),
                (6, tex(&quarters[1])),
            ],
        );
        let pp_output = pipeline(
            r,
            "outline output",
            wgsl!("pp_output_vs"),
            wgsl!("pp_output_fs"),
            &quad,
            out.options.format,
            None,
            false,
        );
        let pp_output_bind = bind(
            r,
            pp_output.get_bind_group_layout(0),
            &[
                (0, sampler(&self.linear)),
                (1, tex(&color)),
                (2, sampler(&self.linear)),
                (3, tex(&composite)),
            ],
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
        // The blur direction of each pass (x, y, x, y) with identity uv
        // transforms around it.
        for (i, object) in self.blur_objects.iter().enumerate() {
            let direction = if i % 2 == 0 { [1., 0.] } else { [0., 1.] };
            r.queue.write_buffer(
                object,
                0,
                &pack(
                    wgsl!("blur_fs"),
                    "objectStruct",
                    &[
                        ("nodeUniform2", &m3(Matrix4::IDENTITY)),
                        ("nodeUniform3", &direction),
                        ("nodeUniform4", &m3(Matrix4::IDENTITY)),
                        ("nodeUniform5", &m3(Matrix4::IDENTITY)),
                    ],
                )?,
            );
        }
        r.queue.write_buffer(
            &self.edge_object,
            0,
            &pack(
                wgsl!("edge_fs"),
                "objectStruct",
                &[
                    ("nodeUniform1", &m3(Matrix4::IDENTITY)),
                    ("nodeUniform2", &m3(Matrix4::IDENTITY)),
                    ("nodeUniform3", &m3(Matrix4::IDENTITY)),
                    ("nodeUniform4", &m3(Matrix4::IDENTITY)),
                ],
            )?,
        );
        r.queue.write_buffer(
            &self.composite_object,
            0,
            &pack(
                wgsl!("composite_fs"),
                "objectStruct",
                &[
                    ("nodeUniform1", &m3(Matrix4::IDENTITY)),
                    ("nodeUniform3", &m3(Matrix4::IDENTITY)),
                    ("nodeUniform5", &m3(Matrix4::IDENTITY)),
                ],
            )?,
        );
        self.targets = Some(Targets {
            width: size.0,
            height: size.1,
            format: out.options.format,
            color,
            depth,
            screen,
            plane: (plane, plane_binds),
            sphere: (sphere, sphere_render),
            output: (output, output_binds),
            outline_depth,
            mask,
            half: halves,
            quarter: quarters,
            composite,
            depth_pass: (depth_pass, depth_render, depth_object),
            mask_pass: (mask_pass, mask_render),
            downsample: (downsample, downsample_bind),
            edge: (edge, edge_bind),
            blurs,
            composite_pass: (composite_pass, composite_bind),
            pp_output: (pp_output, pp_output_bind),
        });
        // The sphere's bind groups belong to the old pipelines: rebuild it
        // with the same random state.
        Ok(())
    }
    pub fn render(
        &mut self,
        r: &Renderer,
        s: &mut Scene,
        c: Object3D,
        out: &RenderTarget,
    ) -> Result<bool> {
        let resized = self.targets.as_ref().is_none_or(|t| {
            t.width != out.width || t.height != out.height || t.format != out.options.format
        });
        if resized {
            self.resize(r, out)?;
            self.rebind(r);
        }
        // animate(): a new sphere when generating (and always the first).
        if (std::mem::take(&mut self.pending) && self.params[4]) || self.sphere.is_none() {
            self.new_sphere(r)?;
        }
        s.update()?;
        let (camera, world) = s.camera(c)?;
        let Camera::Perspective(p) = camera else {
            return Err(Error::Invalid("memory camera"));
        };
        let (near, far) = (p.near, p.far);
        let (projection, view) = (camera.projection_matrix()?, world.inverse());
        let shadows = self.params[0];
        let light = self.light;
        // The default DirectionalLightShadow camera: ±5, near 0.5, far 500.
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
            -1. / 499.5,
            0.,
            0.,
            0.,
            -0.5 / 499.5,
            1.,
        ]);
        // Matrix4.lookAt nudges the light looking straight down off the up axis.
        let shadow_view = (Matrix4::from_translation(light)
            * look_rotation(light, Vector3::ZERO, Vector3::Y))
        .inverse();
        let bias = Matrix4::from_cols_array(&[
            0.5, 0., 0., 0., 0., 0.5, 0., 0., 0., 0., 1., 0., 0.5, 0.5, 0., 1.,
        ]);
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
        let plane_model = Matrix4::from_translation(Vector3::new(0., -100., 0.))
            * Matrix4::from_rotation_x(-PI / 2.);
        write(
            &self.plane_render,
            wgsl!("plane_fs"),
            "renderStruct",
            &[
                ("cameraProjectionMatrix", m4(projection)),
                ("cameraViewMatrix", m4(view)),
                ("nodeUniform7", light.to_array().to_vec()),
                ("nodeUniform8", vec![0.; 3]),
                ("nodeUniform9", vec![1.; 3]),
                ("nodeUniform11", m4(bias * shadow_projection * shadow_view)),
                ("nodeUniform12", vec![0.]),
                ("nodeUniform13", vec![0.]),
                ("nodeUniform15", vec![1.]),
                ("nodeUniform16", vec![SHADOW as f64; 2]),
                // Cast shadow off: the shadow factor is 1, as the shader
                // three.js recompiles without it.
                ("nodeUniform17", vec![if shadows { 1. } else { 0. }]),
            ],
        )?;
        write(
            &self.plane_object,
            wgsl!("plane_fs"),
            "objectStruct",
            &[
                (
                    "nodeUniform0",
                    Color::from_hex(0xcccccc).0.to_array().to_vec(),
                ),
                ("nodeUniform1", vec![1.]),
                ("nodeUniform3", vec![1.]),
                ("nodeUniform5", m3(plane_model.inverse().transpose())),
                ("nodeUniform10", m4(plane_model)),
            ],
        )?;
        write(
            &self.sphere_render,
            wgsl!("sphere_fs"),
            "renderStruct",
            &[
                ("cameraProjectionMatrix", m4(projection)),
                ("cameraViewMatrix", m4(view)),
                ("nodeUniform9", light.to_array().to_vec()),
                ("nodeUniform10", vec![0.; 3]),
                ("nodeUniform11", vec![1.; 3]),
            ],
        )?;
        write(
            &self.sphere_object,
            wgsl!("sphere_fs"),
            "objectStruct",
            &[
                ("nodeUniform0", vec![1.; 3]),
                ("nodeUniform2", m3(Matrix4::IDENTITY)),
                ("nodeUniform3", vec![1.]),
                ("nodeUniform5", vec![1.]),
                ("nodeUniform7", m3(Matrix4::IDENTITY)),
                ("nodeUniform13", m4(Matrix4::IDENTITY)),
            ],
        )?;
        write(
            &self.shadow_object,
            wgsl!("shadow_fs"),
            "objectStruct",
            &[
                ("nodeUniform1", m3(Matrix4::IDENTITY)),
                ("nodeUniform2", vec![1.]),
                ("nodeUniform5", m4(Matrix4::IDENTITY)),
            ],
        )?;
        let mut camera_data: Vec<f32> =
            shadow_projection.to_cols_array().map(|v| v as f32).to_vec();
        camera_data.extend(shadow_view.to_cols_array().map(|v| v as f32));
        r.queue
            .write_buffer(&self.shadow_render.0, 0, bytemuck::cast_slice(&camera_data));
        let (post, outline) = (self.params[1], self.params[2]);
        let t = self
            .targets
            .as_ref()
            .ok_or(Error::Invalid("memory targets"))?;
        let sphere = self
            .sphere
            .as_ref()
            .ok_or(Error::Invalid("memory sphere"))?;
        let groups = sphere
            .groups
            .as_ref()
            .ok_or(Error::Invalid("memory sphere"))?;
        if post && outline {
            let mut main: Vec<f32> = projection.to_cols_array().map(|v| v as f32).to_vec();
            main.extend(view.to_cols_array().map(|v| v as f32));
            r.queue
                .write_buffer(&self.depth_render, 0, bytemuck::cast_slice(&main));
            write(
                &self.depth_object,
                wgsl!("outline_depth_fs"),
                "objectStruct",
                &[
                    ("nodeUniform0", vec![1.]),
                    ("nodeUniform3", m4(plane_model)),
                ],
            )?;
            write(
                &self.mask_render,
                wgsl!("mask_fs"),
                "renderStruct",
                &[
                    ("cameraProjectionMatrix", m4(projection)),
                    ("cameraViewMatrix", m4(view)),
                    ("nodeUniform6", vec![t.width as f64, t.height as f64]),
                ],
            )?;
            write(
                &self.mask_object,
                wgsl!("mask_fs"),
                "objectStruct",
                &[
                    ("nodeUniform1", m4(Matrix4::IDENTITY)),
                    ("nodeUniform2", vec![near]),
                    ("nodeUniform3", vec![far]),
                    ("nodeUniform5", m3(Matrix4::IDENTITY)),
                    ("nodeUniform7", vec![1.]),
                ],
            )?;
        }
        let frustum = Frustum::from_projection(projection * view);
        let sphere_visible = frustum.intersects_sphere(Sphere {
            center: Vector3::ZERO,
            radius: 50.,
        });
        let plane_visible = frustum.intersects_sphere(Sphere {
            center: plane_model.transform_point3(Vector3::ZERO),
            radius: 500. * 2f64.sqrt(),
        });
        let mut encoder = r.device.create_command_encoder(&Default::default());
        let draw_sphere = |pass: &mut wgpu::RenderPass, slots: &[&wgpu::Buffer]| {
            for (slot, buffer) in slots.iter().enumerate() {
                pass.set_vertex_buffer(slot as u32, buffer.slice(..));
            }
            pass.set_index_buffer(sphere.index.slice(..), wgpu::IndexFormat::Uint32);
            pass.draw_indexed(0..sphere.count, 0, 0..1);
        };
        let draw_plane = |pass: &mut wgpu::RenderPass, slots: &[&wgpu::Buffer]| {
            for (slot, buffer) in slots.iter().enumerate() {
                pass.set_vertex_buffer(slot as u32, buffer.slice(..));
            }
            pass.set_index_buffer(self.plane.2.slice(..), wgpu::IndexFormat::Uint32);
            pass.draw_indexed(0..6, 0, 0..1);
        };
        if shadows {
            let frustum = Frustum::from_projection(shadow_projection * shadow_view);
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("memory shadow"),
                color_attachments: &[target(&self.shadow_map.0, wgpu::Color::TRANSPARENT)],
                depth_stencil_attachment: depth_attachment(&self.shadow_map.1),
                ..Default::default()
            });
            if frustum.intersects_sphere(Sphere {
                center: Vector3::ZERO,
                radius: 50.,
            }) {
                pass.set_pipeline(&self.shadow_pipeline);
                pass.set_bind_group(0, &self.shadow_render.1, &[]);
                pass.set_bind_group(1, &sphere.shadow, &[]);
                draw_sphere(&mut pass, &[&sphere.uvs, &sphere.positions]);
            }
        }
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("memory scene"),
                color_attachments: &[target(&t.color, wgpu::Color::WHITE)],
                depth_stencil_attachment: depth_attachment(&t.depth),
                ..Default::default()
            });
            if plane_visible {
                pass.set_pipeline(&t.plane.0);
                pass.set_bind_group(0, &t.plane.1[0], &[]);
                pass.set_bind_group(1, &t.plane.1[1], &[]);
                draw_plane(&mut pass, &[&self.plane.0, &self.plane.1]);
            }
            if sphere_visible {
                pass.set_pipeline(&t.sphere.0);
                pass.set_bind_group(0, &t.sphere.1, &[]);
                pass.set_bind_group(1, &groups.0, &[]);
                draw_sphere(
                    &mut pass,
                    &[&sphere.uvs, &sphere.normals, &sphere.positions],
                );
            }
        }
        let quad = |encoder: &mut wgpu::CommandEncoder,
                    view: &wgpu::TextureView,
                    pipeline: &wgpu::RenderPipeline,
                    group: &wgpu::BindGroup| {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("outline quad"),
                color_attachments: &[target(view, wgpu::Color::WHITE)],
                ..Default::default()
            });
            pass.set_pipeline(pipeline);
            pass.set_bind_group(0, group, &[]);
            pass.set_vertex_buffer(0, self.quad_uv.slice(..));
            pass.draw(0..3, 0..1);
        };
        if post && outline {
            // 1. Non-selected objects into the depth buffer.
            {
                let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("outline depth"),
                    color_attachments: &[target(&t.outline_depth.0, wgpu::Color::WHITE)],
                    depth_stencil_attachment: depth_attachment(&t.outline_depth.1),
                    ..Default::default()
                });
                if plane_visible {
                    pass.set_pipeline(&t.depth_pass.0);
                    pass.set_bind_group(0, &t.depth_pass.1, &[]);
                    pass.set_bind_group(1, &t.depth_pass.2, &[]);
                    draw_plane(&mut pass, &[&self.plane.1]);
                }
            }
            // 2. The selected sphere against that depth.
            {
                let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("outline mask"),
                    color_attachments: &[target(&t.mask.0, wgpu::Color::WHITE)],
                    depth_stencil_attachment: depth_attachment(&t.mask.1),
                    ..Default::default()
                });
                if sphere_visible {
                    pass.set_pipeline(&t.mask_pass.0);
                    pass.set_bind_group(0, &t.mask_pass.1, &[]);
                    pass.set_bind_group(1, &groups.1, &[]);
                    draw_sphere(&mut pass, &[&sphere.positions]);
                }
            }
            quad(&mut encoder, &t.half[0], &t.downsample.0, &t.downsample.1);
            quad(&mut encoder, &t.half[1], &t.edge.0, &t.edge.1);
            let targets = [&t.half[2], &t.half[1], &t.quarter[0], &t.quarter[1]];
            for (i, (pipeline, group)) in t.blurs.iter().enumerate() {
                quad(&mut encoder, targets[i], pipeline, group);
            }
            quad(
                &mut encoder,
                &t.composite,
                &t.composite_pass.0,
                &t.composite_pass.1,
            );
        }
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("memory output"),
                color_attachments: &[target(&t.screen.view, wgpu::Color::TRANSPARENT)],
                ..Default::default()
            });
            if post && outline {
                pass.set_pipeline(&t.pp_output.0);
                pass.set_bind_group(0, &t.pp_output.1, &[]);
                pass.set_vertex_buffer(0, self.quad_uv.slice(..));
            } else {
                pass.set_pipeline(&t.output.0);
                pass.set_bind_group(0, &t.output.1[0], &[]);
                pass.set_bind_group(1, &t.output.1[1], &[]);
                pass.set_vertex_buffer(0, self.quad.slice(..));
            }
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
        _s: &mut Scene,
        _c: Object3D,
        _dx: f64,
        _dy: f64,
        _wheel: f64,
        _pan: bool,
        _height: f64,
    ) -> Result<()> {
        Ok(())
    }
    /// cast shadow, post-processing, outline, AO, recreate light, start and
    /// stop creating meshes.
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        let on = value > 0.5;
        match index {
            0..=3 => self.params[index] = on,
            // A new DirectionalLight at a random position over the scene.
            4 => {
                let x = self.random() * 200. - 100.;
                let z = self.random() * 200. - 100.;
                self.light = Vector3::new(x, 100., z);
            }
            5 => self.params[4] = true,
            6 => self.params[4] = false,
            _ => return Err(Error::Invalid("memory parameter")),
        }
        Ok(())
    }
    pub fn seek(&mut self, _t: f64) {
        self.pending = true;
    }
}
