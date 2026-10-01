//! webgpu_deferred: the teapot, eight orbiting point lights with their
//! spheres and six transparent double-sided planes under royal_esplanade's
//! UltraHDR sky. Deferred mode renders the opaque meshes unlit into an MRT
//! ( diffuse color, view position with metalness, view normal with
//! roughness ), resolves the lighting on a full-screen quad over the sky
//! background with the opaque depth, renders the transparent planes over
//! that depth and composites the two; forward mode renders the scene lit in
//! one pass. The sky is CubeMapNode's 1024² cube in the background and a
//! lodMax 9 PMREM for the lighting. Every stage runs the WGSL three.js r186
//! generates for the page (in `deferred/`).
use super::controls_attributes::{Controls, camera_state};
use super::gltf_viewer::fetch;
use super::lights_projector::{m3, m4, pack};
use super::models_modifiers::teapot;
use super::pmrem_cube_uv::{Pmrem, bind, flat_camera, raw_pipeline, source_pipeline};
use super::probes_hdr::ultra_hdr;
use super::retro::{mipmapped_raw, target, uniform};
use super::shadowmap_opacity::Mipmaps;
use crate::{
    Error, Result, camera::*, geometry::*, math::*, render_target::*, renderer::*, scene::*,
};
use std::f64::consts::PI;
use wgpu::util::DeviceExt;

