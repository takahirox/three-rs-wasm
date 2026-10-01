//! webgpu_shadowmap_csm: two rows of forty Phong boxes on a large floor under
//! an ambient light, a dim blue directional light and a white directional
//! light whose shadow is a CSMShadowNode: four cascades split along the view
//! frustum (uniform, logarithmic or practical), each with its own 2048²
//! orthographic shadow camera fitted around its slice and snapped to shadow
//! texels, as the addon computes them on the CPU each frame. The receivers
//! pick their cascade by orthographic depth. The CSMHelper (frustum, cascade
//! boxes, planes and shadow bounds) and the orthographic camera are
//! available. Every stage runs the WGSL three.js r186 generates for the page
//! (in `shadowmap_csm/`).
use super::controls_attributes::{Controls, camera_state};
use super::lights_projector::{m3, m4, pack};
use crate::{
    Error, Result, camera::*, geometry::*, math::*, render_target::*, renderer::*, scene::*,
};
use std::f64::consts::PI;
use wgpu::util::DeviceExt;

const HALF: wgpu::TextureFormat = wgpu::TextureFormat::Rgba16Float;
const DEPTH: wgpu::TextureFormat = wgpu::TextureFormat::Depth24Plus;
const CASCADES: usize = 4;
const SHADOW_SIZE: u32 = 2048;
macro_rules! wgsl {
    ($name:literal) => {
        include_str!(concat!("shadowmap_csm/", $name, ".wgsl"))
    };
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
            label: Some("csm target"),
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
/// An orthographic projection in WebGPU clip space.
fn orthographic(left: f64, right: f64, top: f64, bottom: f64, near: f64, far: f64) -> Matrix4 {
    Matrix4::from_cols_array(&[
        2. / (right - left),
        0.,
        0.,
        0.,
        0.,
        2. / (top - bottom),
        0.,
        0.,
        0.,
        0.,
        -1. / (far - near),
        0.,
        -(right + left) / (right - left),
        -(top + bottom) / (top - bottom),
        -near / (far - near),
        1.,
    ])
}
/// Matrix4.lookAt( eye, target, up ): the rotation whose z axis points from
/// the target to the eye.
pub(super) fn look_rotation(eye: Vector3, target: Vector3, up: Vector3) -> Matrix4 {
    let mut z = eye - target;
    if z.length_squared() == 0. {
        z.z = 1.;
    }
    z = z.normalize();
    let mut x = up.cross(z);
    if x.length_squared() == 0. {
        if up.z.abs() == 1. {
            z.x += 0.0001;
        } else {
            z.z += 0.0001;
        }
        z = z.normalize();
        x = up.cross(z);
    }
    x = x.normalize();
    let y = z.cross(x);
    Matrix4::from_cols(x.extend(0.), y.extend(0.), z.extend(0.), glam::DVec4::W)
}
/// CSMFrustum: the near and far corners (3 0 / 2 1 order) in view space.
#[derive(Clone, Copy, Default)]
struct Corners {
    near: [Vector3; 4],
    far: [Vector3; 4],
}
const CORNERS: [(f64, f64); 4] = [(1., 1.), (1., -1.), (-1., -1.), (-1., 1.)];
/// CSMShadowNode's state for one frame.
struct Cascades {
    main: Corners,
    frustums: [Corners; CASCADES],
    /// Each cascade's [start, end] in orthographic depth.
    ranges: [(f64, f64); CASCADES],
    /// Each shadow camera's square width and view matrix.
    width: [f64; CASCADES],
    view: [Matrix4; CASCADES],
}
/// Mesh geometry: vertex buffers, index and bounding radius about its origin.
struct Geometry {
    positions: wgpu::Buffer,
    normals: Option<wgpu::Buffer>,
    index: wgpu::Buffer,
    count: u32,
    radius: f64,
}
/// A Phong mesh: geometry index, color and model matrix.
struct Shape {
    geometry: usize,
    color: u32,
    model: Matrix4,
    phong: wgpu::Buffer,
    depth: wgpu::Buffer,
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
    lines: (wgpu::RenderPipeline, wgpu::BindGroup, Vec<wgpu::BindGroup>),
    /// Back then front faces.
    planes: [(wgpu::RenderPipeline, wgpu::BindGroup, Vec<wgpu::BindGroup>); 2],
    output: (wgpu::RenderPipeline, [wgpu::BindGroup; 2]),
}
/// What the helper drew at its last update.
#[derive(Clone, Copy)]
struct HelperState {
    frustum: [Vector3; 8],
    camera: Matrix4,
    boxes: [Matrix4; CASCADES],
    planes: [Matrix4; CASCADES],
    shadows: [Matrix4; CASCADES],
}
pub(super) struct Demo {
    controls: Controls,
    /// orthographic, shadows, maxFar, mode, light x, y, z, margin, shadow
    /// near, shadow far, helper visible, display frustum, display planes,
    /// display shadow bounds, auto update.
    params: [f64; 15],
    margin: f64,
    helper_pending: bool,
    helper: Option<HelperState>,
    geometries: Vec<Geometry>,
    shapes: Vec<Shape>,
    shadow_maps: Vec<(wgpu::TextureView, wgpu::TextureView)>,
    depth_pipeline: wgpu::RenderPipeline,
    shadow_renders: Vec<(wgpu::Buffer, wgpu::BindGroup)>,
    depth_objects: Vec<wgpu::BindGroup>,
    phong_render: wgpu::Buffer,
    cascades: wgpu::Buffer,
    line_render: wgpu::Buffer,
    /// Frustum lines, the cascade boxes, the shadow boxes.
    line_objects: Vec<wgpu::Buffer>,
    plane_render: wgpu::Buffer,
    plane_objects: Vec<wgpu::Buffer>,
    frustum_positions: wgpu::Buffer,
    output_render: wgpu::Buffer,
    output_object: wgpu::Buffer,
    quad: wgpu::Buffer,
    sampler: wgpu::Sampler,
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
            fov: 70.,
            near: 0.1,
            far: 5000.,
            aspect,
            ..Default::default()
        }));
        s.get_mut(c)?.position = Vector3::new(60., 60., 0.);
        let mut controls = Controls::new(None, (0., f64::INFINITY), PI / 2., true);
        controls.set_target(Vector3::new(-100., 10., 0.));
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
        let geometry = |g: BufferGeometry, normals: bool| -> Result<Geometry> {
            let f = |name: &str| -> Result<Vec<f32>> {
                let a = g
                    .attributes
                    .get(name)
                    .ok_or(Error::Invalid("csm attribute"))?;
                (0..a.count())
                    .flat_map(|i| (0..3).map(move |k| (i, k)))
                    .map(|(i, k)| a.get_component(i, k).map(|v| v as f32))
                    .collect()
            };
            let positions = f("position")?;
            let index = g.index.clone().ok_or(Error::Invalid("csm index"))?;
            Ok(Geometry {
                radius: positions
                    .chunks(3)
                    .map(|p| (p[0] as f64).hypot(p[1] as f64).hypot(p[2] as f64))
                    .fold(0., f64::max),
                positions: init("csm positions", bytemuck::cast_slice(&positions), vertex),
                normals: if normals {
                    Some(init(
                        "csm normals",
                        bytemuck::cast_slice(&f("normal")?),
                        vertex,
                    ))
                } else {
                    None
                },
                index: init(
                    "csm index",
                    bytemuck::cast_slice(&index),
                    wgpu::BufferUsages::INDEX,
                ),
                count: index.len() as u32,
            })
        };
        // Box3Helper's unit box: eight corners, twelve edges.
        let corners: [f32; 24] = [
            1., 1., 1., -1., 1., 1., -1., -1., 1., 1., -1., 1., 1., 1., -1., -1., 1., -1., -1.,
            -1., -1., 1., -1., -1.,
        ];
        let edges: [u32; 24] = [
            0, 1, 1, 2, 2, 3, 3, 0, 4, 5, 5, 6, 6, 7, 7, 4, 0, 4, 1, 5, 2, 6, 3, 7,
        ];
        let geometries = vec![
            geometry(PlaneGeometry::build(10000., 10000., 8, 8)?, true)?,
            geometry(BoxGeometry::build(10., 10., 10.)?, true)?,
            Geometry {
                positions: init("box helper", bytemuck::cast_slice(&corners), vertex),
                normals: None,
                index: init(
                    "box helper index",
                    bytemuck::cast_slice(&edges),
                    wgpu::BufferUsages::INDEX,
                ),
                count: 24,
                radius: 3f64.sqrt(),
            },
            geometry(PlaneGeometry::build(1., 1., 1, 1)?, false)?,
        ];
        // The seeded Math.random: each pair's heights.
        let mut seed = 186u32;
        let mut random = || {
            seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
            seed as f64 / 4294967296.
        };
        let shape = |geometry, color, model| Shape {
            geometry,
            color,
            model,
            phong: uniform(r, "phong object", 176),
            depth: uniform(r, "depth object", 80),
        };
        let mut shapes = vec![shape(0, 0x252a34, Matrix4::from_rotation_x(-PI / 2.))];
        let (cyan, pink) = (0x08d9d6, 0xff2e63);
        for i in 0..40 {
            let x = -(i as f64) * 25.;
            let (first, second) = if i % 2 == 0 {
                (cyan, pink)
            } else {
                (pink, cyan)
            };
            for (z, color) in [(30., first), (-30., second)] {
                let height = random() * 2. + 6.;
                shapes.push(shape(
                    1,
                    color,
                    Matrix4::from_translation(Vector3::new(x, 20., z))
                        * Matrix4::from_scale(Vector3::new(1., height, 1.)),
                ));
            }
        }
        let module = |label, source: &str| {
            r.device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some(label),
                source: wgpu::ShaderSource::Wgsl(source.into()),
            })
        };
        let position = wgpu::vertex_attr_array![0 => Float32x3];
        let depth_pipeline = r
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("csm shadow"),
                layout: None,
                vertex: wgpu::VertexState {
                    module: &module("shadow", include_str!("shadowmap_vsm/depth_vs.wgsl")),
                    entry_point: Some("main"),
                    compilation_options: Default::default(),
                    buffers: &[wgpu::VertexBufferLayout {
                        array_stride: 12,
                        step_mode: wgpu::VertexStepMode::Vertex,
                        attributes: &position,
                    }],
                },
                fragment: Some(wgpu::FragmentState {
                    module: &module("shadow", include_str!("shadowmap_vsm/depth_fs.wgsl")),
                    entry_point: Some("main"),
                    compilation_options: Default::default(),
                    targets: &[Some(wgpu::TextureFormat::Rgba8Unorm.into())],
                }),
                // As the page's shadow pipeline: clockwise front faces.
                primitive: wgpu::PrimitiveState {
                    front_face: wgpu::FrontFace::Cw,
                    cull_mode: Some(wgpu::Face::Back),
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
            });
        let shadow_size = (SHADOW_SIZE, SHADOW_SIZE);
        let shadow_maps = (0..CASCADES)
            .map(|_| {
                (
                    texture(r, shadow_size, wgpu::TextureFormat::Rgba8Unorm, 1, false),
                    texture(r, shadow_size, DEPTH, 1, true),
                )
            })
            .collect();
        let shadow_renders = (0..CASCADES)
            .map(|_| {
                let buffer = uniform(r, "cascade camera", 128);
                let group = bind(
                    r,
                    depth_pipeline.get_bind_group_layout(0),
                    &[(0, buffer.as_entire_binding())],
                );
                (buffer, group)
            })
            .collect();
        let depth_objects = shapes
            .iter()
            .map(|shape| {
                bind(
                    r,
                    depth_pipeline.get_bind_group_layout(1),
                    &[(0, shape.depth.as_entire_binding())],
                )
            })
            .collect();
        Ok(Self {
            controls,
            params: [
                0., 1., 1000., 2., -1., -1., -1., 100., 1., 2000., 0., 1., 1., 1., 1.,
            ],
            // CSMShadowNode's lightMargin starts at 200; the GUI's 100 applies
            // once the control changes.
            margin: 200.,
            helper_pending: false,
            helper: None,
            geometries,
            shapes,
            shadow_maps,
            depth_pipeline,
            shadow_renders,
            depth_objects,
            phong_render: uniform(
                r,
                "phong render",
                pack(wgsl!("phong_fs"), "renderStruct", &[])?.len() as u64,
            ),
            cascades: uniform(r, "cascades", 64),
            line_render: uniform(r, "line render", 128),
            line_objects: (0..1 + 2 * CASCADES)
                .map(|_| uniform(r, "line object", 80))
                .collect(),
            plane_render: uniform(r, "plane render", 144),
            plane_objects: (0..CASCADES)
                .map(|_| uniform(r, "plane object", 80))
                .collect(),
            frustum_positions: r.device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("frustum lines"),
                size: 96,
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            }),
            output_render: uniform(r, "output render", 144),
            output_object: uniform(r, "output object", 64),
            quad: init(
                "quad",
                bytemuck::cast_slice(&[-1f32, 3., 0., -1., -1., 0., 3., -1., 0.]),
                vertex,
            ),
            sampler: r.device.create_sampler(&wgpu::SamplerDescriptor {
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
    /// The main light: its direction from the controls, 200 from the origin.
    fn light_position(&self) -> Vector3 {
        Vector3::new(self.params[4], self.params[5], self.params[6]).normalize() * -200.
    }
    /// updateFrustums() and updateBefore(): the splits, the slices, each shadow
    /// camera's width and its texel-snapped placement.
    fn cascades(&self, projection: Matrix4, near: f64, far: f64, camera: Matrix4) -> Cascades {
        let max_far = self.params[2];
        let far_split = far.min(max_far);
        let amount = CASCADES as f64;
        let uniform_split = |i: f64| (near + (far_split - near) * i / amount) / far_split;
        let log_split = |i: f64| (near * (far_split / near).powf(i / amount)) / far_split;
        let mut breaks = [1.; CASCADES];
        for (i, b) in breaks.iter_mut().enumerate().take(CASCADES - 1) {
            let i = (i + 1) as f64;
            *b = match self.params[3].round() as usize {
                0 => uniform_split(i),
                1 => log_split(i),
                _ => uniform_split(i) + (log_split(i) - uniform_split(i)) * 0.5,
            };
        }
        // CSMFrustum.setFromProjectionMatrix: WebGPU clip depth spans 0..1.
        let inverse = projection.inverse();
        let orthographic = projection.z_axis.w == 0.;
        let mut main = Corners::default();
        for (j, (x, y)) in CORNERS.iter().enumerate() {
            main.near[j] = inverse.project_point3(Vector3::new(*x, *y, 0.));
            let mut v = inverse.project_point3(Vector3::new(*x, *y, 1.));
            let scale = (max_far / v.z.abs()).min(1.);
            if orthographic {
                v.z *= scale;
            } else {
                v *= scale;
            }
            main.far[j] = v;
        }
        let (z_near, z_far) = (main.near[0].z, main.far[0].z);
        let mut frustums = [Corners::default(); CASCADES];
        for (i, cascade) in frustums.iter_mut().enumerate() {
            let lerp = |alpha: f64, j: usize| main.near[j] + (main.far[j] - main.near[j]) * alpha;
            for j in 0..4 {
                cascade.near[j] = if i == 0 {
                    main.near[j]
                } else {
                    lerp((breaks[i - 1] * z_far - z_near) / (z_far - z_near), j)
                };
                cascade.far[j] = if i == CASCADES - 1 {
                    main.far[j]
                } else {
                    lerp((breaks[i] * z_far - z_near) / (z_far - z_near), j)
                };
            }
        }
        let mut width = [0.; CASCADES];
        for (i, f) in frustums.iter().enumerate() {
            let point1 = f.far[0];
            let point2 = if point1.distance(f.far[2]) > point1.distance(f.near[2]) {
                f.far[2]
            } else {
                f.near[2]
            };
            width[i] = point1.distance(point2);
        }
        let ranges = std::array::from_fn(|i| (if i == 0 { 0. } else { breaks[i - 1] }, breaks[i]));
        let light = self.light_position();
        let direction = (Vector3::ZERO - light).normalize();
        let orientation = look_rotation(light, Vector3::ZERO, Vector3::Y);
        let to_light = orientation.inverse() * camera;
        let mut view = [Matrix4::IDENTITY; CASCADES];
        for i in 0..CASCADES {
            let texel = width[i] / SHADOW_SIZE as f64;
            let (mut lo, mut hi) = (Vector3::splat(f64::MAX), Vector3::splat(f64::MIN));
            for p in frustums[i].near.iter().chain(&frustums[i].far) {
                let p = to_light.transform_point3(*p);
                lo = lo.min(p);
                hi = hi.max(p);
            }
            let mut c = (lo + hi) / 2.;
            c.z = hi.z + self.margin;
            c.x = (c.x / texel).floor() * texel;
            c.y = (c.y / texel).floor() * texel;
            let c = orientation.transform_point3(c);
            view[i] = Matrix4::look_at_rh(c, c + direction, Vector3::Y);
        }
        Cascades {
            main,
            frustums,
            ranges,
            width,
            view,
        }
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
        let blend = wgpu::BlendState {
            color: wgpu::BlendComponent {
                src_factor: wgpu::BlendFactor::SrcAlpha,
                dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                operation: wgpu::BlendOperation::Add,
            },
            alpha: wgpu::BlendComponent {
                src_factor: wgpu::BlendFactor::One,
                dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                operation: wgpu::BlendOperation::Add,
            },
        };
        // (topology lines, front face clockwise, transparent)
        let pipeline = |label,
                        vs: &str,
                        fs: &str,
                        buffers: &[wgpu::VertexBufferLayout],
                        format,
                        scene: bool,
                        style: (bool, bool, bool)| {
            let (lines, cw, transparent) = style;
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
                        targets: &[Some(wgpu::ColorTargetState {
                            format,
                            blend: transparent.then_some(blend),
                            write_mask: wgpu::ColorWrites::ALL,
                        })],
                    }),
                    primitive: wgpu::PrimitiveState {
                        topology: if lines {
                            wgpu::PrimitiveTopology::LineList
                        } else {
                            wgpu::PrimitiveTopology::TriangleList
                        },
                        front_face: if cw {
                            wgpu::FrontFace::Cw
                        } else {
                            wgpu::FrontFace::Ccw
                        },
                        cull_mode: Some(wgpu::Face::Back),
                        ..Default::default()
                    },
                    depth_stencil: scene.then(|| wgpu::DepthStencilState {
                        format: DEPTH,
                        depth_write_enabled: !transparent,
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
        let opaque = (false, false, false);
        let phong = pipeline(
            "csm phong",
            wgsl!("phong_vs"),
            wgsl!("phong_fs"),
            &buffers,
            HALF,
            true,
            opaque,
        );
        let phong_render = bind(
            r,
            phong.get_bind_group_layout(0),
            &[
                (0, self.phong_render.as_entire_binding()),
                (1, self.cascades.as_entire_binding()),
            ],
        );
        let compare = wgpu::BindingResource::Sampler(&self.compare);
        let phong_objects = self
            .shapes
            .iter()
            .map(|shape| {
                let mut entries = vec![(0, shape.phong.as_entire_binding())];
                for (i, (_, depth)) in self.shadow_maps.iter().enumerate() {
                    entries.push((1 + 2 * i as u32, compare.clone()));
                    entries.push((2 + 2 * i as u32, wgpu::BindingResource::TextureView(depth)));
                }
                bind(r, phong.get_bind_group_layout(1), &entries)
            })
            .collect();
        let lines = pipeline(
            "csm helper lines",
            wgsl!("line_vs"),
            wgsl!("line_fs"),
            &buffers[..1],
            HALF,
            true,
            (true, false, false),
        );
        let lines_render = bind(
            r,
            lines.get_bind_group_layout(0),
            &[(0, self.line_render.as_entire_binding())],
        );
        let line_objects = self
            .line_objects
            .iter()
            .map(|object| {
                bind(
                    r,
                    lines.get_bind_group_layout(1),
                    &[(0, object.as_entire_binding())],
                )
            })
            .collect();
        let planes = [true, false].map(|cw| {
            let p = pipeline(
                "csm helper planes",
                wgsl!("plane_vs"),
                wgsl!("plane_fs"),
                &buffers[..1],
                HALF,
                true,
                (false, cw, true),
            );
            let render = bind(
                r,
                p.get_bind_group_layout(0),
                &[(0, self.plane_render.as_entire_binding())],
            );
            let objects = self
                .plane_objects
                .iter()
                .map(|object| {
                    bind(
                        r,
                        p.get_bind_group_layout(1),
                        &[(0, object.as_entire_binding())],
                    )
                })
                .collect();
            (p, render, objects)
        });
        let output = pipeline(
            "render output",
            include_str!("lights_dynamic/output_vs.wgsl"),
            include_str!("lights_dynamic/output_fs.wgsl"),
            &buffers[..1],
            out.options.format,
            false,
            opaque,
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
                    (0, wgpu::BindingResource::Sampler(&self.sampler)),
                    (
                        1,
                        wgpu::BindingResource::TextureView(resolve.as_ref().unwrap_or(&color)),
                    ),
                    (2, self.output_object.as_entire_binding()),
                ],
            ),
        ];
        let mut data: Vec<f32> = [Matrix4::IDENTITY; 2]
            .iter()
            .flat_map(|m| m.to_cols_array().map(|v| v as f32))
            .collect();
        data.extend([size.0 as f32, size.1 as f32, 0., 0.]);
        r.queue
            .write_buffer(&self.output_render, 0, bytemuck::cast_slice(&data));
        r.queue.write_buffer(
            &self.output_object,
            0,
            bytemuck::cast_slice(&Matrix4::IDENTITY.to_cols_array().map(|v| v as f32)),
        );
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
            phong: (phong, phong_render, phong_objects),
            lines: (lines, lines_render, line_objects),
            planes,
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
        let target = self.controls.target();
        let (camera, world) = s.camera(c)?;
        let Camera::Perspective(p) = camera else {
            return Err(Error::Invalid("csm camera"));
        };
        // The orthographic camera follows the perspective one: its height is
        // the distance to the controls' target.
        let ortho = self.params[0] > 0.5;
        let (projection, near, far) = if ortho {
            let size = target.distance(world.transform_point3(Vector3::ZERO));
            let half = (size * p.aspect / 2., size / 2.);
            (
                orthographic(-half.0, half.0, half.1, -half.1, 0.1, 2000.),
                0.1,
                2000.,
            )
        } else {
            (camera.projection_matrix()?, p.near, p.far)
        };
        let view = world.inverse();
        let csm = self.cascades(projection, near, far, world);
        let shadows = self.params[1] > 0.5;
        let (shadow_near, shadow_far) = (self.params[8], self.params[9]);
        let shadow_projections: [Matrix4; CASCADES] = std::array::from_fn(|i| {
            let half = csm.width[i] / 2.;
            orthographic(-half, half, half, -half, shadow_near, shadow_far)
        });
        let bias = Matrix4::from_cols_array(&[
            0.5, 0., 0., 0., 0., 0.5, 0., 0., 0., 0., 1., 0., 0.5, 0.5, 0., 1.,
        ]);
        let linear = |hex: u32| Color::from_hex(hex).0.to_array().to_vec();
        let scale = |v: Vec<f64>, k: f64| v.iter().map(|x| x * k).collect::<Vec<_>>();
        let additional = Vector3::new(-1., -1., -1.).normalize() * -200.;
        let mut values: Vec<(String, Vec<f64>)> = vec![
            ("cameraProjectionMatrix".into(), m4(projection)),
            ("cameraViewMatrix".into(), m4(view)),
            ("nodeUniform6".into(), vec![1.5; 3]),
            ("nodeUniform12".into(), scale(linear(0x000020), 1.5)),
            ("nodeUniform16".into(), vec![3.; 3]),
            ("nodeUniform10".into(), additional.to_array().to_vec()),
            ("nodeUniform11".into(), vec![0.; 3]),
            (
                "nodeUniform14".into(),
                self.light_position().to_array().to_vec(),
            ),
            ("nodeUniform15".into(), vec![0.; 3]),
            ("shadowFar".into(), vec![self.params[2].min(far)]),
            ("nodeUniform18".into(), vec![near]),
        ];
        for (i, (projection, view)) in shadow_projections.iter().zip(&csm.view).enumerate() {
            let base = 20 + 7 * i;
            let name = |k: usize| format!("nodeUniform{}", base + k);
            values.extend([
                (name(0), m4(bias * *projection * *view)),
                (name(1), vec![0.]),
                (name(2), vec![0.]),
                (name(4), vec![1.]),
                (name(5), vec![SHADOW_SIZE as f64; 2]),
                // Shadows off: the receivers' shadow factor is 1, as without
                // the shadow code three.js recompiles to.
                (name(6), vec![if shadows { 1. } else { 0. }]),
            ]);
        }
        let values: Vec<(&str, &[f64])> =
            values.iter().map(|(n, v)| (n.as_str(), &v[..])).collect();
        r.queue.write_buffer(
            &self.phong_render,
            0,
            &pack(wgsl!("phong_fs"), "renderStruct", &values)?,
        );
        let ranges: Vec<f32> = csm
            .ranges
            .iter()
            .flat_map(|&(a, b)| [a as f32, b as f32, 0., 0.])
            .collect();
        r.queue
            .write_buffer(&self.cascades, 0, bytemuck::cast_slice(&ranges));
        for (i, (buffer, _)) in self.shadow_renders.iter().enumerate() {
            let mut data: Vec<f32> = shadow_projections[i]
                .to_cols_array()
                .map(|v| v as f32)
                .to_vec();
            data.extend(csm.view[i].to_cols_array().map(|v| v as f32));
            r.queue.write_buffer(buffer, 0, bytemuck::cast_slice(&data));
        }
        for shape in &self.shapes {
            r.queue.write_buffer(
                &shape.phong,
                0,
                &pack(
                    wgsl!("phong_fs"),
                    "objectStruct",
                    &[
                        ("nodeUniform0", &linear(shape.color)),
                        ("nodeUniform1", &[1.]),
                        ("nodeUniform2", &[30.]),
                        ("nodeUniform3", &linear(0x111111)),
                        ("nodeUniform5", &[1.]),
                        ("nodeUniform8", &m3(shape.model.inverse().transpose())),
                        ("nodeUniform13", &m4(shape.model)),
                    ],
                )?,
            );
            let mut depth = vec![1f32, 0., 0., 0.];
            depth.extend(shape.model.to_cols_array().map(|v| v as f32));
            r.queue
                .write_buffer(&shape.depth, 0, bytemuck::cast_slice(&depth));
        }
        // CSMHelper.update(): on every frame with auto update, or on demand.
        if self.params[14] > 0.5
            || std::mem::take(&mut self.helper_pending)
            || self.helper.is_none()
        {
            let unit_box = |lo: Vector3, hi: Vector3| {
                Matrix4::from_translation((lo + hi) / 2.) * Matrix4::from_scale((hi - lo) / 2.)
            };
            let main = &csm.main;
            let order = [0, 3, 2, 1];
            let frustum = std::array::from_fn(|k| {
                if k < 4 {
                    main.far[order[k]]
                } else {
                    main.near[order[k - 4]]
                }
            });
            self.helper = Some(HelperState {
                frustum,
                camera: world,
                boxes: std::array::from_fn(|i| {
                    let f = &csm.frustums[i];
                    let mut hi = f.far[0];
                    hi.z += 1e-4;
                    unit_box(f.far[2], hi)
                }),
                planes: std::array::from_fn(|i| {
                    let f = &csm.frustums[i];
                    let mut size = f.far[0] - f.far[2];
                    size.z = 1e-4;
                    Matrix4::from_translation((f.far[0] + f.far[2]) * 0.5)
                        * Matrix4::from_scale(size)
                }),
                shadows: std::array::from_fn(|i| {
                    let half = csm.width[i] / 2.;
                    csm.view[i].inverse()
                        * unit_box(
                            Vector3::new(-half, -half, -shadow_far),
                            Vector3::new(half, half, -shadow_near),
                        )
                }),
            });
        }
        let helper = self.helper.ok_or(Error::Invalid("csm helper"))?;
        let helper_visible = self.params[10] > 0.5;
        let (show_frustum, show_planes, show_bounds) = (
            self.params[11] > 0.5,
            self.params[11] > 0.5 && self.params[12] > 0.5,
            self.params[13] > 0.5,
        );
        let mut camera_data: Vec<f32> = projection.to_cols_array().map(|v| v as f32).to_vec();
        camera_data.extend(view.to_cols_array().map(|v| v as f32));
        r.queue
            .write_buffer(&self.line_render, 0, bytemuck::cast_slice(&camera_data));
        let mut plane_render = camera_data.clone();
        plane_render.extend([1.5f32, 1.5, 1.5, 0.]);
        r.queue
            .write_buffer(&self.plane_render, 0, bytemuck::cast_slice(&plane_render));
        let line_object = |buffer: &wgpu::Buffer, color: [f32; 3], model: Matrix4| {
            let mut data = color.to_vec();
            data.push(1.);
            data.extend(model.to_cols_array().map(|v| v as f32));
            r.queue.write_buffer(buffer, 0, bytemuck::cast_slice(&data));
        };
        // Opaque draws: the meshes and the helper's lines, front to back.
        enum Draw {
            Shape(usize),
            Line(usize, usize),
        }
        let screen = projection * view;
        let frustum = Frustum::from_projection(screen);
        let visible = |model: Matrix4, radius: f64, frustum: &Frustum| {
            let scale = model
                .x_axis
                .truncate()
                .length()
                .max(model.y_axis.truncate().length())
                .max(model.z_axis.truncate().length());
            frustum.intersects_sphere(Sphere {
                center: model.transform_point3(Vector3::ZERO),
                radius: radius * scale,
            })
        };
        let depth_of = |model: Matrix4| {
            screen
                .project_point3(model.transform_point3(Vector3::ZERO))
                .z
        };
        let mut draws: Vec<(f64, usize, Draw)> = vec![];
        for (i, shape) in self.shapes.iter().enumerate() {
            if visible(
                shape.model,
                self.geometries[shape.geometry].radius,
                &frustum,
            ) {
                draws.push((depth_of(shape.model), 1 + i, Draw::Shape(i)));
            }
        }
        let mut transparent = vec![];
        if helper_visible {
            let positions: Vec<f32> = helper
                .frustum
                .iter()
                .flat_map(|v| [v.x as f32, v.y as f32, v.z as f32])
                .collect();
            r.queue
                .write_buffer(&self.frustum_positions, 0, bytemuck::cast_slice(&positions));
            line_object(&self.line_objects[0], [1.; 3], helper.camera);
            if show_frustum {
                draws.push((depth_of(helper.camera), 0, Draw::Line(0, 0)));
            }
            let order = self.shapes.len() + 1;
            for i in 0..CASCADES {
                let cascade = helper.camera * helper.boxes[i];
                line_object(&self.line_objects[1 + i], [1.; 3], cascade);
                if show_frustum && visible(cascade, 3f64.sqrt(), &frustum) {
                    draws.push((depth_of(cascade), order + 3 * i, Draw::Line(1 + i, 2)));
                }
                let bounds = helper.shadows[i];
                line_object(&self.line_objects[1 + CASCADES + i], [1., 1., 0.], bounds);
                if show_bounds && visible(bounds, 3f64.sqrt(), &frustum) {
                    draws.push((
                        depth_of(bounds),
                        order + 3 * i + 2,
                        Draw::Line(1 + CASCADES + i, 2),
                    ));
                }
                let plane = helper.camera * helper.planes[i];
                let mut data = vec![1f32, 1., 1., 0.1];
                data.extend(plane.to_cols_array().map(|v| v as f32));
                r.queue
                    .write_buffer(&self.plane_objects[i], 0, bytemuck::cast_slice(&data));
                if show_planes && visible(plane, 0.5f64.sqrt(), &frustum) {
                    transparent.push((depth_of(plane), order + 3 * i + 1, i));
                }
            }
        }
        draws.sort_by(|a, b| {
            a.0.partial_cmp(&b.0)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then(a.1.cmp(&b.1))
        });
        // Transparent: back to front.
        transparent.sort_by(|a, b| {
            b.0.partial_cmp(&a.0)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then(a.1.cmp(&b.1))
        });
        let t = self.targets.as_ref().ok_or(Error::Invalid("csm targets"))?;
        let mut encoder = r.device.create_command_encoder(&Default::default());
        let draw_geometry = |pass: &mut wgpu::RenderPass, g: &Geometry, normals: bool| {
            if normals && let Some(n) = &g.normals {
                pass.set_vertex_buffer(0, n.slice(..));
                pass.set_vertex_buffer(1, g.positions.slice(..));
            } else {
                pass.set_vertex_buffer(0, g.positions.slice(..));
            }
            pass.set_index_buffer(g.index.slice(..), wgpu::IndexFormat::Uint32);
            pass.draw_indexed(0..g.count, 0, 0..1);
        };
        // Each cascade's shadow map, culled against its own camera.
        if shadows {
            for (i, (color, depth)) in self.shadow_maps.iter().enumerate() {
                let frustum = Frustum::from_projection(shadow_projections[i] * csm.view[i]);
                let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("csm shadow"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: color,
                        depth_slice: None,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
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
                pass.set_pipeline(&self.depth_pipeline);
                pass.set_bind_group(0, &self.shadow_renders[i].1, &[]);
                for (k, shape) in self.shapes.iter().enumerate() {
                    let g = &self.geometries[shape.geometry];
                    if !visible(shape.model, g.radius, &frustum) {
                        continue;
                    }
                    pass.set_bind_group(1, &self.depth_objects[k], &[]);
                    draw_geometry(&mut pass, g, false);
                }
            }
        }
        {
            let background = Color::from_hex(0x454e61).0;
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("csm scene"),
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
            for (_, _, draw) in &draws {
                match draw {
                    Draw::Shape(i) => {
                        pass.set_pipeline(&t.phong.0);
                        pass.set_bind_group(0, &t.phong.1, &[]);
                        pass.set_bind_group(1, &t.phong.2[*i], &[]);
                        draw_geometry(&mut pass, &self.geometries[self.shapes[*i].geometry], true);
                    }
                    Draw::Line(object, geometry) => {
                        pass.set_pipeline(&t.lines.0);
                        pass.set_bind_group(0, &t.lines.1, &[]);
                        pass.set_bind_group(1, &t.lines.2[*object], &[]);
                        if *object == 0 {
                            let g = &self.geometries[2];
                            pass.set_vertex_buffer(0, self.frustum_positions.slice(..));
                            pass.set_index_buffer(g.index.slice(..), wgpu::IndexFormat::Uint32);
                            pass.draw_indexed(0..24, 0, 0..1);
                        } else {
                            draw_geometry(&mut pass, &self.geometries[*geometry], false);
                        }
                    }
                }
            }
            for &(_, _, i) in &transparent {
                for (pipeline, render, objects) in &t.planes {
                    pass.set_pipeline(pipeline);
                    pass.set_bind_group(0, render, &[]);
                    pass.set_bind_group(1, &objects[i], &[]);
                    draw_geometry(&mut pass, &self.geometries[3], false);
                }
            }
        }
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("csm output"),
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
    /// The page's controls in order; the last is the helper's update button.
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        if index == 15 {
            self.helper_pending = true;
            return Ok(());
        }
        *self
            .params
            .get_mut(index)
            .ok_or(Error::Invalid("csm parameter"))? = value as f64;
        if index == 7 {
            self.margin = value as f64;
        }
        Ok(())
    }
    pub fn seek(&mut self, _t: f64) {}
}
