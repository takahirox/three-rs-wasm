//! webgpu_compute_cloth: a 31 × 31 Verlet cloth hanging from six pinned
//! vertices, blown by triNoise3D wind and pushed by a swinging sphere, under
//! royal_esplanade's UltraHDR sky. Each frame advances the simulation in
//! fixed 1/360 s steps ( at most 1/60 s per frame ): per step the spring
//! forces and then the vertex forces run as compute passes on the resident
//! storage buffers. The cloth mesh reads the positions in its vertex stage
//! and renders double-sided with MeshPhysicalNodeMaterial's sheen; the
//! wireframe mode draws the springs as instanced lines and the vertices as
//! instanced sprites. Every stage runs the WGSL three.js r186 generates for
//! the page (in `compute_cloth/`), without its unused subgroup-size builtin.
use super::controls_attributes::{Controls, camera_state};
use super::deferred::{Draw, color, depth_attachment, esplanade, sampled_pipeline, set};
use super::lights_projector::{m3, m4, pack};
use super::pmrem_cube_uv::{Pmrem, bind, raw_pipeline};
use super::retro::{target, uniform};
use crate::{
    Error, Result, camera::*, geometry::*, math::*, render_target::*, renderer::*, scene::*,
};
use std::f64::consts::PI;
use wgpu::util::DeviceExt;

