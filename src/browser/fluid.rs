//! webgpu_compute_particles_fluid: an MLS-MPM fluid of 32,768 particles in
//! a 64³ grid, rendered as instanced icosahedra under royal_esplanade's
//! UltraHDR sky. Each frame runs the page's compute kernels on the resident
//! storage buffers: the workgroup kernel writes the indirect dispatch sizes,
//! the grid clears, particle-to-grid scatters mass and momentum as
//! fixed-point atomics ( two passes ), the grid update divides by mass, and
//! grid-to-particle moves the particles inside the rounded box, pushed by
//! the pointer ray. The particles read their positions in the vertex stage.
//! Every stage runs the WGSL three.js r186 generates for the page (in
//! `fluid/`), without its unused subgroup-size builtin.
use super::controls_attributes::{Controls, camera_state};
use super::deferred::{Draw, color, depth_attachment, esplanade, sampled_pipeline, set};
use super::lights_projector::{m3, m4, pack};
use super::models_modifiers::merge_vertices;
use super::pmrem_cube_uv::{Pmrem, bind, raw_pipeline};
use super::retro::{target, uniform};
use crate::{
    Error, Result, camera::*, geometry::*, math::*, render_target::*, renderer::*, scene::*,
};
use std::f64::consts::PI;
use wgpu::util::DeviceExt;

