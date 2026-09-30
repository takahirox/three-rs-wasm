//! webgpu_lights_clustered: 876 point lights riding on bouncing emissive
//! spheres between four large Phong spheres on a Phong floor, shaded through
//! ClusteredLighting: each frame the lights are sorted by view depth into a
//! float texture with per-slice light ranges (on the CPU, as the addon does),
//! a compute pass assigns up to 64 lights to each 32-pixel tile of 24
//! logarithmic depth slices, and the materials loop over their cluster's
//! lights. The scene pass is followed by renderOutput (NeutralToneMapping,
//! with the optional cluster heat map) and FXAA. Every stage runs the WGSL
//! three.js r186 generates for the page (in `lights_clustered/`), with the
//! tile grid set for the drawing-buffer size.
use super::controls_attributes::{Controls, camera_state};
use super::interactive_scenes::hsl;
use crate::{
    Error, Result, camera::*, geometry::*, math::*, render_target::*, renderer::*, scene::*,
};
use std::f64::consts::PI;
use wgpu::util::DeviceExt;

const GRID: usize = 30;
const SPACING: f64 = 5.;
const RADIUS: f64 = 0.5;
const WAVE_HEIGHT: f64 = 4.;
const BIG_RADIUS: f64 = 6.;
const BASE_POWER: f64 = 45.;
const MAX_LIGHTS: usize = 1024;
const TILE: u32 = 32;
const SLICES: u32 = 24;
const CHUNKS: u64 = 16;
const HALF: wgpu::TextureFormat = wgpu::TextureFormat::Rgba16Float;
const DEPTH: wgpu::TextureFormat = wgpu::TextureFormat::Depth24Plus;
macro_rules! wgsl {
    ($name:literal) => {
        include_str!(concat!("lights_clustered/", $name, ".wgsl"))
    };
}
/// One bouncing sphere and its light.
struct Ball {
    x: f64,
    z: f64,
    phase: f64,
    y: f64,
    color: [f64; 3],
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
    fn i(mut self, offset: usize, v: i32) -> Self {
        self.0[offset..offset + 4].copy_from_slice(&v.to_le_bytes());
        self
    }
    fn v3(self, offset: usize, v: [f64; 3]) -> Self {
        self.f(offset, v[0]).f(offset + 4, v[1]).f(offset + 8, v[2])
    }
    fn m4(mut self, offset: usize, m: Matrix4) -> Self {
        for (i, v) in m.to_cols_array().iter().enumerate() {
            self = self.f(offset + i * 4, *v);
        }
        self
    }
    fn m3(self, offset: usize) -> Self {
        self.f(offset, 1.).f(offset + 20, 1.).f(offset + 40, 1.)
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
/// Pipelines, targets and bind groups for one drawing-buffer size.
struct Targets {
    width: u32,
    height: u32,
    format: wgpu::TextureFormat,
    grid: (u32, u32),
    color: wgpu::TextureView,
    depth: wgpu::TextureView,
    toned: wgpu::TextureView,
    screen: RenderTarget,
    clusters: (wgpu::ComputePipeline, [wgpu::BindGroup; 2]),
    phong: (wgpu::RenderPipeline, wgpu::BindGroup, Vec<wgpu::BindGroup>),
    spheres: (wgpu::RenderPipeline, [wgpu::BindGroup; 2]),
    output: (wgpu::RenderPipeline, [wgpu::BindGroup; 2]),
    fxaa: (wgpu::RenderPipeline, wgpu::BindGroup),
}
pub(super) struct Demo {
    controls: Controls,
    balls: Vec<Ball>,
    /// intensity, animate, lights per tile, z-slice.
    params: [f64; 4],
    time: f64,
    lights: wgpu::Texture,
    ranges: wgpu::Texture,
    light_indexes: Option<wgpu::Buffer>,
    render_uniforms: wgpu::Buffer,
    cluster_render: wgpu::Buffer,
    cluster_object: wgpu::Buffer,
    phong_objects: Vec<wgpu::Buffer>,
    sphere_object: wgpu::Buffer,
    instances: wgpu::Buffer,
    output_render: wgpu::Buffer,
    output_object: wgpu::Buffer,
    fxaa_steps: wgpu::Buffer,
    fxaa_size: wgpu::Buffer,
    /// Floor, big sphere and small sphere geometry: positions, normals, index.
    meshes: Vec<(wgpu::Buffer, wgpu::Buffer, wgpu::Buffer, u32)>,
    colors: wgpu::Buffer,
    quad_uv: wgpu::Buffer,
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
            fov: 50.,
            near: 1.,
            far: 200.,
            aspect,
            ..Default::default()
        }));
        s.get_mut(c)?.position = Vector3::new(36., 18., 36.);
        let mut controls = Controls::new(Some(0.05), (4., 400.), PI * 0.49, true);
        controls.set_target(Vector3::new(0., 6., 0.));
        controls.update(s, c)?;
        let bigs = [(-9., -9.), (9., -9.), (-9., 9.), (9., 9.)];
        let span = (GRID - 1) as f64 * SPACING;
        let mut balls = vec![];
        for ix in 0..GRID {
            for iz in 0..GRID {
                let x = ix as f64 * SPACING - span / 2.;
                let z = iz as f64 * SPACING - span / 2.;
                if bigs
                    .iter()
                    .any(|&(bx, bz): &(f64, f64)| (x - bx).hypot(z - bz) < BIG_RADIUS + 1.)
                {
                    continue;
                }
                balls.push(Ball {
                    x,
                    z,
                    phase: (ix + iz) as f64 * 0.5,
                    y: RADIUS,
                    color: hsl((ix * GRID + iz) as f64 / (GRID * GRID) as f64, 1., 0.5),
                });
            }
        }
        let texture = |label, width, height| {
            r.device.create_texture(&wgpu::TextureDescriptor {
                label: Some(label),
                size: wgpu::Extent3d {
                    width,
                    height,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba32Float,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                view_formats: &[],
            })
        };
        let init = |label, data: &[u8], usage| {
            r.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some(label),
                    contents: data,
                    usage,
                })
        };
        let vertex = wgpu::BufferUsages::VERTEX;
        let mesh = |g: BufferGeometry| -> Result<(wgpu::Buffer, wgpu::Buffer, wgpu::Buffer, u32)> {
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
            let index = g.index.clone().ok_or(Error::Invalid("mesh index"))?;
            Ok((
                init(
                    "clustered positions",
                    bytemuck::cast_slice(&f("position")?),
                    vertex,
                ),
                init(
                    "clustered normals",
                    bytemuck::cast_slice(&f("normal")?),
                    vertex,
                ),
                init(
                    "clustered index",
                    bytemuck::cast_slice(&index),
                    wgpu::BufferUsages::INDEX,
                ),
                index.len() as u32,
            ))
        };
        let mut floor = PlaneGeometry::build(1000., 1000., 1, 1)?;
        floor.apply_matrix4(Matrix4::from_rotation_x(-PI / 2.))?;
        let meshes = vec![
            mesh(floor)?,
            mesh(SphereGeometry::build(BIG_RADIUS, 64, 32)?)?,
            mesh(SphereGeometry::build(RADIUS, 32, 16)?)?,
        ];
        let colors: Vec<f32> = balls
            .iter()
            .flat_map(|b| b.color.map(|v| v as f32))
            .chain(std::iter::repeat_n(0., (GRID * GRID - balls.len()) * 3))
            .collect();
        let demo = Self {
            controls,
            params: [1., 1., 0., 20.],
            time: 0.,
            lights: texture("clustered lights", MAX_LIGHTS as u32, 2),
            ranges: texture("clustered slices", SLICES, 1),
            light_indexes: None,
            render_uniforms: uniform(r, "clustered render", 208),
            cluster_render: uniform(r, "clusters render", 144),
            cluster_object: uniform(r, "clusters object", 16),
            phong_objects: (0..5).map(|_| uniform(r, "phong object", 176)).collect(),
            sphere_object: uniform(r, "spheres object", 80),
            instances: uniform(r, "sphere matrices", (GRID * GRID * 64) as u64),
            output_render: uniform(r, "output render", 96),
            output_object: uniform(r, "output object", 32),
            fxaa_steps: uniform(r, "fxaa steps", 96),
            fxaa_size: uniform(r, "fxaa size", 16),
            meshes,
            colors: init("sphere colors", bytemuck::cast_slice(&colors), vertex),
            quad_uv: init(
                "quad uv",
                bytemuck::cast_slice(&[0f32, -1., 0., 1., 2., 1.]),
                vertex,
            ),
            sampler: r.device.create_sampler(&wgpu::SamplerDescriptor {
                mag_filter: wgpu::FilterMode::Linear,
                min_filter: wgpu::FilterMode::Linear,
                ..Default::default()
            }),
            balls,
            targets: None,
        };
        let mut steps = Layout::new(96);
        for (i, v) in [1., 1.5, 2., 2., 2., 4.].iter().enumerate() {
            steps = steps.f(i * 16, *v);
        }
        steps.write(r, &demo.fxaa_steps);
        Ok(demo)
    }
    /// animate(): the spheres and their lights follow the wave.
    fn wave(&mut self) {
        if self.params[1] > 0.5 {
            let time = self.time;
            for ball in &mut self.balls {
                ball.y = RADIUS + (0.5 + 0.5 * (time * 1.5 + ball.phase).sin()) * WAVE_HEIGHT;
            }
        }
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, dt: f64, animate: bool) -> Result<()> {
        if animate {
            self.time += dt;
        }
        Ok(())
    }
    pub fn prepare(&mut self, s: &mut Scene, c: Object3D, _r: &Renderer) -> Result<()> {
        self.wave();
        self.controls.frame_update(s, c)
    }
    fn resize(&mut self, r: &Renderer, out: &RenderTarget) -> Result<()> {
        let (width, height) = (out.width, out.height);
        // ClusteredLightsNode.create: tiles over the size fitted to 32 pixels.
        let fit = |v: u32| v.div_ceil(TILE) * TILE;
        let (nx, ny) = (fit(width) / TILE, fit(height) / TILE);
        let clusters = nx * ny * SLICES;
        let grid = |source: &str| {
            source
                .replace("NXY_u", &format!("{}u", nx * ny))
                .replace("NX_u", &format!("{nx}u"))
                .replace("NY_u", &format!("{ny}u"))
                .replace("NXY_", &format!("{}", nx * ny))
                .replace("NX_", &format!("{nx}"))
                .replace("X_STEP", &format!("{:?}", 2. / nx as f64))
                .replace("Y_STEP", &format!("{:?}", 2. / ny as f64))
        };
        let module = |label, source: &str| {
            r.device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some(label),
                source: wgpu::ShaderSource::Wgsl(grid(source).into()),
            })
        };
        let indexes = r.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("light indexes"),
            size: clusters as u64 * CHUNKS * 16,
            usage: wgpu::BufferUsages::STORAGE,
            mapped_at_creation: false,
        });
        let view = |t: &wgpu::Texture| t.create_view(&Default::default());
        let (lights, ranges) = (view(&self.lights), view(&self.ranges));
        let texture = |format, sampled: bool| {
            r.device
                .create_texture(&wgpu::TextureDescriptor {
                    label: Some("clustered target"),
                    size: wgpu::Extent3d {
                        width,
                        height,
                        depth_or_array_layers: 1,
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
                .create_view(&Default::default())
        };
        let (color, depth, toned) = (
            texture(HALF, true),
            texture(DEPTH, false),
            texture(HALF, true),
        );
        let compute = r
            .device
            .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                label: Some("clusters"),
                layout: None,
                module: &module("clusters", wgsl!("clusters")),
                entry_point: Some("main"),
                compilation_options: Default::default(),
                cache: None,
            });
        Layout::new(16)
            .i(0, clusters as i32)
            .write(r, &self.cluster_object);
        let clusters_binds = [
            bind(
                r,
                compute.get_bind_group_layout(0),
                &[(0, self.cluster_render.as_entire_binding())],
            ),
            bind(
                r,
                compute.get_bind_group_layout(1),
                &[
                    (0, indexes.as_entire_binding()),
                    (1, wgpu::BindingResource::TextureView(&ranges)),
                    (2, wgpu::BindingResource::TextureView(&lights)),
                    (3, self.cluster_object.as_entire_binding()),
                ],
            ),
        ];
        let attributes: Vec<[wgpu::VertexAttribute; 1]> = [
            (0, wgpu::VertexFormat::Float32x3),
            (1, wgpu::VertexFormat::Float32x3),
            (2, wgpu::VertexFormat::Float32x3),
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
        let layout = |i: usize, stride, step| wgpu::VertexBufferLayout {
            array_stride: stride,
            step_mode: step,
            attributes: &attributes[i],
        };
        let vertex = wgpu::VertexStepMode::Vertex;
        let pipeline = |label,
                        vs: &str,
                        fs: &str,
                        buffers: &[wgpu::VertexBufferLayout],
                        format,
                        depth: bool| {
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
                        targets: &[Some(format)],
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
        let half_target = wgpu::ColorTargetState::from(HALF);
        let phong_pipeline = pipeline(
            "phong",
            wgsl!("phong_vs"),
            wgsl!("phong_fs"),
            &[layout(0, 12, vertex), layout(1, 12, vertex)],
            half_target.clone(),
            true,
        );
        let phong_render = bind(
            r,
            phong_pipeline.get_bind_group_layout(0),
            &[(0, self.render_uniforms.as_entire_binding())],
        );
        let phong_objects = self
            .phong_objects
            .iter()
            .map(|object| {
                bind(
                    r,
                    phong_pipeline.get_bind_group_layout(1),
                    &[
                        (0, object.as_entire_binding()),
                        (1, indexes.as_entire_binding()),
                        (2, wgpu::BindingResource::TextureView(&lights)),
                    ],
                )
            })
            .collect();
        let spheres_pipeline = pipeline(
            "spheres",
            wgsl!("spheres_vs"),
            wgsl!("spheres_fs"),
            &[
                layout(0, 12, vertex),
                layout(1, 12, vertex),
                layout(2, 12, wgpu::VertexStepMode::Instance),
            ],
            half_target.clone(),
            true,
        );
        let spheres_binds = [
            bind(
                r,
                spheres_pipeline.get_bind_group_layout(0),
                &[(0, self.render_uniforms.as_entire_binding())],
            ),
            bind(
                r,
                spheres_pipeline.get_bind_group_layout(1),
                &[
                    (0, self.sphere_object.as_entire_binding()),
                    (1, indexes.as_entire_binding()),
                    (2, wgpu::BindingResource::TextureView(&lights)),
                    (3, self.instances.as_entire_binding()),
                ],
            ),
        ];
        let sampler = wgpu::BindingResource::Sampler(&self.sampler);
        let output_pipeline = pipeline(
            "render output",
            wgsl!("output_vs"),
            wgsl!("output_fs"),
            &[layout(3, 8, vertex)],
            half_target,
            false,
        );
        let output_binds = [
            bind(
                r,
                output_pipeline.get_bind_group_layout(0),
                &[(0, self.output_render.as_entire_binding())],
            ),
            bind(
                r,
                output_pipeline.get_bind_group_layout(1),
                &[
                    (0, sampler.clone()),
                    (1, wgpu::BindingResource::TextureView(&color)),
                    (2, self.output_object.as_entire_binding()),
                    (3, indexes.as_entire_binding()),
                ],
            ),
        ];
        let fxaa_pipeline = pipeline(
            "fxaa",
            wgsl!("fxaa_vs"),
            wgsl!("fxaa_fs"),
            &[layout(3, 8, vertex)],
            out.options.format.into(),
            false,
        );
        let fxaa_bind = bind(
            r,
            fxaa_pipeline.get_bind_group_layout(0),
            &[
                (0, sampler.clone()),
                (1, wgpu::BindingResource::TextureView(&toned)),
                (2, self.fxaa_steps.as_entire_binding()),
                (3, self.fxaa_size.as_entire_binding()),
            ],
        );
        Layout::new(16)
            .f(0, 1. / width as f64)
            .f(4, 1. / height as f64)
            .write(r, &self.fxaa_size);
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
        self.light_indexes = Some(indexes);
        self.targets = Some(Targets {
            width,
            height,
            format: out.options.format,
            grid: (nx, ny),
            color,
            depth,
            toned,
            screen,
            clusters: (compute, clusters_binds),
            phong: (phong_pipeline, phong_render, phong_objects),
            spheres: (spheres_pipeline, spheres_binds),
            output: (output_pipeline, output_binds),
            fxaa: (fxaa_pipeline, fxaa_bind),
        });
        Ok(())
    }
    /// ClusteredLightsNode.updateLightsTexture: the lights sorted by view z,
    /// then each depth slice's range of lights that may reach it.
    fn lights_texture(&self, r: &Renderer, view: Matrix4, near: f64, far: f64) {
        let count = self.balls.len();
        let intensity = BASE_POWER * self.params[0] / (4. * PI);
        let view_z: Vec<f32> = self
            .balls
            .iter()
            .map(|b| view.transform_point3(Vector3::new(b.x, b.y, b.z)).z as f32)
            .collect();
        let mut order: Vec<usize> = (0..count).collect();
        order.sort_by(|&a, &b| {
            (view_z[a] as f64 - view_z[b] as f64)
                .partial_cmp(&0.)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        let mut data = vec![0f32; MAX_LIGHTS * 8];
        let line = MAX_LIGHTS * 4;
        for (i, &k) in order.iter().enumerate() {
            let b = &self.balls[k];
            data[i * 4..i * 4 + 4].copy_from_slice(&[b.x as f32, b.y as f32, b.z as f32, 9.]);
            data[line + i * 4..line + i * 4 + 4].copy_from_slice(&[
                (b.color[0] * intensity) as f32,
                (b.color[1] * intensity) as f32,
                (b.color[2] * intensity) as f32,
                2.,
            ]);
        }
        r.queue.write_texture(
            self.lights.as_image_copy(),
            bytemuck::cast_slice(&data),
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(MAX_LIGHTS as u32 * 16),
                rows_per_image: Some(2),
            },
            self.lights.size(),
        );
        let mut ranges = vec![0f32; SLICES as usize * 4];
        for z in 0..SLICES as usize {
            let slice_near = -(near * (far / near).powf(z as f64 / SLICES as f64));
            let slice_far = -(near * (far / near).powf((z + 1) as f64 / SLICES as f64));
            let (mut start, mut end) = (count, 0);
            for (i, &k) in order.iter().enumerate() {
                let vz = view_z[k] as f64;
                let radius = 9.;
                if vz + radius >= slice_far && vz - radius <= slice_near {
                    start = start.min(i);
                    end = end.max(i + 1);
                }
            }
            if start >= count {
                (start, end) = (0, 0);
            }
            ranges[z * 4] = start as f32;
            ranges[z * 4 + 1] = end as f32;
        }
        r.queue.write_texture(
            self.ranges.as_image_copy(),
            bytemuck::cast_slice(&ranges),
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(SLICES * 16),
                rows_per_image: Some(1),
            },
            self.ranges.size(),
        );
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
            return Err(Error::Invalid("clustered camera"));
        };
        let (near, far) = (p.near, p.far);
        let (projection, view) = (camera.projection_matrix()?, world.inverse());
        self.lights_texture(r, view, near, far);
        Layout::new(144)
            .f(0, near)
            .f(4, far)
            .m4(16, view)
            .m4(80, projection)
            .write(r, &self.cluster_render);
        Layout::new(208)
            .f(0, near)
            .f(4, far)
            .m4(16, view)
            .m4(80, projection)
            .m4(144, view)
            .write(r, &self.render_uniforms);
        // The floor, then the four big spheres: MeshPhongNodeMaterial with
        // specular 0x111111 and shininess 80.
        let bigs = [(-9., -9.), (9., -9.), (-9., 9.), (9., 9.)];
        let specular = Color::from_hex(0x111111).0.to_array();
        for (i, object) in self.phong_objects.iter().enumerate() {
            let (color, model) = if i == 0 {
                (Color::from_hex(0x2a2a2a), Matrix4::IDENTITY)
            } else {
                let (x, z) = bigs[i - 1];
                (
                    Color::from_hex(0xdddddd),
                    Matrix4::from_translation(Vector3::new(x, BIG_RADIUS, z)),
                )
            };
            Layout::new(176)
                .v3(0, color.0.to_array())
                .f(12, 1.)
                .f(16, 80.)
                .v3(32, specular)
                .v3(48, [0.; 3])
                .f(60, 1.)
                .m4(64, model)
                .m3(128)
                .write(r, object);
        }
        Layout::new(80)
            .f(0, 1.)
            .m4(16, Matrix4::IDENTITY)
            .write(r, &self.sphere_object);
        let matrices: Vec<f32> = self
            .balls
            .iter()
            .flat_map(|b| {
                Matrix4::from_translation(Vector3::new(b.x, b.y, b.z))
                    .to_cols_array()
                    .map(|v| v as f32)
            })
            .collect();
        r.queue
            .write_buffer(&self.instances, 0, bytemuck::cast_slice(&matrices));
        let t = self
            .targets
            .as_ref()
            .ok_or(Error::Invalid("clustered targets"))?;
        Layout::new(96)
            .f(0, near)
            .f(4, far)
            .m4(16, projection.inverse())
            .f(80, 1.)
            .write(r, &self.output_render);
        Layout::new(32)
            .i(0, self.params[3].round() as i32)
            .f(8, t.grid.0 as f64)
            .f(12, t.grid.1 as f64)
            .f(16, self.params[2])
            .write(r, &self.output_object);
        let mut encoder = r.device.create_command_encoder(&Default::default());
        {
            let mut pass = encoder.begin_compute_pass(&Default::default());
            pass.set_pipeline(&t.clusters.0);
            pass.set_bind_group(0, &t.clusters.1[0], &[]);
            pass.set_bind_group(1, &t.clusters.1[1], &[]);
            pass.dispatch_workgroups((t.grid.0 * t.grid.1 * SLICES).div_ceil(64), 1, 1);
        }
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("clustered scene"),
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
                        store: wgpu::StoreOp::Discard,
                    }),
                    stencil_ops: None,
                }),
                ..Default::default()
            });
            pass.set_pipeline(&t.phong.0);
            pass.set_bind_group(0, &t.phong.1, &[]);
            for (i, object) in t.phong.2.iter().enumerate() {
                let m = &self.meshes[if i == 0 { 0 } else { 1 }];
                pass.set_bind_group(1, object, &[]);
                pass.set_vertex_buffer(0, m.0.slice(..));
                pass.set_vertex_buffer(1, m.1.slice(..));
                pass.set_index_buffer(m.2.slice(..), wgpu::IndexFormat::Uint32);
                pass.draw_indexed(0..m.3, 0, 0..1);
            }
            let m = &self.meshes[2];
            pass.set_pipeline(&t.spheres.0);
            pass.set_bind_group(0, &t.spheres.1[0], &[]);
            pass.set_bind_group(1, &t.spheres.1[1], &[]);
            pass.set_vertex_buffer(0, m.0.slice(..));
            pass.set_vertex_buffer(1, m.1.slice(..));
            pass.set_vertex_buffer(2, self.colors.slice(..));
            pass.set_index_buffer(m.2.slice(..), wgpu::IndexFormat::Uint32);
            pass.draw_indexed(0..m.3, 0, 0..self.balls.len() as u32);
        }
        let mut post = |view: &wgpu::TextureView,
                        pipeline: &wgpu::RenderPipeline,
                        binds: &[&wgpu::BindGroup]| {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("clustered post"),
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
            pass.set_vertex_buffer(0, self.quad_uv.slice(..));
            pass.draw(0..3, 0..1);
        };
        post(&t.toned, &t.output.0, &[&t.output.1[0], &t.output.1[1]]);
        post(&t.screen.view, &t.fxaa.0, &[&t.fxaa.1]);
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
    /// light intensity, animate, lights per tile and the z-slice.
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        *self
            .params
            .get_mut(index)
            .ok_or(Error::Invalid("clustered parameter"))? = value as f64;
        Ok(())
    }
    pub fn seek(&mut self, t: f64) {
        self.time = t;
    }
}