const HALF: wgpu::TextureFormat = wgpu::TextureFormat::Rgba16Float;
const DEPTH: wgpu::TextureFormat = wgpu::TextureFormat::Depth24Plus;
const SEGMENTS: usize = 30;
const SPHERE_RADIUS: f64 = 0.15;
/// The uniform buffers for the steps of one frame ( 1/60 s plus the
/// remainder is at most seven 1/360 s steps ).
const MAX_STEPS: usize = 8;
macro_rules! wgsl {
    ($name:literal) => {
        include_str!(concat!("compute_cloth/", $name, ".wgsl"))
    };
}
const X3: wgpu::VertexFormat = wgpu::VertexFormat::Float32x3;
fn attribute(location: u32, format: wgpu::VertexFormat) -> [wgpu::VertexAttribute; 1] {
    [wgpu::VertexAttribute {
        format,
        offset: 0,
        shader_location: location,
    }]
}
/// The Verlet system as the page builds it: vertex positions, pinned flags
/// and per-vertex spring lists, springs with their vertices and rest lengths,
/// and the vertex ids by column.
struct Verlet {
    positions: Vec<[f64; 3]>,
    fixed: Vec<bool>,
    spring_ids: Vec<Vec<u32>>,
    springs: Vec<[u32; 2]>,
    columns: Vec<Vec<u32>>,
}
fn verlet() -> Verlet {
    let mut v = Verlet {
        positions: vec![],
        fixed: vec![],
        spring_ids: vec![],
        springs: vec![],
        columns: vec![],
    };
    for x in 0..=SEGMENTS {
        let mut column = vec![];
        for y in 0..=SEGMENTS {
            let id = v.positions.len() as u32;
            v.positions.push([
                x as f64 * (1. / SEGMENTS as f64) - 0.5,
                0.5,
                y as f64 * (1. / SEGMENTS as f64),
            ]);
            v.fixed.push(y == 0 && x % 5 == 0);
            v.spring_ids.push(vec![]);
            column.push(id);
        }
        v.columns.push(column);
    }
    let spring = |v: &mut Verlet, a: u32, b: u32| {
        let id = v.springs.len() as u32;
        v.spring_ids[a as usize].push(id);
        v.spring_ids[b as usize].push(id);
        v.springs.push([a, b]);
    };
    for x in 0..=SEGMENTS {
        for y in 0..=SEGMENTS {
            let a = v.columns[x][y];
            if x > 0 {
                let b = v.columns[x - 1][y];
                spring(&mut v, a, b);
            }
            if y > 0 {
                let b = v.columns[x][y - 1];
                spring(&mut v, a, b);
            }
            if x > 0 && y > 0 {
                let b = v.columns[x - 1][y - 1];
                spring(&mut v, a, b);
            }
            if x > 0 && y < SEGMENTS {
                let b = v.columns[x - 1][y + 1];
                spring(&mut v, a, b);
            }
        }
    }
    v
}
struct Targets {
    width: u32,
    height: u32,
    format: wgpu::TextureFormat,
    samples: u32,
    msaa: Option<(wgpu::TextureView, wgpu::TextureView)>,
    color: wgpu::TextureView,
    depth: wgpu::TextureView,
    background: Draw,
    sphere: Draw,
    /// Back faces, then front faces.
    cloth: [Draw; 2],
    lines: Draw,
    sprites: Draw,
    output: Draw,
    screen: RenderTarget,
}
pub(super) struct Demo {
    controls: Controls,
    time: f64,
    /// The timer's last time in milliseconds, the accumulated step time and
    /// the simulation timestamp.
    previous: f64,
    since_step: f64,
    timestamp: f64,
    /// Requested frames not yet rendered, in order.
    pending: Vec<f64>,
    /// stiffness, wireframe, sphere, wind, color, roughness, sheen,
    /// sheenRoughness, sheenColor.
    params: [f64; 9],
    vertex_count: u32,
    spring_count: u32,
    /// positions, forces, params, spring list, spring vertex ids, rest
    /// lengths and spring forces.
    positions: wgpu::Buffer,
    springs_pipeline: wgpu::ComputePipeline,
    springs_group: wgpu::BindGroup,
    springs_object: wgpu::Buffer,
    vertices_pipeline: wgpu::ComputePipeline,
    vertices_render: (wgpu::Buffer, wgpu::BindGroup),
    /// One object buffer per step: the sphere moves between steps.
    vertices_steps: Vec<(wgpu::Buffer, wgpu::BindGroup)>,
    spring_ids: wgpu::Buffer,
    cloth: [wgpu::Buffer; 2],
    cloth_index: (wgpu::Buffer, u32),
    sphere: (wgpu::Buffer, wgpu::Buffer, u32),
    background: (wgpu::Buffer, wgpu::Buffer, wgpu::Buffer, u32),
    line_buffers: [wgpu::Buffer; 2],
    sprite: (wgpu::Buffer, wgpu::Buffer),
    output_quad: wgpu::Buffer,
    pmrem: Pmrem,
    clamp: wgpu::Sampler,
    background_render: wgpu::Buffer,
    background_object: wgpu::Buffer,
    lit_render: wgpu::Buffer,
    sphere_object: wgpu::Buffer,
    cloth_render: wgpu::Buffer,
    cloth_object: wgpu::Buffer,
    lines_render: wgpu::Buffer,
    lines_object: wgpu::Buffer,
    sprites_render: wgpu::Buffer,
    sprites_object: wgpu::Buffer,
    output_render: wgpu::Buffer,
    output_object: wgpu::Buffer,
    sphere_position: Vector3,
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
        n.position = Vector3::new(-1.6, -0.1, -1.6);
        n.quaternion = Quaternion::IDENTITY;
        let mut controls = Controls::new(None, (1., 3.), PI, true);
        controls.set_target(Vector3::new(0., -0.1, 0.));
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
        let v = verlet();
        let init = |label, data: &[u8], usage| {
            r.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some(label),
                    contents: data,
                    usage,
                })
        };
        let storage = wgpu::BufferUsages::STORAGE;
        // vec3 and uvec3 storage elements take 16 bytes.
        let positions: Vec<f32> = v
            .positions
            .iter()
            .flat_map(|p| [p[0] as f32, p[1] as f32, p[2] as f32, 0.])
            .collect();
        let mut list: Vec<u32> = vec![];
        let mut params = vec![];
        for (fixed, ids) in v.fixed.iter().zip(&v.spring_ids) {
            if *fixed {
                params.extend([1u32, 0, 0, 0]);
            } else {
                params.extend([0, ids.len() as u32, list.len() as u32, 0]);
                list.extend(ids);
            }
        }
        let distance = |a: [f64; 3], b: [f64; 3]| {
            ((a[0] - b[0]).powi(2) + (a[1] - b[1]).powi(2) + (a[2] - b[2]).powi(2)).sqrt()
        };
        let rest: Vec<f32> = v
            .springs
            .iter()
            .map(|[a, b]| distance(v.positions[*a as usize], v.positions[*b as usize]) as f32)
            .collect();
        let vertex_count = v.positions.len() as u32;
        let spring_count = v.springs.len() as u32;
        let positions = init(
            "cloth positions",
            bytemuck::cast_slice(&positions),
            storage | wgpu::BufferUsages::COPY_SRC,
        );
        let forces = init(
            "cloth forces",
            &vec![0u8; vertex_count as usize * 16],
            storage,
        );
        let params = init("cloth params", bytemuck::cast_slice(&params), storage);
        let list = init("cloth springs", bytemuck::cast_slice(&list), storage);
        let spring_ids = init(
            "cloth spring ids",
            bytemuck::cast_slice(&v.springs),
            storage,
        );
        let rest = init("cloth rest lengths", bytemuck::cast_slice(&rest), storage);
        // instancedArray( springCount × 3, 'vec3' ).
        let spring_forces = init(
            "cloth spring forces",
            &vec![0u8; spring_count as usize * 3 * 16],
            storage,
        );
        let compute = |label, source: &str| {
            r.device
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
                })
        };
        let springs_pipeline = compute("cloth spring forces", wgsl!("springs"));
        let springs_object = uniform(r, "cloth spring forces", wgsl!("springs"), "objectStruct")?;
        let springs_group = bind(
            r,
            springs_pipeline.get_bind_group_layout(0),
            &[
                (0, positions.as_entire_binding()),
                (1, spring_ids.as_entire_binding()),
                (2, spring_forces.as_entire_binding()),
                (3, rest.as_entire_binding()),
                (4, springs_object.as_entire_binding()),
            ],
        );
        let vertices_pipeline = compute("cloth vertex forces", wgsl!("vertices"));
        let time = uniform(r, "cloth time", wgsl!("vertices"), "renderStruct")?;
        let time_group = bind(
            r,
            vertices_pipeline.get_bind_group_layout(0),
            &[(0, time.as_entire_binding())],
        );
        let vertices_steps = (0..MAX_STEPS)
            .map(|_| -> Result<_> {
                let object = uniform(r, "cloth vertex forces", wgsl!("vertices"), "objectStruct")?;
                let group = bind(
                    r,
                    vertices_pipeline.get_bind_group_layout(1),
                    &[
                        (0, params.as_entire_binding()),
                        (1, positions.as_entire_binding()),
                        (2, forces.as_entire_binding()),
                        (3, object.as_entire_binding()),
                        (4, list.as_entire_binding()),
                        (5, spring_forces.as_entire_binding()),
                        (6, spring_ids.as_entire_binding()),
                    ],
                );
                Ok((object, group))
            })
            .collect::<Result<_>>()?;
        // The cloth mesh: 30 × 30 vertices, each the average of a grid cell's
        // four Verlet vertices ( its position attribute is unused ).
        let cells = SEGMENTS * SEGMENTS;
        let mut vertex_ids = vec![0u32; cells * 4];
        let mut index = vec![];
        let at = |x: usize, y: usize| (y * SEGMENTS + x) as u32;
        for x in 0..SEGMENTS {
            for y in 0..SEGMENTS {
                let i = at(x, y) as usize;
                vertex_ids[i * 4..i * 4 + 4].copy_from_slice(&[
                    v.columns[x][y],
                    v.columns[x + 1][y],
                    v.columns[x][y + 1],
                    v.columns[x + 1][y + 1],
                ]);
                if x > 0 && y > 0 {
                    index.extend([at(x, y), at(x - 1, y), at(x - 1, y - 1)]);
                    index.extend([at(x, y), at(x - 1, y - 1), at(x, y - 1)]);
                }
            }
        }
        let vertex = wgpu::BufferUsages::VERTEX;
        let read = |g: &BufferGeometry, name: &str| -> Result<Vec<f32>> {
            let a = g
                .attributes
                .get(name)
                .ok_or(Error::Invalid("cloth attribute"))?;
            (0..a.count())
                .flat_map(|i| (0..3).map(move |k| (i, k)))
                .map(|(i, k)| a.get_component(i, k).map(|v| v as f32))
                .collect()
        };
        let sphere = IcosahedronGeometry::build(SPHERE_RADIUS * 0.95, 4)?;
        let ball = SphereGeometry::build(1., 32, 32)?;
        let ball_index = ball
            .index
            .clone()
            .ok_or(Error::Invalid("background index"))?;
        let quad = PlaneGeometry::build(0.01, 0.01, 1, 1)?;
        let quad_index = quad.index.clone().ok_or(Error::Invalid("sprite index"))?;
        let object = |label, source| uniform(r, label, source, "objectStruct");
        let render = |label, source| uniform(r, label, source, "renderStruct");
        let mut demo = Self {
            controls,
            time: 0.,
            previous: 0.,
            since_step: 0.,
            timestamp: 0.,
            pending: vec![],
            params: [
                0.2,
                0.,
                1.,
                1.,
                0x204080 as f64,
                1.,
                1.,
                0.5,
                0xffffff as f64,
            ],
            vertex_count,
            spring_count,
            positions,
            springs_pipeline,
            springs_group,
            springs_object,
            vertices_pipeline,
            vertices_render: (time, time_group),
            vertices_steps,
            spring_ids,
            cloth: [
                init("cloth position", &vec![0u8; cells * 12], vertex),
                init(
                    "cloth vertex ids",
                    bytemuck::cast_slice(&vertex_ids),
                    vertex,
                ),
            ],
            cloth_index: (
                init(
                    "cloth index",
                    bytemuck::cast_slice(&index),
                    wgpu::BufferUsages::INDEX,
                ),
                index.len() as u32,
            ),
            sphere: (
                init(
                    "cloth sphere",
                    bytemuck::cast_slice(&read(&sphere, "normal")?),
                    vertex,
                ),
                init(
                    "cloth sphere",
                    bytemuck::cast_slice(&read(&sphere, "position")?),
                    vertex,
                ),
                sphere.attributes.get("position").map_or(0, |a| a.count()) as u32,
            ),
            background: (
                init(
                    "cloth background",
                    bytemuck::cast_slice(&read(&ball, "normal")?),
                    vertex,
                ),
                init(
                    "cloth background",
                    bytemuck::cast_slice(&read(&ball, "position")?),
                    vertex,
                ),
                init(
                    "cloth background",
                    bytemuck::cast_slice(&ball_index),
                    wgpu::BufferUsages::INDEX,
                ),
                ball_index.len() as u32,
            ),
            line_buffers: [
                init("cloth lines", &[0u8; 24], vertex),
                init("cloth lines", bytemuck::cast_slice(&[0u32, 1]), vertex),
            ],
            sprite: (
                init(
                    "cloth sprite",
                    bytemuck::cast_slice(&read(&quad, "position")?),
                    vertex,
                ),
                init(
                    "cloth sprite",
                    bytemuck::cast_slice(&quad_index),
                    wgpu::BufferUsages::INDEX,
                ),
            ),
            output_quad: init(
                "cloth quad",
                bytemuck::cast_slice(&[-1f32, 3., 0., -1., -1., 0., 3., -1., 0.]),
                vertex,
            ),
            pmrem,
            clamp,
            background_render: render("cloth background", wgsl!("background_fs"))?,
            background_object: object("cloth background", wgsl!("background_fs"))?,
            lit_render: render("cloth sphere", wgsl!("sphere_fs"))?,
            sphere_object: object("cloth sphere", wgsl!("sphere_fs"))?,
            cloth_render: render("cloth", wgsl!("cloth_fs"))?,
            cloth_object: object("cloth", wgsl!("cloth_fs"))?,
            lines_render: render("cloth lines", wgsl!("lines_vs"))?,
            lines_object: object("cloth lines", wgsl!("lines_fs"))?,
            sprites_render: render("cloth sprites", wgsl!("sprites_vs"))?,
            sprites_object: object("cloth sprites", wgsl!("sprites_fs"))?,
            output_render: render("cloth output", wgsl!("output_fs"))?,
            output_object: object("cloth output", wgsl!("output_vs"))?,
            sphere_position: Vector3::ZERO,
            targets: None,
        };
        demo.update_sphere();
        Ok(demo)
    }
    /// updateSphere().
    fn update_sphere(&mut self) {
        let t = self.timestamp;
        self.sphere_position = Vector3::new((t * 2.1).sin() * 0.1, 0., (t * 0.8).sin());
    }
    /// timer.update() at `t`: the capped delta advances the fixed steps.
    fn advance(&mut self, t: f64) -> Vec<Vector3> {
        let now = t * 1000.;
        let delta = ((now - self.previous) / 1000.).min(1. / 60.);
        self.previous = now;
        self.time = t;
        let per_step = 1. / 360.;
        self.since_step += delta;
        let mut spheres = vec![];
        while self.since_step >= per_step {
            self.timestamp += per_step;
            self.since_step -= per_step;
            self.update_sphere();
            spheres.push(self.sphere_position);
        }
        spheres
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
    fn resize(&mut self, r: &Renderer, out: &RenderTarget) -> Result<()> {
        let size = (out.width, out.height);
        let samples = if out.options.samples > 1 { 4 } else { 1 };
        let tex = wgpu::BindingResource::TextureView;
        let sampler = wgpu::BindingResource::Sampler;
        let less = Some((wgpu::CompareFunction::LessEqual, true));
        let triangles = (samples, wgpu::PrimitiveTopology::TriangleList);
        let (normal, position) = (attribute(0, X3), attribute(1, X3));
        let mesh_layouts = [
            wgpu::VertexBufferLayout {
                array_stride: 12,
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes: &normal,
            },
            wgpu::VertexBufferLayout {
                array_stride: 12,
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes: &position,
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
        let background = sampled_pipeline(
            r,
            "cloth background",
            (wgsl!("background_vs"), wgsl!("background_fs")),
            &mesh_layouts,
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
        // The sphere's or the cloth's object, the DFG LUT and the PMREM.
        let lit = |cloth: bool| {
            let object = if cloth {
                &self.cloth_object
            } else {
                &self.sphere_object
            };
            vec![
                (0, object.as_entire_binding()),
                (1, sampler(&self.clamp)),
                (2, tex(&r.dfg)),
                (3, sampler(&self.clamp)),
                (4, tex(&self.pmrem.view)),
            ]
        };
        let sphere = sampled_pipeline(
            r,
            "cloth sphere",
            (wgsl!("sphere_vs"), wgsl!("sphere_fs")),
            &mesh_layouts,
            &[HALF],
            less,
            (false, false),
            triangles,
        );
        let sphere_groups = groups(&sphere, &self.lit_render, lit(false));
        let ids = attribute(1, wgpu::VertexFormat::Uint32x4);
        // position ( unused ) and vertexIds.
        let cloth_layouts = [
            wgpu::VertexBufferLayout {
                array_stride: 12,
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes: &normal,
            },
            wgpu::VertexBufferLayout {
                array_stride: 16,
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes: &ids,
            },
        ];
        let cloth = [true, false].map(|cw| {
            let p = sampled_pipeline(
                r,
                "cloth",
                (wgsl!("cloth_vs"), wgsl!("cloth_fs")),
                &cloth_layouts,
                &[HALF],
                less,
                (cw, true),
                triangles,
            );
            let mut object = lit(true);
            object.push((5, self.positions.as_entire_binding()));
            let g = groups(&p, &self.cloth_render, object);
            (p, g)
        });
        let vertex_index = attribute(1, wgpu::VertexFormat::Uint32);
        let line_layouts = [
            wgpu::VertexBufferLayout {
                array_stride: 12,
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes: &normal,
            },
            wgpu::VertexBufferLayout {
                array_stride: 4,
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes: &vertex_index,
            },
        ];
        let lines = sampled_pipeline(
            r,
            "cloth lines",
            (wgsl!("lines_vs"), wgsl!("lines_fs")),
            &line_layouts,
            &[HALF],
            less,
            (false, false),
            (samples, wgpu::PrimitiveTopology::LineStrip),
        );
        let lines_groups = groups(
            &lines,
            &self.lines_render,
            vec![
                (0, self.lines_object.as_entire_binding()),
                (1, self.positions.as_entire_binding()),
                (2, self.spring_ids.as_entire_binding()),
            ],
        );
        let sprites = sampled_pipeline(
            r,
            "cloth sprites",
            (wgsl!("sprites_vs"), wgsl!("sprites_fs")),
            &line_layouts[..1],
            &[HALF],
            less,
            (false, true),
            triangles,
        );
        let sprites_groups = groups(
            &sprites,
            &self.sprites_render,
            vec![
                (0, self.sprites_object.as_entire_binding()),
                (1, self.positions.as_entire_binding()),
            ],
        );
        let color_target = target(r, size, HALF, 1);
        let output = raw_pipeline(
            r,
            "cloth output",
            wgsl!("output_vs"),
            wgsl!("output_fs"),
            &line_layouts[..1],
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
            sphere: (sphere, sphere_groups),
            cloth,
            lines: (lines, lines_groups),
            sprites: (sprites, sprites_groups),
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
        // Only requested frames run animate(): its timer drives the steps.
        for t in std::mem::take(&mut self.pending) {
            let spheres = self.advance(t);
            self.simulate(r, &spheres)?;
        }
        let t = self
            .targets
            .as_ref()
            .ok_or(Error::Invalid("cloth targets"))?;
        self.controls.update(s, c)?;
        s.update()?;
        let (camera, world) = s.camera(c)?;
        let (projection, view) = (camera.projection_matrix()?, world.inverse());
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
        let [
            _,
            wireframe,
            sphere_on,
            _,
            hex,
            roughness,
            sheen,
            sheen_roughness,
            sheen_hex,
        ] = self.params;
        let (wireframe, sphere_on) = (wireframe > 0.5, sphere_on > 0.5);
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
            wgsl!("background_fs"),
            "renderStruct",
            &values,
        )?;
        write(
            &self.background_object,
            wgsl!("background_fs"),
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
        let mut lit_render = camera_values.clone();
        lit_render.push(("cameraWorldMatrix", m4(world)));
        write(
            &self.lit_render,
            wgsl!("sphere_fs"),
            "renderStruct",
            &lit_render,
        )?;
        write(
            &self.cloth_render,
            wgsl!("cloth_fs"),
            "renderStruct",
            &lit_render,
        )?;
        let sphere_model = Matrix4::from_translation(self.sphere_position);
        write(
            &self.sphere_object,
            wgsl!("sphere_fs"),
            "objectStruct",
            &[
                ("nodeUniform0", vec![1.; 3]),
                ("nodeUniform1", vec![1.]),
                ("nodeUniform2", vec![0.]),
                ("nodeUniform3", vec![1.]),
                ("nodeUniform5", identity3.clone()),
                ("nodeUniform6", vec![0.; 3]),
                ("nodeUniform7", vec![1.]),
                ("nodeUniform9", m4(sphere_model)),
                ("nodeUniform10", vec![9.]),
                ("nodeUniform11", m4(Matrix4::IDENTITY)),
                ("nodeUniform13", vec![1. / 1536.]),
                ("nodeUniform14", vec![1. / 2048.]),
                ("nodeUniform16", vec![1.]),
            ],
        )?;
        // MeshPhysicalNodeMaterial: color and sheenColor set from sRGB hex.
        let srgb = |hex: f64| Color::from_hex(hex as u32).0.to_array().to_vec();
        write(
            &self.cloth_object,
            wgsl!("cloth_fs"),
            "objectStruct",
            &[
                ("nodeUniform1", srgb(hex)),
                ("nodeUniform2", vec![0.85]),
                ("nodeUniform3", vec![0.]),
                ("nodeUniform4", vec![roughness]),
                ("nodeUniform5", vec![1.5]),
                ("nodeUniform6", vec![1.; 3]),
                ("nodeUniform7", vec![1.]),
                ("nodeUniform8", srgb(sheen_hex)),
                ("nodeUniform9", vec![sheen]),
                ("nodeUniform10", vec![sheen_roughness]),
                ("nodeUniform11", vec![0.; 3]),
                ("nodeUniform12", vec![1.]),
                ("nodeUniform15", identity3),
                ("nodeUniform16", m4(Matrix4::IDENTITY)),
                ("nodeUniform17", vec![9.]),
                ("nodeUniform18", m4(Matrix4::IDENTITY)),
                ("nodeUniform20", vec![1. / 1536.]),
                ("nodeUniform21", vec![1. / 2048.]),
                ("nodeUniform23", vec![1.]),
            ],
        )?;
        write(
            &self.lines_render,
            wgsl!("lines_vs"),
            "renderStruct",
            &camera_values,
        )?;
        write(
            &self.lines_object,
            wgsl!("lines_fs"),
            "objectStruct",
            &[
                ("nodeUniform2", vec![1.; 3]),
                ("nodeUniform3", vec![1.]),
                ("nodeUniform6", m4(Matrix4::IDENTITY)),
            ],
        )?;
        write(
            &self.sprites_render,
            wgsl!("sprites_vs"),
            "renderStruct",
            &camera_values,
        )?;
        write(
            &self.sprites_object,
            wgsl!("sprites_fs"),
            "objectStruct",
            &[
                ("nodeUniform1", vec![1.; 3]),
                ("nodeUniform2", vec![1.]),
                ("nodeUniform5", m4(Matrix4::IDENTITY)),
                ("nodeUniform6", vec![0.]),
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
                ("nodeUniform2", vec![1.]),
            ],
        )?;
        write(
            &self.output_object,
            wgsl!("output_vs"),
            "objectStruct",
            &[("nodeUniform5", m4(Matrix4::IDENTITY))],
        )?;
        let mut encoder = r.device.create_command_encoder(&Default::default());
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("cloth scene"),
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
            // Opaque objects front to back: the spring lines ( at the origin )
            // and the sphere.
            let screen = projection * view;
            let depth = |p: Vector3| screen.project_point3(p).z;
            let mut opaque = vec![];
            if wireframe {
                opaque.push((depth(Vector3::ZERO), 0));
            }
            if sphere_on
                && Frustum::from_projection(screen).intersects_sphere(Sphere {
                    center: self.sphere_position,
                    radius: SPHERE_RADIUS * 0.95,
                })
            {
                opaque.push((depth(self.sphere_position), 1));
            }
            opaque.sort_by(|a, b| a.0.total_cmp(&b.0));
            for (_, i) in opaque {
                if i == 0 {
                    set(&mut pass, &t.lines);
                    pass.set_vertex_buffer(0, self.line_buffers[0].slice(..));
                    pass.set_vertex_buffer(1, self.line_buffers[1].slice(..));
                    pass.draw(0..2, 0..self.spring_count);
                } else {
                    set(&mut pass, &t.sphere);
                    pass.set_vertex_buffer(0, self.sphere.0.slice(..));
                    pass.set_vertex_buffer(1, self.sphere.1.slice(..));
                    pass.draw(0..self.sphere.2, 0..1);
                }
            }
            if wireframe {
                set(&mut pass, &t.sprites);
                pass.set_vertex_buffer(0, self.sprite.0.slice(..));
                pass.set_index_buffer(self.sprite.1.slice(..), wgpu::IndexFormat::Uint32);
                pass.draw_indexed(0..6, 0, 0..self.vertex_count);
            } else {
                for draw in &t.cloth {
                    set(&mut pass, draw);
                    pass.set_vertex_buffer(0, self.cloth[0].slice(..));
                    pass.set_vertex_buffer(1, self.cloth[1].slice(..));
                    pass.set_index_buffer(self.cloth_index.0.slice(..), wgpu::IndexFormat::Uint32);
                    pass.draw_indexed(0..self.cloth_index.1, 0, 0..1);
                }
            }
        }
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("cloth output"),
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
    /// One animate(): the frame's time and the steps' sphere positions,
    /// then per step the spring forces and the vertex forces.
    fn simulate(&self, r: &Renderer, spheres: &[Vector3]) -> Result<()> {
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
        let [stiffness, _, sphere_on, wind, ..] = self.params;
        let sphere_on = sphere_on > 0.5;
        write(
            &self.springs_object,
            wgsl!("springs"),
            "objectStruct",
            &[
                ("nodeUniform4", vec![stiffness]),
                ("nodeUniform5", vec![self.spring_count as f64]),
            ],
        )?;
        write(
            &self.vertices_render.0,
            wgsl!("vertices"),
            "renderStruct",
            &[("nodeUniform7", vec![self.time])],
        )?;
        for (position, (buffer, _)) in spheres.iter().zip(&self.vertices_steps) {
            write(
                buffer,
                wgsl!("vertices"),
                "objectStruct",
                &[
                    ("nodeUniform3", vec![0.99]),
                    ("nodeUniform8", vec![wind]),
                    ("nodeUniform9", position.to_array().to_vec()),
                    ("nodeUniform10", vec![if sphere_on { 1. } else { 0. }]),
                    ("nodeUniform11", vec![self.vertex_count as f64]),
                ],
            )?;
        }
        let mut encoder = r.device.create_command_encoder(&Default::default());
        // renderer.compute( computeSpringForces ), renderer.compute( computeVertexForces ) per step.
        for (_, group) in self.vertices_steps.iter().take(spheres.len()) {
            {
                let mut pass = encoder.begin_compute_pass(&Default::default());
                pass.set_pipeline(&self.springs_pipeline);
                pass.set_bind_group(0, &self.springs_group, &[]);
                pass.dispatch_workgroups(self.spring_count.div_ceil(64), 1, 1);
            }
            let mut pass = encoder.begin_compute_pass(&Default::default());
            pass.set_pipeline(&self.vertices_pipeline);
            pass.set_bind_group(0, &self.vertices_render.1, &[]);
            pass.set_bind_group(1, group, &[]);
            pass.dispatch_workgroups(self.vertex_count.div_ceil(64), 1, 1);
        }
        r.queue.submit([encoder.finish()]);
        Ok(())
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
    /// stiffness, wireframe, sphere, wind, color, roughness, sheen,
    /// sheenRoughness and sheenColor.
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        *self
            .params
            .get_mut(index)
            .ok_or(Error::Invalid("cloth parameter"))? = value as f64;
        Ok(())
    }
    pub fn seek(&mut self, t: f64) {
        self.pending.push(t);
    }
}
