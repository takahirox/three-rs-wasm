//! webgpu_compute_rasterizer: 160,000 rotating teapots ( 400 × 400 ) drawn
//! by a compute-shader software rasterizer with a hardware fallback, under
//! FirstPersonControls. Each frame the page's five compute passes run on the
//! resident storage buffers: Compute Clear empties the packed visibility
//! buffers, Compute Frustum culls the instances and their 64-triangle chunks
//! against the camera, picks a teapot LOD by its projected error and appends
//! the visible chunks to the work queue, a one-thread pass writes the
//! indirect dispatch, Compute Rasterize rasterizes the small triangles into
//! the depth-packed atomic buffers and queues the large ones, and a last pass
//! writes the hardware queue's indirect draw. A fullscreen quad resolves the
//! visibility buffers ( meshlet colors or the uv grid ) and its depth, the
//! queued large triangles render with real depth testing through that indirect
//! draw, and the output pass converts to sRGB. Every stage runs the WGSL
//! three.js r186 generates for the page ( in `compute_rasterizer/` ), without
//! its unused subgroup-size builtin. The page's intermediate output pass after
//! the quad ( overwritten by the final one ) is not repeated.
use super::deferred::Draw;
use super::gltf_viewer::{decode_texture_image, fetch};
use super::lights_projector::{m4, pack};
use super::models_modifiers::teapot;
use super::pmrem_cube_uv::{bind, raw_pipeline};
use super::retro::{mipmapped, target, uniform};
use super::shadowmap_opacity::Mipmaps;
use super::trackball_sprites::FirstPerson;
use crate::{
    Error, Result, camera::*, geometry::*, math::*, render_target::*, renderer::*, scene::*,
};
use wgpu::util::DeviceExt;

