//! webgpu_lights_clustered: 876 point lights riding on bouncing emissive
//! spheres between four large Phong spheres on a Phong floor, shaded through
//! ClusteredLighting: each frame the lights are sorted by view depth into a
//! float texture with per-slice light ranges (on the CPU, as the addon does),
//! a compute pass assigns up to 64 lights to each 32-pixel tile of 24
//! logarithmic depth slices, and the materials loop over their cluster's
//! lights. The scene pass is followed by renderOutput (NeutralToneMapping,
//! with the optional cluster heat map) and FXAA. Every stage runs the WGSL
//! three.js r187 generates for the page (in `lights_clustered/`): the tile
//! grid is a uniform, so a resize keeps the pipelines, and the light index
//! storage is only replaced when it has to grow.
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
/// The light texture's rows: position and distance, color and decay, spot
/// direction and cone, bounding sphere.
const ROWS: u32 = 4;
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
/// The pipelines, built once for the output format: the shaders do not
/// depend on the drawing-buffer size.
struct Pipelines {
    format: wgpu::TextureFormat,
    clusters: wgpu::ComputePipeline,
    phong: wgpu::RenderPipeline,
    spheres: wgpu::RenderPipeline,
    output: wgpu::RenderPipeline,
    fxaa: wgpu::RenderPipeline,
}
/// Targets and bind groups for one drawing-buffer size.
struct Targets {
    width: u32,
    height: u32,
    format: wgpu::TextureFormat,
    grid: (u32, u32),
    color: wgpu::TextureView,
    depth: wgpu::TextureView,
    toned: wgpu::TextureView,
    screen: RenderTarget,
    clusters: [wgpu::BindGroup; 2],
    phong: (wgpu::BindGroup, Vec<wgpu::BindGroup>),
    spheres: [wgpu::BindGroup; 2],
    output: [wgpu::BindGroup; 2],
    fxaa: wgpu::BindGroup,
}
pub(super) struct Demo {
    controls: Controls,
    balls: Vec<Ball>,
    /// intensity, animate, lights per tile, z-slice.
    params: [f64; 4],
    time: f64,
    lights: wgpu::Texture,
    ranges: wgpu::Texture,
    /// ClusteredLightsNode's lightIndexes: 16 ivec4 chunks per cluster,
    /// allocated with headroom and replaced only when the grid outgrows it.
    light_indexes: Option<wgpu::Buffer>,
    pipelines: Option<Pipelines>,
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
            lights: texture("clustered lights", MAX_LIGHTS as u32, ROWS),
            ranges: texture("clustered slices", SLICES, 1),
            light_indexes: None,
            pipelines: None,
            render_uniforms: uniform(r, "clustered render", 208),
            cluster_render: uniform(r, "clusters render", 144),
            cluster_object: uniform(r, "clusters object", 16),
            phong_objects: (0..5).map(|_| uniform(r, "phong object", 192)).collect(),
            sphere_object: uniform(r, "spheres object", 80),
            // InstancedMesh's instanceMatrix: a per-instance vertex attribute.
            instances: r.device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("sphere matrices"),
                size: (GRID * GRID * 64) as u64,
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            }),
            output_render: uniform(r, "output render", 96),
            output_object: uniform(r, "output object", 16),
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
    /// The pipelines, once per output format.
    fn pipelines(&mut self, r: &Renderer, format: wgpu::TextureFormat) {
        if self.pipelines.as_ref().is_some_and(|p| p.format == format) {
            return;
        }
        let module = |label, source: &str| {
            r.device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some(label),
                source: wgpu::ShaderSource::Wgsl(source.into()),
            })
        };
        let clusters = r
            .device
            .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                label: Some("clusters"),
                layout: None,
                module: &module("clusters", wgsl!("clusters")),
                entry_point: Some("main"),
                compilation_options: Default::default(),
                cache: None,
            });
        let attribute = |shader_location, format, offset| wgpu::VertexAttribute {
            format,
            offset,
            shader_location,
        };
        let float3 = wgpu::VertexFormat::Float32x3;
        let float4 = wgpu::VertexFormat::Float32x4;
        let position = [attribute(0, float3, 0)];
        let normal = [attribute(1, float3, 0)];
        // instanceMatrix's four columns and instanceColor.
        let matrix = [
            attribute(2, float4, 0),
            attribute(3, float4, 16),
            attribute(4, float4, 32),
            attribute(5, float4, 48),
        ];
        let color = [attribute(6, float3, 0)];
        let uv = [attribute(0, wgpu::VertexFormat::Float32x2, 0)];
        let layout = |attributes, stride, step_mode| wgpu::VertexBufferLayout {
            array_stride: stride,
            step_mode,
            attributes,
        };
        let vertex = wgpu::VertexStepMode::Vertex;
        let instance = wgpu::VertexStepMode::Instance;
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
        let phong = pipeline(
            "phong",
            wgsl!("phong_vs"),
            wgsl!("phong_fs"),
            &[layout(&position, 12, vertex), layout(&normal, 12, vertex)],
            half_target.clone(),
            true,
        );
        let spheres = pipeline(
            "spheres",
            wgsl!("spheres_vs"),
            wgsl!("spheres_fs"),
            &[
                layout(&position, 12, vertex),
                layout(&normal, 12, vertex),
                layout(&matrix, 64, instance),
                layout(&color, 12, instance),
            ],
            half_target.clone(),
            true,
        );
        let output = pipeline(
            "render output",
            wgsl!("output_vs"),
            wgsl!("output_fs"),
            &[layout(&uv, 8, vertex)],
            half_target,
            false,
        );
        let fxaa = pipeline(
            "fxaa",
            wgsl!("fxaa_vs"),
            wgsl!("fxaa_fs"),
            &[layout(&uv, 8, vertex)],
            format.into(),
            false,
        );
        self.pipelines = Some(Pipelines {
            format,
            clusters,
            phong,
            spheres,
            output,
            fxaa,
        });
    }
    fn resize(&mut self, r: &Renderer, out: &RenderTarget) -> Result<()> {
        let (width, height) = (out.width, out.height);
        self.pipelines(r, out.options.format);
        let pipelines = self.pipelines.as_ref().unwrap();
        // ClusteredLightsNode._resize: tiles over the size fitted to 32 pixels.
        let fit = |v: u32| v.div_ceil(TILE) * TILE;
        let (nx, ny) = (fit(width) / TILE, fit(height) / TILE);
        let clusters = nx * ny * SLICES;
        // _create: the storage grows geometrically ( 1.5 × the first grid,
        // then doubling ), so most resizes keep it.
        let required = clusters as u64 * CHUNKS * 16;
        let previous = self.light_indexes.as_ref().map_or(0, |b| b.size());
        if previous < required {
            let size = if previous == 0 {
                (clusters as u64 * CHUNKS).div_ceil(2) * 3 * 16
            } else {
                required.max(previous * 2)
            };
            self.light_indexes = Some(r.device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("light indexes"),
                size,
                usage: wgpu::BufferUsages::STORAGE,
                mapped_at_creation: false,
            }));
        }
        let indexes = self.light_indexes.as_ref().unwrap();
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
        // The grid dimensions and the compute count.
        let mut object = Layout::new(16).f(0, nx as f64).f(4, ny as f64);
        object.0[8..12].copy_from_slice(&clusters.to_le_bytes());
        object.write(r, &self.cluster_object);
        let clusters_binds = [
            bind(
                r,
                pipelines.clusters.get_bind_group_layout(0),
                &[(0, self.cluster_render.as_entire_binding())],
            ),
            bind(
                r,
                pipelines.clusters.get_bind_group_layout(1),
                &[
                    (0, wgpu::BindingResource::TextureView(&ranges)),
                    (1, self.cluster_object.as_entire_binding()),
                    (2, wgpu::BindingResource::TextureView(&lights)),
                    (3, indexes.as_entire_binding()),
                ],
            ),
        ];
        let phong_render = bind(
            r,
            pipelines.phong.get_bind_group_layout(0),
            &[(0, self.render_uniforms.as_entire_binding())],
        );
        let phong_objects = self
            .phong_objects
            .iter()
            .map(|object| {
                bind(
                    r,
                    pipelines.phong.get_bind_group_layout(1),
                    &[
                        (0, object.as_entire_binding()),
                        (1, indexes.as_entire_binding()),
                        (2, wgpu::BindingResource::TextureView(&lights)),
                    ],
                )
            })
            .collect();
        let spheres_binds = [
            bind(
                r,
                pipelines.spheres.get_bind_group_layout(0),
                &[(0, self.render_uniforms.as_entire_binding())],
            ),
            bind(
                r,
                pipelines.spheres.get_bind_group_layout(1),
                &[
                    (0, self.sphere_object.as_entire_binding()),
                    (1, indexes.as_entire_binding()),
                    (2, wgpu::BindingResource::TextureView(&lights)),
                ],
            ),
        ];
        let sampler = wgpu::BindingResource::Sampler(&self.sampler);
        let output_binds = [
            bind(
                r,
                pipelines.output.get_bind_group_layout(0),
                &[(0, self.output_render.as_entire_binding())],
            ),
            bind(
                r,
                pipelines.output.get_bind_group_layout(1),
                &[
                    (0, sampler.clone()),
                    (1, wgpu::BindingResource::TextureView(&color)),
                    (2, self.output_object.as_entire_binding()),
                    (3, indexes.as_entire_binding()),
                ],
            ),
        ];
        let fxaa_bind = bind(
            r,
            pipelines.fxaa.get_bind_group_layout(0),
            &[
                (0, sampler.clone()),
                (1, wgpu::BindingResource::TextureView(&toned)),
                (2, self.fxaa_size.as_entire_binding()),
                (3, self.fxaa_steps.as_entire_binding()),
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
        self.targets = Some(Targets {
            width,
            height,
            format: out.options.format,
            grid: (nx, ny),
            color,
            depth,
            toned,
            screen,
            clusters: clusters_binds,
            phong: (phong_render, phong_objects),
            spheres: spheres_binds,
            output: output_binds,
            fxaa: fxaa_bind,
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
        let mut data = vec![0f32; MAX_LIGHTS * 4 * ROWS as usize];
        let line = MAX_LIGHTS * 4;
        for (i, &k) in order.iter().enumerate() {
            let b = &self.balls[k];
            let row = |n: usize| n * line + i * 4..n * line + i * 4 + 4;
            data[row(0)].copy_from_slice(&[b.x as f32, b.y as f32, b.z as f32, 9.]);
            data[row(1)].copy_from_slice(&[
                (b.color[0] * intensity) as f32,
                (b.color[1] * intensity) as f32,
                (b.color[2] * intensity) as f32,
                2.,
            ]);
            // A point light: no spot cone ( attenuation 1 ), bounded by its distance.
            data[row(2)].copy_from_slice(&[0., 0., 0., -1.]);
            data[row(3)].copy_from_slice(&[b.x as f32, b.y as f32, b.z as f32, 9.]);
        }
        r.queue.write_texture(
            self.lights.as_image_copy(),
            bytemuck::cast_slice(&data),
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(MAX_LIGHTS as u32 * 16),
                rows_per_image: Some(ROWS),
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
        let grid = self.targets.as_ref().map_or((1, 1), |t| t.grid);
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
            Layout::new(192)
                .v3(0, color.0.to_array())
                .f(12, 1.)
                .f(16, 80.)
                .v3(32, specular)
                .v3(48, [0.; 3])
                .f(60, 1.)
                .f(64, grid.0 as f64)
                .f(68, grid.1 as f64)
                .m4(80, model)
                .m3(144)
                .write(r, object);
        }
        Layout::new(80)
            .f(0, 1.)
            .f(8, grid.0 as f64)
            .f(12, grid.1 as f64)
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
        let pipelines = self
            .pipelines
            .as_ref()
            .ok_or(Error::Invalid("clustered pipelines"))?;
        Layout::new(96)
            .f(0, near)
            .f(4, far)
            .m4(16, projection.inverse())
            .f(80, 1.)
            .write(r, &self.output_render);
        Layout::new(16)
            .f(0, t.grid.0 as f64)
            .f(4, t.grid.1 as f64)
            .i(8, self.params[3].round() as i32)
            .f(12, self.params[2])
            .write(r, &self.output_object);
        let mut encoder = r.device.create_command_encoder(&Default::default());
        {
            let mut pass = encoder.begin_compute_pass(&Default::default());
            pass.set_pipeline(&pipelines.clusters);
            pass.set_bind_group(0, &t.clusters[0], &[]);
            pass.set_bind_group(1, &t.clusters[1], &[]);
            pass.dispatch_workgroups((t.grid.0 * t.grid.1 * SLICES).div_ceil(64), 1, 1);
        }
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("clustered scene"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &t.color,
                    depth_slice: None,
                    resolve_target: None,
                    // WebGPURenderer( { alpha: true } ): cleared to transparent
                    // black, so the heat map's mix leaves the sky translucent.
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
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
            pass.set_pipeline(&pipelines.phong);
            pass.set_bind_group(0, &t.phong.0, &[]);
            for (i, object) in t.phong.1.iter().enumerate() {
                let m = &self.meshes[if i == 0 { 0 } else { 1 }];
                pass.set_bind_group(1, object, &[]);
                pass.set_vertex_buffer(0, m.0.slice(..));
                pass.set_vertex_buffer(1, m.1.slice(..));
                pass.set_index_buffer(m.2.slice(..), wgpu::IndexFormat::Uint32);
                pass.draw_indexed(0..m.3, 0, 0..1);
            }
            let m = &self.meshes[2];
            pass.set_pipeline(&pipelines.spheres);
            pass.set_bind_group(0, &t.spheres[0], &[]);
            pass.set_bind_group(1, &t.spheres[1], &[]);
            pass.set_vertex_buffer(0, m.0.slice(..));
            pass.set_vertex_buffer(1, m.1.slice(..));
            pass.set_vertex_buffer(2, self.instances.slice(..));
            pass.set_vertex_buffer(3, self.colors.slice(..));
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
        post(&t.toned, &pipelines.output, &[&t.output[0], &t.output[1]]);
        post(&t.screen.view, &pipelines.fxaa, &[&t.fxaa]);
        r.queue.submit([encoder.finish()]);
        Ok(true)
    }
    pub fn output(&self) -> Option<&RenderTarget> {
        self.targets.as_ref().map(|t| &t.screen)
    }
    /// The target `output` returns, to render into the canvas ( see
    /// RenderTarget::alias_color ).
    pub fn output_mut(&mut self) -> Option<&mut RenderTarget> {
        self.targets.as_mut().map(|t| &mut t.screen)
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

#[allow(unused_variables)]
impl super::example::Example for Demo {
    fn on_update(
        &mut self,
        scene: &mut crate::scene::Scene,
        camera: crate::scene::Object3D,
        delta: f64,
        animate: bool,
    ) -> Option<crate::Result<()>> {
        Some(self.update(scene, camera, delta, animate))
    }
    fn on_render(
        &mut self,
        renderer: &crate::renderer::Renderer,
        scene: &mut crate::scene::Scene,
        camera: crate::scene::Object3D,
        target: &crate::renderer::RenderTarget,
    ) -> Option<crate::Result<bool>> {
        Some(self.render(renderer, scene, camera, target))
    }
    fn on_output_target(&self) -> Option<Option<&crate::renderer::RenderTarget>> {
        if let Some(target) = self.output() {
            return Some(Some(target));
        }
        None
    }
    fn on_output_target_mut(&mut self) -> Option<Option<&mut crate::renderer::RenderTarget>> {
        Some(self.output_mut())
    }
    fn on_prepare(
        &mut self,
        renderer: &crate::renderer::Renderer,
        scene: &mut crate::scene::Scene,
        camera: crate::scene::Object3D,
        aspect: f64,
    ) -> crate::Result<()> {
        self.prepare(scene, camera, renderer)?;
        Ok(())
    }
    fn on_tsl_parameter(&mut self, index: usize, value: f32) -> Option<crate::Result<()>> {
        Some(self.parameter(index, value))
    }
    fn on_key(&mut self, code: u32, down: bool) {
        self.key(code, down);
    }
    fn on_draw(&mut self, kind: u32, x: f64, y: f64) -> Option<crate::Result<()>> {
        self.draw(kind, x, y);
        Some(Ok(()))
    }
    fn on_seek(&mut self, seconds: f64) {
        self.seek(seconds);
    }
    fn on_input(
        &mut self,
        scene: &mut crate::scene::Scene,
        camera: crate::scene::Object3D,
        dx: f64,
        dy: f64,
        wheel: f64,
        pan: bool,
        height: f64,
    ) -> Option<crate::Result<()>> {
        Some(self.input(scene, camera, dx, dy, wheel, pan, height))
    }
}

/// The program of three.js examples/webgpu_lights_clustered.html: its renderer, its scene and its
/// frame loop.
#[wasm_bindgen::prelude::wasm_bindgen]
pub async fn webgpu_lights_clustered(
    canvas: web_sys::HtmlCanvasElement,
    animate: bool,
) -> std::result::Result<super::BrowserApp, wasm_bindgen::JsValue> {
    super::program(
        canvas,
        animate,
        super::PageOptions {
            samples: 1,
            encode_srgb: true,
            format: super::TargetFormat::Canvas,
            raw_output: true,
            ..super::PageOptions::GALLERY
        },
        async |page| {
            let (scene, camera, renderer, _aspect) = page.gallery_scene()?;
            Ok(super::expanded::module(
                super::gltf_viewer::OrbitViewer::from_camera(crate::math::Vector3::ZERO, 1.),
                1.,
                200.,
                Box::new(Demo::create(scene, camera, 368, renderer).await?),
            ))
        },
    )
    .await
}
