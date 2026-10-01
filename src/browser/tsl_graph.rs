//! webgpu_tsl_graph: four ShaderBall.glb models whose materials come from the
//! page's TSL graph file (`shaders/tsl-graphs.json`): triNoise3D on a
//! physical material, a fractal-noise moss on a standard material, animated
//! Worley noise on Phong and an animated, transparent fractal-noise basic
//! material. A directional light and the monochrome_studio PMREM light them
//! over the transparent grid ground, with 4× MSAA and the NeutralToneMapping
//! output pass. Every stage runs the WGSL three.js r186 generates for the
//! page (in `tsl_graph/`; the PMREM, mipmap and output modules are the retro,
//! shadowmap_opacity and compute_cloth ones, which are byte-identical).
use super::controls_attributes::{Controls, camera_state};
use super::deferred::{Draw, sampled_pipeline, set};
use super::gltf_viewer::fetch;
use super::lights_projector::{m3, m4, pack};
use super::pmrem_cube_uv::{Pmrem, bind, raw_pipeline, rgbe_pmrem};
use super::retro::{target, uniform};
use crate::{
    Error, Result, camera::*, geometry::*, math::*, render_target::*, renderer::*, scene::*,
};
use std::f64::consts::PI;
use wgpu::util::DeviceExt;