const HALF: wgpu::TextureFormat = wgpu::TextureFormat::Rgba16Float;
const DEPTH: wgpu::TextureFormat = wgpu::TextureFormat::Depth24Plus;
const MAX_PARTICLES: usize = 8192 * 16;
const GRID: u32 = 64;
const CELLS: u32 = GRID * GRID * GRID;
macro_rules! wgsl {
    ($name:literal) => {
        include_str!(concat!("fluid/", $name, ".wgsl"))
    };
}
macro_rules! cloth {
    ($name:literal) => {
        include_str!(concat!("compute_cloth/", $name, ".wgsl"))
    };
}
const X3_0: [wgpu::VertexAttribute; 1] = wgpu::vertex_attr_array![0 => Float32x3];
const X3_1: [wgpu::VertexAttribute; 1] = wgpu::vertex_attr_array![1 => Float32x3];
struct Targets {
    width: u32,
    height: u32,
    format: wgpu::TextureFormat,
    samples: u32,
    msaa: Option<(wgpu::TextureView, wgpu::TextureView)>,
    color: wgpu::TextureView,
    depth: wgpu::TextureView,
    background: Draw,
    particles: Draw,
    output: Draw,
    screen: RenderTarget,
}
/// A compute kernel and its bind group.
struct Kernel {
    pipeline: wgpu::ComputePipeline,
    group: wgpu::BindGroup,
    object: wgpu::Buffer,
}
pub(super) struct Demo {
    controls: Controls,
    time: f64,
    /// The timer's last time in milliseconds.
    previous: f64,
    pending: Vec<f64>,
    particle_count: u32,
    /// The mouse ray ( origin shifted by 0.5 in x and z ), the plane hit and the last hit.
    ray: (Vector3, Vector3),
    mouse: Vector3,
    previous_mouse: Vector3,
    particles: wgpu::Buffer,
    indirect: [wgpu::Buffer; 3],
    workgroups: Kernel,
    clear: Kernel,
    p2g1: Kernel,
    p2g2: Kernel,
    update_grid: Kernel,
    g2p: Kernel,
    mesh: (wgpu::Buffer, wgpu::Buffer, wgpu::Buffer, u32),
    background: (wgpu::Buffer, wgpu::Buffer, wgpu::Buffer, u32),
    output_quad: wgpu::Buffer,
    pmrem: Pmrem,
    clamp: wgpu::Sampler,
    background_render: wgpu::Buffer,
    background_object: wgpu::Buffer,
    particle_render: wgpu::Buffer,
    particle_object: wgpu::Buffer,
    output_render: wgpu::Buffer,
    output_object: wgpu::Buffer,
    targets: Option<Targets>,
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, _id: u32, r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 40.,
            near: 0.01,
            far: 10.,
            aspect,
            ..Default::default()
        }));
        let n = s.get_mut(c)?;
        n.position = Vector3::new(-1.3, 1.3, -1.3);
        n.quaternion = Quaternion::IDENTITY;
        let mut controls = Controls::new(None, (1., 3.), PI * 0.35, true);
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
        let (_, _, pmrem, _) = esplanade(r, &linear, &clamp).await?;
        let init = |label, data: &[u8], usage| {
            r.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some(label),
                    contents: data,
                    usage,
                })
        };
        // setupBuffers(): positions from the fixture's seeded Math.random, in
        // the page's order ( x, y, z per particle ); 20 floats per particle.
        let mut seed = 186u32;
        let mut random = || {
            seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
            seed as f64 / 4294967296.
        };
        let mut particles = vec![0f32; MAX_PARTICLES * 20];
        for i in 0..MAX_PARTICLES {
            for k in 0..3 {
                particles[i * 20 + k] = (random() * 0.8 + 0.1) as f32;
            }
        }
        let storage = wgpu::BufferUsages::STORAGE;
        let particles = init(
            "fluid particles",
            bytemuck::cast_slice(&particles),
            storage | wgpu::BufferUsages::COPY_SRC,
        );
        let cells = init("fluid cells", &vec![0u8; CELLS as usize * 16], storage);
        let cells_float = init(
            "fluid cell velocities",
            &vec![0u8; CELLS as usize * 16],
            storage,
        );
        let particle_count: u32 = 8192 * 4;
        let groups = particle_count.div_ceil(64);
        let indirect = [0; 3].map(|_| {
            init(
                "fluid indirect",
                bytemuck::cast_slice(&[groups, 1, 1]),
                storage | wgpu::BufferUsages::INDIRECT,
            )
        });
        // A kernel over storage buffers by binding, with its object uniforms at `object`.
        let kernel = |label,
                      source: &str,
                      object: u32,
                      buffers: &[(u32, &wgpu::Buffer)]|
         -> Result<Kernel> {
            let pipeline = r
                .device
                .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                    label: Some(label),
                    layout: None,
                    module: &r.device.create_shader_module(wgpu::ShaderModuleDescriptor {
                        label: Some(label),
                        source: wgpu::ShaderSource::Wgsl(source.into()),
                    }),
                    entry_point: Some("main"),
                    compilation_options: Default::default(),
                    cache: None,
                });
            let uniforms = uniform(r, label, source, "objectStruct")?;
            let mut entries: Vec<(u32, wgpu::BindingResource)> = buffers
                .iter()
                .map(|(i, b)| (*i, b.as_entire_binding()))
                .collect();
            entries.push((object, uniforms.as_entire_binding()));
            let group = bind(r, pipeline.get_bind_group_layout(0), &entries);
            Ok(Kernel {
                pipeline,
                group,
                object: uniforms,
            })
        };
        let workgroups = kernel(
            "fluid workgroups",
            wgsl!("workgroups"),
            1,
            &[(0, &indirect[0]), (2, &indirect[1]), (3, &indirect[2])],
        )?;
        let clear = kernel("fluid clear grid", wgsl!("clear_grid"), 1, &[(0, &cells)])?;
        let p2g1 = kernel(
            "fluid p2g1",
            wgsl!("p2g1"),
            1,
            &[(0, &particles), (2, &cells)],
        )?;
        let p2g2 = kernel(
            "fluid p2g2",
            wgsl!("p2g2"),
            1,
            &[(0, &particles), (2, &cells)],
        )?;
        let update_grid = kernel(
            "fluid update grid",
            wgsl!("update_grid"),
            2,
            &[(0, &cells), (1, &cells_float)],
        )?;
        let g2p = kernel(
            "fluid g2p",
            wgsl!("g2p"),
            1,
            &[(0, &particles), (2, &cells_float)],
        )?;
        // mergeVertices( IcosahedronGeometry( 0.008, 1 ) without uv ).
        let ico = IcosahedronGeometry::build(0.008, 1)?;
        let read = |g: &BufferGeometry, name: &str| -> Result<Vec<f32>> {
            let a = g
                .attributes
                .get(name)
                .ok_or(Error::Invalid("fluid attribute"))?;
            (0..a.count())
                .flat_map(|i| (0..3).map(move |k| (i, k)))
                .map(|(i, k)| a.get_component(i, k).map(|v| v as f32))
                .collect()
        };
        let (positions, normals, _, index) =
            merge_vertices(&read(&ico, "position")?, &read(&ico, "normal")?, &[]);
        let vertex = wgpu::BufferUsages::VERTEX;
        let ball = SphereGeometry::build(1., 32, 32)?;
        let ball_index = ball
            .index
            .clone()
            .ok_or(Error::Invalid("background index"))?;
        Ok(Self {
            controls,
            time: 0.,
            previous: 0.,
            pending: vec![],
            particle_count,
            ray: (Vector3::ZERO, Vector3::ZERO),
            mouse: Vector3::ZERO,
            previous_mouse: Vector3::ZERO,
            particles,
            indirect,
            workgroups,
            clear,
            p2g1,
            p2g2,
            update_grid,
            g2p,
            mesh: (
                init("fluid particle", bytemuck::cast_slice(&positions), vertex),
                init("fluid particle", bytemuck::cast_slice(&normals), vertex),
                init(
                    "fluid particle",
                    bytemuck::cast_slice(&index),
                    wgpu::BufferUsages::INDEX,
                ),
                index.len() as u32,
            ),
            background: (
                init(
                    "fluid background",
                    bytemuck::cast_slice(&read(&ball, "normal")?),
                    vertex,
                ),
                init(
                    "fluid background",
                    bytemuck::cast_slice(&read(&ball, "position")?),
                    vertex,
                ),
                init(
                    "fluid background",
                    bytemuck::cast_slice(&ball_index),
                    wgpu::BufferUsages::INDEX,
                ),
                ball_index.len() as u32,
            ),
            output_quad: init(
                "fluid quad",
                bytemuck::cast_slice(&[-1f32, 3., 0., -1., -1., 0., 3., -1., 0.]),
                vertex,
            ),
            pmrem,
            clamp,
            background_render: uniform(
                r,
                "fluid background",
                cloth!("background_fs"),
                "renderStruct",
            )?,
            background_object: uniform(
                r,
                "fluid background",
                cloth!("background_fs"),
                "objectStruct",
            )?,
            particle_render: uniform(r, "fluid particle", wgsl!("particle_fs"), "renderStruct")?,
            particle_object: uniform(r, "fluid particle", wgsl!("particle_fs"), "objectStruct")?,
            output_render: uniform(r, "fluid output", wgsl!("output_fs"), "renderStruct")?,
            output_object: uniform(r, "fluid output", cloth!("output_vs"), "objectStruct")?,
            targets: None,
        })
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, dt: f64, animate: bool) -> Result<()> {
        if animate {
            self.pending.push(self.time + dt);
        }
        Ok(())
    }
    pub fn prepare(&mut self, _s: &mut Scene, _c: Object3D, _r: &Renderer) -> Result<()> {
        Ok(())
    }
    /// render(): the timer's delta clamped to [ 0.00001, 1/60 ], the mouse
    /// force, then the kernels.
    fn simulate(&mut self, r: &Renderer, t: f64) -> Result<()> {
        let now = t * 1000.;
        let dt = ((now - self.previous) / 1000.).clamp(0.00001, 1. / 60.);
        self.previous = now;
        self.time = t;
        let mut force = (self.mouse - self.previous_mouse) * 2.;
        let length = force.length();
        if length > 0.3 {
            force *= 0.3 / length;
        }
        self.previous_mouse = self.mouse;
        let write =
            |buffer: &wgpu::Buffer, source: &str, values: &[(&str, Vec<f64>)]| -> Result<()> {
                let values: Vec<(&str, &[f64])> =
                    values.iter().map(|(n, v)| (*n, &v[..])).collect();
                r.queue
                    .write_buffer(buffer, 0, &pack(source, "objectStruct", &values)?);
                Ok(())
            };
        let count = vec![self.particle_count as f64];
        let grid = vec![GRID as f64; 3];
        write(
            &self.workgroups.object,
            wgsl!("workgroups"),
            &[("nodeUniform1", count.clone()), ("nodeUniform4", vec![1.])],
        )?;
        write(
            &self.clear.object,
            wgsl!("clear_grid"),
            &[("nodeUniform1", vec![CELLS as f64])],
        )?;
        write(
            &self.p2g1.object,
            wgsl!("p2g1"),
            &[
                ("nodeUniform1", grid.clone()),
                ("nodeUniform3", count.clone()),
            ],
        )?;
        write(
            &self.p2g2.object,
            wgsl!("p2g2"),
            &[
                ("nodeUniform1", grid.clone()),
                ("nodeUniform3", vec![1.5]),
                ("nodeUniform4", vec![50.]),
                ("nodeUniform5", vec![0.1]),
                ("nodeUniform6", vec![dt]),
                ("nodeUniform7", count.clone()),
            ],
        )?;
        write(
            &self.update_grid.object,
            wgsl!("update_grid"),
            &[("nodeUniform2", vec![CELLS as f64])],
        )?;
        write(
            &self.g2p.object,
            wgsl!("g2p"),
            &[
                ("nodeUniform1", grid),
                ("nodeUniform3", vec![0., -(9.81 * 9.81), 0.]),
                ("nodeUniform4", vec![dt]),
                // The mouse force, the ray direction and its origin.
                ("nodeUniform5", force.to_array().to_vec()),
                ("nodeUniform6", self.ray.1.to_array().to_vec()),
                ("nodeUniform7", self.ray.0.to_array().to_vec()),
                ("nodeUniform8", count),
            ],
        )?;
        let mut encoder = r.device.create_command_encoder(&Default::default());
        let dispatch = |encoder: &mut wgpu::CommandEncoder,
                        k: &Kernel,
                        groups: Option<u32>,
                        indirect: Option<&wgpu::Buffer>| {
            let mut pass = encoder.begin_compute_pass(&Default::default());
            pass.set_pipeline(&k.pipeline);
            pass.set_bind_group(0, &k.group, &[]);
            match indirect {
                Some(buffer) => pass.dispatch_workgroups_indirect(buffer, 0),
                None => pass.dispatch_workgroups(groups.unwrap_or(1), 1, 1),
            }
        };
        dispatch(&mut encoder, &self.workgroups, Some(1), None);
        dispatch(&mut encoder, &self.clear, Some(CELLS.div_ceil(64)), None);
        dispatch(&mut encoder, &self.p2g1, None, Some(&self.indirect[0]));
        dispatch(&mut encoder, &self.p2g2, None, Some(&self.indirect[1]));
        dispatch(
            &mut encoder,
            &self.update_grid,
            Some(CELLS.div_ceil(64)),
            None,
        );
        dispatch(&mut encoder, &self.g2p, None, Some(&self.indirect[2]));
        r.queue.submit([encoder.finish()]);
        Ok(())
    }
    fn resize(&mut self, r: &Renderer, out: &RenderTarget) -> Result<()> {
        let size = (out.width, out.height);
        let samples = if out.options.samples > 1 { 4 } else { 1 };
        let tex = wgpu::BindingResource::TextureView;
        let sampler = wgpu::BindingResource::Sampler;
        let layouts = [
            wgpu::VertexBufferLayout {
                array_stride: 12,
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes: &X3_0,
            },
            wgpu::VertexBufferLayout {
                array_stride: 12,
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes: &X3_1,
            },
        ];
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
        let triangles = (samples, wgpu::PrimitiveTopology::TriangleList);
        let background = sampled_pipeline(
            r,
            "fluid background",
            (cloth!("background_vs"), cloth!("background_fs")),
            &layouts,
            &[HALF],
            Some((wgpu::CompareFunction::Always, false)),
            (true, false),
            triangles,
        );
        let background_groups = groups(
            &background,
            &self.background_render,
            vec![
                (0, self.background_object.as_entire_binding()),
                (1, sampler(&self.clamp)),
                (2, tex(&self.pmrem.view)),
            ],
        );
        let particles = sampled_pipeline(
            r,
            "fluid particles",
            (wgsl!("particle_vs"), wgsl!("particle_fs")),
            &layouts,
            &[HALF],
            Some((wgpu::CompareFunction::LessEqual, true)),
            (false, false),
            triangles,
        );
        let particle_groups = groups(
            &particles,
            &self.particle_render,
            vec![
                (0, self.particle_object.as_entire_binding()),
                (1, sampler(&self.clamp)),
                (2, tex(&r.dfg)),
                (3, sampler(&self.clamp)),
                (4, tex(&self.pmrem.view)),
                (5, self.particles.as_entire_binding()),
            ],
        );
        let color_target = target(r, size, HALF, 1);
        let output = raw_pipeline(
            r,
            "fluid output",
            cloth!("output_vs"),
            wgsl!("output_fs"),
            &layouts[..1],
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
                    (1, tex(&color_target)),
                    (2, self.output_object.as_entire_binding()),
                ],
            ),
        ];
        self.targets = Some(Targets {
            width: size.0,
            height: size.1,
            format: out.options.format,
            samples,
            msaa: (samples > 1).then(|| {
                (
                    target(r, size, HALF, samples),
                    target(r, size, DEPTH, samples),
                )
            }),
            color: color_target,
            depth: target(r, size, DEPTH, 1),
            background: (background, background_groups),
            particles: (particles, particle_groups),
            output: (output, output_groups),
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
            t.width != out.width
                || t.height != out.height
                || t.format != out.options.format
                || t.samples != if out.options.samples > 1 { 4 } else { 1 }
        }) {
            self.resize(r, out)?;
        }
        // Each requested frame runs render() once: the kernels, then the draw.
        for t in std::mem::take(&mut self.pending) {
            self.simulate(r, t)?;
        }
        self.controls.update(s, c)?;
        s.update()?;
        let (camera, world) = s.camera(c)?;
        let (projection, view) = (camera.projection_matrix()?, world.inverse());
        let t = self
            .targets
            .as_ref()
            .ok_or(Error::Invalid("fluid targets"))?;
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
        let camera_values = vec![
            ("cameraProjectionMatrix", m4(projection)),
            ("cameraViewMatrix", m4(view)),
        ];
        let identity3 = m3(Matrix4::IDENTITY);
        let mut values = camera_values.clone();
        values.extend([
            ("nodeUniform0", vec![0.5]),
            ("nodeUniform9", vec![1.]),
            ("nodeUniform3", m4(Matrix4::IDENTITY)),
        ]);
        write(
            &self.background_render,
            cloth!("background_fs"),
            "renderStruct",
            &values,
        )?;
        write(
            &self.background_object,
            cloth!("background_fs"),
            "objectStruct",
            &[
                ("nodeUniform1", vec![9.]),
                ("nodeUniform2", m4(Matrix4::IDENTITY)),
                ("nodeUniform5", identity3.clone()),
                ("nodeUniform6", vec![1. / 1536.]),
                ("nodeUniform7", vec![1. / 2048.]),
                ("nodeUniform10", vec![1.]),
                ("nodeUniform12", m4(Matrix4::IDENTITY)),
            ],
        )?;
        let mut values = camera_values;
        values.push(("cameraWorldMatrix", m4(world)));
        write(
            &self.particle_render,
            wgsl!("particle_fs"),
            "renderStruct",
            &values,
        )?;
        write(
            &self.particle_object,
            wgsl!("particle_fs"),
            "objectStruct",
            &[
                (
                    "nodeUniform1",
                    Color::from_hex(0x0066ff).0.to_array().to_vec(),
                ),
                ("nodeUniform2", vec![1.]),
                ("nodeUniform3", vec![0.]),
                ("nodeUniform4", vec![1.]),
                ("nodeUniform6", identity3),
                ("nodeUniform7", vec![0.; 3]),
                ("nodeUniform8", vec![1.]),
                (
                    "nodeUniform10",
                    m4(Matrix4::from_translation(Vector3::new(-0.5, 0., -0.5))),
                ),
                ("nodeUniform11", vec![9.]),
                ("nodeUniform12", m4(Matrix4::IDENTITY)),
                ("nodeUniform14", vec![1. / 1536.]),
                ("nodeUniform15", vec![1. / 2048.]),
                ("nodeUniform17", vec![1.]),
            ],
        )?;
        write(
            &self.output_render,
            wgsl!("output_fs"),
            "renderStruct",
            &[
                (
                    "cameraProjectionMatrix",
                    vec![
                        1., 0., 0., 0., 0., 1., 0., 0., 0., 0., -1., 0., 0., 0., 0., 1.,
                    ],
                ),
                ("cameraViewMatrix", m4(Matrix4::IDENTITY)),
                ("nodeUniform1", vec![t.width as f64, t.height as f64]),
                ("nodeUniform2", vec![1.35]),
            ],
        )?;
        write(
            &self.output_object,
            cloth!("output_vs"),
            "objectStruct",
            &[("nodeUniform5", m4(Matrix4::IDENTITY))],
        )?;
        let mut encoder = r.device.create_command_encoder(&Default::default());
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("fluid scene"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: t.msaa.as_ref().map_or(&t.color, |m| &m.0),
                    depth_slice: None,
                    resolve_target: t.msaa.as_ref().map(|_| &t.color),
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: depth_attachment(
                    t.msaa.as_ref().map_or(&t.depth, |m| &m.1),
                    wgpu::LoadOp::Clear(1.),
                ),
                ..Default::default()
            });
            set(&mut pass, &t.background);
            pass.set_vertex_buffer(0, self.background.0.slice(..));
            pass.set_vertex_buffer(1, self.background.1.slice(..));
            pass.set_index_buffer(self.background.2.slice(..), wgpu::IndexFormat::Uint32);
            pass.draw_indexed(0..self.background.3, 0, 0..1);
            // frustumCulled = false: every particle instance draws.
            set(&mut pass, &t.particles);
            pass.set_vertex_buffer(0, self.mesh.0.slice(..));
            pass.set_vertex_buffer(1, self.mesh.1.slice(..));
            pass.set_index_buffer(self.mesh.2.slice(..), wgpu::IndexFormat::Uint32);
            pass.draw_indexed(0..self.mesh.3, 0, 0..self.particle_count);
        }
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("fluid output"),
                color_attachments: &[color(&t.screen.view, wgpu::Color::BLACK)],
                ..Default::default()
            });
            set(&mut pass, &t.output);
            pass.set_vertex_buffer(0, self.output_quad.slice(..));
            pass.draw(0..3, 0..1);
        }
        r.queue.submit([encoder.finish()]);
        Ok(true)
    }
    pub fn output(&self) -> Option<&RenderTarget> {
        self.targets.as_ref().map(|t| &t.screen)
    }
    /// pointermove: the camera ray through the pointer, shifted by 0.5 in x
    /// and z ( the particles' offset ), and its hit on the y = 0 plane.
    pub fn gpu_pointer(&mut self, s: &Scene, c: Object3D, x: f64, y: f64) -> Result<bool> {
        let (camera, world) = s.camera(c)?;
        let projection = camera.projection_matrix()?;
        let origin = world.transform_point3(Vector3::ZERO);
        let point = (world * projection.inverse()).project_point3(Vector3::new(x, y, 0.5));
        let direction = (point - origin).normalize();
        let origin = origin + Vector3::new(0.5, 0., 0.5);
        self.ray = (origin, direction);
        if direction.y != 0. {
            let distance = -origin.y / direction.y;
            if distance >= 0. {
                self.mouse = origin + direction * distance;
            }
        }
        Ok(false)
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
    /// particleCount: the instances and the kernels' counts.
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        match index {
            0 => self.particle_count = (value as u32).clamp(4096, MAX_PARTICLES as u32),
            _ => return Err(Error::Invalid("fluid parameter")),
        }
        Ok(())
    }
    pub fn seek(&mut self, t: f64) {
        self.pending.push(t);
    }
}