const HALF: wgpu::TextureFormat = wgpu::TextureFormat::Rgba16Float;
const DEPTH: wgpu::TextureFormat = wgpu::TextureFormat::Depth24Plus;
const ROWS: usize = 400;
const COLS: usize = 400;
const INSTANCES: usize = ROWS * COLS;
const MAX_RASTER_SIZE: f64 = 16.;
const MAX_WORK_ITEMS: u64 = 2_820_000;
const MAX_HW_TRIANGLES: u64 = 100_000;
/// TeapotGeometry( 1, segments ) and its LOD error.
const LODS: [(u32, f64); 7] = [
    (10, 0.),
    (8, 0.005),
    (6, 0.015),
    (5, 0.03),
    (4, 0.06),
    (3, 0.1),
    (2, 0.2),
];
macro_rules! wgsl {
    ($name:literal) => {
        include_str!(concat!("compute_rasterizer/", $name, ".wgsl"))
    };
}
const CLEAR: &str = wgsl!("0");
const FRUSTUM: &str = wgsl!("1");
const DISPATCH: &str = wgsl!("2");
const RASTERIZE: &str = wgsl!("3");
const HW_ARGS: &str = wgsl!("4");
const QUAD_VS: &str = wgsl!("6");
const QUAD_FS: &str = wgsl!("7");
const OUTPUT_VS: &str = wgsl!("8");
const OUTPUT_FS: &str = wgsl!("9");
const HW_VS: &str = wgsl!("10");
const HW_FS: &str = wgsl!("11");
/// The resident storage buffers, by the page's names.
struct Buffers {
    lod_offsets: wgpu::Buffer,
    chunk_bounds: wgpu::Buffer,
    vertices: wgpu::Buffer,
    uvs: wgpu::Buffer,
    indices: wgpu::Buffer,
    meshlet_ids: wgpu::Buffer,
    instance_data: wgpu::Buffer,
    instance_world: wgpu::Buffer,
    instance_mvp: wgpu::Buffer,
    work_queue_count: wgpu::Buffer,
    dispatch: wgpu::Buffer,
    work_queue: wgpu::Buffer,
    hw_queue: wgpu::Buffer,
    hw_draw: wgpu::Buffer,
    /// The hardware mesh's ( unread ) position attribute.
    hw_positions: wgpu::Buffer,
    quad_uv: wgpu::Buffer,
    quad_position: wgpu::Buffer,
}
/// The uniform buffers, one per shader struct.
struct Uniforms {
    clear_object: wgpu::Buffer,
    frustum_render: wgpu::Buffer,
    frustum_object: wgpu::Buffer,
    frustum_planes: wgpu::Buffer,
    dispatch_object: wgpu::Buffer,
    rasterize_render: wgpu::Buffer,
    rasterize_object: wgpu::Buffer,
    hw_args_object: wgpu::Buffer,
    quad_render: wgpu::Buffer,
    quad_object: wgpu::Buffer,
    hw_render: wgpu::Buffer,
    hw_object: wgpu::Buffer,
    output_render: wgpu::Buffer,
    output_object: wgpu::Buffer,
}
struct Targets {
    width: u32,
    height: u32,
    format: wgpu::TextureFormat,
    color: wgpu::TextureView,
    depth: wgpu::TextureView,
    /// screenTri and screenInst: the packed visibility buffers.
    _screen_buffers: [wgpu::Buffer; 2],
    clear: Vec<wgpu::BindGroup>,
    rasterize: Vec<wgpu::BindGroup>,
    quad: Draw,
    output: Draw,
    screen: RenderTarget,
}
pub(super) struct Demo {
    walker: FirstPerson,
    /// Mode ( 0 Meshlet Debug, 1 Texture ), Rasterizer ( 0 SW Only, 1 HW
    /// Only, 2 Both ) and Animation Speed.
    params: [f64; 3],
    time: f64,
    delta: f64,
    buffers: Buffers,
    uniforms: Uniforms,
    map: wgpu::TextureView,
    repeat: wgpu::Sampler,
    clamp: wgpu::Sampler,
    pipelines: [wgpu::ComputePipeline; 5],
    frustum_groups: Vec<wgpu::BindGroup>,
    dispatch_group: wgpu::BindGroup,
    hw_args_group: wgpu::BindGroup,
    quad_pipeline: wgpu::RenderPipeline,
    hw: Draw,
    targets: Option<Targets>,
}
fn compute_pipeline(r: &Renderer, label: &str, source: &str) -> wgpu::ComputePipeline {
    let module = r.device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some(label),
        source: wgpu::ShaderSource::Wgsl(source.into()),
    });
    r.device
        .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some(label),
            layout: None,
            module: &module,
            entry_point: Some("main"),
            compilation_options: Default::default(),
            cache: None,
        })
}
fn storage(r: &Renderer, label: &str, contents: &[u8], indirect: bool) -> wgpu::Buffer {
    let usage = wgpu::BufferUsages::STORAGE
        | wgpu::BufferUsages::COPY_DST
        | if indirect {
            wgpu::BufferUsages::INDIRECT
        } else {
            wgpu::BufferUsages::empty()
        };
    r.device
        .create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(label),
            contents,
            usage,
        })
}
/// A zero-filled storage buffer.
fn zeroed(r: &Renderer, label: &str, size: u64) -> wgpu::Buffer {
    r.device.create_buffer(&wgpu::BufferDescriptor {
        label: Some(label),
        size,
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    })
}
fn entire(buffer: &wgpu::Buffer) -> wgpu::BindingResource<'_> {
    buffer.as_entire_binding()
}
fn floats(g: &BufferGeometry, name: &str) -> Result<Vec<f32>> {
    match g.attributes.get(name) {
        Some(Attribute::F32(a)) => Ok(a.array().to_vec()),
        _ => Err(Error::Invalid("teapot attribute")),
    }
}
/// The packed LOD geometry: vec4 positions, uvs, indices, a meshlet id per
/// triangle ( 126 triangles each ), the chunk bounding spheres and the LOD
/// offsets ( triangleStart, numTriangles, chunkStart, 0 ).
type Lods = (Vec<f32>, Vec<f32>, Vec<u32>, Vec<u32>, Vec<f32>, Vec<u32>);
fn build_lods() -> Result<Lods> {
    let mut lods = vec![];
    for (segments, _) in LODS {
        let g = teapot(1., segments, true, true, true, true, true)?;
        let positions = floats(&g, "position")?;
        let uvs = floats(&g, "uv")?;
        let count = positions.len() / 3;
        let indices = g
            .index
            .clone()
            .unwrap_or_else(|| (0..count as u32).collect());
        lods.push((positions, uvs, indices));
    }
    let (mut vertices, mut uv_array, mut index_array, mut meshlets) =
        (vec![], vec![], vec![], vec![]);
    let mut offsets = vec![];
    let mut meshlet = 1u32;
    for (positions, uvs, indices) in &lods {
        let base = (vertices.len() / 4) as u32;
        offsets.push((index_array.len() / 3) as u32);
        for (p, uv) in positions.chunks(3).zip(uvs.chunks(2)) {
            vertices.extend([p[0], p[1], p[2], 1.]);
            uv_array.extend([uv[0], uv[1]]);
        }
        let mut count = 0;
        for t in indices.chunks(3) {
            index_array.extend(t.iter().map(|&i| base + i));
            if count >= 126 {
                meshlet += 1;
                count = 0;
            }
            meshlets.push(meshlet);
            count += 1;
        }
        meshlet += 1;
    }
    let (mut bounds, mut lod_offsets) = (vec![], vec![]);
    for ((positions, _, indices), start) in lods.iter().zip(offsets) {
        let triangles = indices.len() / 3;
        lod_offsets.extend([start, triangles as u32, (bounds.len() / 4) as u32, 0]);
        let at = |i: u32, k: usize| positions[i as usize * 3 + k] as f64;
        for c in 0..triangles.div_ceil(64) {
            let chunk = &indices[c * 64 * 3..(c * 64 + 64).min(triangles) * 3];
            let mut center = [0.; 3];
            for &i in chunk {
                for (k, v) in center.iter_mut().enumerate() {
                    *v += at(i, k);
                }
            }
            let center = center.map(|v| v / chunk.len() as f64);
            let mut max = 0f64;
            for &i in chunk {
                let d: f64 = (0..3).map(|k| (at(i, k) - center[k]).powi(2)).sum();
                max = max.max(d);
            }
            bounds.extend([center[0], center[1], center[2], max.sqrt()].map(|v| v as f32));
        }
    }
    Ok((
        vertices,
        uv_array,
        index_array,
        meshlets,
        bounds,
        lod_offsets,
    ))
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, _id: u32, r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 50.,
            near: 0.25,
            far: 1_000_000.,
            aspect,
            ..Default::default()
        }));
        let n = s.get_mut(c)?;
        n.position = Vector3::new(0., 15., 50.);
        n.quaternion = Quaternion::IDENTITY;
        // controls.lookAt( 0, - 1.5, 0 ): the controls take the orientation.
        let position = s.get(c)?.position;
        let look = (Vector3::new(0., -1.5, 0.) - position).normalize();
        s.look_at(c, Vector3::new(0., -1.5, 0.))?;
        let mut walker = FirstPerson::default();
        walker.speed = 30.;
        walker.lat = 90. - look.y.clamp(-1., 1.).acos().to_degrees();
        walker.lon = look.x.atan2(look.z).to_degrees();
        let (vertices, uvs, indices, meshlets, bounds, lod_offsets) = build_lods()?;
        let mut instance_data = Vec::with_capacity(INSTANCES * 4);
        for i in 0..ROWS {
            for j in 0..COLS {
                instance_data.extend([
                    ((i as f64 - ROWS as f64 / 2.) * 4.) as f32,
                    -1.,
                    ((j as f64 - COLS as f64 / 2.) * 4.) as f32,
                    1.,
                ]);
            }
        }
        let bytes = |v: &[f32]| bytemuck::cast_slice::<f32, u8>(v).to_vec();
        let words = |v: &[u32]| bytemuck::cast_slice::<u32, u8>(v).to_vec();
        let mat_bytes = INSTANCES as u64 * 64;
        let buffers = Buffers {
            lod_offsets: storage(r, "lod offsets", &words(&lod_offsets), false),
            chunk_bounds: storage(r, "chunk bounds", &bytes(&bounds), false),
            vertices: storage(r, "teapot vertices", &bytes(&vertices), false),
            uvs: storage(r, "teapot uvs", &bytes(&uvs), false),
            indices: storage(r, "teapot indices", &words(&indices), false),
            meshlet_ids: storage(r, "meshlet ids", &words(&meshlets), false),
            instance_data: storage(r, "instance data", &bytes(&instance_data), false),
            instance_world: zeroed(r, "instance world", mat_bytes),
            instance_mvp: zeroed(r, "instance mvp", mat_bytes),
            work_queue_count: zeroed(r, "work queue count", 4),
            dispatch: storage(r, "dispatch", &[0; 12], true),
            work_queue: zeroed(r, "work queue", MAX_WORK_ITEMS * 16),
            hw_queue: zeroed(r, "hw queue", (MAX_HW_TRIANGLES + 1) * 4),
            hw_draw: storage(r, "hw draw", &[0; 16], true),
            hw_positions: r.device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("hw positions"),
                size: MAX_HW_TRIANGLES * 36,
                usage: wgpu::BufferUsages::VERTEX,
                mapped_at_creation: false,
            }),
            quad_uv: r
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("quad uv"),
                    contents: bytemuck::cast_slice(&[0f32, -1., 0., 1., 2., 1.]),
                    usage: wgpu::BufferUsages::VERTEX,
                }),
            quad_position: r
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("quad position"),
                    contents: bytemuck::cast_slice(&[-1f32, 3., 0., -1., -1., 0., 3., -1., 0.]),
                    usage: wgpu::BufferUsages::VERTEX,
                }),
        };
        let object = |label, source| uniform(r, label, source, "objectStruct");
        let render = |label, source| uniform(r, label, source, "renderStruct");
        let uniforms = Uniforms {
            clear_object: object("compute clear", CLEAR)?,
            frustum_render: render("compute frustum", FRUSTUM)?,
            frustum_object: object("compute frustum", FRUSTUM)?,
            frustum_planes: r.device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("frustum planes"),
                size: 96,
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            }),
            dispatch_object: object("compute dispatch", DISPATCH)?,
            rasterize_render: render("compute rasterize", RASTERIZE)?,
            rasterize_object: object("compute rasterize", RASTERIZE)?,
            hw_args_object: object("compute hw args", HW_ARGS)?,
            quad_render: render("sw quad", QUAD_FS)?,
            quad_object: object("sw quad", QUAD_FS)?,
            hw_render: render("hw mesh", HW_VS)?,
            hw_object: object("hw mesh", HW_VS)?,
            output_render: render("rasterizer output", OUTPUT_FS)?,
            output_object: object("rasterizer output", OUTPUT_VS)?,
        };
        let write = |buffer: &wgpu::Buffer, source: &str, name: &str, values: &[(&str, &[f64])]| {
            pack(source, name, values).map(|data| r.queue.write_buffer(buffer, 0, &data))
        };
        // The constant uniforms.
        write(
            &uniforms.dispatch_object,
            DISPATCH,
            "objectStruct",
            &[("nodeUniform2", &[1.])],
        )?;
        write(
            &uniforms.hw_args_object,
            HW_ARGS,
            "objectStruct",
            &[("nodeUniform2", &[1.])],
        )?;
        write(
            &uniforms.rasterize_object,
            RASTERIZE,
            "objectStruct",
            &[("nodeUniform6", &[MAX_RASTER_SIZE])],
        )?;
        write(
            &uniforms.output_object,
            OUTPUT_VS,
            "objectStruct",
            &[("nodeUniform4", &m4(Matrix4::IDENTITY))],
        )?;
        let mut image = decode_texture_image(
            &fetch("/web/gallery/assets/tsl-procedural/uv_grid_directx.jpg").await?,
        )
        .await?;
        image.srgb = true;
        // TextureLoader's texture is flipped ( flipY ).
        let row = image.width as usize * 4;
        let image = crate::material::Texture {
            rgba: image.rgba.chunks(row).rev().flatten().copied().collect(),
            bitmap: None,
            ..image
        };
        let mut mipmaps = Mipmaps::new(r);
        let map = mipmapped(r, &mut mipmaps, &image, wgpu::TextureFormat::Rgba8UnormSrgb);
        let repeat = r.device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::Repeat,
            address_mode_v: wgpu::AddressMode::Repeat,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        let clamp = r.device.create_sampler(&wgpu::SamplerDescriptor {
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        let pipelines = [
            compute_pipeline(r, "Compute Clear", CLEAR),
            compute_pipeline(r, "Compute Frustum", FRUSTUM),
            compute_pipeline(r, "Compute Dispatch", DISPATCH),
            compute_pipeline(r, "Compute Rasterize", RASTERIZE),
            compute_pipeline(r, "Compute HW Args", HW_ARGS),
        ];
        let b = &buffers;
        let frustum_groups = vec![
            bind(
                r,
                pipelines[1].get_bind_group_layout(0),
                &[(0, entire(&uniforms.frustum_render))],
            ),
            bind(
                r,
                pipelines[1].get_bind_group_layout(1),
                &[
                    (0, entire(&uniforms.frustum_planes)),
                    (1, entire(&b.instance_data)),
                    (2, entire(&uniforms.frustum_object)),
                    (3, entire(&b.lod_offsets)),
                    (4, entire(&b.chunk_bounds)),
                    (5, entire(&b.work_queue_count)),
                    (6, entire(&b.work_queue)),
                    (7, entire(&b.instance_world)),
                    (8, entire(&b.instance_mvp)),
                ],
            ),
        ];
        let dispatch_group = bind(
            r,
            pipelines[2].get_bind_group_layout(0),
            &[
                (0, entire(&b.dispatch)),
                (1, entire(&b.work_queue_count)),
                (2, entire(&uniforms.dispatch_object)),
            ],
        );
        let hw_args_group = bind(
            r,
            pipelines[4].get_bind_group_layout(0),
            &[
                (0, entire(&b.hw_queue)),
                (1, entire(&b.hw_draw)),
                (2, entire(&uniforms.hw_args_object)),
            ],
        );
        let uv_attribute = [wgpu::VertexAttribute {
            format: wgpu::VertexFormat::Float32x2,
            offset: 0,
            shader_location: 0,
        }];
        let less = Some((wgpu::CompareFunction::LessEqual, true));
        let quad_pipeline = raw_pipeline(
            r,
            "sw quad",
            QUAD_VS,
            QUAD_FS,
            &[wgpu::VertexBufferLayout {
                array_stride: 8,
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes: &uv_attribute,
            }],
            HALF,
            1,
            less,
            false,
        );
        let hw_pipeline = raw_pipeline(
            r,
            "hw mesh",
            HW_VS,
            HW_FS,
            &[POSITION],
            HALF,
            1,
            less,
            false,
        );
        let sampler = wgpu::BindingResource::Sampler;
        let tex = wgpu::BindingResource::TextureView;
        let hw_groups = vec![
            bind(
                r,
                hw_pipeline.get_bind_group_layout(0),
                &[(0, entire(&uniforms.hw_render))],
            ),
            bind(
                r,
                hw_pipeline.get_bind_group_layout(1),
                &[
                    (0, entire(&uniforms.hw_object)),
                    (1, entire(&b.meshlet_ids)),
                    (2, sampler(&repeat)),
                    (3, tex(&map)),
                    (4, entire(&b.uvs)),
                    (5, entire(&b.indices)),
                    (6, entire(&b.hw_queue)),
                    (7, entire(&b.instance_world)),
                    (8, entire(&b.vertices)),
                ],
            ),
        ];
        Ok(Self {
            walker,
            params: [0., 2., 1.],
            time: 0.,
            delta: 0.,
            frustum_groups,
            dispatch_group,
            hw_args_group,
            quad_pipeline,
            hw: (hw_pipeline, hw_groups),
            pipelines,
            buffers,
            uniforms,
            map,
            repeat,
            clamp,
            targets: None,
        })
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, dt: f64, animate: bool) -> Result<()> {
        if animate {
            self.time += dt;
            self.delta += dt;
        }
        Ok(())
    }
    pub fn prepare(&mut self, s: &mut Scene, c: Object3D, _r: &Renderer) -> Result<()> {
        // controls.update( timer.getDelta() ).
        let delta = std::mem::take(&mut self.delta);
        self.walker.update(s, c, delta)
    }
    /// createScreenBuffers(): the visibility buffers at the drawing-buffer
    /// size, with the targets and bind groups that read them.
    fn resize(&mut self, r: &Renderer, out: &RenderTarget) -> Result<()> {
        let size = (out.width, out.height);
        let pixels = u64::from(size.0) * u64::from(size.1);
        let screen_buffers = [
            zeroed(r, "screen tri", pixels * 4),
            zeroed(r, "screen inst", pixels * 4),
        ];
        let (b, u) = (&self.buffers, &self.uniforms);
        let [tri, inst] = &screen_buffers;
        let clear = vec![bind(
            r,
            self.pipelines[0].get_bind_group_layout(0),
            &[
                (0, entire(tri)),
                (1, entire(inst)),
                (2, entire(&b.work_queue_count)),
                (3, entire(&b.hw_queue)),
                (4, entire(&u.clear_object)),
            ],
        )];
        let rasterize = vec![
            bind(
                r,
                self.pipelines[3].get_bind_group_layout(0),
                &[(0, entire(&u.rasterize_render))],
            ),
            bind(
                r,
                self.pipelines[3].get_bind_group_layout(1),
                &[
                    (0, entire(&b.work_queue_count)),
                    (1, entire(&b.work_queue)),
                    (2, entire(&b.instance_mvp)),
                    (3, entire(&b.vertices)),
                    (4, entire(&b.indices)),
                    (5, entire(&u.rasterize_object)),
                    (6, entire(tri)),
                    (7, entire(inst)),
                    (8, entire(&b.hw_queue)),
                ],
            ),
        ];
        let sampler = wgpu::BindingResource::Sampler;
        let tex = wgpu::BindingResource::TextureView;
        let quad = vec![
            bind(
                r,
                self.quad_pipeline.get_bind_group_layout(0),
                &[(0, entire(&u.quad_render))],
            ),
            bind(
                r,
                self.quad_pipeline.get_bind_group_layout(1),
                &[
                    (0, entire(tri)),
                    (1, entire(&u.quad_object)),
                    (2, entire(&b.meshlet_ids)),
                    (3, entire(inst)),
                    (4, sampler(&self.repeat)),
                    (5, tex(&self.map)),
                    (6, entire(&b.uvs)),
                    (7, entire(&b.indices)),
                    (8, entire(&b.instance_mvp)),
                    (9, entire(&b.vertices)),
                ],
            ),
        ];
        let color = target(r, size, HALF, 1);
        let output = raw_pipeline(
            r,
            "rasterizer output",
            OUTPUT_VS,
            OUTPUT_FS,
            &[POSITION],
            out.options.format,
            1,
            None,
            false,
        );
        let output_groups = vec![
            bind(
                r,
                output.get_bind_group_layout(0),
                &[(0, entire(&u.output_render))],
            ),
            bind(
                r,
                output.get_bind_group_layout(1),
                &[
                    (0, sampler(&self.clamp)),
                    (1, tex(&color)),
                    (2, entire(&u.output_object)),
                ],
            ),
        ];
        let write = |buffer: &wgpu::Buffer, source: &str, name: &str, values: &[(&str, &[f64])]| {
            pack(source, name, values).map(|data| r.queue.write_buffer(buffer, 0, &data))
        };
        let (w, h) = (f64::from(size.0), f64::from(size.1));
        write(
            &u.clear_object,
            CLEAR,
            "objectStruct",
            &[("nodeUniform4", &[pixels as f64])],
        )?;
        write(
            &u.rasterize_render,
            RASTERIZE,
            "renderStruct",
            &[("nodeUniform5", &[w, h])],
        )?;
        write(
            &u.quad_render,
            QUAD_FS,
            "renderStruct",
            &[("nodeUniform1", &[w, h])],
        )?;
        write(
            &u.output_render,
            OUTPUT_FS,
            "renderStruct",
            &[
                (
                    "cameraProjectionMatrix",
                    &[
                        1., 0., 0., 0., 0., 1., 0., 0., 0., 0., -1., 0., 0., 0., 0., 1.,
                    ],
                ),
                ("cameraViewMatrix", &m4(Matrix4::IDENTITY)),
                ("nodeUniform1", &[w, h]),
            ],
        )?;
        self.targets = Some(Targets {
            width: size.0,
            height: size.1,
            format: out.options.format,
            color,
            depth: target(r, size, DEPTH, 1),
            _screen_buffers: screen_buffers,
            clear,
            rasterize,
            quad: (self.quad_pipeline.clone(), quad),
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
            t.width != out.width || t.height != out.height || t.format != out.options.format
        }) {
            self.resize(r, out)?;
        }
        let t = self
            .targets
            .as_ref()
            .ok_or(Error::Invalid("rasterizer targets"))?;
        s.update()?;
        let (camera, world) = s.camera(c)?;
        let (projection, view) = (camera.projection_matrix()?, world.inverse());
        let screen = projection * view;
        let position = world.w_axis.truncate();
        // frustum.setFromProjectionMatrix( projScreenMatrix ) in the WebGL
        // coordinate system, as the page calls it.
        let me = screen.to_cols_array();
        let row = |i: usize| [me[i], me[i + 4], me[i + 8], me[i + 12]];
        let (r0, r1, r2, r3) = (row(0), row(1), row(2), row(3));
        let combine = |a: [f64; 4], b: [f64; 4], sign: f64| {
            std::array::from_fn::<f64, 4, _>(|k| a[k] + sign * b[k])
        };
        let planes = [
            combine(r3, r0, -1.),
            combine(r3, r0, 1.),
            combine(r3, r1, 1.),
            combine(r3, r1, -1.),
            combine(r3, r2, -1.),
            combine(r3, r2, 1.),
        ];
        let mut plane_data = [0f32; 24];
        for (i, p) in planes.iter().enumerate() {
            let length = (p[0] * p[0] + p[1] * p[1] + p[2] * p[2]).sqrt();
            for k in 0..4 {
                plane_data[i * 4 + k] = (p[k] / length) as f32;
            }
        }
        let u = &self.uniforms;
        r.queue
            .write_buffer(&u.frustum_planes, 0, bytemuck::cast_slice(&plane_data));
        let write = |buffer: &wgpu::Buffer, source: &str, name: &str, values: &[(&str, &[f64])]| {
            pack(source, name, values).map(|data| r.queue.write_buffer(buffer, 0, &data))
        };
        let (w, h) = (f64::from(t.width), f64::from(t.height));
        let [mode, raster, speed] = self.params;
        write(
            &u.frustum_render,
            FRUSTUM,
            "renderStruct",
            &[("nodeUniform7", &[self.time]), ("nodeUniform4", &[w, h])],
        )?;
        write(
            &u.frustum_object,
            FRUSTUM,
            "objectStruct",
            &[
                ("nodeUniform2", &[projection.y_axis.y]),
                ("nodeUniform3", &[position.x, position.y, position.z]),
                ("nodeUniform5", &[4.]),
                ("nodeUniform8", &[speed]),
                ("nodeUniform14", &m4(screen)),
                ("nodeUniform15", &[INSTANCES as f64]),
            ],
        )?;
        write(
            &u.quad_object,
            QUAD_FS,
            "objectStruct",
            &[("nodeUniform2", &[mode]), ("nodeUniform10", &[1.])],
        )?;
        write(
            &u.hw_render,
            HW_VS,
            "renderStruct",
            &[
                ("cameraProjectionMatrix", &m4(projection)),
                ("cameraViewMatrix", &m4(view)),
            ],
        )?;
        write(
            &u.hw_object,
            HW_VS,
            "objectStruct",
            &[
                ("nodeUniform5", &[mode]),
                ("nodeUniform10", &m4(Matrix4::IDENTITY)),
            ],
        )?;
        let b = &self.buffers;
        let mut encoder = r.device.create_command_encoder(&Default::default());
        {
            let mut pass = encoder.begin_compute_pass(&Default::default());
            let pixels = t.width * t.height;
            pass.set_pipeline(&self.pipelines[0]);
            pass.set_bind_group(0, &t.clear[0], &[]);
            pass.dispatch_workgroups(pixels.div_ceil(256), 1, 1);
            pass.set_pipeline(&self.pipelines[1]);
            for (i, g) in self.frustum_groups.iter().enumerate() {
                pass.set_bind_group(i as u32, g, &[]);
            }
            pass.dispatch_workgroups((INSTANCES as u32).div_ceil(64), 1, 1);
            pass.set_pipeline(&self.pipelines[2]);
            pass.set_bind_group(0, &self.dispatch_group, &[]);
            pass.dispatch_workgroups(1, 1, 1);
            pass.set_pipeline(&self.pipelines[3]);
            for (i, g) in t.rasterize.iter().enumerate() {
                pass.set_bind_group(i as u32, g, &[]);
            }
            pass.dispatch_workgroups_indirect(&b.dispatch, 0);
            pass.set_pipeline(&self.pipelines[4]);
            pass.set_bind_group(0, &self.hw_args_group, &[]);
            pass.dispatch_workgroups(1, 1, 1);
        }
        let (sw, hw) = (raster != 1., raster != 0.);
        let attachment = |load| {
            Some(wgpu::RenderPassColorAttachment {
                view: &t.color,
                depth_slice: None,
                resolve_target: None,
                ops: wgpu::Operations {
                    load,
                    store: wgpu::StoreOp::Store,
                },
            })
        };
        let depth = |load| {
            Some(wgpu::RenderPassDepthStencilAttachment {
                view: &t.depth,
                depth_ops: Some(wgpu::Operations {
                    load,
                    store: wgpu::StoreOp::Store,
                }),
                stencil_ops: None,
            })
        };
        if sw {
            // quadMesh.render( renderer ).
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("sw quad"),
                color_attachments: &[attachment(wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT))],
                depth_stencil_attachment: depth(wgpu::LoadOp::Clear(1.)),
                ..Default::default()
            });
            super::deferred::set(&mut pass, &t.quad);
            pass.set_vertex_buffer(0, b.quad_uv.slice(..));
            pass.draw(0..3, 0..1);
        }
        if hw {
            // renderer.render( hwScene, camera ): over the quad ( autoClear
            // false ), or cleared to the background in HW Only.
            let (color_load, depth_load) = if sw {
                (wgpu::LoadOp::Load, wgpu::LoadOp::Load)
            } else {
                (
                    wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.1,
                        g: 0.1,
                        b: 0.1,
                        a: 1.,
                    }),
                    wgpu::LoadOp::Clear(1.),
                )
            };
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("hw mesh"),
                color_attachments: &[attachment(color_load)],
                depth_stencil_attachment: depth(depth_load),
                ..Default::default()
            });
            super::deferred::set(&mut pass, &self.hw);
            pass.set_vertex_buffer(0, b.hw_positions.slice(..));
            pass.draw_indirect(&b.hw_draw, 0);
        }
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("rasterizer output"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &t.screen.view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                ..Default::default()
            });
            super::deferred::set(&mut pass, &t.output);
            pass.set_vertex_buffer(0, b.quad_position.slice(..));
            pass.draw(0..3, 0..1);
        }
        r.queue.submit([encoder.finish()]);
        Ok(true)
    }
    pub fn output(&self) -> Option<&RenderTarget> {
        self.targets.as_ref().map(|t| &t.screen)
    }
    pub fn draw(&mut self, kind: u32, x: f64, y: f64) {
        self.walker.pointer(kind, x, y);
    }
    pub fn key(&mut self, code: u32, down: bool) {
        self.walker.key(code, down);
    }
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
    /// Mode, Rasterizer and Animation Speed.
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        *self
            .params
            .get_mut(index)
            .ok_or(Error::Invalid("rasterizer parameter"))? = value as f64;
        Ok(())
    }
    pub fn seek(&mut self, t: f64) {
        self.time = t;
    }
}
const POSITION_ATTRIBUTE: [wgpu::VertexAttribute; 1] = [wgpu::VertexAttribute {
    format: wgpu::VertexFormat::Float32x3,
    offset: 0,
    shader_location: 0,
}];
const POSITION: wgpu::VertexBufferLayout<'static> = wgpu::VertexBufferLayout {
    array_stride: 12,
    step_mode: wgpu::VertexStepMode::Vertex,
    attributes: &POSITION_ATTRIBUTE,
};
