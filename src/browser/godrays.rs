//! webgpu_postprocessing_godrays: the godrays_demo pillars and base inside
//! black backdrop walls, lit by a pink point light (2048² cube shadow) with a
//! small white sphere at its position. After the scene pass, GodraysNode
//! ray-marches from the camera through the light's shadow box at half
//! resolution, a two-pass bilateral blur smooths it, and depthAwareBlend
//! mixes the blend color into the scene along depth-consistent offsets. Every
//! stage runs the WGSL three.js r186 generates for the page (in `godrays/`).
use super::controls_attributes::{Controls, camera_state};
use super::gltf_viewer::load_asset;
use super::lights_projector::{m3, m4, pack};
use crate::{
    Error, Result, camera::*, geometry::*, math::*, render_target::*, renderer::*, scene::*,
};
use std::f64::consts::PI;
use wgpu::util::DeviceExt;

const HALF: wgpu::TextureFormat = wgpu::TextureFormat::Rgba16Float;
const DEPTH: wgpu::TextureFormat = wgpu::TextureFormat::Depth24Plus;
const CUBE: u32 = 2048;
const LIGHT: [f64; 3] = [0., 50., 0.];
const SHADOW_NEAR: f64 = 0.5;
const SHADOW_FAR: f64 = 500.;
macro_rules! wgsl {
    ($name:literal) => {
        include_str!(concat!("godrays/", $name, ".wgsl"))
    };
}
/// PointShadowNode's WebGPU cube faces: directions and ups.
const FACES: [([f64; 3], [f64; 3]); 6] = [
    ([1., 0., 0.], [0., -1., 0.]),
    ([-1., 0., 0.], [0., -1., 0.]),
    ([0., -1., 0.], [0., 0., -1.]),
    ([0., 1., 0.], [0., 0., 1.]),
    ([0., 0., 1.], [0., -1., 0.]),
    ([0., 0., -1.], [0., -1., 0.]),
];
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
fn texture(
    r: &Renderer,
    size: (u32, u32, u32),
    format: wgpu::TextureFormat,
    sampled: bool,
) -> wgpu::Texture {
    r.device.create_texture(&wgpu::TextureDescriptor {
        label: Some("godrays target"),
        size: wgpu::Extent3d {
            width: size.0,
            height: size.1,
            depth_or_array_layers: size.2,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT
            | if sampled {
                wgpu::TextureUsages::TEXTURE_BINDING
            } else {
                wgpu::TextureUsages::empty()
            },
        view_formats: &[],
    })
}
fn view(t: &wgpu::Texture) -> wgpu::TextureView {
    t.create_view(&Default::default())
}
/// One mesh: buffers, index, local bounding sphere, model and material.
struct Mesh {
    positions: wgpu::Buffer,
    normals: wgpu::Buffer,
    index: wgpu::Buffer,
    count: u32,
    center: Vector3,
    radius: f64,
    model: Matrix4,
    kind: Kind,
    object: wgpu::Buffer,
    shadow_object: wgpu::Buffer,
}
#[derive(Clone, Copy, PartialEq)]
enum Kind {
    /// MeshBasicMaterial black, double-sided.
    Backdrop,
    /// MeshStandardMaterial 0x333333: the pillars (front side) and the base
    /// (double-sided).
    Pillars,
    Base,
    /// The light's white sphere: no shadow.
    Sphere,
}
struct Targets {
    width: u32,
    height: u32,
    format: wgpu::TextureFormat,
    color: wgpu::TextureView,
    depth: wgpu::TextureView,
    /// Godrays, horizontal and vertical blur at half resolution.
    rays: [wgpu::TextureView; 3],
    half: (u32, u32),
    screen: RenderTarget,
    /// Per kind: pipeline, render group, object groups by mesh.
    scene: Vec<(
        Kind,
        wgpu::RenderPipeline,
        wgpu::BindGroup,
        Vec<Option<wgpu::BindGroup>>,
    )>,
    godrays: (wgpu::RenderPipeline, [wgpu::BindGroup; 2]),
    blur: (wgpu::RenderPipeline, [wgpu::BindGroup; 2]),
    /// Output over the blurred, or the raw, godrays.
    output: (wgpu::RenderPipeline, [wgpu::BindGroup; 2]),
}
pub(super) struct Demo {
    controls: Controls,
    /// raymarch steps, density, max density, distance attenuation, edge
    /// radius, edge strength, blur enabled.
    params: [f64; 7],
    meshes: Vec<Mesh>,
    shadow_pipelines: [wgpu::RenderPipeline; 2],
    faces: Vec<(
        wgpu::TextureView,
        wgpu::TextureView,
        wgpu::Buffer,
        [wgpu::BindGroup; 2],
    )>,
    shadow_objects: Vec<[wgpu::BindGroup; 2]>,
    cube: wgpu::TextureView,
    standard_render: wgpu::Buffer,
    basic_render: wgpu::Buffer,
    godrays_render: wgpu::Buffer,
    godrays_object: wgpu::Buffer,
    planes: [wgpu::Buffer; 2],
    blur_objects: [wgpu::Buffer; 2],
    output_object: wgpu::Buffer,
    quad_uv: wgpu::Buffer,
    linear: wgpu::Sampler,
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
            fov: 60.,
            near: 0.1,
            far: 1000.,
            aspect,
            ..Default::default()
        }));
        s.get_mut(c)?.position = Vector3::new(-175., 50., 0.);
        let mut controls = Controls::new(Some(0.05), (0., 200.), PI, true);
        controls.set_target(Vector3::new(0., 0.5, 0.));
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
        let make =
            |positions: &[f32], normals: &[f32], index: &[u32], model: Matrix4, kind: Kind| {
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
                    positions: init("godrays positions", bytemuck::cast_slice(positions), vertex),
                    normals: init("godrays normals", bytemuck::cast_slice(normals), vertex),
                    index: init(
                        "godrays index",
                        bytemuck::cast_slice(index),
                        wgpu::BufferUsages::INDEX,
                    ),
                    count: index.len() as u32,
                    center,
                    radius: points
                        .iter()
                        .map(|p| (*p - center).length())
                        .fold(0., f64::max),
                    model,
                    kind,
                    object: uniform(r, "godrays object", 176),
                    shadow_object: uniform(r, "godrays shadow object", 80),
                }
            };
        let geometry = |g: BufferGeometry| -> Result<(Vec<f32>, Vec<f32>, Vec<u32>)> {
            let f = |name: &str| -> Result<Vec<f32>> {
                let a = g
                    .attributes
                    .get(name)
                    .ok_or(Error::Invalid("godrays attribute"))?;
                (0..a.count())
                    .flat_map(|i| (0..3).map(move |k| (i, k)))
                    .map(|(i, k)| a.get_component(i, k).map(|v| v as f32))
                    .collect()
            };
            Ok((
                f("position")?,
                f("normal")?,
                g.index.clone().ok_or(Error::Invalid("godrays index"))?,
            ))
        };
        let mut meshes = vec![];
        // setupBackdrop(): five black walls 200 from the origin.
        let wall = geometry(PlaneGeometry::build(400., 200., 1, 1)?)?;
        let turn_y = Matrix4::from_rotation_y(PI / 2.);
        for model in [
            Matrix4::from_translation(Vector3::new(-200., 100., 0.)) * turn_y,
            Matrix4::from_translation(Vector3::new(200., 100., 0.)) * turn_y,
            Matrix4::from_translation(Vector3::new(0., 100., -200.)),
            Matrix4::from_translation(Vector3::new(0., 100., 200.)),
            Matrix4::from_translation(Vector3::new(0., 200., 0.))
                * Matrix4::from_rotation_x(PI / 2.)
                * Matrix4::from_scale(Vector3::new(3., 6., 1.)),
        ] {
            meshes.push(make(&wall.0, &wall.1, &wall.2, model, Kind::Backdrop));
        }
        let (asset, buffers, _) = load_asset("/web/gallery/assets/gltf/godrays_demo.glb").await?;
        for node in asset.nodes() {
            let Some(primitive) = node.mesh().and_then(|m| m.primitives().next()) else {
                continue;
            };
            let reader = primitive.reader(|b| buffers.get(b.index()).map(Vec::as_slice));
            let positions: Vec<f32> = reader
                .read_positions()
                .ok_or(Error::Invalid("godrays positions"))?
                .flatten()
                .collect();
            let normals: Vec<f32> = reader
                .read_normals()
                .ok_or(Error::Invalid("godrays normals"))?
                .flatten()
                .collect();
            let index: Vec<u32> = reader
                .read_indices()
                .ok_or(Error::Invalid("godrays index"))?
                .into_u32()
                .collect();
            let (t, q, sc) = node.transform().decomposed();
            let model = Matrix4::from_scale_rotation_translation(
                Vector3::new(sc[0] as f64, sc[1] as f64, sc[2] as f64),
                Quaternion::from_xyzw(q[0] as f64, q[1] as f64, q[2] as f64, q[3] as f64),
                Vector3::new(t[0] as f64, t[1] as f64, t[2] as f64),
            );
            let kind = if node.name() == Some("base") {
                Kind::Base
            } else {
                Kind::Pillars
            };
            meshes.push(make(&positions, &normals, &index, model, kind));
        }
        let sphere = geometry(SphereGeometry::build(0.5, 16, 16)?)?;
        meshes.push(make(
            &sphere.0,
            &sphere.1,
            &sphere.2,
            Matrix4::from_translation(Vector3::from_array(LIGHT)),
            Kind::Sphere,
        ));
        let module = |label, source: &str| {
            r.device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some(label),
                source: wgpu::ShaderSource::Wgsl(source.into()),
            })
        };
        let position = wgpu::vertex_attr_array![0 => Float32x3];
        // Front-side casters draw back faces (clockwise); double-sided ones
        // draw both.
        let shadow_pipelines = [false, true].map(|single| {
            r.device
                .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    label: Some("godrays shadow"),
                    layout: None,
                    vertex: wgpu::VertexState {
                        module: &module("shadow", wgsl!("shadow_vs")),
                        entry_point: Some("main"),
                        compilation_options: Default::default(),
                        buffers: &[wgpu::VertexBufferLayout {
                            array_stride: 12,
                            step_mode: wgpu::VertexStepMode::Vertex,
                            attributes: &position,
                        }],
                    },
                    fragment: Some(wgpu::FragmentState {
                        module: &module("shadow", wgsl!("shadow_fs")),
                        entry_point: Some("main"),
                        compilation_options: Default::default(),
                        targets: &[Some(wgpu::TextureFormat::Rgba8Unorm.into())],
                    }),
                    primitive: wgpu::PrimitiveState {
                        front_face: if single {
                            wgpu::FrontFace::Cw
                        } else {
                            wgpu::FrontFace::Ccw
                        },
                        cull_mode: single.then_some(wgpu::Face::Back),
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
        });
        let color = texture(r, (CUBE, CUBE, 6), wgpu::TextureFormat::Rgba8Unorm, false);
        let depth = texture(r, (CUBE, CUBE, 6), DEPTH, true);
        let layer = |t: &wgpu::Texture, i: u32| {
            t.create_view(&wgpu::TextureViewDescriptor {
                dimension: Some(wgpu::TextureViewDimension::D2),
                base_array_layer: i,
                array_layer_count: Some(1),
                ..Default::default()
            })
        };
        let faces = (0..6)
            .map(|i| {
                let buffer = uniform(r, "cube face camera", 128);
                let groups = shadow_pipelines.each_ref().map(|p| {
                    bind(
                        r,
                        p.get_bind_group_layout(0),
                        &[(0, buffer.as_entire_binding())],
                    )
                });
                (layer(&color, i), layer(&depth, i), buffer, groups)
            })
            .collect();
        let shadow_objects = meshes
            .iter()
            .map(|m| {
                shadow_pipelines.each_ref().map(|p| {
                    bind(
                        r,
                        p.get_bind_group_layout(1),
                        &[(0, m.shadow_object.as_entire_binding())],
                    )
                })
            })
            .collect();
        let cube = depth.create_view(&wgpu::TextureViewDescriptor {
            dimension: Some(wgpu::TextureViewDimension::Cube),
            ..Default::default()
        });
        Ok(Self {
            controls,
            params: [60., 0.7, 0.5, 2., 2., 2., 1.],
            meshes,
            shadow_pipelines,
            faces,
            shadow_objects,
            cube,
            standard_render: sized(r, "standard render", wgsl!("concrete_fs"), "renderStruct")?,
            basic_render: sized(r, "basic render", wgsl!("backdrop_fs"), "renderStruct")?,
            godrays_render: uniform(r, "godrays render", 16),
            godrays_object: sized(r, "godrays object", wgsl!("godrays_fs"), "objectStruct")?,
            planes: [0; 2].map(|_| uniform(r, "godrays planes", 96)),
            blur_objects: [0; 2].map(|_| uniform(r, "bilateral blur", 16)),
            output_object: sized(r, "blend object", wgsl!("output_fs"), "objectStruct")?,
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
            compare: r.device.create_sampler(&wgpu::SamplerDescriptor {
                mag_filter: wgpu::FilterMode::Linear,
                min_filter: wgpu::FilterMode::Linear,
                compare: Some(wgpu::CompareFunction::LessEqual),
                ..Default::default()
            }),
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
        let (width, height) = (out.width, out.height);
        let half = (
            ((width as f64 * 0.5).round() as u32).max(1),
            ((height as f64 * 0.5).round() as u32).max(1),
        );
        let color = view(&texture(r, (width, height, 1), HALF, true));
        let depth = view(&texture(r, (width, height, 1), DEPTH, true));
        let rays = [0; 3].map(|_| {
            view(&texture(
                r,
                (half.0, half.1, 1),
                wgpu::TextureFormat::Rgba8Unorm,
                true,
            ))
        });
        let module = |label, source: &str| {
            r.device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some(label),
                source: wgpu::ShaderSource::Wgsl(source.into()),
            })
        };
        let attributes = [
            wgpu::vertex_attr_array![0 => Float32x3],
            wgpu::vertex_attr_array![1 => Float32x3],
            wgpu::vertex_attr_array![0 => Float32x2],
        ];
        let layout = |i: usize, stride| wgpu::VertexBufferLayout {
            array_stride: stride,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &attributes[i],
        };
        let pipeline = |label,
                        vs: &str,
                        fs: &str,
                        buffers: &[wgpu::VertexBufferLayout],
                        format,
                        depth: bool,
                        cull: Option<wgpu::Face>| {
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
                        cull_mode: cull,
                        ..Default::default()
                    },
                    depth_stencil: depth.then(|| wgpu::DepthStencilState {
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
        let back = Some(wgpu::Face::Back);
        let tex = wgpu::BindingResource::TextureView;
        let sampler = wgpu::BindingResource::Sampler;
        let mut scene = vec![];
        for (kind, vs, fs, normals_first, cull) in [
            (
                Kind::Backdrop,
                wgsl!("backdrop_vs"),
                wgsl!("backdrop_fs"),
                false,
                None,
            ),
            (
                Kind::Pillars,
                wgsl!("standard_vs"),
                wgsl!("concrete_fs"),
                true,
                back,
            ),
            (
                Kind::Base,
                wgsl!("standard_vs"),
                wgsl!("base_fs"),
                true,
                None,
            ),
            (
                Kind::Sphere,
                wgsl!("sphere_vs"),
                wgsl!("backdrop_fs"),
                false,
                back,
            ),
        ] {
            // The basic materials read positions; the standard ones normals
            // then positions.
            let buffers = if normals_first {
                vec![layout(0, 12), layout(1, 12)]
            } else {
                vec![layout(0, 12)]
            };
            let fs = if kind == Kind::Sphere {
                wgsl!("sphere_fs")
            } else {
                fs
            };
            let p = pipeline("godrays scene", vs, fs, &buffers, HALF, true, cull);
            let basic = matches!(kind, Kind::Backdrop | Kind::Sphere);
            let render = bind(
                r,
                p.get_bind_group_layout(0),
                &[(
                    0,
                    if basic {
                        self.basic_render.as_entire_binding()
                    } else {
                        self.standard_render.as_entire_binding()
                    },
                )],
            );
            let objects = self
                .meshes
                .iter()
                .map(|m| {
                    (m.kind == kind).then(|| {
                        if basic {
                            bind(
                                r,
                                p.get_bind_group_layout(1),
                                &[(0, m.object.as_entire_binding())],
                            )
                        } else {
                            bind(
                                r,
                                p.get_bind_group_layout(1),
                                &[
                                    (0, m.object.as_entire_binding()),
                                    (1, sampler(&self.linear)),
                                    (2, tex(&r.dfg)),
                                    (3, sampler(&self.compare)),
                                    (4, tex(&self.cube)),
                                ],
                            )
                        }
                    })
                })
                .collect();
            scene.push((kind, p, render, objects));
        }
        let quad = [layout(2, 8)];
        let godrays = pipeline(
            "godrays",
            wgsl!("godrays_vs"),
            wgsl!("godrays_fs"),
            &quad,
            wgpu::TextureFormat::Rgba8Unorm,
            false,
            back,
        );
        let godrays_binds = [
            bind(
                r,
                godrays.get_bind_group_layout(0),
                &[(0, self.godrays_render.as_entire_binding())],
            ),
            bind(
                r,
                godrays.get_bind_group_layout(1),
                &[
                    (0, tex(&depth)),
                    (1, self.godrays_object.as_entire_binding()),
                    (2, self.planes[0].as_entire_binding()),
                    (3, self.planes[1].as_entire_binding()),
                    (4, sampler(&self.compare)),
                    (5, tex(&self.cube)),
                ],
            ),
        ];
        let blur = pipeline(
            "bilateral blur",
            wgsl!("blur_vs"),
            wgsl!("blur_fs"),
            &quad,
            wgpu::TextureFormat::Rgba8Unorm,
            false,
            back,
        );
        let blur_binds = [0, 1].map(|i| {
            bind(
                r,
                blur.get_bind_group_layout(0),
                &[
                    (0, sampler(&self.linear)),
                    (1, tex(&rays[i])),
                    (2, self.blur_objects[i].as_entire_binding()),
                ],
            )
        });
        let output = pipeline(
            "depth aware blend",
            wgsl!("output_vs"),
            wgsl!("output_fs"),
            &quad,
            out.options.format,
            false,
            back,
        );
        let output_binds = [&rays[2], &rays[0]].map(|rays| {
            bind(
                r,
                output.get_bind_group_layout(0),
                &[
                    (0, sampler(&self.linear)),
                    (1, tex(&color)),
                    (2, self.output_object.as_entire_binding()),
                    (3, tex(&depth)),
                    (4, sampler(&self.linear)),
                    (5, tex(rays)),
                ],
            )
        });
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
            format: out.options.format,
            color,
            depth,
            rays,
            half,
            screen,
            scene,
            godrays: (godrays, godrays_binds),
            blur: (blur, blur_binds),
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
            t.width != out.width || t.height != out.height || t.format != out.options.format
        }) {
            self.resize(r, out)?;
        }
        s.update()?;
        let (camera, world) = s.camera(c)?;
        let Camera::Perspective(p) = camera else {
            return Err(Error::Invalid("godrays camera"));
        };
        let (near, far) = (p.near, p.far);
        let (projection, view) = (camera.projection_matrix()?, world.inverse());
        let light = Vector3::from_array(LIGHT);
        let light_color = Color::from_hex(0xf6287d)
            .0
            .to_array()
            .map(|v| v * 10000.)
            .to_vec();
        let ambient = Color::from_hex(0xcccccc)
            .0
            .to_array()
            .map(|v| v * 0.4)
            .to_vec();
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
        write(
            &self.standard_render,
            wgsl!("concrete_fs"),
            "renderStruct",
            &[
                ("cameraProjectionMatrix", m4(projection)),
                ("cameraViewMatrix", m4(view)),
                ("nodeUniform10", ambient.clone()),
                (
                    "nodeUniform11",
                    view.transform_point3(light).to_array().to_vec(),
                ),
                ("nodeUniform12", light_color),
                ("nodeUniform13", m4(Matrix4::from_translation(-light))),
                ("nodeUniform15", vec![0.]),
                ("nodeUniform16", vec![SHADOW_FAR]),
                ("nodeUniform17", vec![SHADOW_NEAR]),
                ("nodeUniform18", vec![-0.00001]),
                ("nodeUniform20", vec![1.]),
                ("nodeUniform21", vec![CUBE as f64; 2]),
                ("nodeUniform22", vec![1.]),
                ("nodeUniform23", vec![0.]),
                ("nodeUniform24", vec![2.]),
            ],
        )?;
        write(
            &self.basic_render,
            wgsl!("backdrop_fs"),
            "renderStruct",
            &[
                ("cameraProjectionMatrix", m4(projection)),
                ("cameraViewMatrix", m4(view)),
                ("nodeUniform2", ambient),
            ],
        )?;
        for m in &self.meshes {
            match m.kind {
                Kind::Backdrop | Kind::Sphere => {
                    let color = if m.kind == Kind::Sphere { 1. } else { 0. };
                    write(
                        &m.object,
                        wgsl!("backdrop_fs"),
                        "objectStruct",
                        &[
                            ("nodeUniform0", vec![color; 3]),
                            ("nodeUniform1", vec![1.]),
                            ("nodeUniform5", m4(m.model)),
                        ],
                    )?;
                }
                _ => write(
                    &m.object,
                    wgsl!("concrete_fs"),
                    "objectStruct",
                    &[
                        (
                            "nodeUniform0",
                            Color::from_hex(0x333333).0.to_array().to_vec(),
                        ),
                        ("nodeUniform1", vec![1.]),
                        ("nodeUniform2", vec![0.]),
                        ("nodeUniform3", vec![1.]),
                        ("nodeUniform5", m3(m.model.inverse().transpose())),
                        ("nodeUniform7", vec![1.]),
                        ("nodeUniform9", m4(m.model)),
                    ],
                )?,
            }
            let mut object = vec![1f32, 0., 0., 0.];
            object.extend(m.model.to_cols_array().map(|v| v as f32));
            r.queue
                .write_buffer(&m.shadow_object, 0, bytemuck::cast_slice(&object));
        }
        let [
            steps,
            density,
            max_density,
            attenuation,
            edge_radius,
            edge_strength,
            blurred,
        ] = self.params;
        let camera_position = world.transform_point3(Vector3::ZERO);
        write(
            &self.godrays_object,
            wgsl!("godrays_fs"),
            "objectStruct",
            &[
                ("nodeUniform1", m4(projection.inverse())),
                ("nodeUniform2", m4(world)),
                ("nodeUniform3", camera_position.to_array().to_vec()),
                ("nodeUniform6", vec![steps.round()]),
                ("nodeUniform9", vec![SHADOW_NEAR]),
                ("nodeUniform10", vec![SHADOW_FAR]),
                ("nodeUniform11", vec![density]),
                ("nodeUniform12", vec![attenuation]),
                ("nodeUniform13", vec![max_density]),
            ],
        )?;
        r.queue.write_buffer(
            &self.godrays_render,
            0,
            bytemuck::cast_slice(&[LIGHT[0] as f32, LIGHT[1] as f32, LIGHT[2] as f32, 0.]),
        );
        // _updateLightParams: the planes of the light's shadow box.
        let directions: [[f64; 3]; 6] = [
            [1., 0., 0.],
            [-1., 0., 0.],
            [0., 1., 0.],
            [0., -1., 0.],
            [0., 0., 1.],
            [0., 0., -1.],
        ];
        let mut normals = vec![];
        let mut constants = vec![];
        for d in directions {
            let n = Vector3::from_array(d);
            let point = light + n * SHADOW_FAR;
            normals.extend([n.x as f32, n.y as f32, n.z as f32, 0.]);
            constants.extend([(-point.dot(n)) as f32, 0., 0., 0.]);
        }
        r.queue
            .write_buffer(&self.planes[0], 0, bytemuck::cast_slice(&normals));
        r.queue
            .write_buffer(&self.planes[1], 0, bytemuck::cast_slice(&constants));
        let t = self
            .targets
            .as_ref()
            .ok_or(Error::Invalid("godrays targets"))?;
        let texel = [1. / t.half.0 as f32, 1. / t.half.1 as f32];
        for (object, direction) in self.blur_objects.iter().zip([[1f32, 0.], [0., 1.]]) {
            r.queue.write_buffer(
                object,
                0,
                bytemuck::cast_slice(&[direction[0], direction[1], texel[0], texel[1]]),
            );
        }
        write(
            &self.output_object,
            wgsl!("output_fs"),
            "objectStruct",
            &[
                ("nodeUniform1", vec![near]),
                ("nodeUniform2", vec![far]),
                ("nodeUniform4", vec![edge_radius.round()]),
                (
                    "nodeUniform5",
                    Color::from_hex(0xf6287d).0.to_array().to_vec(),
                ),
                ("nodeUniform7", vec![edge_strength]),
            ],
        )?;
        let visible = |m: &Mesh, frustum: &Frustum| {
            let scale = m
                .model
                .x_axis
                .truncate()
                .length()
                .max(m.model.y_axis.truncate().length())
                .max(m.model.z_axis.truncate().length());
            frustum.intersects_sphere(Sphere {
                center: m.model.transform_point3(m.center),
                radius: m.radius * scale,
            })
        };
        let draw = |pass: &mut wgpu::RenderPass, m: &Mesh, slots: &[&wgpu::Buffer]| {
            for (slot, buffer) in slots.iter().enumerate() {
                pass.set_vertex_buffer(slot as u32, buffer.slice(..));
            }
            pass.set_index_buffer(m.index.slice(..), wgpu::IndexFormat::Uint32);
            pass.draw_indexed(0..m.count, 0, 0..1);
        };
        let mut encoder = r.device.create_command_encoder(&Default::default());
        // The cube shadow: each face culls the casters against its camera.
        let face_projection = Matrix4::perspective_rh(PI / 2., 1., SHADOW_NEAR, SHADOW_FAR);
        for (i, (color, depth, buffer, groups)) in self.faces.iter().enumerate() {
            let (direction, up) = FACES[i];
            let face_view = Matrix4::look_at_rh(
                light,
                light + Vector3::from_array(direction),
                Vector3::from_array(up),
            );
            let mut data: Vec<f32> = face_projection.to_cols_array().map(|v| v as f32).to_vec();
            data.extend(face_view.to_cols_array().map(|v| v as f32));
            r.queue.write_buffer(buffer, 0, bytemuck::cast_slice(&data));
            let frustum = Frustum::from_projection(face_projection * face_view);
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("godrays shadow"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: color,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Discard,
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
            for (k, m) in self.meshes.iter().enumerate() {
                if m.kind == Kind::Sphere || !visible(m, &frustum) {
                    continue;
                }
                let single = usize::from(m.kind == Kind::Pillars);
                pass.set_pipeline(&self.shadow_pipelines[single]);
                pass.set_bind_group(0, &groups[single], &[]);
                pass.set_bind_group(1, &self.shadow_objects[k][single], &[]);
                draw(&mut pass, m, &[&m.positions]);
            }
        }
        let frustum = Frustum::from_projection(projection * view);
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("godrays scene"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &t.color,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &t.depth,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                ..Default::default()
            });
            for (k, m) in self.meshes.iter().enumerate() {
                if !visible(m, &frustum) {
                    continue;
                }
                let Some((_, pipeline, render, objects)) =
                    t.scene.iter().find(|(kind, ..)| *kind == m.kind)
                else {
                    continue;
                };
                let Some(object) = &objects[k] else {
                    continue;
                };
                pass.set_pipeline(pipeline);
                pass.set_bind_group(0, render, &[]);
                pass.set_bind_group(1, object, &[]);
                match m.kind {
                    Kind::Sphere | Kind::Backdrop => draw(&mut pass, m, &[&m.positions]),
                    _ => draw(&mut pass, m, &[&m.normals, &m.positions]),
                }
            }
        }
        let quad = |encoder: &mut wgpu::CommandEncoder,
                    target: &wgpu::TextureView,
                    clear: wgpu::Color,
                    pipeline: &wgpu::RenderPipeline,
                    groups: &[&wgpu::BindGroup]| {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("godrays post"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: target,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(clear),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                ..Default::default()
            });
            pass.set_pipeline(pipeline);
            for (i, group) in groups.iter().enumerate() {
                pass.set_bind_group(i as u32, *group, &[]);
            }
            pass.set_vertex_buffer(0, self.quad_uv.slice(..));
            pass.draw(0..3, 0..1);
        };
        quad(
            &mut encoder,
            &t.rays[0],
            wgpu::Color::WHITE,
            &t.godrays.0,
            &[&t.godrays.1[0], &t.godrays.1[1]],
        );
        let use_blur = blurred > 0.5;
        if use_blur {
            quad(
                &mut encoder,
                &t.rays[1],
                wgpu::Color::BLACK,
                &t.blur.0,
                &[&t.blur.1[0]],
            );
            quad(
                &mut encoder,
                &t.rays[2],
                wgpu::Color::BLACK,
                &t.blur.0,
                &[&t.blur.1[1]],
            );
        }
        quad(
            &mut encoder,
            &t.screen.view,
            wgpu::Color::TRANSPARENT,
            &t.output.0,
            &[&t.output.1[if use_blur { 0 } else { 1 }]],
        );
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
    /// raymarch steps, density, max density, distance attenuation, edge
    /// radius, edge strength and blur.
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        *self
            .params
            .get_mut(index)
            .ok_or(Error::Invalid("godrays parameter"))? = value as f64;
        Ok(())
    }
    pub fn seek(&mut self, _t: f64) {}
}
