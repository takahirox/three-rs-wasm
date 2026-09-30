//! webgpu_shadowmap_vsm: a spinning torus knot and four pillars on a ground
//! plane, all Phong, lit by an ambient light, a shadow-casting spot light and
//! a directional light circling on a group, with VSM shadows and linear fog.
//! Each frame renders both depth maps, the VSM vertical pass (mean and
//! deviation of the depths along a column) and horizontal pass, then the
//! multisampled scene pass (front to back, frustum culled) and the sRGB output
//! pass. Every stage runs the WGSL three.js r186 generates for the page (in
//! `shadowmap_vsm/`).
use super::controls_attributes::{Controls, camera_state};
use crate::{
    Error, Result, camera::*, geometry::*, math::*, render_target::*, renderer::*, scene::*,
};
use std::f64::consts::PI;
use wgpu::util::DeviceExt;

const HALF: wgpu::TextureFormat = wgpu::TextureFormat::Rgba16Float;
const DEPTH: wgpu::TextureFormat = wgpu::TextureFormat::Depth24Plus;
const VSM: wgpu::TextureFormat = wgpu::TextureFormat::Rg16Float;
macro_rules! wgsl {
    ($name:literal) => {
        include_str!(concat!("shadowmap_vsm/", $name, ".wgsl"))
    };
}
/// A std140-style writer for the generated uniform structs.
struct Layout(Vec<u8>);
impl Layout {
    fn new(size: usize) -> Self {
        Self(vec![0; size])
    }
    fn f(mut self, offset: usize, v: f64) -> Self {
        self.0[offset..offset + 4].copy_from_slice(&(v as f32).to_le_bytes());
        self
    }
    fn v3(self, offset: usize, v: [f64; 3]) -> Self {
        self.f(offset, v[0]).f(offset + 4, v[1]).f(offset + 8, v[2])
    }
    fn p3(self, offset: usize, v: Vector3) -> Self {
        self.v3(offset, v.to_array())
    }
    fn m4(mut self, offset: usize, m: Matrix4) -> Self {
        for (i, v) in m.to_cols_array().iter().enumerate() {
            self = self.f(offset + i * 4, *v);
        }
        self
    }
    /// mat3x3<f32>: three columns padded to 16 bytes.
    fn m3(mut self, offset: usize, m: Matrix4) -> Self {
        for col in 0..3 {
            let c = m.col(col);
            self = self.v3(offset + col * 16, [c.x, c.y, c.z]);
        }
        self
    }
    fn write(self, r: &Renderer, buffer: &wgpu::Buffer) {
        r.queue.write_buffer(buffer, 0, &self.0);
    }
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
fn texture(
    r: &Renderer,
    size: (u32, u32),
    format: wgpu::TextureFormat,
    samples: u32,
    sampled: bool,
) -> wgpu::TextureView {
    r.device
        .create_texture(&wgpu::TextureDescriptor {
            label: Some("vsm target"),
            size: wgpu::Extent3d {
                width: size.0,
                height: size.1,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: samples,
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
        .create_view(&Default::default())
}
/// Perspective and orthographic projections in WebGPU clip space (z in 0..1).
fn perspective(fov: f64, aspect: f64, near: f64, far: f64) -> Matrix4 {
    let top = near * (fov.to_radians() / 2.).tan();
    let (x, y) = (near / (top * aspect), near / top);
    Matrix4::from_cols_array(&[
        x,
        0.,
        0.,
        0.,
        0.,
        y,
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
    ])
}
fn orthographic(half: f64, near: f64, far: f64) -> Matrix4 {
    Matrix4::from_cols_array(&[
        1. / half,
        0.,
        0.,
        0.,
        0.,
        1. / half,
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
    ])
}
/// One mesh: geometry buffers, index count, bounding radius and model matrix.
/// The geometries are centered on their origin.
struct Mesh {
    positions: wgpu::Buffer,
    normals: wgpu::Buffer,
    index: wgpu::Buffer,
    count: u32,
    radius: f64,
    model: Matrix4,
    /// Its object uniforms: the depth passes and the Phong pass.
    depth_object: wgpu::Buffer,
    phong_object: wgpu::Buffer,
    /// Phong color and specular.
    specular: u32,
}
impl Mesh {
    /// Frustum.intersectsObject: the bounding sphere in world space.
    fn visible(&self, frustum: &Frustum) -> bool {
        let m = self.model;
        let scale = m
            .x_axis
            .truncate()
            .length()
            .max(m.y_axis.truncate().length())
            .max(m.z_axis.truncate().length());
        frustum.intersects_sphere(Sphere {
            center: m.transform_point3(Vector3::ZERO),
            radius: self.radius * scale,
        })
    }
}
/// A shadow: its map size, camera uniforms, depth target and VSM passes.
struct Shadow {
    size: u32,
    render: wgpu::Buffer,
    blur: wgpu::Buffer,
    color: wgpu::TextureView,
    depth: wgpu::TextureView,
    vertical: wgpu::TextureView,
    horizontal: wgpu::TextureView,
    binds: [wgpu::BindGroup; 5],
    depth_objects: Vec<wgpu::BindGroup>,
}
struct Targets {
    width: u32,
    height: u32,
    format: wgpu::TextureFormat,
    samples: u32,
    color: wgpu::TextureView,
    resolve: Option<wgpu::TextureView>,
    depth: wgpu::TextureView,
    screen: RenderTarget,
    phong: (wgpu::RenderPipeline, wgpu::BindGroup, Vec<wgpu::BindGroup>),
    output: (wgpu::RenderPipeline, [wgpu::BindGroup; 2]),
}
pub(super) struct Demo {
    controls: Controls,
    time: f64,
    last: f64,
    /// spot radius, spot samples, directional radius, directional samples, animate.
    params: [f64; 5],
    knot_rotation: Vector3,
    group_rotation: f64,
    dir_z: f64,
    meshes: Vec<Mesh>,
    shadows: [Shadow; 2],
    depth_pipeline: wgpu::RenderPipeline,
    vertical_pipeline: wgpu::RenderPipeline,
    horizontal_pipeline: wgpu::RenderPipeline,
    render_uniforms: wgpu::Buffer,
    output_render: wgpu::Buffer,
    output_object: wgpu::Buffer,
    quad: wgpu::Buffer,
    sampler: wgpu::Sampler,
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
            near: 1.,
            far: 1000.,
            aspect,
            ..Default::default()
        }));
        s.get_mut(c)?.position = Vector3::new(0., 10., 30.);
        let mut controls = Controls::new(None, (0., f64::INFINITY), PI, true);
        controls.set_target(Vector3::new(0., 2., 0.));
        controls.update(s, c)?;
        let init = |label, data: &[u8], usage| {
            r.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some(label),
                    contents: data,
                    usage,
                })
        };
        let mesh = |g: &BufferGeometry, model: Matrix4, specular: u32| -> Result<Mesh> {
            let f = |name: &str| -> Result<Vec<f32>> {
                let a = g
                    .attributes
                    .get(name)
                    .ok_or(Error::Invalid("mesh attribute"))?;
                (0..a.count())
                    .flat_map(|i| (0..3).map(move |k| (i, k)))
                    .map(|(i, k)| a.get_component(i, k).map(|v| v as f32))
                    .collect()
            };
            let positions = f("position")?;
            let radius = positions
                .chunks(3)
                .map(|p| (p[0] as f64).hypot(p[1] as f64).hypot(p[2] as f64))
                .fold(0., f64::max);
            let index = g.index.clone().ok_or(Error::Invalid("mesh index"))?;
            Ok(Mesh {
                positions: init(
                    "vsm positions",
                    bytemuck::cast_slice(&positions),
                    wgpu::BufferUsages::VERTEX,
                ),
                normals: init(
                    "vsm normals",
                    bytemuck::cast_slice(&f("normal")?),
                    wgpu::BufferUsages::VERTEX,
                ),
                index: init(
                    "vsm index",
                    bytemuck::cast_slice(&index),
                    wgpu::BufferUsages::INDEX,
                ),
                count: index.len() as u32,
                radius,
                model,
                depth_object: uniform(r, "depth object", 80),
                phong_object: uniform(r, "phong object", 272),
                specular,
            })
        };
        let knot = TorusKnotGeometry::build(25., 8., 75, 20, 2, 3)?;
        let pillar = CylinderGeometry::build(0.75, 0.75, 7., 32, 1, false, 0., PI * 2.)?;
        let plane = PlaneGeometry::build(200., 200., 1, 1)?;
        let mut meshes = vec![mesh(&knot, Matrix4::IDENTITY, 0x222222)?];
        for (x, z) in [(8., 8.), (8., -8.), (-8., 8.), (-8., -8.)] {
            meshes.push(mesh(
                &pillar,
                Matrix4::from_translation(Vector3::new(x, 3.5, z)),
                0x222222,
            )?);
        }
        meshes.push(mesh(
            &plane,
            Matrix4::from_rotation_x(-PI / 2.) * Matrix4::from_scale(Vector3::splat(3.)),
            0x111111,
        )?);
        let module = |label, source: &str| {
            r.device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some(label),
                source: wgpu::ShaderSource::Wgsl(source.into()),
            })
        };
        let position = wgpu::vertex_attr_array![0 => Float32x3];
        let pipeline = |label, vs: &str, fs: &str, format, depth: bool| {
            let buffers = [wgpu::VertexBufferLayout {
                array_stride: 12,
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes: &position,
            }];
            r.device
                .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    label: Some(label),
                    layout: None,
                    vertex: wgpu::VertexState {
                        module: &module(label, vs),
                        entry_point: Some("main"),
                        compilation_options: Default::default(),
                        buffers: if depth { &buffers } else { &[] },
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
        let depth_pipeline = pipeline(
            "vsm depth",
            wgsl!("depth_vs"),
            wgsl!("depth_fs"),
            wgpu::TextureFormat::Rgba8Unorm,
            true,
        );
        let vertical_pipeline = pipeline(
            "vsm vertical",
            wgsl!("blur_vs"),
            wgsl!("vertical_fs"),
            VSM,
            false,
        );
        let horizontal_pipeline = pipeline(
            "vsm horizontal",
            wgsl!("blur_vs"),
            wgsl!("horizontal_fs"),
            VSM,
            false,
        );
        let sampler = r.device.create_sampler(&wgpu::SamplerDescriptor {
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        let blur_object = uniform(r, "vsm uv", 48);
        Layout::new(48)
            .m3(0, Matrix4::IDENTITY)
            .write(r, &blur_object);
        let shadow = |size: u32| {
            let render = uniform(r, "shadow render", 128);
            let blur = uniform(r, "vsm blur", 16);
            let color = texture(r, (size, size), wgpu::TextureFormat::Rgba8Unorm, 1, false);
            let depth = texture(r, (size, size), DEPTH, 1, true);
            let vertical = texture(r, (size, size), VSM, 1, true);
            let horizontal = texture(r, (size, size), VSM, 1, true);
            let binds = [
                bind(
                    r,
                    depth_pipeline.get_bind_group_layout(0),
                    &[(0, render.as_entire_binding())],
                ),
                bind(
                    r,
                    vertical_pipeline.get_bind_group_layout(0),
                    &[(0, blur.as_entire_binding())],
                ),
                bind(
                    r,
                    vertical_pipeline.get_bind_group_layout(1),
                    &[
                        (0, wgpu::BindingResource::TextureView(&depth)),
                        (1, blur_object.as_entire_binding()),
                    ],
                ),
                bind(
                    r,
                    horizontal_pipeline.get_bind_group_layout(0),
                    &[(0, blur.as_entire_binding())],
                ),
                bind(
                    r,
                    horizontal_pipeline.get_bind_group_layout(1),
                    &[
                        (0, wgpu::BindingResource::Sampler(&sampler)),
                        (1, wgpu::BindingResource::TextureView(&vertical)),
                        (2, blur_object.as_entire_binding()),
                    ],
                ),
            ];
            let depth_objects = meshes
                .iter()
                .map(|m| {
                    bind(
                        r,
                        depth_pipeline.get_bind_group_layout(1),
                        &[(0, m.depth_object.as_entire_binding())],
                    )
                })
                .collect();
            Shadow {
                size,
                render,
                blur,
                color,
                depth,
                vertical,
                horizontal,
                binds,
                depth_objects,
            }
        };
        let shadows = [shadow(256), shadow(512)];
        Ok(Self {
            controls,
            time: 0.,
            last: 0.,
            params: [4., 8., 4., 8., 1.],
            knot_rotation: Vector3::ZERO,
            group_rotation: 0.,
            dir_z: 17.,
            meshes,
            shadows,
            depth_pipeline,
            vertical_pipeline,
            horizontal_pipeline,
            render_uniforms: uniform(r, "vsm render", 464),
            output_render: uniform(r, "output render", 144),
            output_object: uniform(r, "output object", 64),
            quad: init(
                "quad",
                bytemuck::cast_slice(&[-1f32, 3., 0., -1., -1., 0., 3., -1., 0.]),
                wgpu::BufferUsages::VERTEX,
            ),
            sampler,
            targets: None,
        })
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, dt: f64, animate: bool) -> Result<()> {
        if animate {
            self.time += dt;
        }
        Ok(())
    }
    /// animate(): the timer's delta turns the knot and the light group.
    pub fn prepare(&mut self, _s: &mut Scene, _c: Object3D, _r: &Renderer) -> Result<()> {
        let delta = self.time - self.last;
        self.last = self.time;
        if self.params[4] > 0.5 {
            self.knot_rotation += Vector3::new(0.25, 0.5, 1.) * delta;
            self.group_rotation += 0.7 * delta;
            self.dir_z = 17. + self.time.sin() * 5.;
        }
        let k = self.knot_rotation;
        self.meshes[0].model = Matrix4::from_translation(Vector3::new(0., 3., 0.))
            * Matrix4::from_euler(glam::EulerRot::XYZ, k.x, k.y, k.z)
            * Matrix4::from_scale(Vector3::splat(1. / 18.));
        Ok(())
    }
    fn resize(&mut self, r: &Renderer, out: &RenderTarget) -> Result<()> {
        let size = (out.width, out.height);
        let samples = out.options.samples.max(1);
        let color = texture(r, size, HALF, samples, samples == 1);
        let resolve = (samples > 1).then(|| texture(r, size, HALF, 1, true));
        let depth = texture(r, size, DEPTH, samples, false);
        let module = |label, source: &str| {
            r.device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some(label),
                source: wgpu::ShaderSource::Wgsl(source.into()),
            })
        };
        let attributes = [
            wgpu::vertex_attr_array![0 => Float32x3],
            wgpu::vertex_attr_array![1 => Float32x3],
        ];
        let buffers = attributes
            .each_ref()
            .map(|attributes| wgpu::VertexBufferLayout {
                array_stride: 12,
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes,
            });
        let pipeline = |label, vs: &str, fs: &str, buffers, format, scene: bool| {
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
        let phong = pipeline(
            "vsm phong",
            wgsl!("phong_vs"),
            wgsl!("phong_fs"),
            &buffers[..],
            HALF,
            true,
        );
        let phong_render = bind(
            r,
            phong.get_bind_group_layout(0),
            &[(0, self.render_uniforms.as_entire_binding())],
        );
        let sampler = wgpu::BindingResource::Sampler(&self.sampler);
        let objects = self
            .meshes
            .iter()
            .map(|m| {
                bind(
                    r,
                    phong.get_bind_group_layout(1),
                    &[
                        (0, m.phong_object.as_entire_binding()),
                        (1, sampler.clone()),
                        (
                            2,
                            wgpu::BindingResource::TextureView(&self.shadows[0].horizontal),
                        ),
                        (3, sampler.clone()),
                        (
                            4,
                            wgpu::BindingResource::TextureView(&self.shadows[1].horizontal),
                        ),
                    ],
                )
            })
            .collect();
        let output = pipeline(
            "render output",
            include_str!("lights_dynamic/output_vs.wgsl"),
            include_str!("lights_dynamic/output_fs.wgsl"),
            &buffers[..1],
            out.options.format,
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
                    (0, sampler),
                    (
                        1,
                        wgpu::BindingResource::TextureView(resolve.as_ref().unwrap_or(&color)),
                    ),
                    (2, self.output_object.as_entire_binding()),
                ],
            ),
        ];
        Layout::new(144)
            .m4(0, Matrix4::IDENTITY)
            .m4(64, Matrix4::IDENTITY)
            .f(128, size.0 as f64)
            .f(132, size.1 as f64)
            .write(r, &self.output_render);
        Layout::new(64)
            .m4(0, Matrix4::IDENTITY)
            .write(r, &self.output_object);
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
            resolve,
            depth,
            screen,
            phong: (phong, phong_render, objects),
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
        // Lights: the spot light at (8, 10, 5) and the directional light in
        // its turning group, both aimed at the origin.
        let spot = Vector3::new(8., 10., 5.);
        let dir = Matrix4::from_rotation_y(self.group_rotation)
            .transform_point3(Vector3::new(3., 12., self.dir_z));
        let bias = Matrix4::from_cols_array(&[
            0.5, 0., 0., 0., 0., 0.5, 0., 0., 0., 0., 1., 0., 0.5, 0.5, 0., 1.,
        ]);
        let look = |eye: Vector3| Matrix4::look_at_rh(eye, Vector3::ZERO, Vector3::Y);
        let shadow_cameras = [
            (
                perspective(2. * (PI / 5.).to_degrees(), 1., 8., 200.),
                look(spot),
            ),
            (orthographic(17., 0.1, 500.), look(dir)),
        ];
        let linear = |hex| Color::from_hex(hex).0.to_array();
        let mut layout = Layout::new(464)
            .m4(0, projection)
            .m4(64, view)
            .v3(128, linear(0x444444))
            .f(140, (PI / 5.).cos())
            .f(144, (PI / 5. * (1. - 0.3)).cos())
            .f(148, 0.)
            .f(152, 2.)
            .v3(160, linear(0xff8888).map(|v| v * 400.))
            .v3(176, linear(0x8888ff).map(|v| v * 3.))
            .p3(192, view.transform_point3(spot))
            .p3(208, spot)
            .p3(224, Vector3::ZERO)
            .p3(240, dir)
            .p3(256, Vector3::ZERO);
        for (i, (base, bias_value)) in [(272, -0.002), (352, -0.0005)].into_iter().enumerate() {
            let (p, v) = shadow_cameras[i];
            layout = layout
                .m4(base, bias * p * v)
                .f(base + 64, 0.)
                .f(base + 68, bias_value)
                .f(base + 72, 1.);
        }
        layout
            .v3(432, linear(0x222244))
            .f(444, 50.)
            .f(448, 100.)
            .write(r, &self.render_uniforms);
        for (i, shadow) in self.shadows.iter().enumerate() {
            let (p, v) = shadow_cameras[i];
            Layout::new(128).m4(0, p).m4(64, v).write(r, &shadow.render);
            let (radius, samples) = (self.params[i * 2], self.params[i * 2 + 1].round());
            Layout::new(16)
                .f(0, samples)
                .f(4, radius)
                .f(8, shadow.size as f64)
                .f(12, shadow.size as f64)
                .write(r, &shadow.blur);
        }
        let colors = linear(0x999999);
        let screen = projection * view;
        let frustum = Frustum::from_projection(screen);
        let mut draws = vec![];
        for (i, m) in self.meshes.iter().enumerate() {
            Layout::new(80)
                .f(0, 1.)
                .m4(16, m.model)
                .write(r, &m.depth_object);
            Layout::new(272)
                .v3(0, colors)
                .f(12, 1.)
                .f(16, 0.)
                .v3(32, linear(m.specular))
                .f(60, 1.)
                .m3(64, m.model.inverse().transpose())
                .m4(112, m.model)
                .m3(176, Matrix4::IDENTITY)
                .m3(224, Matrix4::IDENTITY)
                .write(r, &m.phong_object);
            if m.visible(&frustum) {
                let center = m.model.transform_point3(Vector3::ZERO);
                draws.push((screen.project_point3(center).z, i));
            }
        }
        draws.sort_by(|a, b| {
            a.0.partial_cmp(&b.0)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then(a.1.cmp(&b.1))
        });
        let t = self.targets.as_ref().ok_or(Error::Invalid("vsm targets"))?;
        let mut encoder = r.device.create_command_encoder(&Default::default());
        let attachment = |view, resolve_target| {
            Some(wgpu::RenderPassColorAttachment {
                view,
                depth_slice: None,
                resolve_target,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                    store: wgpu::StoreOp::Store,
                },
            })
        };
        let draw = |pass: &mut wgpu::RenderPass, m: &Mesh, slots: &[&wgpu::Buffer]| {
            for (slot, buffer) in slots.iter().enumerate() {
                pass.set_vertex_buffer(slot as u32, buffer.slice(..));
            }
            pass.set_index_buffer(m.index.slice(..), wgpu::IndexFormat::Uint32);
            pass.draw_indexed(0..m.count, 0, 0..1);
        };
        for (shadow, (p, v)) in self.shadows.iter().zip(shadow_cameras) {
            // Each shadow camera culls against its own frustum.
            let frustum = Frustum::from_projection(p * v);
            {
                let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("vsm depth"),
                    color_attachments: &[attachment(&shadow.color, None)],
                    depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                        view: &shadow.depth,
                        depth_ops: Some(wgpu::Operations {
                            load: wgpu::LoadOp::Clear(1.),
                            store: wgpu::StoreOp::Store,
                        }),
                        stencil_ops: None,
                    }),
                    ..Default::default()
                });
                pass.set_pipeline(&self.depth_pipeline);
                pass.set_bind_group(0, &shadow.binds[0], &[]);
                for (m, object) in self.meshes.iter().zip(&shadow.depth_objects) {
                    if !m.visible(&frustum) {
                        continue;
                    }
                    pass.set_bind_group(1, object, &[]);
                    draw(&mut pass, m, &[&m.positions]);
                }
            }
            for (view, pipeline, binds) in [
                (
                    &shadow.vertical,
                    &self.vertical_pipeline,
                    [&shadow.binds[1], &shadow.binds[2]],
                ),
                (
                    &shadow.horizontal,
                    &self.horizontal_pipeline,
                    [&shadow.binds[3], &shadow.binds[4]],
                ),
            ] {
                let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("vsm blur"),
                    color_attachments: &[attachment(view, None)],
                    ..Default::default()
                });
                pass.set_pipeline(pipeline);
                pass.set_bind_group(0, binds[0], &[]);
                pass.set_bind_group(1, binds[1], &[]);
                pass.draw(0..3, 0..1);
            }
        }
        {
            let background = linear(0x222244);
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("vsm scene"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &t.color,
                    depth_slice: None,
                    resolve_target: t.resolve.as_ref(),
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: background[0],
                            g: background[1],
                            b: background[2],
                            a: 1.,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &t.depth,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.),
                        store: wgpu::StoreOp::Discard,
                    }),
                    stencil_ops: None,
                }),
                ..Default::default()
            });
            pass.set_pipeline(&t.phong.0);
            pass.set_bind_group(0, &t.phong.1, &[]);
            for &(_, i) in &draws {
                let m = &self.meshes[i];
                pass.set_bind_group(1, &t.phong.2[i], &[]);
                draw(&mut pass, m, &[&m.normals, &m.positions]);
            }
        }
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("vsm output"),
                color_attachments: &[attachment(&t.screen.view, None)],
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
    /// Spot radius and samples, directional radius and samples, animate.
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        *self
            .params
            .get_mut(index)
            .ok_or(Error::Invalid("vsm parameter"))? = value as f64;
        Ok(())
    }
    pub fn seek(&mut self, t: f64) {
        self.time = t;
    }
}
