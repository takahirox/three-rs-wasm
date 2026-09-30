//! webgpu_tsl_vfx_linkedparticles: 8,192 short-lived particles spawned five
//! per frame toward the pointer, moved by fractal-noise turbulence, each
//! linked by two additive ribbons to its two nearest living particles (a
//! search over every particle, per particle, per frame), drawn as additive
//! hue-cycling sprites inside a flat-shaded metallic icosahedron lit by an
//! orbiting point light, with BloomNode, ACES tone mapping and auto-rotating
//! OrbitControls. Every stage runs the WGSL three.js r186 generates for the
//! page's TSL (in `linked_particles/`), over resident storage buffers that
//! the sprites and ribbons read as vertex attributes, as in the original.
use super::controls_attributes::{Controls, camera_state, viewport_css};
use crate::{
    Error, Result, camera::*, geometry::*, math::*, render_target::*, renderer::*, scene::*,
};
use std::f64::consts::PI;
use wgpu::util::DeviceExt;

const COUNT: u32 = 8192;
const LINK_VERTICES: u32 = COUNT * 8;
const HALF: wgpu::TextureFormat = wgpu::TextureFormat::Rgba16Float;
const DEPTH: wgpu::TextureFormat = wgpu::TextureFormat::Depth24Plus;
macro_rules! wgsl {
    ($name:literal) => {
        include_str!(concat!("linked_particles/", $name, ".wgsl"))
    };
}
/// The GUI values: auto rotate, its speed, timeScale, spawn rate, size,
/// lifetime, links width, color variance, color rotation speed, friction,
/// frequency, amplitude, octaves, lacunarity, gain, bloom threshold,
/// strength and radius.
const DEFAULTS: [f64; 18] = [
    1., 2., 1., 5., 1., 0.5, 0.005, 2., 1., 0.01, 0.5, 0.5, 2., 2., 0.5, 0.5, 0.75, 0.1,
];
struct Buffers {
    positions: wgpu::Buffer,
    velocities: wgpu::Buffer,
    link_vertices: wgpu::Buffer,
    link_colors: wgpu::Buffer,
    link_index: wgpu::Buffer,
    instances: wgpu::Buffer,
    quad: [wgpu::Buffer; 4],
    background: wgpu::Buffer,
    quad_uv: wgpu::Buffer,
}
/// The uniform buffers, laid out as the generated structs.
struct Uniforms {
    init: wgpu::Buffer,
    update_render: wgpu::Buffer,
    update: wgpu::Buffer,
    spawn: wgpu::Buffer,
    background_render: wgpu::Buffer,
    background: wgpu::Buffer,
    sprites_render: wgpu::Buffer,
    sprites: wgpu::Buffer,
    links_render: wgpu::Buffer,
    links: wgpu::Buffer,
    high_pass: wgpu::Buffer,
    blur: Vec<wgpu::Buffer>,
    composite: wgpu::Buffer,
    tints: wgpu::Buffer,
    output: wgpu::Buffer,
}
struct Computes {
    update: wgpu::ComputePipeline,
    spawn: wgpu::ComputePipeline,
    update_binds: [wgpu::BindGroup; 2],
    spawn_bind: wgpu::BindGroup,
}
/// Pipelines and bind groups that follow the sample count and the size.
struct Targets {
    width: u32,
    height: u32,
    samples: u32,
    format: wgpu::TextureFormat,
    color: wgpu::TextureView,
    resolve: Option<wgpu::TextureView>,
    depth: wgpu::TextureView,
    bright: wgpu::TextureView,
    bright_size: (u32, u32),
    horizontal: Vec<(wgpu::TextureView, (u32, u32))>,
    vertical: Vec<wgpu::TextureView>,
    screen: RenderTarget,
    background: (wgpu::RenderPipeline, [wgpu::BindGroup; 2]),
    sprites: (wgpu::RenderPipeline, [wgpu::BindGroup; 2]),
    links: [(wgpu::RenderPipeline, [wgpu::BindGroup; 2]); 2],
    high_pass: (wgpu::RenderPipeline, wgpu::BindGroup),
    blurs: Vec<(wgpu::RenderPipeline, [wgpu::BindGroup; 2])>,
    composite: (wgpu::RenderPipeline, wgpu::BindGroup),
    output: (wgpu::RenderPipeline, [wgpu::BindGroup; 2]),
}
pub(super) struct Demo {
    controls: Controls,
    params: [f64; 18],
    buffers: Buffers,
    uniforms: Uniforms,
    computes: Computes,
    targets: Option<Targets>,
    sampler: wgpu::Sampler,
    background_count: u32,
    /// The page's CPU state.
    spawn_index: f64,
    spawn: Vector3,
    previous_spawn: Vector3,
    pointer: Vector2,
    scene_pointer: Vector3,
    plane_normal: Vector3,
    color_offset: f64,
    time: f64,
    last: f64,
    pending: Option<f64>,
    light: Vector3,
}
fn module(r: &Renderer, label: &str, source: &str) -> wgpu::ShaderModule {
    r.device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some(label),
        source: wgpu::ShaderSource::Wgsl(source.into()),
    })
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
    fn u(mut self, offset: usize, v: u32) -> Self {
        self.0[offset..offset + 4].copy_from_slice(&v.to_le_bytes());
        self
    }
    fn v3(self, offset: usize, v: Vector3) -> Self {
        self.f(offset, v.x).f(offset + 4, v.y).f(offset + 8, v.z)
    }
    fn m4(mut self, offset: usize, m: Matrix4) -> Self {
        for (i, v) in m.to_cols_array().iter().enumerate() {
            self = self.f(offset + i * 4, *v);
        }
        self
    }
    /// An identity mat3x3 (columns padded to 16 bytes).
    fn m3(self, offset: usize) -> Self {
        self.f(offset, 1.).f(offset + 20, 1.).f(offset + 40, 1.)
    }
    fn write(self, r: &Renderer, buffer: &wgpu::Buffer) {
        r.queue.write_buffer(buffer, 0, &self.0);
    }
}
/// Euler.setFromQuaternion( q, 'XYZ' ).
fn euler_xyz(q: Quaternion) -> Vector3 {
    let m = Matrix4::from_quat(q);
    let (m11, m12, m13) = (m.x_axis.x, m.y_axis.x, m.z_axis.x);
    let (m22, m23) = (m.y_axis.y, m.z_axis.y);
    let (m32, m33) = (m.y_axis.z, m.z_axis.z);
    let y = m13.clamp(-1., 1.).asin();
    if m13.abs() < 0.9999999 {
        Vector3::new((-m23).atan2(m33), y, (-m12).atan2(m11))
    } else {
        Vector3::new(m32.atan2(m22), y, 0.)
    }
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
            far: 200.,
            aspect,
            ..Default::default()
        }));
        let n = s.get_mut(c)?;
        n.position = Vector3::new(0., 0., 10.);
        n.quaternion = Quaternion::IDENTITY;
        let mut controls = Controls::new(Some(0.05), (0., 75.), PI, true);
        controls.auto_rotate = Some(2.);
        let storage = |label, count: u32, extra: wgpu::BufferUsages| {
            r.device.create_buffer(&wgpu::BufferDescriptor {
                label: Some(label),
                size: count as u64 * 16,
                usage: wgpu::BufferUsages::STORAGE | extra,
                mapped_at_creation: false,
            })
        };
        let vertex = wgpu::BufferUsages::VERTEX;
        let init = |label, data: &[u8], usage| {
            r.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some(label),
                    contents: data,
                    usage,
                })
        };
        let mut index = vec![];
        for i in 0..COUNT {
            for j in 0..2 {
                let o = i * 8 + j * 4;
                index.extend([o, o + 1, o + 2, o, o + 2, o + 3]);
            }
        }
        let identity: Vec<f32> = (0..COUNT)
            .flat_map(|_| Matrix4::IDENTITY.to_cols_array().map(|v| v as f32))
            .collect();
        let plane = PlaneGeometry::build(0.05, 0.05, 1, 1)?;
        let floats = |name: &str| -> Result<Vec<f32>> {
            let a = plane
                .attributes
                .get(name)
                .ok_or(Error::Invalid("plane attribute"))?;
            (0..a.count())
                .flat_map(|i| (0..a.item_size()).map(move |k| (i, k)))
                .map(|(i, k)| a.get_component(i, k).map(|v| v as f32))
                .collect()
        };
        // IcosahedronGeometry( 100, 5 ) mirrored by makeScale( -1, 1, 1 ).
        let sky: Vec<f32> = IcosahedronGeometry::build(100., 5)?
            .positions()?
            .iter()
            .flat_map(|p| [(-p.x) as f32, p.y as f32, p.z as f32])
            .collect();
        let buffers = Buffers {
            positions: storage("particle positions", COUNT, vertex),
            velocities: storage("particle velocities", COUNT, vertex),
            link_vertices: storage("link vertices", LINK_VERTICES, vertex),
            link_colors: storage("link colors", LINK_VERTICES, vertex),
            link_index: init(
                "link index",
                bytemuck::cast_slice(&index),
                wgpu::BufferUsages::INDEX,
            ),
            instances: init(
                "particle instances",
                bytemuck::cast_slice(&identity),
                vertex,
            ),
            quad: [
                init(
                    "sprite position",
                    bytemuck::cast_slice(&floats("position")?),
                    vertex,
                ),
                init(
                    "sprite normal",
                    bytemuck::cast_slice(&floats("normal")?),
                    vertex,
                ),
                init("sprite uv", bytemuck::cast_slice(&floats("uv")?), vertex),
                init(
                    "sprite index",
                    bytemuck::cast_slice(
                        plane.index.as_ref().ok_or(Error::Invalid("plane index"))?,
                    ),
                    wgpu::BufferUsages::INDEX,
                ),
            ],
            background: init("background", bytemuck::cast_slice(&sky), vertex),
            quad_uv: init(
                "quad uv",
                bytemuck::cast_slice(&[0f32, -1., 0., 1., 2., 1.]),
                vertex,
            ),
        };
        let uniforms = Uniforms {
            init: uniform(r, "init", 16),
            update_render: uniform(r, "update render", 16),
            update: uniform(r, "update", 48),
            spawn: uniform(r, "spawn", 64),
            background_render: uniform(r, "background render", 176),
            background: uniform(r, "background", 96),
            sprites_render: uniform(r, "sprites render", 144),
            sprites: uniform(r, "sprites", 96),
            links_render: uniform(r, "links render", 128),
            links: uniform(r, "links", 80),
            high_pass: uniform(r, "high pass", 16),
            blur: (0..10).map(|_| uniform(r, "blur", 160)).collect(),
            composite: uniform(r, "composite", 272),
            tints: uniform(r, "tints", 80),
            output: uniform(r, "output", 16),
        };
        let compute = |label, source| {
            r.device
                .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                    label: Some(label),
                    layout: None,
                    module: &module(r, label, source),
                    entry_point: Some("main"),
                    compilation_options: Default::default(),
                    cache: None,
                })
        };
        fn buffer(b: &wgpu::Buffer) -> wgpu::BindingResource<'_> {
            b.as_entire_binding()
        }
        // Initialize: every particle dead, far away.
        let init_pipeline = compute("init", wgsl!("init"));
        Layout::new(16).u(0, COUNT).write(r, &uniforms.init);
        let init_bind = bind(
            r,
            init_pipeline.get_bind_group_layout(0),
            &[(0, buffer(&buffers.positions)), (1, buffer(&uniforms.init))],
        );
        let update = compute("update particles", wgsl!("update"));
        let update_binds = [
            bind(
                r,
                update.get_bind_group_layout(0),
                &[(0, buffer(&uniforms.update_render))],
            ),
            bind(
                r,
                update.get_bind_group_layout(1),
                &[
                    (0, buffer(&buffers.positions)),
                    (1, buffer(&buffers.velocities)),
                    (2, buffer(&uniforms.update)),
                    (3, buffer(&buffers.link_vertices)),
                    (4, buffer(&buffers.link_colors)),
                ],
            ),
        ];
        let spawn = compute("spawn particles", wgsl!("spawn"));
        let spawn_bind = bind(
            r,
            spawn.get_bind_group_layout(0),
            &[
                (0, buffer(&buffers.positions)),
                (1, buffer(&uniforms.spawn)),
                (2, buffer(&buffers.velocities)),
            ],
        );
        let mut encoder = r.device.create_command_encoder(&Default::default());
        {
            let mut pass = encoder.begin_compute_pass(&Default::default());
            pass.set_pipeline(&init_pipeline);
            pass.set_bind_group(0, &init_bind, &[]);
            pass.dispatch_workgroups(COUNT / 64, 1, 1);
        }
        r.queue.submit([encoder.finish()]);
        let sampler = r.device.create_sampler(&wgpu::SamplerDescriptor {
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        // BloomNode's five white tints.
        let mut tints = Layout::new(80);
        for i in 0..5 {
            tints = tints.v3(i * 16, Vector3::ONE);
        }
        tints.write(r, &uniforms.tints);
        Ok(Self {
            controls,
            params: DEFAULTS,
            background_count: (sky.len() / 3) as u32,
            buffers,
            uniforms,
            computes: Computes {
                update,
                spawn,
                update_binds,
                spawn_bind,
            },
            targets: None,
            sampler,
            spawn_index: 0.,
            spawn: Vector3::ZERO,
            previous_spawn: Vector3::ZERO,
            pointer: Vector2::ZERO,
            scene_pointer: Vector3::ZERO,
            plane_normal: Vector3::Z,
            color_offset: 0.,
            time: 0.,
            last: 0.,
            pending: Some(0.),
            light: Vector3::ZERO,
        })
    }
    /// animate(): the compute passes, then the page's CPU updates.
    fn step(&mut self, s: &mut Scene, c: Object3D, r: &Renderer, delta: f64) -> Result<()> {
        let p = self.params;
        Layout::new(16)
            .f(0, delta)
            .write(r, &self.uniforms.update_render);
        Layout::new(48)
            .f(0, p[10])
            .f(4, p[12])
            .f(8, p[13])
            .f(12, p[14])
            .f(16, p[11])
            .f(20, p[9])
            .f(24, p[2])
            .f(28, p[5])
            .f(32, p[6])
            .f(36, self.color_offset)
            .f(40, p[7])
            .u(44, COUNT)
            .write(r, &self.uniforms.update);
        Layout::new(64)
            .f(0, self.spawn_index)
            .v3(16, self.previous_spawn)
            .v3(32, self.spawn)
            .f(44, p[3])
            .u(48, 5)
            .write(r, &self.uniforms.spawn);
        let mut encoder = r.device.create_command_encoder(&Default::default());
        {
            let mut pass = encoder.begin_compute_pass(&Default::default());
            pass.set_pipeline(&self.computes.update);
            pass.set_bind_group(0, &self.computes.update_binds[0], &[]);
            pass.set_bind_group(1, &self.computes.update_binds[1], &[]);
            pass.dispatch_workgroups(COUNT / 64, 1, 1);
            pass.set_pipeline(&self.computes.spawn);
            pass.set_bind_group(0, &self.computes.spawn_bind, &[]);
            pass.dispatch_workgroups(1, 1, 1);
        }
        r.queue.submit([encoder.finish()]);
        self.spawn_index = (self.spawn_index + p[3]) % COUNT as f64;
        // raycastPlane.normal.applyEuler( camera.rotation ), then the pointer ray.
        s.update()?;
        let (camera, world) = s.camera(c)?;
        let rotation = euler_xyz(s.get(c)?.quaternion);
        self.plane_normal =
            Quaternion::from_euler(glam::EulerRot::XYZ, rotation.x, rotation.y, rotation.z)
                * self.plane_normal;
        let Camera::Perspective(pc) = camera else {
            return Err(Error::Invalid("particles camera"));
        };
        let tan = (pc.fov.to_radians() * 0.5).tan();
        let origin = world.w_axis.truncate();
        let direction = world
            .transform_vector3(Vector3::new(
                self.pointer.x * tan * pc.aspect,
                self.pointer.y * tan,
                -1.,
            ))
            .normalize();
        let denominator = self.plane_normal.dot(direction);
        let hit = if denominator == 0. {
            (self.plane_normal.dot(origin) == 0.).then_some(0.)
        } else {
            let t = -self.plane_normal.dot(origin) / denominator;
            (t >= 0.).then_some(t)
        };
        if let Some(t) = hit {
            self.scene_pointer = origin + direction * t;
        }
        self.previous_spawn = self.spawn;
        self.spawn = self.spawn.lerp(self.scene_pointer, 0.1);
        self.color_offset += delta * p[8] * p[2];
        let e = self.time;
        self.light = Vector3::new(
            (e * 0.5).sin() * 30.,
            (e * 0.3).cos() * 30.,
            (e * 0.2).sin() * 30.,
        );
        self.controls.auto_rotate = (p[0] > 0.5).then_some(p[1]);
        self.controls.frame_update(s, c)
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, dt: f64, animate: bool) -> Result<()> {
        if animate {
            self.time += dt;
            self.pending = Some(dt);
        }
        Ok(())
    }
    pub fn prepare(&mut self, s: &mut Scene, c: Object3D, r: &Renderer) -> Result<()> {
        if let Some(delta) = self.pending.take() {
            self.step(s, c, r, delta)?;
        }
        Ok(())
    }
    fn resize(&mut self, r: &Renderer, out: &RenderTarget) -> Result<()> {
        let samples = out.options.samples.max(1);
        let (width, height) = (out.width, out.height);
        let texture = |w: u32, h: u32, format, samples, sampled: bool| {
            r.device
                .create_texture(&wgpu::TextureDescriptor {
                    label: Some("linked particles target"),
                    size: wgpu::Extent3d {
                        width: w.max(1),
                        height: h.max(1),
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
        };
        let color = texture(width, height, HALF, samples, samples == 1);
        let resolve = (samples > 1).then(|| texture(width, height, HALF, 1, true));
        let depth = texture(width, height, DEPTH, samples, false);
        // BloomNode.setSize: half resolution, then halving per mip.
        let (mut w, mut h) = (width / 2, height / 2);
        let bright = texture(w, h, HALF, 1, true);
        let bright_size = (w, h);
        let mut horizontal = vec![];
        let mut vertical = vec![];
        for level in 0..5 {
            horizontal.push((texture(w, h, HALF, 1, true), (w, h)));
            vertical.push(texture(w, h, HALF, 1, true));
            for (k, direction) in [(1., 0.), (0., 1.)].into_iter().enumerate() {
                Layout::new(160)
                    .m3(0)
                    .m3(48)
                    .f(96, 1. / w as f64)
                    .f(100, 1. / h as f64)
                    .f(104, direction.0)
                    .f(108, direction.1)
                    .m3(112)
                    .write(r, &self.uniforms.blur[level * 2 + k]);
            }
            w /= 2;
            h /= 2;
        }
        let scene_view = resolve.as_ref().unwrap_or(&color);
        let format = out.options.format;
        let module_vs = |label, source| module(r, label, source);
        let scene_pipeline = |label,
                              vs: &wgpu::ShaderModule,
                              fs: &wgpu::ShaderModule,
                              buffers: &[wgpu::VertexBufferLayout],
                              blend: Option<wgpu::BlendState>,
                              depth_write: bool,
                              compare: wgpu::CompareFunction,
                              front: wgpu::FrontFace| {
            r.device
                .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    label: Some(label),
                    layout: None,
                    vertex: wgpu::VertexState {
                        module: vs,
                        entry_point: Some("main"),
                        compilation_options: Default::default(),
                        buffers,
                    },
                    fragment: Some(wgpu::FragmentState {
                        module: fs,
                        entry_point: Some("main"),
                        compilation_options: Default::default(),
                        targets: &[Some(wgpu::ColorTargetState {
                            format: HALF,
                            blend,
                            write_mask: wgpu::ColorWrites::ALL,
                        })],
                    }),
                    primitive: wgpu::PrimitiveState {
                        front_face: front,
                        cull_mode: Some(wgpu::Face::Back),
                        ..Default::default()
                    },
                    depth_stencil: Some(wgpu::DepthStencilState {
                        format: DEPTH,
                        depth_write_enabled: depth_write,
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
        };
        let additive = Some(wgpu::BlendState {
            color: wgpu::BlendComponent {
                src_factor: wgpu::BlendFactor::SrcAlpha,
                dst_factor: wgpu::BlendFactor::One,
                operation: wgpu::BlendOperation::Add,
            },
            alpha: wgpu::BlendComponent {
                src_factor: wgpu::BlendFactor::One,
                dst_factor: wgpu::BlendFactor::One,
                operation: wgpu::BlendOperation::Add,
            },
        });
        // Vertex attributes: (location, format) arrays that outlive the layouts.
        let attributes: Vec<[wgpu::VertexAttribute; 1]> = [
            (0, wgpu::VertexFormat::Float32x3),
            (1, wgpu::VertexFormat::Float32x3),
            (2, wgpu::VertexFormat::Float32x2),
            (7, wgpu::VertexFormat::Float32x4),
            (8, wgpu::VertexFormat::Float32x4),
            (0, wgpu::VertexFormat::Float32x4),
            (1, wgpu::VertexFormat::Float32x4),
            (2, wgpu::VertexFormat::Float32x4),
            (0, wgpu::VertexFormat::Float32x2),
        ]
        .iter()
        .map(|&(shader_location, format)| {
            [wgpu::VertexAttribute {
                format,
                offset: 0,
                shader_location,
            }]
        })
        .collect();
        let attribute = |i: usize, stride, step| wgpu::VertexBufferLayout {
            array_stride: stride,
            step_mode: step,
            attributes: &attributes[i],
        };
        use wgpu::VertexStepMode::{Instance, Vertex};
        let less_equal = wgpu::CompareFunction::LessEqual;
        let ccw = wgpu::FrontFace::Ccw;
        let background_pipeline = scene_pipeline(
            "background",
            &module_vs("background vs", wgsl!("background_vs")),
            &module_vs("background fs", wgsl!("background_fs")),
            &[attribute(0, 12, Vertex)],
            None,
            true,
            less_equal,
            ccw,
        );
        let dfg = &r.dfg;
        let background = (
            bind(
                r,
                background_pipeline.get_bind_group_layout(0),
                &[(0, self.uniforms.background_render.as_entire_binding())],
            ),
            bind(
                r,
                background_pipeline.get_bind_group_layout(1),
                &[
                    (0, self.uniforms.background.as_entire_binding()),
                    (1, wgpu::BindingResource::Sampler(&self.sampler)),
                    (2, wgpu::BindingResource::TextureView(dfg)),
                ],
            ),
        );
        let instance_matrix = wgpu::VertexBufferLayout {
            array_stride: 64,
            step_mode: Instance,
            attributes: &wgpu::vertex_attr_array![3 => Float32x4, 4 => Float32x4, 5 => Float32x4, 6 => Float32x4],
        };
        let sprites_pipeline = scene_pipeline(
            "sprites",
            &module_vs("sprites vs", wgsl!("sprites_vs")),
            &module_vs("sprites fs", wgsl!("sprites_fs")),
            &[
                attribute(0, 12, Vertex),
                attribute(1, 12, Vertex),
                attribute(2, 8, Vertex),
                instance_matrix,
                attribute(3, 16, Instance),
                attribute(4, 16, Instance),
            ],
            additive,
            false,
            less_equal,
            ccw,
        );
        let sprites = (
            bind(
                r,
                sprites_pipeline.get_bind_group_layout(0),
                &[(0, self.uniforms.sprites_render.as_entire_binding())],
            ),
            bind(
                r,
                sprites_pipeline.get_bind_group_layout(1),
                &[(0, self.uniforms.sprites.as_entire_binding())],
            ),
        );
        // DoubleSide transparent: the back faces, then the front faces.
        let links_vs = module_vs("links vs", wgsl!("links_vs"));
        let links_fs = module_vs("links fs", wgsl!("links_fs"));
        let link_buffers = [
            attribute(5, 16, Vertex),
            attribute(6, 16, Vertex),
            attribute(7, 16, Vertex),
        ];
        let links_pipelines = [wgpu::FrontFace::Cw, ccw].map(|front| {
            scene_pipeline(
                "links",
                &links_vs,
                &links_fs,
                &link_buffers,
                additive,
                false,
                wgpu::CompareFunction::Always,
                front,
            )
        });
        // Default layouts: each pipeline binds groups made from its own layouts.
        let links = links_pipelines.map(|pipeline| {
            let binds = [
                bind(
                    r,
                    pipeline.get_bind_group_layout(0),
                    &[(0, self.uniforms.links_render.as_entire_binding())],
                ),
                bind(
                    r,
                    pipeline.get_bind_group_layout(1),
                    &[(0, self.uniforms.links.as_entire_binding())],
                ),
            ];
            (pipeline, binds)
        });
        // The post passes: QuadMesh with its uv attribute.
        let quad_vs = module_vs("quad", wgsl!("quad_vs"));
        let post = |label, source: &str, target| {
            r.device
                .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    label: Some(label),
                    layout: None,
                    vertex: wgpu::VertexState {
                        module: &quad_vs,
                        entry_point: Some("main"),
                        compilation_options: Default::default(),
                        buffers: &[attribute(8, 8, Vertex)],
                    },
                    fragment: Some(wgpu::FragmentState {
                        module: &module(r, label, source),
                        entry_point: Some("main"),
                        compilation_options: Default::default(),
                        targets: &[Some(target)],
                    }),
                    primitive: wgpu::PrimitiveState {
                        cull_mode: Some(wgpu::Face::Back),
                        ..Default::default()
                    },
                    depth_stencil: None,
                    multisample: Default::default(),
                    multiview: None,
                    cache: None,
                })
        };
        let sampler = wgpu::BindingResource::Sampler(&self.sampler);
        let high_pass_pipeline = post("high pass", wgsl!("high_pass"), HALF.into());
        let high_pass = bind(
            r,
            high_pass_pipeline.get_bind_group_layout(0),
            &[
                (0, sampler.clone()),
                (1, wgpu::BindingResource::TextureView(scene_view)),
                (2, self.uniforms.high_pass.as_entire_binding()),
            ],
        );
        let sources = [
            wgsl!("blur3"),
            wgsl!("blur5"),
            wgsl!("blur7"),
            wgsl!("blur9"),
            wgsl!("blur11"),
        ];
        let mut blurs = vec![];
        for level in 0..5 {
            let pipeline = post("blur", sources[level], HALF.into());
            let input = if level == 0 {
                &bright
            } else {
                &vertical[level - 1]
            };
            let binds = [(input, 0), (&horizontal[level].0, 1)].map(|(view, k)| {
                bind(
                    r,
                    pipeline.get_bind_group_layout(0),
                    &[
                        (0, sampler.clone()),
                        (1, wgpu::BindingResource::TextureView(view)),
                        (2, self.uniforms.blur[level * 2 + k].as_entire_binding()),
                    ],
                )
            });
            blurs.push((pipeline, binds));
        }
        let composite_pipeline = post("composite", wgsl!("composite"), HALF.into());
        let mut entries = vec![
            (0, self.uniforms.composite.as_entire_binding()),
            (1, self.uniforms.tints.as_entire_binding()),
        ];
        for (i, view) in vertical.iter().enumerate() {
            entries.push((2 + i as u32 * 2, sampler.clone()));
            entries.push((3 + i as u32 * 2, wgpu::BindingResource::TextureView(view)));
        }
        let composite = bind(r, composite_pipeline.get_bind_group_layout(0), &entries);
        let output_pipeline = post("output", wgsl!("output"), format.into());
        let output = [
            bind(
                r,
                output_pipeline.get_bind_group_layout(0),
                &[(0, self.uniforms.output.as_entire_binding())],
            ),
            bind(
                r,
                output_pipeline.get_bind_group_layout(1),
                &[
                    (0, sampler.clone()),
                    (1, wgpu::BindingResource::TextureView(scene_view)),
                    (2, sampler.clone()),
                    (3, wgpu::BindingResource::TextureView(&horizontal[0].0)),
                ],
            ),
        ];
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
            samples,
            format,
            color,
            resolve,
            depth,
            bright,
            bright_size,
            horizontal,
            vertical,
            screen,
            background: (background_pipeline, [background.0, background.1]),
            sprites: (sprites_pipeline, [sprites.0, sprites.1]),
            links,
            high_pass: (high_pass_pipeline, high_pass),
            blurs,
            composite: (composite_pipeline, composite),
            output: (output_pipeline, output),
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
                || t.samples != out.options.samples.max(1)
                || t.format != out.options.format
        }) {
            self.resize(r, out)?;
        }
        s.update()?;
        let (camera, world) = s.camera(c)?;
        let (projection, view) = (camera.projection_matrix()?, world.inverse());
        let p = self.params;
        let light = view.transform_point3(self.light);
        Layout::new(176)
            .m4(0, projection)
            .m4(64, view)
            .v3(128, Vector3::splat(3000.))
            .f(140, 0.)
            .f(144, 2.)
            .v3(160, light)
            .write(r, &self.uniforms.background_render);
        Layout::new(96)
            .f(0, 1.)
            .f(4, 0.9)
            .f(8, 0.4)
            .m4(16, Matrix4::IDENTITY)
            .v3(80, Vector3::ZERO)
            .f(92, 1.)
            .write(r, &self.uniforms.background);
        Layout::new(144)
            .f(0, self.time)
            .m4(16, projection)
            .m4(80, view)
            .write(r, &self.uniforms.sprites_render);
        Layout::new(96)
            .f(0, self.color_offset)
            .f(4, p[7])
            .m4(16, Matrix4::IDENTITY)
            .f(80, p[4])
            .write(r, &self.uniforms.sprites);
        Layout::new(128)
            .m4(0, projection)
            .m4(64, view)
            .write(r, &self.uniforms.links_render);
        Layout::new(80)
            .v3(0, Vector3::ONE)
            .m4(16, Matrix4::IDENTITY)
            .write(r, &self.uniforms.links);
        Layout::new(16)
            .f(0, p[15])
            .f(4, 0.01)
            .write(r, &self.uniforms.high_pass);
        Layout::new(272)
            .f(0, p[17])
            .m3(16)
            .m3(64)
            .m3(112)
            .m3(160)
            .m3(208)
            .f(256, p[16])
            .write(r, &self.uniforms.composite);
        Layout::new(16).f(0, 1.).write(r, &self.uniforms.output);
        let t = self
            .targets
            .as_ref()
            .ok_or(Error::Invalid("particle targets"))?;
        let b = &self.buffers;
        let background = Color::from_hex(0x14171a).0;
        let mut encoder = r.device.create_command_encoder(&Default::default());
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("linked particles scene"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &t.color,
                    depth_slice: None,
                    resolve_target: t.resolve.as_ref(),
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: background.x,
                            g: background.y,
                            b: background.z,
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
            pass.set_pipeline(&t.background.0);
            pass.set_bind_group(0, &t.background.1[0], &[]);
            pass.set_bind_group(1, &t.background.1[1], &[]);
            pass.set_vertex_buffer(0, b.background.slice(..));
            pass.draw(0..self.background_count, 0..1);
            pass.set_pipeline(&t.sprites.0);
            pass.set_bind_group(0, &t.sprites.1[0], &[]);
            pass.set_bind_group(1, &t.sprites.1[1], &[]);
            for (i, buffer) in [
                &b.quad[0],
                &b.quad[1],
                &b.quad[2],
                &b.instances,
                &b.positions,
                &b.velocities,
            ]
            .into_iter()
            .enumerate()
            {
                pass.set_vertex_buffer(i as u32, buffer.slice(..));
            }
            pass.set_index_buffer(b.quad[3].slice(..), wgpu::IndexFormat::Uint32);
            pass.draw_indexed(0..6, 0, 0..COUNT);
            for (pipeline, binds) in &t.links {
                pass.set_pipeline(pipeline);
                pass.set_bind_group(0, &binds[0], &[]);
                pass.set_bind_group(1, &binds[1], &[]);
                pass.set_vertex_buffer(0, b.link_colors.slice(..));
                pass.set_vertex_buffer(1, b.link_vertices.slice(..));
                pass.set_vertex_buffer(2, b.link_colors.slice(..));
                pass.set_index_buffer(b.link_index.slice(..), wgpu::IndexFormat::Uint32);
                pass.draw_indexed(0..COUNT * 12, 0, 0..1);
            }
        }
        let mut post = |view: &wgpu::TextureView,
                        pipeline: &wgpu::RenderPipeline,
                        binds: &[&wgpu::BindGroup]| {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("linked particles post"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                ..Default::default()
            });
            pass.set_pipeline(pipeline);
            for (i, bind) in binds.iter().enumerate() {
                pass.set_bind_group(i as u32, *bind, &[]);
            }
            pass.set_vertex_buffer(0, b.quad_uv.slice(..));
            pass.draw(0..3, 0..1);
        };
        let _ = t.bright_size;
        post(&t.bright, &t.high_pass.0, &[&t.high_pass.1]);
        for (level, (pipeline, binds)) in t.blurs.iter().enumerate() {
            post(&t.horizontal[level].0, pipeline, &[&binds[0]]);
            post(&t.vertical[level], pipeline, &[&binds[1]]);
        }
        post(&t.horizontal[0].0, &t.composite.0, &[&t.composite.1]);
        post(
            &t.screen.view,
            &t.output.0,
            &[&t.output.1[0], &t.output.1[1]],
        );
        r.queue.submit([encoder.finish()]);
        Ok(true)
    }
    pub fn output(&self) -> Option<&RenderTarget> {
        self.targets.as_ref().map(|t| &t.screen)
    }
    /// window pointermove: the screen pointer.
    pub fn draw(&mut self, kind: u32, x: f64, y: f64) {
        if kind == 0 {
            let (w, h, _) = viewport_css();
            self.pointer = Vector2::new(x / w * 2. - 1., -(y / h) * 2. + 1.);
        }
    }
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
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        *self
            .params
            .get_mut(index)
            .ok_or(Error::Invalid("particles parameter"))? = value as f64;
        Ok(())
    }
    /// A still frame: one animate() with the timer at t.
    pub fn seek(&mut self, t: f64) {
        self.pending = Some(t - self.last);
        self.last = t;
        self.time = t;
    }
}