const HALF: wgpu::TextureFormat = wgpu::TextureFormat::Rgba16Float;
const DEPTH: wgpu::TextureFormat = wgpu::TextureFormat::Depth24Plus;
/// Color( 0x444444 ) in linear space: the background clear.
const BACKGROUND: f64 = 0.057805430183792694;
macro_rules! wgsl {
    ($name:literal) => {
        include_str!(concat!("tsl_graph/", $name, ".wgsl"))
    };
}
const OUTPUT_VS: &str = include_str!("compute_cloth/output_vs.wgsl");
const OUTPUT_FS: &str = include_str!("compute_cloth/output_fs.wgsl");
/// The materials in the page's order: physical, standard, Phong and basic.
#[derive(Clone, Copy, PartialEq)]
enum Kind {
    Physical,
    Standard,
    Phong,
    Basic,
}
const KINDS: [Kind; 4] = [Kind::Physical, Kind::Standard, Kind::Phong, Kind::Basic];
impl Kind {
    fn shaders(self) -> (&'static str, &'static str) {
        match self {
            Kind::Physical => (wgsl!("physical_vs"), wgsl!("physical_fs")),
            Kind::Standard => (wgsl!("standard_vs"), wgsl!("standard_fs")),
            Kind::Phong => (wgsl!("phong_vs"), wgsl!("phong_fs")),
            Kind::Basic => (wgsl!("basic_vs"), wgsl!("basic_fs")),
        }
    }
    /// The module declaring the object struct.
    fn object_source(self) -> &'static str {
        if self == Kind::Basic {
            wgsl!("basic_vs")
        } else {
            self.shaders().1
        }
    }
}
/// One of the ShaderBall's two meshes: the interleaved quantized vertices
/// (snorm16 position, normal and unorm16 uv in 20-byte strides, as
/// KHR_mesh_quantization stores and three.js uploads them) and the 16-bit
/// index, its node transform and local bounding sphere.
struct Part {
    vertices: wgpu::Buffer,
    index: wgpu::Buffer,
    count: u32,
    node: Matrix4,
    sphere: Sphere,
}
/// A shader ball's mesh: its part (0 calibration, 1 preview), material,
/// world model and object struct.
struct Ball {
    part: usize,
    kind: Kind,
    model: Matrix4,
    object: wgpu::Buffer,
}
struct Targets {
    width: u32,
    height: u32,
    format: wgpu::TextureFormat,
    samples: u32,
    msaa: Option<(wgpu::TextureView, wgpu::TextureView)>,
    color: wgpu::TextureView,
    depth: wgpu::TextureView,
    /// Per ball, its draw; then the ground's.
    balls: Vec<Draw>,
    ground: Draw,
    output: Draw,
    screen: RenderTarget,
}
pub(super) struct Demo {
    controls: Controls,
    /// The example clock: TSL time.
    time: f64,
    /// Calibration and preview mesh visibility.
    params: [f64; 2],
    parts: [Part; 2],
    balls: Vec<Ball>,
    ground: (wgpu::Buffer, wgpu::Buffer, u32, wgpu::Buffer),
    pmrem: Pmrem,
    renders: Vec<wgpu::Buffer>,
    ground_render: wgpu::Buffer,
    output_render: wgpu::Buffer,
    output_object: wgpu::Buffer,
    output_quad: wgpu::Buffer,
    clamp: wgpu::Sampler,
    targets: Option<Targets>,
}
/// The glTF bufferView `view` of buffer 0.
fn view_bytes<'a>(asset: &gltf::Gltf, buffers: &'a [Vec<u8>], view: usize) -> Result<&'a [u8]> {
    let v = asset
        .views()
        .nth(view)
        .ok_or(Error::Invalid("ShaderBall view"))?;
    buffers
        .first()
        .and_then(|b| b.get(v.offset()..v.offset() + v.length()))
        .ok_or(Error::Invalid("ShaderBall buffer"))
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
            far: 200.,
            aspect,
            ..Default::default()
        }));
        s.get_mut(c)?.position = Vector3::new(3., 5., 8.);
        let mut controls = Controls::new(Some(0.05), (2., 40.), PI, true);
        controls.set_target(Vector3::ZERO);
        controls.update(s, c)?;
        let init = |label, data: &[u8], usage| {
            r.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some(label),
                    contents: data,
                    usage,
                })
        };
        // The GLB as stored: its quantized views are uploaded unchanged.
        let asset = gltf::Gltf::from_slice_without_validation(
            &fetch("/web/gallery/assets/gltf/ShaderBall.glb").await?,
        )
        .map_err(|e| Error::Asset(e.to_string()))?;
        let buffers = vec![
            asset
                .blob
                .clone()
                .ok_or(Error::Invalid("ShaderBall blob"))?,
        ];
        // Mesh 1 is Calibration_Mesh and mesh 0 Preview_Mesh, each indexed
        // by its own accessor into the shared index view.
        let mut parts = vec![];
        for name in ["Calibration_Mesh", "Preview_Mesh"] {
            let node = asset
                .nodes()
                .find(|n| n.name() == Some(name))
                .ok_or(Error::Invalid("ShaderBall node"))?;
            let primitive = node
                .mesh()
                .and_then(|m| m.primitives().next())
                .ok_or(Error::Invalid("ShaderBall mesh"))?;
            let position = primitive
                .get(&gltf::Semantic::Positions)
                .ok_or(Error::Invalid("ShaderBall position"))?;
            let view = position.view().ok_or(Error::Invalid("ShaderBall view"))?;
            let vertices = view_bytes(&asset, &buffers, view.index())?;
            let indices = primitive
                .indices()
                .ok_or(Error::Invalid("ShaderBall index"))?;
            let index_view = indices
                .view()
                .ok_or(Error::Invalid("ShaderBall index view"))?;
            let index_bytes = view_bytes(&asset, &buffers, index_view.index())?;
            let start = indices.offset();
            let index = &index_bytes[start..start + indices.count() * 2];
            // The dequantized positions' bounding sphere.
            let points: Vec<Vector3> = vertices
                .chunks_exact(20)
                .map(|v| {
                    let c = |k: usize| i16::from_le_bytes([v[k * 2], v[k * 2 + 1]]) as f64 / 32767.;
                    Vector3::new(c(0).max(-1.), c(1).max(-1.), c(2).max(-1.))
                })
                .collect();
            let (lo, hi) = points.iter().fold(
                (
                    Vector3::splat(f64::INFINITY),
                    Vector3::splat(f64::NEG_INFINITY),
                ),
                |(lo, hi), p| (lo.min(*p), hi.max(*p)),
            );
            let center = (lo + hi) * 0.5;
            let (t, q, sc) = node.transform().decomposed();
            parts.push(Part {
                vertices: init("ShaderBall vertices", vertices, wgpu::BufferUsages::VERTEX),
                // A copy keeps the 4-byte size alignment the index buffer needs.
                index: init(
                    "ShaderBall index",
                    &[index, &[0, 0][..index.len() % 4]].concat(),
                    wgpu::BufferUsages::INDEX,
                ),
                count: indices.count() as u32,
                node: Matrix4::from_scale_rotation_translation(
                    Vector3::new(sc[0] as f64, sc[1] as f64, sc[2] as f64),
                    Quaternion::from_xyzw(q[0] as f64, q[1] as f64, q[2] as f64, q[3] as f64),
                    Vector3::new(t[0] as f64, t[1] as f64, t[2] as f64),
                ),
                sphere: Sphere {
                    center,
                    radius: points.iter().map(|p| p.distance(center)).fold(0., f64::max),
                },
            });
        }
        let parts: [Part; 2] = parts
            .try_into()
            .map_err(|_| Error::Invalid("ShaderBall parts"))?;
        // createShaderBall( materials[ i ], ( ( i & 1 ) * 4 - 2, 0, ( i & 2 ) * 2 - 2 ) ).
        let mut balls = vec![];
        for (i, kind) in KINDS.into_iter().enumerate() {
            let position = Vector3::new((i & 1) as f64 * 4. - 2., 0., (i & 2) as f64 * 2. - 2.);
            for (p, part) in parts.iter().enumerate() {
                let model = Matrix4::from_translation(position) * part.node;
                let object = uniform(r, "tsl graph object", kind.object_source(), "objectStruct")?;
                let values: Vec<(&str, Vec<f64>)> = match kind {
                    Kind::Physical => vec![
                        ("nodeUniform0", m4(model)),
                        ("nodeUniform1", vec![1.]),
                        ("nodeUniform3", m3(model.inverse().transpose())),
                        ("nodeUniform4", vec![1.5]),
                        ("nodeUniform5", vec![1.; 3]),
                        ("nodeUniform6", vec![1.]),
                        ("nodeUniform7", vec![0.; 3]),
                        ("nodeUniform8", vec![1.]),
                        ("nodeUniform14", vec![8.]),
                        ("nodeUniform15", m4(Matrix4::IDENTITY)),
                        ("nodeUniform17", vec![1. / 768.]),
                        ("nodeUniform18", vec![1. / 1024.]),
                        ("nodeUniform20", vec![1.]),
                    ],
                    Kind::Standard => vec![
                        ("nodeUniform0", m4(model)),
                        ("nodeUniform2", m3(model.inverse().transpose())),
                        ("nodeUniform4", vec![1.]),
                        ("nodeUniform5", vec![0.; 3]),
                        ("nodeUniform6", vec![1.]),
                        ("nodeUniform11", vec![8.]),
                        ("nodeUniform12", m4(Matrix4::IDENTITY)),
                        ("nodeUniform14", vec![1. / 768.]),
                        ("nodeUniform15", vec![1. / 1024.]),
                        ("nodeUniform17", vec![1.]),
                    ],
                    Kind::Phong => vec![
                        ("nodeUniform0", m4(model)),
                        ("nodeUniform2", vec![30.]),
                        (
                            "nodeUniform3",
                            Color::from_hex(0x111111).0.to_array().to_vec(),
                        ),
                        ("nodeUniform4", vec![0.; 3]),
                        ("nodeUniform5", vec![1.]),
                        ("nodeUniform7", m3(model.inverse().transpose())),
                    ],
                    Kind::Basic => vec![
                        ("nodeUniform1", m3(model.inverse().transpose())),
                        ("nodeUniform5", m4(model)),
                    ],
                };
                let values: Vec<(&str, &[f64])> =
                    values.iter().map(|(n, v)| (*n, &v[..])).collect();
                r.queue.write_buffer(
                    &object,
                    0,
                    &pack(kind.object_source(), "objectStruct", &values)?,
                );
                balls.push(Ball {
                    part: p,
                    kind,
                    model,
                    object,
                });
            }
        }
        // The ground: CircleGeometry( 40 ) turned flat.
        let circle = CircleGeometry::build(40., 32, 0., 2. * PI)?;
        let positions = circle
            .attributes
            .get("position")
            .ok_or(Error::Invalid("circle"))?;
        let positions: Vec<f32> = (0..positions.count())
            .flat_map(|i| (0..3).map(move |k| (i, k)))
            .map(|(i, k)| positions.get_component(i, k).map(|v| v as f32))
            .collect::<Result<_>>()?;
        let circle_index = circle.index.clone().ok_or(Error::Invalid("circle index"))?;
        let ground_object = uniform(r, "tsl graph ground", wgsl!("ground_fs"), "objectStruct")?;
        r.queue.write_buffer(
            &ground_object,
            0,
            &pack(
                wgsl!("ground_fs"),
                "objectStruct",
                &[
                    ("nodeUniform0", &m4(Matrix4::from_rotation_x(-PI / 2.))),
                    ("nodeUniform1", &[1.]),
                ],
            )?,
        );
        let ground = (
            init(
                "ground positions",
                bytemuck::cast_slice(&positions),
                wgpu::BufferUsages::VERTEX,
            ),
            init(
                "ground index",
                bytemuck::cast_slice(&circle_index),
                wgpu::BufferUsages::INDEX,
            ),
            circle_index.len() as u32,
            ground_object,
        );
        let clamp = r.device.create_sampler(&wgpu::SamplerDescriptor {
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        // scene.environment: the equirect HDR through PMREMGenerator.fromEquirectangular.
        let (pmrem, _) =
            rgbe_pmrem(r, &clamp, "/web/environments/monochrome_studio_02_1k.hdr").await?;
        Ok(Self {
            controls,
            time: 0.,
            params: [1., 1.],
            parts,
            balls,
            ground,
            pmrem,
            renders: KINDS
                .iter()
                .map(|k| uniform(r, "tsl graph render", k.shaders().1, "renderStruct"))
                .collect::<Result<_>>()?,
            ground_render: uniform(
                r,
                "tsl graph ground render",
                wgsl!("ground_vs"),
                "renderStruct",
            )?,
            output_render: uniform(r, "tsl graph output render", OUTPUT_FS, "renderStruct")?,
            output_object: uniform(r, "tsl graph output object", OUTPUT_VS, "objectStruct")?,
            output_quad: init(
                "tsl graph quad",
                bytemuck::cast_slice(&[-1f32, 3., 0., -1., -1., 0., 3., -1., 0.]),
                wgpu::BufferUsages::VERTEX,
            ),
            clamp,
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
        let size = (out.width, out.height);
        let samples = if out.options.samples > 1 { 4 } else { 1 };
        let snorm = wgpu::VertexFormat::Snorm16x4;
        // Position then normal, or (the basic material) normal then position.
        let ordered = [
            wgpu::vertex_attr_array![0 => Snorm16x4, 1 => Snorm16x4],
            [
                wgpu::VertexAttribute {
                    format: snorm,
                    offset: 8,
                    shader_location: 0,
                },
                wgpu::VertexAttribute {
                    format: snorm,
                    offset: 0,
                    shader_location: 1,
                },
            ],
        ];
        let ball_layout = |i: usize| wgpu::VertexBufferLayout {
            array_stride: 20,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &ordered[i],
        };
        let depth = Some((wgpu::CompareFunction::LessEqual, true));
        let pipelines: Vec<wgpu::RenderPipeline> = KINDS
            .iter()
            .map(|&kind| {
                let basic = kind == Kind::Basic;
                let layouts = [ball_layout(usize::from(basic))];
                sampled_pipeline(
                    r,
                    "tsl graph ball",
                    kind.shaders(),
                    &layouts,
                    &[HALF],
                    depth,
                    (false, basic),
                    (samples, wgpu::PrimitiveTopology::TriangleList),
                )
            })
            .collect();
        let tex = wgpu::BindingResource::TextureView;
        let sampler = wgpu::BindingResource::Sampler;
        let balls = self
            .balls
            .iter()
            .map(|b| {
                let i = KINDS.iter().position(|k| *k == b.kind).unwrap_or(0);
                let p = &pipelines[i];
                let mut entries = vec![(0, b.object.as_entire_binding())];
                if matches!(b.kind, Kind::Physical | Kind::Standard) {
                    entries.extend([
                        (1, sampler(&self.clamp)),
                        (2, tex(&r.dfg)),
                        (3, sampler(&self.clamp)),
                        (4, tex(&self.pmrem.view)),
                    ]);
                }
                (
                    p.clone(),
                    vec![
                        bind(
                            r,
                            p.get_bind_group_layout(0),
                            &[(0, self.renders[i].as_entire_binding())],
                        ),
                        bind(r, p.get_bind_group_layout(1), &entries),
                    ],
                )
            })
            .collect();
        let position = [wgpu::vertex_attr_array![0 => Float32x3]];
        let ground_pipeline = sampled_pipeline(
            r,
            "tsl graph ground",
            (wgsl!("ground_vs"), wgsl!("ground_fs")),
            &[wgpu::VertexBufferLayout {
                array_stride: 12,
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes: &position[0],
            }],
            &[HALF],
            depth,
            (false, true),
            (samples, wgpu::PrimitiveTopology::TriangleList),
        );
        let ground = (
            ground_pipeline.clone(),
            vec![
                bind(
                    r,
                    ground_pipeline.get_bind_group_layout(0),
                    &[(0, self.ground_render.as_entire_binding())],
                ),
                bind(
                    r,
                    ground_pipeline.get_bind_group_layout(1),
                    &[(0, self.ground.3.as_entire_binding())],
                ),
            ],
        );
        let color = target(r, size, HALF, 1);
        let quad = [wgpu::vertex_attr_array![0 => Float32x3]];
        let output_pipeline = raw_pipeline(
            r,
            "tsl graph output",
            OUTPUT_VS,
            OUTPUT_FS,
            &[wgpu::VertexBufferLayout {
                array_stride: 12,
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes: &quad[0],
            }],
            out.options.format,
            1,
            None,
            false,
        );
        let output = (
            output_pipeline.clone(),
            vec![
                bind(
                    r,
                    output_pipeline.get_bind_group_layout(0),
                    &[(0, self.output_render.as_entire_binding())],
                ),
                bind(
                    r,
                    output_pipeline.get_bind_group_layout(1),
                    &[
                        (0, sampler(&self.clamp)),
                        (1, tex(&color)),
                        (2, self.output_object.as_entire_binding()),
                    ],
                ),
            ],
        );
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
            depth: target(r, size, DEPTH, 1),
            color,
            balls,
            ground,
            output,
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
        let t = self
            .targets
            .as_ref()
            .ok_or(Error::Invalid("tsl graph targets"))?;
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
        // The directional light at ( 1, 1, 1 ) toward the origin, white at intensity 1.
        let light = [1.; 3].to_vec();
        for (kind, buffer) in KINDS.iter().zip(&self.renders) {
            let mut values = vec![
                ("cameraProjectionMatrix", m4(projection)),
                ("cameraViewMatrix", m4(view)),
                ("cameraWorldMatrix", m4(world)),
            ];
            match kind {
                Kind::Physical => values.extend([
                    ("nodeUniform13", light.clone()),
                    ("nodeUniform11", light.clone()),
                    ("nodeUniform12", vec![0.; 3]),
                ]),
                Kind::Standard => values.extend([
                    ("nodeUniform10", light.clone()),
                    ("nodeUniform8", light.clone()),
                    ("nodeUniform9", vec![0.; 3]),
                ]),
                Kind::Phong => values.extend([
                    ("nodeUniform1", vec![self.time]),
                    ("nodeUniform11", light.clone()),
                    ("nodeUniform9", light.clone()),
                    ("nodeUniform10", vec![0.; 3]),
                ]),
                Kind::Basic => values.push(("nodeUniform3", vec![self.time])),
            }
            write(buffer, kind.shaders().1, "renderStruct", &values)?;
        }
        write(
            &self.ground_render,
            wgsl!("ground_vs"),
            "renderStruct",
            &[
                ("cameraProjectionMatrix", m4(projection)),
                ("cameraViewMatrix", m4(view)),
            ],
        )?;
        write(
            &self.output_render,
            OUTPUT_FS,
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
                ("nodeUniform2", vec![0.9]),
            ],
        )?;
        write(
            &self.output_object,
            OUTPUT_VS,
            "objectStruct",
            &[("nodeUniform5", m4(Matrix4::IDENTITY))],
        )?;
        let frustum = Frustum::from_projection(projection * view);
        let visible = |b: &Ball| {
            let part = &self.parts[b.part];
            self.params[b.part] > 0.5
                && frustum.intersects_sphere(Sphere {
                    center: b.model.transform_point3(part.sphere.center),
                    radius: part.sphere.radius
                        * b.model.to_scale_rotation_translation().0.max_element(),
                })
        };
        let mut encoder = r.device.create_command_encoder(&Default::default());
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("tsl graph scene"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: t.msaa.as_ref().map_or(&t.color, |m| &m.0),
                    depth_slice: None,
                    resolve_target: t.msaa.as_ref().map(|_| &t.color),
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: BACKGROUND,
                            g: BACKGROUND,
                            b: BACKGROUND,
                            a: 1.,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: t.msaa.as_ref().map_or(&t.depth, |m| &m.1),
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.),
                        store: wgpu::StoreOp::Discard,
                    }),
                    stencil_ops: None,
                }),
                ..Default::default()
            });
            let draw = |pass: &mut wgpu::RenderPass, b: &Ball, d: &Draw| {
                let part = &self.parts[b.part];
                set(pass, d);
                pass.set_vertex_buffer(0, part.vertices.slice(..));
                pass.set_index_buffer(part.index.slice(..), wgpu::IndexFormat::Uint16);
                pass.draw_indexed(0..part.count, 0, 0..1);
            };
            // Opaque balls, then the transparent ground ( renderOrder -1 ) and
            // the basic material's calibration ( 1 ) and preview ( 2 ) meshes.
            for (b, d) in self.balls.iter().zip(&t.balls) {
                if b.kind != Kind::Basic && visible(b) {
                    draw(&mut pass, b, d);
                }
            }
            set(&mut pass, &t.ground);
            pass.set_vertex_buffer(0, self.ground.0.slice(..));
            pass.set_index_buffer(self.ground.1.slice(..), wgpu::IndexFormat::Uint32);
            pass.draw_indexed(0..self.ground.2, 0, 0..1);
            for (b, d) in self.balls.iter().zip(&t.balls) {
                if b.kind == Kind::Basic && visible(b) {
                    draw(&mut pass, b, d);
                }
            }
        }
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("tsl graph output"),
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
        Ok(())
    }
    /// Calibration and preview mesh visibility.
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        *self
            .params
            .get_mut(index)
            .ok_or(Error::Invalid("tsl graph parameter"))? = value as f64;
        Ok(())
    }
    pub fn seek(&mut self, t: f64) {
        self.time = t;
    }
}
