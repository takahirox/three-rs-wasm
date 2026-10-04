//! webgpu_compute_reduce: the page's six reductions of 262,144 ones, one per
//! half of the view as its two renderers are: the N/2 pass chain, the naive
//! accumulation, the workgroup reduction, the subgroup reduction and the
//! optimized subgroup reduction over the vectorized buffer, and the
//! incorrect baseline. Every second each half's stepAnimation runs its
//! algorithm, then validates ( the plane turns green or red on whether
//! element 0 holds the element count ), then resets its buffers. Each half
//! shows one of the four display modes ( the input as a 512 × 512 grid, its
//! power-of-two elements, element 0 and the workgroup sums ). Every compute
//! and render stage runs the WGSL three.js r186 generates for the page ( in
//! `compute_reduce/`; the output pass is the compute_rasterizer one ), with
//! the device's subgroups feature.
use super::deferred::{Draw, culled_pipeline, set};
use super::lights_projector::pack;
use super::pmrem_cube_uv::{bind, raw_pipeline};
use crate::{Error, Result, math::*, render_target::*, renderer::*, scene::*};
use std::sync::{Arc, Mutex};
use wgpu::util::DeviceExt;

const HALF: wgpu::TextureFormat = wgpu::TextureFormat::Rgba16Float;
const DEPTH: wgpu::TextureFormat = wgpu::TextureFormat::Depth24Plus;
const SIZE: u32 = 262144;
/// The device's maxComputeWorkgroupSizeX ( the default limits ).
const MAX_WORKGROUP: u32 = 256;
macro_rules! wgsl {
    ($name:literal) => {
        include_str!(concat!("compute_reduce/", $name, ".wgsl"))
    };
}
const OUTPUT_VS: &str = include_str!("compute_rasterizer/8.wgsl");
const OUTPUT_FS: &str = include_str!("compute_rasterizer/9.wgsl");
/// The algorithms in the GUI's order.
pub(super) const ALGORITHMS: [&str; 6] = [
    "Reduce 0 (N/2)",
    "Reduce 1 (Naive Accumulate)",
    "Reduce 2 (Workgroup Reduction)",
    "Reduce 3 (Subgroup Reduce)",
    "Reduce 4 (Subgroup Optimized)",
    "Incorrect Baseline",
];
/// One compute call: its module, the workgroups three dispatches for its
/// count ( ceil( count / workgroupSize ) ) and its object struct.
struct Kernel {
    source: &'static str,
    groups: u32,
    object: Vec<(&'static str, Vec<f64>)>,
}
fn kernel(
    source: &'static str,
    count: u32,
    workgroup: u32,
    object: &[(&'static str, f64)],
) -> Kernel {
    Kernel {
        source,
        groups: count.div_ceil(workgroup),
        object: object.iter().map(|(n, v)| (*n, vec![*v])).collect(),
    }
}
/// A side's calls per algorithm, its two reset calls, and its display
/// modes' stages.
struct Sources {
    algorithms: Vec<Vec<Kernel>>,
    reset: [Kernel; 2],
    modes: [(&'static str, &'static str); 4],
}
fn sources(left: bool) -> Sources {
    macro_rules! side {
        ($l:literal, $r:literal) => {
            if left { wgsl!($l) } else { wgsl!($r) }
        };
    }
    let w = MAX_WORKGROUP;
    // stepAnimation's N/2 chain: m = size / 2, halved per call.
    let mut reduce0 = vec![];
    let mut m = SIZE / 2;
    while m >= 1 {
        reduce0.push(kernel(
            side!("reduce0_l", "reduce0_r"),
            m,
            w,
            &[("nodeUniform1", m as f64), ("nodeUniform2", m as f64)],
        ));
        m /= 2;
    }
    let count = |n: u32| [("nodeUniform1", n as f64)];
    Sources {
        algorithms: vec![
            reduce0,
            vec![
                kernel(side!("reduce1a_l", "reduce1a_r"), w * w, w, &count(w * w)),
                kernel(side!("reduce1b_l", "reduce1b_r"), w, w, &count(w)),
                kernel(side!("reduce1c_l", "reduce1c_r"), 1, 1, &count(1)),
            ],
            vec![
                kernel(side!("reduce2a_l", "reduce2a_r"), w * w, w, &[]),
                kernel(side!("reduce2b_l", "reduce2b_r"), w, w, &[]),
            ],
            vec![
                kernel(side!("reduce3a_l", "reduce3a_r"), w * 128, w, &[]),
                kernel(side!("reduce3b_l", "reduce3b_r"), 32, 32, &[]),
            ],
            vec![
                // numInvocations: divRoundUp( size, w × 4 × 4 ) workgroups.
                kernel(
                    side!("reduce4a_l", "reduce4a_r"),
                    SIZE.div_ceil(w * 16) * w,
                    w,
                    &[],
                ),
                kernel(side!("reduce4b_l", "reduce4b_r"), 32, 32, &[]),
            ],
            vec![kernel(
                side!("baseline_l", "baseline_r"),
                SIZE,
                64,
                &count(SIZE),
            )],
        ],
        reset: [
            kernel(side!("reset_l", "reset_r"), SIZE, 64, &count(SIZE)),
            kernel(side!("reset_sums_l", "reset_sums_r"), 256, 64, &count(256)),
        ],
        modes: [
            (wgsl!("grid_vs"), side!("grid_l_fs", "grid_r_fs")),
            (wgsl!("log2_vs"), side!("log2_l_fs", "log2_r_fs")),
            (wgsl!("element_vs"), side!("element_l_fs", "element_r_fs")),
            (wgsl!("sums_vs"), side!("sums_l_fs", "sums_r_fs")),
        ],
    }
}
/// The display modes in the GUI's order.
pub(super) const MODES: [&str; 4] = [
    "Input Grid",
    "Input Log2",
    "Input Element 0",
    "Workgroup Sum Grid",
];
/// The state the side's next step acts on.
#[derive(Clone, Copy, PartialEq, Debug)]
enum State {
    Run,
    Validate,
    Reset,
}
/// The storage buffers a module binds, by their name's prefix.
struct Storage {
    input: wgpu::Buffer,
    vectorized: wgpu::Buffer,
    sums: wgpu::Buffer,
    debug: wgpu::Buffer,
}
impl Storage {
    fn by_name(&self, name: &str) -> Option<&wgpu::Buffer> {
        if name.starts_with("CurrentVectorized_") {
            Some(&self.vectorized)
        } else if name.starts_with("Current_") {
            Some(&self.input)
        } else if name.starts_with("WorkgroupSums_") {
            Some(&self.sums)
        } else if name.starts_with("Debug_") {
            Some(&self.debug)
        } else {
            None
        }
    }
}
/// A module's group bindings: ( group, binding, variable name ).
fn bindings(source: &str) -> Vec<(u32, u32, String)> {
    let mut out = vec![];
    let mut rest = source;
    while let Some(i) = rest.find("@binding( ") {
        rest = &rest[i + 10..];
        let Some((b, after)) = rest.split_once(" ) @group( ") else {
            break;
        };
        let Some((g, after)) = after.split_once(" )") else {
            break;
        };
        let decl = after[..after.find(':').unwrap_or(0)].trim();
        let name = decl.rsplit(' ').next().unwrap_or("").to_string();
        if let (Ok(b), Ok(g)) = (b.parse(), g.parse()) {
            out.push((g, b, name));
        }
    }
    out
}
/// A compiled call: pipeline, bind group and workgroups.
struct Call {
    pipeline: wgpu::ComputePipeline,
    group: wgpu::BindGroup,
    groups: u32,
}
fn call(r: &Renderer, k: &Kernel, storage: &Storage) -> Result<Call> {
    let module = r.device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("compute reduce"),
        source: wgpu::ShaderSource::Wgsl(k.source.into()),
    });
    let pipeline = r
        .device
        .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("compute reduce"),
            layout: None,
            module: &module,
            entry_point: Some("main"),
            compilation_options: Default::default(),
            cache: None,
        });
    let object = if k.source.contains("struct objectStruct {") {
        let values: Vec<(&str, &[f64])> = k.object.iter().map(|(n, v)| (*n, &v[..])).collect();
        Some(
            r.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("compute reduce object"),
                    contents: &pack(k.source, "objectStruct", &values)?,
                    usage: wgpu::BufferUsages::UNIFORM,
                }),
        )
    } else {
        None
    };
    let mut entries = vec![];
    for (_, binding, name) in bindings(k.source) {
        let buffer = if name == "object" {
            object.as_ref()
        } else {
            storage.by_name(&name)
        }
        .ok_or(Error::Invalid("compute reduce binding"))?;
        entries.push((binding, buffer.as_entire_binding()));
    }
    Ok(Call {
        group: bind(r, pipeline.get_bind_group_layout(0), &entries),
        pipeline,
        groups: k.groups,
    })
}
/// One half: its buffers, calls, display draws and step state.
struct Side {
    storage: Storage,
    algorithms: Vec<Vec<Call>>,
    reset: [Call; 2],
    modes: [(&'static str, &'static str); 4],
    algo: usize,
    mode: usize,
    state: State,
    highlight: f64,
    /// The next stepAnimation time ( ms ).
    next: f64,
    render: wgpu::Buffer,
    objects: [wgpu::Buffer; 4],
    /// The background: Color( 0x313131 ) left, 0x212121 right ( linear ).
    background: f64,
}
/// A pending buffer log ( a ReadbackBuffer ).
struct Readback {
    buffer: wgpu::Buffer,
    ready: Arc<Mutex<Option<std::result::Result<(), wgpu::BufferAsyncError>>>>,
}
struct Targets {
    width: u32,
    height: u32,
    format: wgpu::TextureFormat,
    scenes: [wgpu::TextureView; 2],
    depth: wgpu::TextureView,
    draws: [Vec<Draw>; 2],
    output: (wgpu::RenderPipeline, [Vec<wgpu::BindGroup>; 2]),
    screen: RenderTarget,
}
pub(super) struct Demo {
    sides: [Side; 2],
    time: f64,
    pending: bool,
    /// The debug folder's Buffer to Log.
    logged: usize,
    /// Log buttons pressed: ( side, buffer ).
    log_requests: Vec<(usize, usize)>,
    readbacks: Vec<Readback>,
    plane: (wgpu::Buffer, wgpu::Buffer, wgpu::Buffer),
    quad: wgpu::Buffer,
    output_renders: [wgpu::Buffer; 2],
    output_object: wgpu::Buffer,
    linear: wgpu::Sampler,
    targets: Option<Targets>,
}
fn srgb_to_linear(c: f64) -> f64 {
    if c < 0.04045 {
        c * 0.0773993808
    } else {
        (c * 0.9478672986 + 0.0521327014).powf(2.4)
    }
}
impl Demo {
    pub async fn create(_s: &mut Scene, _c: Object3D, _id: u32, r: &Renderer) -> Result<Self> {
        if !r.device.features().contains(wgpu::Features::SUBGROUP) {
            return Err(Error::Gpu(
                "compute reduce: the device lacks the subgroups feature".into(),
            ));
        }
        let storage = |label: &str, data: &[u32]| {
            r.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some(label),
                    contents: bytemuck::cast_slice(data),
                    usage: wgpu::BufferUsages::STORAGE
                        | wgpu::BufferUsages::COPY_SRC
                        | wgpu::BufferUsages::COPY_DST,
                })
        };
        let ones = vec![1u32; SIZE as usize];
        let uniform = |label: &str, source: &str, name: &str| -> Result<wgpu::Buffer> {
            Ok(r.device.create_buffer(&wgpu::BufferDescriptor {
                label: Some(label),
                size: pack(source, name, &[])?.len() as u64,
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            }))
        };
        let make_side = |left: bool| -> Result<Side> {
            let storage = Storage {
                input: storage("reduce input", &ones),
                // The vectorized attribute shares the page's array: its own
                // buffer of ones, never reset.
                vectorized: storage("reduce vectorized", &ones),
                sums: storage("reduce workgroup sums", &[0; 128]),
                debug: storage("reduce debug", &[0; 1024]),
            };
            let src = sources(left);
            let algorithms = src
                .algorithms
                .iter()
                .map(|calls| {
                    calls
                        .iter()
                        .map(|k| call(r, k, &storage))
                        .collect::<Result<Vec<_>>>()
                })
                .collect::<Result<Vec<_>>>()?;
            let reset = [
                call(r, &src.reset[0], &storage)?,
                call(r, &src.reset[1], &storage)?,
            ];
            let objects =
                [0, 1, 2, 3].map(|i| uniform("reduce display", src.modes[i].1, "objectStruct"));
            let [a, b, c, d] = objects;
            Ok(Side {
                storage,
                algorithms,
                reset,
                modes: src.modes,
                algo: if left { 0 } else { 4 },
                mode: if left { 1 } else { 2 },
                state: State::Run,
                highlight: 0.,
                next: 1000.,
                render: uniform("reduce render", src.modes[0].0, "renderStruct")?,
                objects: [a?, b?, c?, d?],
                background: srgb_to_linear(if left { 0x31 } else { 0x21 } as f64 / 255.),
            })
        };
        let sides = [make_side(true)?, make_side(false)?];
        // The page's first renderer.compute( computeResetBuffer ) leaves the
        // ones it was created with.
        let vertex = |label, data: &[u8], usage| {
            r.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some(label),
                    contents: data,
                    usage,
                })
        };
        let v = wgpu::BufferUsages::VERTEX;
        // PlaneGeometry( 1, 1 ).
        let positions: [f32; 12] = [-0.5, 0.5, 0., 0.5, 0.5, 0., -0.5, -0.5, 0., 0.5, -0.5, 0.];
        let uvs: [f32; 8] = [0., 1., 1., 1., 0., 0., 1., 0.];
        let index: [u32; 6] = [0, 2, 1, 2, 3, 1];
        let plane = (
            vertex("reduce plane position", bytemuck::cast_slice(&positions), v),
            vertex("reduce plane uv", bytemuck::cast_slice(&uvs), v),
            vertex(
                "reduce plane index",
                bytemuck::cast_slice(&index),
                wgpu::BufferUsages::INDEX,
            ),
        );
        // QuadMesh's triangle.
        let quad: [f32; 9] = [-1., 3., 0., -1., -1., 0., 3., -1., 0.];
        let output_object = uniform("reduce output", OUTPUT_VS, "objectStruct")?;
        r.queue.write_buffer(
            &output_object,
            0,
            &pack(
                OUTPUT_VS,
                "objectStruct",
                &[("nodeUniform4", &Matrix4::IDENTITY.to_cols_array())],
            )?,
        );
        Ok(Self {
            sides,
            time: 0.,
            pending: true,
            logged: 0,
            log_requests: vec![],
            readbacks: vec![],
            plane,
            quad: vertex("reduce quad", bytemuck::cast_slice(&quad), v),
            output_renders: [
                uniform("reduce output", OUTPUT_FS, "renderStruct")?,
                uniform("reduce output", OUTPUT_FS, "renderStruct")?,
            ],
            output_object,
            linear: r.device.create_sampler(&wgpu::SamplerDescriptor {
                mag_filter: wgpu::FilterMode::Linear,
                min_filter: wgpu::FilterMode::Linear,
                ..Default::default()
            }),
            targets: None,
        })
    }
    /// The vertex stage's attribute buffers from its main().
    fn attributes(vs: &str) -> Vec<(u32, String, wgpu::VertexFormat)> {
        let Some(start) = vs.find("fn main(") else {
            return vec![];
        };
        let signature = &vs[start..start + vs[start..].find("->").unwrap_or(0)];
        signature
            .split("@location(")
            .skip(1)
            .filter_map(|part| {
                let (location, rest) = part.split_once(')')?;
                let (name, ty) = rest.split_once(':')?;
                let format = if ty.contains("vec2") {
                    wgpu::VertexFormat::Float32x2
                } else {
                    wgpu::VertexFormat::Float32x3
                };
                Some((
                    location.trim().parse().ok()?,
                    name.trim().to_string(),
                    format,
                ))
            })
            .collect()
    }
    fn resize(&mut self, r: &Renderer, out: &RenderTarget) -> Result<()> {
        let size = (out.width, out.height);
        let texture = |format| {
            r.device
                .create_texture(&wgpu::TextureDescriptor {
                    label: Some("compute reduce target"),
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
        let mut draws: [Vec<Draw>; 2] = [vec![], vec![]];
        for (k, side) in self.sides.iter().enumerate() {
            for (m, (vs, fs)) in side.modes.iter().enumerate() {
                let attributes = Self::attributes(vs);
                let lists: Vec<[wgpu::VertexAttribute; 1]> = attributes
                    .iter()
                    .map(|(location, _, format)| {
                        [wgpu::VertexAttribute {
                            format: *format,
                            offset: 0,
                            shader_location: *location,
                        }]
                    })
                    .collect();
                let layouts: Vec<wgpu::VertexBufferLayout> = attributes
                    .iter()
                    .zip(&lists)
                    .map(|((_, _, format), list)| wgpu::VertexBufferLayout {
                        array_stride: format.size(),
                        step_mode: wgpu::VertexStepMode::Vertex,
                        attributes: list,
                    })
                    .collect();
                let pipeline = culled_pipeline(
                    r,
                    "compute reduce display",
                    (vs, fs),
                    &layouts,
                    &[HALF],
                    Some((wgpu::CompareFunction::LessEqual, true)),
                    (false, false),
                    (1, wgpu::PrimitiveTopology::TriangleList),
                    Some(wgpu::Face::Back),
                );
                let mut entries = vec![];
                for (group, binding, name) in bindings(fs).into_iter().chain(bindings(vs)) {
                    if group != 1 || entries.iter().any(|(b, _)| *b == binding) {
                        continue;
                    }
                    let buffer = if name == "object" {
                        Some(&side.objects[m])
                    } else {
                        side.storage.by_name(&name)
                    }
                    .ok_or(Error::Invalid("compute reduce display binding"))?;
                    entries.push((binding, buffer.as_entire_binding()));
                }
                let groups = vec![
                    bind(
                        r,
                        pipeline.get_bind_group_layout(0),
                        &[(0, side.render.as_entire_binding())],
                    ),
                    bind(r, pipeline.get_bind_group_layout(1), &entries),
                ];
                draws[k].push((pipeline, groups));
            }
        }
        let scenes = [texture(HALF), texture(HALF)];
        let output = raw_pipeline(
            r,
            "compute reduce output",
            OUTPUT_VS,
            OUTPUT_FS,
            &[wgpu::VertexBufferLayout {
                array_stride: 12,
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes: &wgpu::vertex_attr_array![0 => Float32x3],
            }],
            out.options.format,
            1,
            None,
            false,
        );
        let output_groups = [0, 1].map(|k| {
            vec![
                bind(
                    r,
                    output.get_bind_group_layout(0),
                    &[(0, self.output_renders[k].as_entire_binding())],
                ),
                bind(
                    r,
                    output.get_bind_group_layout(1),
                    &[
                        (0, wgpu::BindingResource::Sampler(&self.linear)),
                        (1, wgpu::BindingResource::TextureView(&scenes[k])),
                        (2, self.output_object.as_entire_binding()),
                    ],
                ),
            ]
        });
        self.targets = Some(Targets {
            width: size.0,
            height: size.1,
            format: out.options.format,
            depth: texture(DEPTH),
            scenes,
            draws,
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
    fn dispatch(encoder: &mut wgpu::CommandEncoder, calls: &[Call]) {
        // renderer.compute: a pass per call.
        for c in calls {
            let mut pass = encoder.begin_compute_pass(&Default::default());
            pass.set_pipeline(&c.pipeline);
            pass.set_bind_group(0, &c.group, &[]);
            pass.dispatch_workgroups(c.groups, 1, 1);
        }
    }
    /// The side's display uniforms: the camera and its mode's object struct.
    fn write_display(&self, r: &Renderer, k: usize, aspect: f64) -> Result<()> {
        let side = &self.sides[k];
        // OrthographicCamera( -aspect, aspect, 1, -1, 0, 2 ) at z 1, in WebGPU depth.
        let projection = [
            1. / aspect,
            0.,
            0.,
            0.,
            0.,
            1.,
            0.,
            0.,
            0.,
            0.,
            -0.5,
            0.,
            0.,
            0.,
            0.,
            1.,
        ];
        let view = Matrix4::from_translation(Vector3::new(0., 0., -1.)).to_cols_array();
        let fs = side.modes[side.mode].1;
        r.queue.write_buffer(
            &side.render,
            0,
            &pack(
                side.modes[0].0,
                "renderStruct",
                &[
                    ("cameraProjectionMatrix", &projection),
                    ("cameraViewMatrix", &view),
                ],
            )?,
        );
        let identity = Matrix4::IDENTITY.to_cols_array();
        let grid = 512.;
        let values: Vec<(&str, Vec<f64>)> = match side.mode {
            0 => vec![
                ("nodeUniform1", vec![grid]),
                ("nodeUniform2", vec![grid]),
                ("nodeUniform3", vec![grid]),
                ("nodeUniform4", vec![grid]),
                ("nodeUniform5", vec![side.highlight]),
                ("nodeUniform6", vec![1.]),
                ("nodeUniform9", identity.to_vec()),
            ],
            1 | 2 => vec![
                ("nodeUniform1", vec![grid]),
                ("nodeUniform2", vec![grid]),
                ("nodeUniform3", vec![side.highlight]),
                ("nodeUniform4", vec![1.]),
                ("nodeUniform7", identity.to_vec()),
            ],
            _ => vec![
                ("nodeUniform1", vec![side.highlight]),
                ("nodeUniform3", vec![1.]),
                ("nodeUniform6", identity.to_vec()),
            ],
        };
        let values: Vec<(&str, &[f64])> = values.iter().map(|(n, v)| (*n, &v[..])).collect();
        r.queue.write_buffer(
            &side.objects[side.mode],
            0,
            &pack(fs, "objectStruct", &values)?,
        );
        Ok(())
    }
    /// One side's scene pass: the background clear and the plane.
    fn draw_side(&self, encoder: &mut wgpu::CommandEncoder, t: &Targets, k: usize) {
        let side = &self.sides[k];
        let half = t.width / 2;
        let b = side.background;
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("compute reduce scene"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &t.scenes[k],
                depth_slice: None,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: b,
                        g: b,
                        b,
                        a: 1.,
                    }),
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
        let (x, w) = if k == 0 {
            (0, half)
        } else {
            (half, t.width - half)
        };
        pass.set_viewport(x as f32, 0., w as f32, t.height as f32, 0., 1.);
        set(&mut pass, &t.draws[k][side.mode]);
        let vs = side.modes[side.mode].0;
        for (slot, (_, name, _)) in Self::attributes(vs).iter().enumerate() {
            let buffer = if name == "uv" {
                &self.plane.1
            } else {
                &self.plane.0
            };
            pass.set_vertex_buffer(slot as u32, buffer.slice(..));
        }
        pass.set_index_buffer(self.plane.2.slice(..), wgpu::IndexFormat::Uint32);
        pass.draw_indexed(0..6, 0, 0..1);
    }
    /// stepAnimation on each side's timer up to the example time: the
    /// state's computes, the step's own render, then the next state.
    fn advance(&mut self, r: &Renderer, aspect: f64) -> Result<()> {
        let now = self.time * 1000.;
        for k in 0..2 {
            while self.sides[k].next <= now {
                let mut encoder = r.device.create_command_encoder(&Default::default());
                let side = &self.sides[k];
                match side.state {
                    State::Reset => Self::dispatch(&mut encoder, &side.reset),
                    State::Run => Self::dispatch(&mut encoder, &side.algorithms[side.algo]),
                    State::Validate => {}
                }
                self.write_display(r, k, aspect)?;
                if let Some(t) = self.targets.as_ref() {
                    self.draw_side(&mut encoder, t, k);
                }
                r.queue.submit([encoder.finish()]);
                let side = &mut self.sides[k];
                (side.state, side.highlight) = match side.state {
                    State::Run => (State::Validate, 1.),
                    State::Validate => (State::Reset, 0.),
                    State::Reset => (State::Run, side.highlight),
                };
                side.next += 1000.;
            }
        }
        Ok(())
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, dt: f64, animate: bool) -> Result<()> {
        if animate {
            self.time += dt;
            self.pending = true;
        }
        Ok(())
    }
    pub fn prepare(&mut self, _s: &mut Scene, _c: Object3D, _r: &Renderer) -> Result<()> {
        Ok(())
    }
    /// Logs a finished readback, as the page's console.log( result ).
    fn flush_logs(&mut self) {
        self.readbacks.retain(|rb| {
            let Some(result) = rb.ready.lock().map(|mut g| g.take()).unwrap_or(None) else {
                return true;
            };
            if result.is_ok() {
                let values: Vec<String> = {
                    let mapped = rb.buffer.slice(..).get_mapped_range();
                    bytemuck::cast_slice::<u8, u32>(&mapped)
                        .iter()
                        .map(u32::to_string)
                        .collect()
                };
                rb.buffer.unmap();
                #[cfg(target_arch = "wasm32")]
                web_sys::console::log_1(
                    &format!("Uint32Array({}) [{}]", values.len(), values.join(", ")).into(),
                );
                #[cfg(not(target_arch = "wasm32"))]
                let _ = values;
            }
            false
        });
    }
    pub fn render(
        &mut self,
        r: &Renderer,
        _s: &mut Scene,
        _c: Object3D,
        out: &RenderTarget,
    ) -> Result<bool> {
        if self.targets.as_ref().is_none_or(|t| {
            t.width != out.width || t.height != out.height || t.format != out.options.format
        }) {
            self.resize(r, out)?;
        }
        for (k, which) in std::mem::take(&mut self.log_requests) {
            // new THREE.ReadbackBuffer( buffer ) and getArrayBufferAsync.
            let s = &self.sides[k].storage;
            let source = [&s.input, &s.vectorized, &s.sums, &s.debug][which];
            let buffer = r.device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("compute reduce readback"),
                size: source.size(),
                usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });
            let mut encoder = r.device.create_command_encoder(&Default::default());
            encoder.copy_buffer_to_buffer(source, 0, &buffer, 0, source.size());
            r.queue.submit([encoder.finish()]);
            let ready = Arc::new(Mutex::new(None));
            let signal = ready.clone();
            buffer
                .slice(..)
                .map_async(wgpu::MapMode::Read, move |result| {
                    if let Ok(mut g) = signal.lock() {
                        *g = Some(result);
                    }
                });
            self.readbacks.push(Readback { buffer, ready });
        }
        self.flush_logs();
        let (width, height) = self
            .targets
            .as_ref()
            .map(|t| (t.width, t.height))
            .ok_or(Error::Invalid("compute reduce targets"))?;
        let aspect = (width / 2) as f64 / height as f64;
        if std::mem::take(&mut self.pending) {
            self.advance(r, aspect)?;
        }
        // animate(): each renderer renders its scene, then the output pass.
        for k in 0..2 {
            self.write_display(r, k, aspect)?;
            r.queue.write_buffer(
                &self.output_renders[k],
                0,
                &pack(
                    OUTPUT_FS,
                    "renderStruct",
                    &[
                        (
                            "cameraProjectionMatrix",
                            &[
                                1., 0., 0., 0., 0., 1., 0., 0., 0., 0., -1., 0., 0., 0., 0., 1.,
                            ],
                        ),
                        ("cameraViewMatrix", &Matrix4::IDENTITY.to_cols_array()),
                        ("nodeUniform1", &[width as f64, height as f64]),
                    ],
                )?,
            );
        }
        let t = self
            .targets
            .as_ref()
            .ok_or(Error::Invalid("compute reduce targets"))?;
        let mut encoder = r.device.create_command_encoder(&Default::default());
        for k in 0..2 {
            self.draw_side(&mut encoder, t, k);
        }
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("compute reduce output"),
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
            let half = t.width / 2;
            for k in 0..2 {
                let (x, w) = if k == 0 {
                    (0, half)
                } else {
                    (half, t.width - half)
                };
                pass.set_scissor_rect(x, 0, w, t.height);
                pass.set_pipeline(&t.output.0);
                for (i, g) in t.output.1[k].iter().enumerate() {
                    pass.set_bind_group(i as u32, g, &[]);
                }
                pass.set_vertex_buffer(0, self.quad.slice(..));
                pass.draw(0..3, 0..1);
            }
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
    /// Left and right algorithm and display mode, the buffer to log, and the
    /// left and right Log buttons.
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        let v = value as usize;
        match index {
            0 | 1 => self.sides[index].algo = v.min(ALGORITHMS.len() - 1),
            2 | 3 => self.sides[index - 2].mode = v.min(MODES.len() - 1),
            4 => self.logged = v.min(3),
            5 | 6 => self.log_requests.push((index - 5, self.logged)),
            _ => return Err(Error::Invalid("compute reduce parameter")),
        }
        self.pending = true;
        Ok(())
    }
    pub fn seek(&mut self, t: f64) {
        self.time = t;
        self.pending = true;
    }
}