const HALF: wgpu::TextureFormat = wgpu::TextureFormat::Rgba16Float;
const DEPTH: wgpu::TextureFormat = wgpu::TextureFormat::Depth24Plus;
const LIGHTS: usize = 8;
const PLANES: usize = 6;
macro_rules! wgsl {
    ($name:literal) => {
        include_str!(concat!("deferred/", $name, ".wgsl"))
    };
}
/// Vertex buffers ( one per slot ), the index and its count.
struct Mesh {
    buffers: Vec<wgpu::Buffer>,
    index: wgpu::Buffer,
    count: u32,
}
impl Mesh {
    fn draw(&self, pass: &mut wgpu::RenderPass) {
        for (slot, buffer) in self.buffers.iter().enumerate() {
            pass.set_vertex_buffer(slot as u32, buffer.slice(..));
        }
        pass.set_index_buffer(self.index.slice(..), wgpu::IndexFormat::Uint32);
        pass.draw_indexed(0..self.count, 0, 0..1);
    }
}
/// `geometry`'s normal and position as two vertex buffers, with its index.
fn mesh(r: &Renderer, geometry: &BufferGeometry) -> Result<Mesh> {
    let read = |name: &str| -> Result<Vec<f32>> {
        let a = geometry
            .attributes
            .get(name)
            .ok_or(Error::Invalid("deferred attribute"))?;
        (0..a.count())
            .flat_map(|i| (0..3).map(move |k| (i, k)))
            .map(|(i, k)| a.get_component(i, k).map(|v| v as f32))
            .collect()
    };
    let init = |data: &[u8], usage| {
        r.device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("deferred mesh"),
                contents: data,
                usage,
            })
    };
    let index = geometry
        .index
        .clone()
        .ok_or(Error::Invalid("deferred index"))?;
    Ok(Mesh {
        buffers: vec![
            init(
                bytemuck::cast_slice(&read("normal")?),
                wgpu::BufferUsages::VERTEX,
            ),
            init(
                bytemuck::cast_slice(&read("position")?),
                wgpu::BufferUsages::VERTEX,
            ),
        ],
        index: init(bytemuck::cast_slice(&index), wgpu::BufferUsages::INDEX),
        count: index.len() as u32,
    })
}
const NORMAL: [wgpu::VertexAttribute; 1] = [wgpu::VertexAttribute {
    format: wgpu::VertexFormat::Float32x3,
    offset: 0,
    shader_location: 0,
}];
const POSITION: [wgpu::VertexAttribute; 1] = [wgpu::VertexAttribute {
    format: wgpu::VertexFormat::Float32x3,
    offset: 0,
    shader_location: 1,
}];
const UV: [wgpu::VertexAttribute; 1] = [wgpu::VertexAttribute {
    format: wgpu::VertexFormat::Float32x2,
    offset: 0,
    shader_location: 0,
}];
/// normal at location 0 and position at 1.
const MESH_LAYOUTS: [wgpu::VertexBufferLayout<'static>; 2] = [
    wgpu::VertexBufferLayout {
        array_stride: 12,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: &NORMAL,
    },
    wgpu::VertexBufferLayout {
        array_stride: 12,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: &POSITION,
    },
];
/// QuadMesh: uv at location 0 and position at 1.
const QUAD_LAYOUTS: [wgpu::VertexBufferLayout<'static>; 2] = [
    wgpu::VertexBufferLayout {
        array_stride: 8,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: &UV,
    },
    wgpu::VertexBufferLayout {
        array_stride: 12,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: &POSITION,
    },
];
/// A triangle pipeline into `formats` ( alpha-blended when `blended` ),
/// back faces culled.
#[allow(clippy::too_many_arguments)]
fn pipeline(
    r: &Renderer,
    label: &str,
    vs: &str,
    fs: &str,
    buffers: &[wgpu::VertexBufferLayout],
    formats: &[wgpu::TextureFormat],
    depth: Option<(wgpu::CompareFunction, bool)>,
    cw: bool,
    blended: bool,
) -> wgpu::RenderPipeline {
    sampled_pipeline(
        r,
        label,
        (vs, fs),
        buffers,
        formats,
        depth,
        (cw, blended),
        (1, wgpu::PrimitiveTopology::TriangleList),
    )
}
/// The same with `samples` and `topology`.
#[allow(clippy::too_many_arguments)]
pub(super) fn sampled_pipeline(
    r: &Renderer,
    label: &str,
    (vs, fs): (&str, &str),
    buffers: &[wgpu::VertexBufferLayout],
    formats: &[wgpu::TextureFormat],
    depth: Option<(wgpu::CompareFunction, bool)>,
    (cw, blended): (bool, bool),
    (samples, topology): (u32, wgpu::PrimitiveTopology),
) -> wgpu::RenderPipeline {
    let module = |source: &str| {
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
    let targets: Vec<Option<wgpu::ColorTargetState>> = formats
        .iter()
        .map(|&format| {
            Some(wgpu::ColorTargetState {
                format,
                blend: blended.then_some(blend),
                write_mask: wgpu::ColorWrites::ALL,
            })
        })
        .collect();
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
                targets: &targets,
            }),
            primitive: wgpu::PrimitiveState {
                topology,
                front_face: if cw {
                    wgpu::FrontFace::Cw
                } else {
                    wgpu::FrontFace::Ccw
                },
                cull_mode: Some(wgpu::Face::Back),
                ..Default::default()
            },
            depth_stencil: depth.map(|(compare, write)| wgpu::DepthStencilState {
                format: DEPTH,
                depth_write_enabled: write,
                depth_compare: compare,
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
pub(super) type Draw = (wgpu::RenderPipeline, Vec<wgpu::BindGroup>);
pub(super) fn color(
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
pub(super) fn depth_attachment(
    view: &wgpu::TextureView,
    load: wgpu::LoadOp<f32>,
) -> Option<wgpu::RenderPassDepthStencilAttachment<'_>> {
    Some(wgpu::RenderPassDepthStencilAttachment {
        view,
        depth_ops: Some(wgpu::Operations {
            load,
            store: wgpu::StoreOp::Store,
        }),
        stencil_ops: None,
    })
}
pub(super) fn set(pass: &mut wgpu::RenderPass, (pipeline, groups): &Draw) {
    pass.set_pipeline(pipeline);
    for (i, g) in groups.iter().enumerate() {
        pass.set_bind_group(i as u32, g, &[]);
    }
}
struct Targets {
    width: u32,
    height: u32,
    format: wgpu::TextureFormat,
    /// The opaque MRT: diffuse color, view position and metalness, view
    /// normal and roughness, and its depth ( read by the resolve ).
    mrt: [wgpu::TextureView; 3],
    opaque_depth: wgpu::TextureView,
    resolved: wgpu::TextureView,
    resolved_depth: wgpu::TextureView,
    transparent: wgpu::TextureView,
    /// The forward pass's color and depth.
    forward: wgpu::TextureView,
    forward_depth: wgpu::TextureView,
    resolve: Draw,
    output: Draw,
    forward_output: Draw,
    screen: RenderTarget,
}
pub(super) struct Demo {
    controls: Controls,
    last: f64,
    pending: Option<f64>,
    /// lightGroup and planesGroup rotation.y.
    rotation: (f64, f64),
    /// Deferred ( else forward ) and animated.
    deferred: bool,
    animated: bool,
    background: Mesh,
    light_sphere: Mesh,
    teapot: Mesh,
    plane: Mesh,
    quad: [wgpu::Buffer; 2],
    output_uv: wgpu::Buffer,
    /// The light colors ( HSL in the working space ) and angles.
    lights: Vec<(Color, f64)>,
    cube: wgpu::TextureView,
    pmrem: Pmrem,
    clamp: wgpu::Sampler,
    linear: wgpu::Sampler,
    /// The resolved and forward passes' background, and the MRT's.
    background_draw: Draw,
    mrt_background: Draw,
    /// Opaque draws in the MRT ( the spheres, then the teapot ) and lit in
    /// the forward pass.
    opaque: Vec<Draw>,
    forward_opaque: Vec<Draw>,
    /// Back and front faces of each plane.
    planes: Vec<[Draw; 2]>,
    background_render: wgpu::Buffer,
    background_object: wgpu::Buffer,
    mrt_background_render: wgpu::Buffer,
    mrt_background_object: wgpu::Buffer,
    opaque_render: wgpu::Buffer,
    opaque_objects: Vec<wgpu::Buffer>,
    lit_render: wgpu::Buffer,
    forward_objects: Vec<wgpu::Buffer>,
    plane_render: wgpu::Buffer,
    plane_objects: Vec<wgpu::Buffer>,
    resolve_render: wgpu::Buffer,
    resolve_object: wgpu::Buffer,
    output_render: wgpu::Buffer,
    forward_output_render: wgpu::Buffer,
    targets: Option<Targets>,
}
/// Color.setHSL in the working ( linear ) color space.
fn hsl(h: f64, s: f64, l: f64) -> Color {
    let hue = |p: f64, q: f64, t: f64| {
        let t = t.rem_euclid(1.);
        if t < 1. / 6. {
            p + (q - p) * 6. * t
        } else if t < 0.5 {
            q
        } else if t < 2. / 3. {
            p + (q - p) * 6. * (2. / 3. - t)
        } else {
            p
        }
    };
    let q = if l <= 0.5 {
        l * (1. + s)
    } else {
        l + s - l * s
    };
    let p = 2. * l - q;
    Color(Vector3::new(
        hue(p, q, h + 1. / 3.),
        hue(p, q, h),
        hue(p, q, h - 1. / 3.),
    ))
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, _id: u32, r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 45.,
            near: 0.25,
            far: 20.,
            aspect,
            ..Default::default()
        }));
        let n = s.get_mut(c)?;
        n.position = Vector3::new(-1.8, 0.6, 2.7);
        n.quaternion = Quaternion::IDENTITY;
        let mut controls = Controls::new(None, (2., 10.), PI, true);
        controls.set_target(Vector3::new(0., 0., -0.2));
        controls.update(s, c)?;
        let sampler = |mipmap| {
            r.device.create_sampler(&wgpu::SamplerDescriptor {
                mag_filter: wgpu::FilterMode::Linear,
                min_filter: wgpu::FilterMode::Linear,
                mipmap_filter: mipmap,
                ..Default::default()
            })
        };
        let linear = sampler(wgpu::FilterMode::Linear);
        let clamp = sampler(wgpu::FilterMode::Nearest);
        let (equirect, height, pmrem, source) = esplanade(r, &linear, &clamp).await?;
        let cube = render_cube(r, &clamp, &equirect, height)?;
        let init = |label, data: &[u8], usage| {
            r.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some(label),
                    contents: data,
                    usage,
                })
        };
        let lights = (0..LIGHTS)
            .map(|i| {
                (
                    hsl(i as f64 / LIGHTS as f64, 1., 0.5),
                    i as f64 / LIGHTS as f64 * PI * 2.,
                )
            })
            .collect();
        let uniforms = |label, source, name, n| -> Result<Vec<wgpu::Buffer>> {
            (0..n).map(|_| uniform(r, label, source, name)).collect()
        };
        let mut demo = Self {
            controls,
            last: 0.,
            pending: None,
            rotation: (0., 0.),
            deferred: true,
            animated: true,
            background: mesh(r, &SphereGeometry::build(1., 32, 32)?)?,
            light_sphere: mesh(r, &SphereGeometry::build(0.03, 16, 8)?)?,
            teapot: mesh(r, &teapot(0.4, 18, true, true, true, true, true)?)?,
            plane: mesh(r, &PlaneGeometry::build(0.4, 0.4, 1, 1)?)?,
            quad: [
                init(
                    "deferred quad",
                    bytemuck::cast_slice(&[0f32, -1., 0., 1., 2., 1.]),
                    wgpu::BufferUsages::VERTEX,
                ),
                init(
                    "deferred quad",
                    bytemuck::cast_slice(&[-1f32, 3., 0., -1., -1., 0., 3., -1., 0.]),
                    wgpu::BufferUsages::VERTEX,
                ),
            ],
            output_uv: init(
                "deferred quad",
                bytemuck::cast_slice(&[0f32, -1., 0., 1., 2., 1.]),
                wgpu::BufferUsages::VERTEX,
            ),
            lights,
            cube,
            pmrem,
            clamp,
            linear,
            background_draw: (source.clone(), vec![]),
            mrt_background: (source, vec![]),
            opaque: vec![],
            forward_opaque: vec![],
            planes: vec![],
            background_render: uniform(
                r,
                "deferred background",
                wgsl!("forward_background_fs"),
                "renderStruct",
            )?,
            background_object: uniform(
                r,
                "deferred background",
                wgsl!("forward_background_fs"),
                "objectStruct",
            )?,
            mrt_background_render: uniform(
                r,
                "deferred background",
                wgsl!("background_fs"),
                "renderStruct",
            )?,
            mrt_background_object: uniform(
                r,
                "deferred background",
                wgsl!("background_fs"),
                "objectStruct",
            )?,
            opaque_render: uniform(r, "deferred opaque", wgsl!("opaque_vs"), "renderStruct")?,
            opaque_objects: uniforms(
                "deferred opaque",
                wgsl!("opaque_fs"),
                "objectStruct",
                LIGHTS + 1,
            )?,
            lit_render: uniform(
                r,
                "deferred lit",
                wgsl!("forward_standard_fs"),
                "renderStruct",
            )?,
            forward_objects: uniforms(
                "deferred lit",
                wgsl!("forward_standard_fs"),
                "objectStruct",
                LIGHTS + 1,
            )?,
            plane_render: uniform(r, "deferred plane", wgsl!("plane_back_fs"), "renderStruct")?,
            plane_objects: uniforms(
                "deferred plane",
                wgsl!("plane_back_fs"),
                "objectStruct",
                PLANES,
            )?,
            resolve_render: uniform(r, "deferred resolve", wgsl!("resolve_fs"), "renderStruct")?,
            resolve_object: uniform(r, "deferred resolve", wgsl!("resolve_fs"), "objectStruct")?,
            output_render: uniform(r, "deferred output", wgsl!("output_fs"), "renderStruct")?,
            forward_output_render: uniform(
                r,
                "deferred output",
                wgsl!("forward_output_fs"),
                "renderStruct",
            )?,
            targets: None,
        };
        demo.build(r);
        Ok(demo)
    }
    /// The draws independent of the drawing-buffer size.
    fn build(&mut self, r: &Renderer) {
        let three = [HALF; 3];
        let less = Some((wgpu::CompareFunction::LessEqual, true));
        let always = Some((wgpu::CompareFunction::Always, false));
        let tex = wgpu::BindingResource::TextureView;
        let sampler = wgpu::BindingResource::Sampler;
        let groups = |p: &wgpu::RenderPipeline,
                      render: &wgpu::Buffer,
                      object: Vec<(u32, wgpu::BindingResource)>| {
            vec![
                bind(
                    r,
                    p.get_bind_group_layout(0),
                    &[(0, render.as_entire_binding())],
                ),
                bind(r, p.get_bind_group_layout(1), &object),
            ]
        };
        let background =
            |vs, fs, formats: &[wgpu::TextureFormat], render, object: &wgpu::Buffer| {
                let p = pipeline(
                    r,
                    "deferred background",
                    vs,
                    fs,
                    &MESH_LAYOUTS,
                    formats,
                    always,
                    true,
                    false,
                );
                let g = groups(
                    &p,
                    render,
                    vec![
                        (0, sampler(&self.linear)),
                        (1, tex(&self.cube)),
                        (2, object.as_entire_binding()),
                    ],
                );
                (p, g)
            };
        self.background_draw = background(
            wgsl!("forward_background_vs"),
            wgsl!("forward_background_fs"),
            &[HALF],
            &self.background_render,
            &self.background_object,
        );
        self.mrt_background = background(
            wgsl!("background_vs"),
            wgsl!("background_fs"),
            &three,
            &self.mrt_background_render,
            &self.mrt_background_object,
        );
        let opaque = pipeline(
            r,
            "deferred opaque",
            wgsl!("opaque_vs"),
            wgsl!("opaque_fs"),
            &MESH_LAYOUTS,
            &three,
            less,
            false,
            false,
        );
        self.opaque = self
            .opaque_objects
            .iter()
            .map(|object| {
                (
                    opaque.clone(),
                    groups(
                        &opaque,
                        &self.opaque_render,
                        vec![(0, object.as_entire_binding())],
                    ),
                )
            })
            .collect();
        let lit_groups =
            |p: &wgpu::RenderPipeline, render: &wgpu::Buffer, object: &wgpu::Buffer| {
                groups(
                    p,
                    render,
                    vec![
                        (0, object.as_entire_binding()),
                        (1, sampler(&self.clamp)),
                        (2, tex(&r.dfg)),
                        (3, sampler(&self.clamp)),
                        (4, tex(&self.pmrem.view)),
                    ],
                )
            };
        let lit = pipeline(
            r,
            "deferred lit",
            wgsl!("plane_vs"),
            wgsl!("forward_standard_fs"),
            &MESH_LAYOUTS,
            &[HALF],
            less,
            false,
            false,
        );
        self.forward_opaque = self
            .forward_objects
            .iter()
            .map(|object| (lit.clone(), lit_groups(&lit, &self.lit_render, object)))
            .collect();
        let back = pipeline(
            r,
            "deferred plane",
            wgsl!("plane_vs"),
            wgsl!("plane_back_fs"),
            &MESH_LAYOUTS,
            &[HALF],
            less,
            true,
            true,
        );
        let front = pipeline(
            r,
            "deferred plane",
            wgsl!("plane_vs"),
            wgsl!("plane_front_fs"),
            &MESH_LAYOUTS,
            &[HALF],
            less,
            false,
            true,
        );
        self.planes = self
            .plane_objects
            .iter()
            .map(|object| {
                [
                    (back.clone(), lit_groups(&back, &self.plane_render, object)),
                    (
                        front.clone(),
                        lit_groups(&front, &self.plane_render, object),
                    ),
                ]
            })
            .collect();
    }
    /// animate(): the groups turn by the timer's delta while animated.
    fn step(&mut self, s: &mut Scene, c: Object3D, dt: f64) -> Result<()> {
        if self.animated {
            self.rotation.0 += dt;
            self.rotation.1 -= dt * 0.5;
        }
        self.controls.update(s, c)
    }
    pub fn update(&mut self, s: &mut Scene, c: Object3D, dt: f64, animate: bool) -> Result<()> {
        if animate {
            self.step(s, c, dt)?;
        }
        Ok(())
    }
    pub fn prepare(&mut self, s: &mut Scene, c: Object3D, _r: &Renderer) -> Result<()> {
        if let Some(t) = self.pending.take() {
            let dt = t - self.last;
            self.last = t;
            self.step(s, c, dt)?;
        }
        Ok(())
    }
    fn resize(&mut self, r: &Renderer, out: &RenderTarget) -> Result<()> {
        let size = (out.width, out.height);
        let readable = |format| {
            r.device
                .create_texture(&wgpu::TextureDescriptor {
                    label: Some("deferred target"),
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
        let mrt = [readable(HALF), readable(HALF), readable(HALF)];
        let opaque_depth = readable(DEPTH);
        let resolved = target(r, size, HALF, 1);
        let transparent = target(r, size, HALF, 1);
        let forward = target(r, size, HALF, 1);
        let tex = wgpu::BindingResource::TextureView;
        let sampler = wgpu::BindingResource::Sampler;
        let resolve = pipeline(
            r,
            "deferred resolve",
            wgsl!("resolve_vs"),
            wgsl!("resolve_fs"),
            &QUAD_LAYOUTS,
            &[HALF],
            Some((wgpu::CompareFunction::LessEqual, true)),
            false,
            false,
        );
        let resolve_groups = vec![
            bind(
                r,
                resolve.get_bind_group_layout(0),
                &[(0, self.resolve_render.as_entire_binding())],
            ),
            bind(
                r,
                resolve.get_bind_group_layout(1),
                &[
                    (0, tex(&opaque_depth)),
                    (1, sampler(&self.clamp)),
                    (2, tex(&mrt[0])),
                    (3, self.resolve_object.as_entire_binding()),
                    (4, sampler(&self.clamp)),
                    (5, tex(&mrt[1])),
                    (6, sampler(&self.clamp)),
                    (7, tex(&mrt[2])),
                    (8, sampler(&self.clamp)),
                    (9, tex(&r.dfg)),
                    (10, sampler(&self.clamp)),
                    (11, tex(&self.pmrem.view)),
                ],
            ),
        ];
        let uv = [wgpu::VertexBufferLayout {
            array_stride: 8,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &UV,
        }];
        let output = raw_pipeline(
            r,
            "deferred output",
            wgsl!("output_vs"),
            wgsl!("output_fs"),
            &uv,
            out.options.format,
            1,
            None,
            false,
        );
        let output_groups = vec![
            bind(
                r,
                output.get_bind_group_layout(0),
                &[(0, self.output_render.as_entire_binding())],
            ),
            bind(
                r,
                output.get_bind_group_layout(1),
                &[
                    (0, sampler(&self.clamp)),
                    (1, tex(&resolved)),
                    (2, sampler(&self.clamp)),
                    (3, tex(&transparent)),
                ],
            ),
        ];
        let forward_output = raw_pipeline(
            r,
            "deferred forward output",
            wgsl!("forward_output_vs"),
            wgsl!("forward_output_fs"),
            &uv,
            out.options.format,
            1,
            None,
            false,
        );
        let forward_output_groups = vec![
            bind(
                r,
                forward_output.get_bind_group_layout(0),
                &[(0, self.forward_output_render.as_entire_binding())],
            ),
            bind(
                r,
                forward_output.get_bind_group_layout(1),
                &[(0, sampler(&self.clamp)), (1, tex(&forward))],
            ),
        ];
        self.targets = Some(Targets {
            width: size.0,
            height: size.1,
            format: out.options.format,
            mrt,
            opaque_depth,
            resolved,
            resolved_depth: target(r, size, DEPTH, 1),
            transparent,
            forward,
            forward_depth: target(r, size, DEPTH, 1),
            resolve: (resolve, resolve_groups),
            output: (output, output_groups),
            forward_output: (forward_output, forward_output_groups),
            screen: RenderTarget::with_options(
                &r.device,
                size.0,
                size.1,
                RenderTargetOptions {
                    samples: 0,
                    depth_buffer: false,
                    ..out.options.clone()
                },
            )?,
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
            t.width != out.width || t.height != out.height || t.format != out.options.format
        }) {
            self.resize(r, out)?;
        }
        let t = self
            .targets
            .as_ref()
            .ok_or(Error::Invalid("deferred targets"))?;
        s.update()?;
        let (camera, world) = s.camera(c)?;
        let (projection, view) = (camera.projection_matrix()?, world.inverse());
        let write = |buffer: &wgpu::Buffer,
                     source: &str,
                     name: &str,
                     values: &[(String, Vec<f64>)]|
         -> Result<()> {
            let values: Vec<(&str, &[f64])> =
                values.iter().map(|(n, v)| (n.as_str(), &v[..])).collect();
            r.queue
                .write_buffer(buffer, 0, &pack(source, name, &values)?);
            Ok(())
        };
        let named = |values: Vec<(&str, Vec<f64>)>| -> Vec<(String, Vec<f64>)> {
            values
                .into_iter()
                .map(|(n, v)| (n.to_string(), v))
                .collect()
        };
        let uniform_name = |i: usize| format!("nodeUniform{i}");
        let identity3 = m3(Matrix4::IDENTITY);
        let camera_values = vec![
            ("cameraProjectionMatrix", m4(projection)),
            ("cameraViewMatrix", m4(view)),
        ];
        let background = [
            ("nodeUniform5", vec![0.]),
            ("nodeUniform6", vec![1.]),
            ("nodeUniform2", m4(Matrix4::IDENTITY)),
        ];
        let mut values = named(camera_values.clone());
        values.extend(named(background.to_vec()));
        write(
            &self.background_render,
            wgsl!("forward_background_fs"),
            "renderStruct",
            &values,
        )?;
        values.push((
            "cameraProjectionMatrixInverse".into(),
            m4(projection.inverse()),
        ));
        write(
            &self.mrt_background_render,
            wgsl!("background_fs"),
            "renderStruct",
            &values,
        )?;
        let background_object = |last: &'static str| {
            named(vec![
                ("nodeUniform1", m4(Matrix4::IDENTITY)),
                ("nodeUniform4", identity3.clone()),
                ("nodeUniform7", vec![1.]),
                (last, m4(Matrix4::IDENTITY)),
            ])
        };
        write(
            &self.background_object,
            wgsl!("forward_background_fs"),
            "objectStruct",
            &background_object("nodeUniform9"),
        )?;
        write(
            &self.mrt_background_object,
            wgsl!("background_fs"),
            "objectStruct",
            &background_object("nodeUniform10"),
        )?;
        // The light group, the planes group and the teapot sit 0.15 below the origin.
        let lights_group = Matrix4::from_translation(Vector3::new(0., -0.15, 0.))
            * Matrix4::from_rotation_y(self.rotation.0);
        let planes_group = Matrix4::from_translation(Vector3::new(0., -0.15, 0.))
            * Matrix4::from_rotation_y(self.rotation.1);
        let light_models: Vec<Matrix4> = self
            .lights
            .iter()
            .map(|(_, angle)| {
                lights_group
                    * Matrix4::from_translation(Vector3::new(
                        angle.cos() * 1.2,
                        0.,
                        angle.sin() * 1.2,
                    ))
            })
            .collect();
        let teapot_model = Matrix4::from_translation(Vector3::new(0., -0.15, 0.));
        let plane_models: Vec<Matrix4> = (0..PLANES)
            .map(|i| {
                planes_group
                    * Matrix4::from_rotation_y(i as f64 / PLANES as f64 * PI * 2.)
                    * Matrix4::from_translation(Vector3::new(0., 0., 1.5))
            })
            .collect();
        // PointLight( color, 5, 5 ): color × intensity, distance, decay and view position.
        let light_values = |first: usize| {
            let mut values = vec![];
            for (k, ((color, _), model)) in self.lights.iter().zip(&light_models).enumerate() {
                let n = |i: usize| uniform_name(first + 4 * k + i);
                values.push((
                    n(0),
                    view.transform_point3(model.w_axis.truncate())
                        .to_array()
                        .to_vec(),
                ));
                values.push((n(1), color.0.to_array().map(|v| v * 5.).to_vec()));
                values.push((n(2), vec![5.]));
                values.push((n(3), vec![2.]));
            }
            values
        };
        // MeshStandardMaterial: the spheres in their light's color ( roughness 1,
        // metalness 0 ), the teapot 0x333333 at roughness 0.2, metalness 0.8.
        let mut meshes: Vec<(Color, f64, f64, Matrix4)> = self
            .lights
            .iter()
            .zip(&light_models)
            .map(|((color, _), model)| (*color, 0., 1., *model))
            .collect();
        meshes.push((Color::from_hex(0x333333), 0.8, 0.2, teapot_model));
        let normal = |model: Matrix4| m3(model.inverse().transpose());
        let pmrem_values = |first: usize| {
            vec![
                (uniform_name(first), vec![9.]),
                (uniform_name(first + 1), m4(Matrix4::IDENTITY)),
                (uniform_name(first + 3), vec![1. / 1536.]),
                (uniform_name(first + 4), vec![1. / 2048.]),
                (uniform_name(first + 6), vec![1.]),
            ]
        };
        let lit_object =
            |color: Color, opacity: f64, metalness: f64, roughness: f64, model: Matrix4| {
                let mut values = named(vec![
                    ("nodeUniform0", color.0.to_array().to_vec()),
                    ("nodeUniform1", vec![opacity]),
                    ("nodeUniform2", vec![metalness]),
                    ("nodeUniform3", vec![roughness]),
                    ("nodeUniform5", normal(model)),
                    ("nodeUniform6", vec![0.; 3]),
                    ("nodeUniform7", vec![1.]),
                    ("nodeUniform9", m4(model)),
                ]);
                values.extend(pmrem_values(42));
                values
            };
        let mut lit_render = named(camera_values.clone());
        lit_render.push(("cameraWorldMatrix".into(), m4(world)));
        lit_render.extend(light_values(10));
        if self.deferred {
            write(
                &self.opaque_render,
                wgsl!("opaque_vs"),
                "renderStruct",
                &named(camera_values.clone()),
            )?;
            for ((color, metalness, roughness, model), buffer) in
                meshes.iter().zip(&self.opaque_objects)
            {
                write(
                    buffer,
                    wgsl!("opaque_fs"),
                    "objectStruct",
                    &named(vec![
                        ("nodeUniform0", color.0.to_array().to_vec()),
                        ("nodeUniform1", vec![1.]),
                        ("nodeUniform2", vec![*metalness]),
                        ("nodeUniform3", vec![*roughness]),
                        ("nodeUniform5", normal(*model)),
                        ("nodeUniform6", vec![0.; 3]),
                        ("nodeUniform7", vec![1.]),
                        ("nodeUniform8", m4(*model)),
                    ]),
                )?;
            }
            let mut values = light_values(8);
            values.push(("cameraViewMatrix".into(), m4(view)));
            values.push(("cameraWorldMatrix".into(), m4(world)));
            write(
                &self.resolve_render,
                wgsl!("resolve_fs"),
                "renderStruct",
                &values,
            )?;
            let mut values = named(vec![
                ("nodeUniform2", vec![1.]),
                ("nodeUniform5", vec![0.; 3]),
                ("nodeUniform6", vec![1.]),
            ]);
            values.extend(pmrem_values(40));
            write(
                &self.resolve_object,
                wgsl!("resolve_fs"),
                "objectStruct",
                &values,
            )?;
            write(
                &self.output_render,
                wgsl!("output_fs"),
                "renderStruct",
                &named(vec![("nodeUniform2", vec![1.])]),
            )?;
        } else {
            write(
                &self.lit_render,
                wgsl!("forward_standard_fs"),
                "renderStruct",
                &lit_render,
            )?;
            for ((color, metalness, roughness, model), buffer) in
                meshes.iter().zip(&self.forward_objects)
            {
                write(
                    buffer,
                    wgsl!("forward_standard_fs"),
                    "objectStruct",
                    &lit_object(*color, 1., *metalness, *roughness, *model),
                )?;
            }
            write(
                &self.forward_output_render,
                wgsl!("forward_output_fs"),
                "renderStruct",
                &named(vec![("nodeUniform1", vec![1.])]),
            )?;
        }
        write(
            &self.plane_render,
            wgsl!("plane_back_fs"),
            "renderStruct",
            &lit_render,
        )?;
        for (model, buffer) in plane_models.iter().zip(&self.plane_objects) {
            write(
                buffer,
                wgsl!("plane_back_fs"),
                "objectStruct",
                &lit_object(Color::from_hex(0x999999), 0.5, 0.5, 0.5, *model),
            )?;
        }
        // Opaque meshes front to back, transparent planes back to front, by
        // projected depth ( culled against the camera frustum ).
        let screen = projection * view;
        let frustum = Frustum::from_projection(screen);
        let depth = |model: &Matrix4| screen.project_point3(model.w_axis.truncate()).z;
        let visible = |model: &Matrix4, radius: f64| {
            frustum.intersects_sphere(Sphere {
                center: model.w_axis.truncate(),
                radius,
            })
        };
        let mut opaque: Vec<(f64, usize)> = meshes
            .iter()
            .enumerate()
            .filter(|(i, (_, _, _, model))| visible(model, if *i < LIGHTS { 0.03 } else { 0.8 }))
            .map(|(i, (_, _, _, model))| (depth(model), i))
            .collect();
        opaque.sort_by(|a, b| a.0.total_cmp(&b.0));
        let mut planes: Vec<(f64, usize)> = plane_models
            .iter()
            .enumerate()
            .filter(|(_, model)| visible(model, 0.2 * 2f64.sqrt()))
            .map(|(i, model)| (depth(model), i))
            .collect();
        planes.sort_by(|a, b| b.0.total_cmp(&a.0));
        let mesh_of = |i: usize| {
            if i < LIGHTS {
                &self.light_sphere
            } else {
                &self.teapot
            }
        };
        let draw_planes = |pass: &mut wgpu::RenderPass| {
            for &(_, i) in &planes {
                for draw in &self.planes[i] {
                    set(pass, draw);
                    self.plane.draw(pass);
                }
            }
        };
        let transparent = wgpu::Color::TRANSPARENT;
        let mut encoder = r.device.create_command_encoder(&Default::default());
        if self.deferred {
            {
                let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("deferred opaque"),
                    color_attachments: &[
                        color(&t.mrt[0], transparent),
                        color(&t.mrt[1], wgpu::Color::BLACK),
                        color(&t.mrt[2], wgpu::Color::BLACK),
                    ],
                    depth_stencil_attachment: depth_attachment(
                        &t.opaque_depth,
                        wgpu::LoadOp::Clear(1.),
                    ),
                    ..Default::default()
                });
                set(&mut pass, &self.mrt_background);
                self.background.draw(&mut pass);
                for &(_, i) in &opaque {
                    set(&mut pass, &self.opaque[i]);
                    mesh_of(i).draw(&mut pass);
                }
            }
            {
                let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("deferred resolve"),
                    color_attachments: &[color(&t.resolved, transparent)],
                    depth_stencil_attachment: depth_attachment(
                        &t.resolved_depth,
                        wgpu::LoadOp::Clear(1.),
                    ),
                    ..Default::default()
                });
                set(&mut pass, &self.background_draw);
                self.background.draw(&mut pass);
                set(&mut pass, &t.resolve);
                pass.set_vertex_buffer(0, self.quad[0].slice(..));
                pass.set_vertex_buffer(1, self.quad[1].slice(..));
                pass.draw(0..3, 0..1);
            }
            {
                let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("deferred transparent"),
                    color_attachments: &[color(&t.transparent, transparent)],
                    depth_stencil_attachment: depth_attachment(&t.opaque_depth, wgpu::LoadOp::Load),
                    ..Default::default()
                });
                draw_planes(&mut pass);
            }
        } else {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("deferred forward"),
                color_attachments: &[color(&t.forward, transparent)],
                depth_stencil_attachment: depth_attachment(
                    &t.forward_depth,
                    wgpu::LoadOp::Clear(1.),
                ),
                ..Default::default()
            });
            set(&mut pass, &self.background_draw);
            self.background.draw(&mut pass);
            for &(_, i) in &opaque {
                set(&mut pass, &self.forward_opaque[i]);
                mesh_of(i).draw(&mut pass);
            }
            draw_planes(&mut pass);
        }
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("deferred output"),
                color_attachments: &[color(&t.screen.view, wgpu::Color::BLACK)],
                ..Default::default()
            });
            set(
                &mut pass,
                if self.deferred {
                    &t.output
                } else {
                    &t.forward_output
                },
            );
            pass.set_vertex_buffer(0, self.output_uv.slice(..));
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
    /// mode ( 0 forward, 1 deferred ) and animated.
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        match index {
            0 => self.deferred = value > 0.5,
            1 => self.animated = value > 0.5,
            _ => return Err(Error::Invalid("deferred parameter")),
        }
        Ok(())
    }
    pub fn seek(&mut self, t: f64) {
        self.pending = Some(t);
    }
}
/// royal_esplanade_2k.hdr.jpg as UltraHDRLoader returns it ( half float,
/// flipped, mipmapped ) with its height, and scene.environment's PMREM
/// ( fromEquirectangular at lodMax 9 ) with the source pipeline.
pub(super) async fn esplanade(
    r: &Renderer,
    linear: &wgpu::Sampler,
    clamp: &wgpu::Sampler,
) -> Result<(wgpu::TextureView, u32, Pmrem, wgpu::RenderPipeline)> {
    let sky = ultra_hdr(&fetch("/web/environments/royal_esplanade_2k.hdr.jpg").await?).await?;
    let row = sky.width as usize * 4;
    let texels: Vec<half::f16> = sky.rgba.chunks(row).rev().flatten().copied().collect();
    let mut mipmaps = Mipmaps::new(r);
    let equirect = mipmapped_raw(
        r,
        &mut mipmaps,
        bytemuck::cast_slice(&texels),
        (sky.width, sky.height),
        8,
        HALF,
    );
    let pmrem = Pmrem::new(r, clamp, 9, (wgsl!("pmrem_ggx_vs"), wgsl!("pmrem_ggx_fs")))?;
    let source = source_pipeline(
        r,
        "PMREM equirect",
        wgsl!("pmrem_equirect_vs"),
        wgsl!("pmrem_equirect_fs"),
    );
    let source_object = r
        .device
        .create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("PMREM equirect"),
            contents: &pack(
                wgsl!("pmrem_equirect_vs"),
                "objectStruct",
                &[("nodeUniform3", &m4(Matrix4::IDENTITY))],
            )?,
            usage: wgpu::BufferUsages::UNIFORM,
        });
    let source_groups = [
        bind(
            r,
            source.get_bind_group_layout(0),
            &[(
                0,
                flat_camera(r, wgsl!("pmrem_equirect_vs"))?.as_entire_binding(),
            )],
        ),
        bind(
            r,
            source.get_bind_group_layout(1),
            &[
                (0, wgpu::BindingResource::Sampler(linear)),
                (1, wgpu::BindingResource::TextureView(&equirect)),
                (2, source_object.as_entire_binding()),
            ],
        ),
    ];
    let mut encoder = r.device.create_command_encoder(&Default::default());
    pmrem.encode(
        &mut encoder,
        &source,
        [&source_groups[0], &source_groups[1]],
    );
    r.queue.submit([encoder.finish()]);
    Ok((equirect, sky.height, pmrem, source))
}
/// CubeMapNode: the equirect texture rendered into a size² cube by a
/// CubeCamera ( near 1, far 10 ) inside a 5 × 5 × 5 back-faced box.
fn render_cube(
    r: &Renderer,
    sampler: &wgpu::Sampler,
    equirect: &wgpu::TextureView,
    size: u32,
) -> Result<wgpu::TextureView> {
    let cube = r.device.create_texture(&wgpu::TextureDescriptor {
        label: Some("deferred cube"),
        size: wgpu::Extent3d {
            width: size,
            height: size,
            depth_or_array_layers: 6,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: HALF,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
        view_formats: &[],
    });
    let depth = target(r, (size, size), DEPTH, 1);
    let geometry = BoxGeometry::build(5., 5., 5.)?;
    let a = geometry
        .attributes
        .get("position")
        .ok_or(Error::Invalid("box position"))?;
    let positions: Vec<f32> = (0..a.count())
        .flat_map(|i| (0..3).map(move |k| (i, k)))
        .map(|(i, k)| a.get_component(i, k).map(|v| v as f32))
        .collect::<Result<_>>()?;
    let index = geometry.index.clone().ok_or(Error::Invalid("box index"))?;
    let init = |data: &[u8], usage| {
        r.device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("deferred cube"),
                contents: data,
                usage,
            })
    };
    let vertices = init(bytemuck::cast_slice(&positions), wgpu::BufferUsages::VERTEX);
    let indices = init(bytemuck::cast_slice(&index), wgpu::BufferUsages::INDEX);
    let attribute = [wgpu::VertexAttribute {
        format: wgpu::VertexFormat::Float32x3,
        offset: 0,
        shader_location: 0,
    }];
    let pipeline = raw_pipeline(
        r,
        "deferred cube",
        wgsl!("cube_vs"),
        wgsl!("cube_fs"),
        &[wgpu::VertexBufferLayout {
            array_stride: 12,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &attribute,
        }],
        HALF,
        1,
        Some((wgpu::CompareFunction::LessEqual, true)),
        true,
    );
    let object = init(
        &pack(
            wgsl!("cube_vs"),
            "objectStruct",
            &[
                ("nodeUniform1", &m4(Matrix4::IDENTITY)),
                ("nodeUniform2", &[1.]),
            ],
        )?,
        wgpu::BufferUsages::UNIFORM,
    );
    let object_group = bind(
        r,
        pipeline.get_bind_group_layout(1),
        &[
            (0, wgpu::BindingResource::Sampler(sampler)),
            (1, wgpu::BindingResource::TextureView(equirect)),
            (2, object.as_entire_binding()),
        ],
    );
    let projection = [
        -1.,
        0.,
        0.,
        0.,
        0.,
        -1.,
        0.,
        0.,
        0.,
        0.,
        -10. / 9.,
        -1.,
        0.,
        0.,
        -10. / 9.,
        0.,
    ];
    let views: [[f64; 16]; 6] = [
        [
            0., 0., 1., 0., 0., -1., 0., 0., 1., 0., 0., 0., 0., 0., 0., 1.,
        ],
        [
            0., 0., -1., 0., 0., -1., 0., 0., -1., 0., 0., 0., 0., 0., 0., 1.,
        ],
        [
            1., 0., 0., 0., 0., 0., -1., 0., 0., 1., 0., 0., 0., 0., 0., 1.,
        ],
        [
            1., 0., 0., 0., 0., 0., 1., 0., 0., -1., 0., 0., 0., 0., 0., 1.,
        ],
        [
            1., 0., 0., 0., 0., -1., 0., 0., 0., 0., -1., 0., 0., 0., 0., 1.,
        ],
        [
            -1., 0., 0., 0., 0., -1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1.,
        ],
    ];
    let mut encoder = r.device.create_command_encoder(&Default::default());
    let mut renders = vec![];
    for (layer, view) in views.iter().enumerate() {
        let render = init(
            &pack(
                wgsl!("cube_vs"),
                "renderStruct",
                &[
                    ("cameraProjectionMatrix", &projection),
                    ("cameraViewMatrix", view),
                ],
            )?,
            wgpu::BufferUsages::UNIFORM,
        );
        let render_group = bind(
            r,
            pipeline.get_bind_group_layout(0),
            &[(0, render.as_entire_binding())],
        );
        let face = cube.create_view(&wgpu::TextureViewDescriptor {
            dimension: Some(wgpu::TextureViewDimension::D2),
            base_array_layer: layer as u32,
            array_layer_count: Some(1),
            ..Default::default()
        });
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("deferred cube"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &face,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &depth,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.),
                        store: wgpu::StoreOp::Discard,
                    }),
                    stencil_ops: None,
                }),
                ..Default::default()
            });
            pass.set_pipeline(&pipeline);
            pass.set_bind_group(0, &render_group, &[]);
            pass.set_bind_group(1, &object_group, &[]);
            pass.set_vertex_buffer(0, vertices.slice(..));
            pass.set_index_buffer(indices.slice(..), wgpu::IndexFormat::Uint32);
            pass.draw_indexed(0..index.len() as u32, 0, 0..1);
        }
        renders.push(render);
    }
    r.queue.submit([encoder.finish()]);
    Ok(cube.create_view(&wgpu::TextureViewDescriptor {
        dimension: Some(wgpu::TextureViewDimension::Cube),
        ..Default::default()
    }))
}
