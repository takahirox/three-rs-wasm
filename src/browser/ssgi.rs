//! webgpu_postprocessing_ssgi: a Cornell box (red and green walls, white
//! floor, back wall, ceiling and two boxes) lit by a shadow-casting point
//! light under a white cylinder, with SSGINode's screen-space ambient
//! occlusion and one-bounce indirect light resolved by TRAA. The scene pass
//! writes color, diffuse color, packed view normals and velocity (MRT); the
//! SSGI pass (2 slices, 8 steps) writes AO (r8unorm) and GI (rg11b10ufloat)
//! with its slice rotation and step offset following frameId; the composite
//! ( color × AO + diffuse × GI ) renders to a texture TRAA jitters, reprojects
//! and blends; the output encodes sRGB. The AO, GI and direct views and the
//! composite without temporal filtering replace the output node. Every stage
//! runs the WGSL three.js r186 generates for the page (in `ssgi/`; the
//! point shadow is the godrays modules, TRAA the volume_traa ones and the
//! output the hdr ones, which are byte-identical).
use super::controls_attributes::{Controls, camera_state};
use super::deferred::{Draw, sampled_pipeline, set};
use super::lights_projector::{m3, m4, pack};
use super::pmrem_cube_uv::{bind, raw_pipeline};
use super::retro::uniform;
use crate::{
    Error, Result, camera::*, geometry::*, math::*, render_target::*, renderer::*, scene::*,
};
use std::f64::consts::PI;
use wgpu::util::DeviceExt;

const HALF: wgpu::TextureFormat = wgpu::TextureFormat::Rgba16Float;
const BYTE: wgpu::TextureFormat = wgpu::TextureFormat::Rgba8Unorm;
const DEPTH: wgpu::TextureFormat = wgpu::TextureFormat::Depth24Plus;
const SHADOW: u32 = 1024;
const LIGHT: [f64; 3] = [0., 13., 0.];
/// Color( 0xaaaaaa ) in linear space: the background clear.
const BACKGROUND: f64 = 0.4019777798219466;
macro_rules! wgsl {
    ($name:literal) => {
        include_str!(concat!("ssgi/", $name, ".wgsl"))
    };
}
const SHADOW_VS: &str = include_str!("godrays/shadow_vs.wgsl");
const SHADOW_FS: &str = include_str!("godrays/shadow_fs.wgsl");
const QUAD_VS: &str = include_str!("deferred/forward_output_vs.wgsl");
const TRAA_VS: &str = include_str!("volume_traa/traa_vs.wgsl");
const TRAA_FS: &str = include_str!("volume_traa/traa_fs.wgsl");
const OUTPUT_VS: &str = include_str!("hdr/output_vs.wgsl");
const OUTPUT_FS: &str = include_str!("hdr/output_fs.wgsl");
/// PointShadowNode's WebGPU cube faces: directions and ups.
const FACES: [([f64; 3], [f64; 3]); 6] = [
    ([1., 0., 0.], [0., -1., 0.]),
    ([-1., 0., 0.], [0., -1., 0.]),
    ([0., -1., 0.], [0., 0., -1.]),
    ([0., 1., 0.], [0., 0., 1.]),
    ([0., 0., 1.], [0., -1., 0.]),
    ([0., 0., -1.], [0., -1., 0.]),
];
/// SSGINode's per-frame slice rotations and step offsets.
const TEMPORAL_ROTATIONS: [f64; 6] = [60., 300., 180., 240., 120., 0.];
const SPATIAL_OFFSETS: [f64; 4] = [0., 0.5, 0.25, 0.75];
/// TAAUtils' computeHaltonOffsets( 32 ): bases 2 and 3 from index 1.
fn halton(index: usize) -> [f64; 2] {
    let h = |mut index: usize, base: usize| {
        let (mut fraction, mut result) = (1., 0.);
        while index > 0 {
            fraction /= base as f64;
            result += fraction * (index % base) as f64;
            index /= base;
        }
        result
    };
    [h(index % 32 + 1, 2), h(index % 32 + 1, 3)]
}
/// An indexed mesh: normals, positions, index and local bounding sphere,
/// its model, color and material ( basic for the light source ).
struct Mesh {
    normals: wgpu::Buffer,
    positions: wgpu::Buffer,
    index: wgpu::Buffer,
    count: u32,
    sphere: Sphere,
    model: Matrix4,
    color: [f64; 3],
    basic: bool,
    caster: bool,
    object: wgpu::Buffer,
    shadow: wgpu::Buffer,
}
impl Mesh {
    fn visible(&self, frustum: &Frustum) -> bool {
        let scale = self.model.to_scale_rotation_translation().0.max_element();
        frustum.intersects_sphere(Sphere {
            center: self.model.transform_point3(self.sphere.center),
            radius: self.sphere.radius * scale,
        })
    }
}
fn texture(r: &Renderer, size: (u32, u32, u32), format: wgpu::TextureFormat) -> wgpu::Texture {
    r.device.create_texture(&wgpu::TextureDescriptor {
        label: Some("ssgi target"),
        size: wgpu::Extent3d {
            width: size.0,
            height: size.1,
            depth_or_array_layers: size.2,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT
            | wgpu::TextureUsages::TEXTURE_BINDING
            | wgpu::TextureUsages::COPY_SRC
            | wgpu::TextureUsages::COPY_DST,
        view_formats: &[],
    })
}
fn view(t: &wgpu::Texture) -> wgpu::TextureView {
    t.create_view(&Default::default())
}
fn color(
    view: &wgpu::TextureView,
    clear: wgpu::Color,
) -> Option<wgpu::RenderPassColorAttachment<'_>> {
    Some(wgpu::RenderPassColorAttachment {
        view,
        depth_slice: None,
        resolve_target: None,
        ops: wgpu::Operations {
            load: wgpu::LoadOp::Clear(clear),
            store: wgpu::StoreOp::Store,
        },
    })
}
fn depth(view: &wgpu::TextureView) -> Option<wgpu::RenderPassDepthStencilAttachment<'_>> {
    Some(wgpu::RenderPassDepthStencilAttachment {
        view,
        depth_ops: Some(wgpu::Operations {
            load: wgpu::LoadOp::Clear(1.),
            store: wgpu::StoreOp::Store,
        }),
        stencil_ops: None,
    })
}
/// The previous frame's matrices VelocityNode reads.
#[derive(Clone, Copy)]
struct Motion {
    projection: Matrix4,
    view: Matrix4,
}
/// TRAANode's camera matrices from its last resolve.
#[derive(Clone, Copy)]
struct Resolved {
    world: Matrix4,
    projection_inverse: Matrix4,
}
struct Targets {
    width: u32,
    height: u32,
    format: wgpu::TextureFormat,
    /// The scene MRT: color, diffuse color, normal and velocity, and depth.
    scene: [(wgpu::Texture, wgpu::TextureView); 4],
    depth: (wgpu::Texture, wgpu::TextureView),
    ao: wgpu::TextureView,
    gi: wgpu::TextureView,
    composite: (wgpu::Texture, wgpu::TextureView),
    resolve: (wgpu::Texture, wgpu::TextureView),
    history: wgpu::Texture,
    history_depth: wgpu::Texture,
    screen: RenderTarget,
    /// Per mesh, its scene draw.
    draws: Vec<Draw>,
    ssgi: Draw,
    composite_draw: Draw,
    /// Resolve with the placeholder, or the history, previous depth.
    traa: (wgpu::RenderPipeline, [wgpu::BindGroup; 2]),
    /// The output of the resolved composite, the AO, the GI, the scene color
    /// and the composite without temporal filtering.
    outputs: [Draw; 5],
}
pub(super) struct Demo {
    controls: Controls,
    /// output, slice count, step count, radius, exp factor, thickness,
    /// backface lighting, AO intensity, GI intensity, use linear thickness,
    /// screen-space sampling and temporal filtering.
    params: [f64; 12],
    /// A frame was requested; other redraws present the last frame again.
    pending: bool,
    /// The loading frame has rendered: the fixture restarts frameId after it.
    loaded: bool,
    /// NodeFrame.frameId over requested frames.
    frame_id: usize,
    jitter: usize,
    /// The TRAA output node has been built: the frame that builds it renders
    /// unjittered ( its before-pipeline hook registers during that frame ).
    built: bool,
    resolves: usize,
    motion: Option<Motion>,
    resolved: Option<Resolved>,
    meshes: Vec<Mesh>,
    shadow_pipeline: wgpu::RenderPipeline,
    shadow_faces: Vec<(wgpu::TextureView, wgpu::TextureView, wgpu::BindGroup)>,
    shadow_objects: Vec<wgpu::BindGroup>,
    shadow_cube: wgpu::TextureView,
    physical_render: wgpu::Buffer,
    basic_render: wgpu::Buffer,
    ssgi_object: wgpu::Buffer,
    traa_object: wgpu::Buffer,
    quad_uv: wgpu::Buffer,
    placeholder: wgpu::TextureView,
    linear: wgpu::Sampler,
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
            fov: 40.,
            near: 0.1,
            far: 100.,
            aspect,
            ..Default::default()
        }));
        s.get_mut(c)?.position = Vector3::new(0., 10., 30.);
        let mut controls = Controls::new(None, (1., 100.), PI, true);
        controls.set_target(Vector3::new(0., 7., 0.));
        controls.update(s, c)?;
        let init = |label, data: &[u8], usage| {
            r.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some(label),
                    contents: data,
                    usage,
                })
        };
        let read = |g: &BufferGeometry, name: &str| -> Result<Vec<f32>> {
            let a = g
                .attributes
                .get(name)
                .ok_or(Error::Invalid("ssgi attribute"))?;
            (0..a.count())
                .flat_map(|i| (0..3).map(move |k| (i, k)))
                .map(|(i, k)| a.get_component(i, k).map(|v| v as f32))
                .collect()
        };
        let t = |x, y, z| Matrix4::from_translation(Vector3::new(x, y, z));
        let sc = |x, y| Matrix4::from_scale(Vector3::new(x, y, 1.));
        let (rx, ry, rz) = (
            Matrix4::from_rotation_x,
            Matrix4::from_rotation_y,
            Matrix4::from_rotation_z,
        );
        let wall = PlaneGeometry::build(1., 1., 1, 1)?;
        let (red, green, white) = ([1., 0., 0.], [0., 1., 0.], [1.; 3]);
        let scene: [(BufferGeometry, Matrix4, [f64; 3], bool, bool); 8] = [
            (
                wall.clone(),
                t(-10., 7.5, 0.) * ry(PI * 0.5) * sc(20., 15.),
                red,
                false,
                false,
            ),
            (
                wall.clone(),
                t(10., 7.5, 0.) * ry(-PI * 0.5) * sc(20., 15.),
                green,
                false,
                false,
            ),
            (
                wall.clone(),
                rx(-PI * 0.5) * sc(20., 20.),
                white,
                false,
                false,
            ),
            (
                wall.clone(),
                t(0., 7.5, -10.) * rz(-PI * 0.5) * sc(15., 20.),
                white,
                false,
                false,
            ),
            (
                wall,
                t(0., 15., 0.) * rx(PI * 0.5) * sc(20., 20.),
                white,
                false,
                false,
            ),
            (
                BoxGeometry::build(5., 7., 5.)?,
                t(-3., 3.5, -2.) * ry(PI * 0.25),
                white,
                false,
                true,
            ),
            (
                BoxGeometry::build(4., 4., 4.)?,
                t(4., 2., 4.) * ry(-PI * 0.1),
                white,
                false,
                true,
            ),
            (
                CylinderGeometry::build(2.5, 2.5, 1., 64, 1, false, 0., 2. * PI)?,
                t(0., 15., 0.),
                white,
                true,
                false,
            ),
        ];
        let mut meshes = vec![];
        for (g, model, color, basic, caster) in scene {
            let positions = read(&g, "position")?;
            let points: Vec<Vector3> = positions
                .chunks(3)
                .map(|p| Vector3::new(p[0] as f64, p[1] as f64, p[2] as f64))
                .collect();
            let (lo, hi) = points.iter().fold(
                (
                    Vector3::splat(f64::INFINITY),
                    Vector3::splat(f64::NEG_INFINITY),
                ),
                |(lo, hi), p| (lo.min(*p), hi.max(*p)),
            );
            let center = (lo + hi) * 0.5;
            let index = g.index.clone().ok_or(Error::Invalid("ssgi index"))?;
            let mut shadow = vec![1f32, 0., 0., 0.];
            shadow.extend(model.to_cols_array().map(|v| v as f32));
            let source = if basic {
                wgsl!("basic_fs")
            } else {
                wgsl!("physical_fs")
            };
            meshes.push(Mesh {
                normals: init(
                    "ssgi normals",
                    bytemuck::cast_slice(&read(&g, "normal")?),
                    wgpu::BufferUsages::VERTEX,
                ),
                positions: init(
                    "ssgi positions",
                    bytemuck::cast_slice(&positions),
                    wgpu::BufferUsages::VERTEX,
                ),
                index: init(
                    "ssgi index",
                    bytemuck::cast_slice(&index),
                    wgpu::BufferUsages::INDEX,
                ),
                count: index.len() as u32,
                sphere: Sphere {
                    center,
                    radius: points.iter().map(|p| p.distance(center)).fold(0., f64::max),
                },
                model,
                color,
                basic,
                caster,
                object: uniform(r, "ssgi object", source, "objectStruct")?,
                shadow: init(
                    "ssgi shadow object",
                    bytemuck::cast_slice(&shadow),
                    wgpu::BufferUsages::UNIFORM,
                ),
            });
        }
        // The point light's 1024² cube shadow ( near 0.5, far = distance 100 ).
        let position = [wgpu::vertex_attr_array![0 => Float32x3]];
        let shadow_pipeline = raw_pipeline(
            r,
            "ssgi shadow",
            SHADOW_VS,
            SHADOW_FS,
            &[wgpu::VertexBufferLayout {
                array_stride: 12,
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes: &position[0],
            }],
            BYTE,
            1,
            Some((wgpu::CompareFunction::LessEqual, true)),
            true,
        );
        let shadow_color = texture(r, (SHADOW, SHADOW, 6), BYTE);
        let shadow_depth = texture(r, (SHADOW, SHADOW, 6), DEPTH);
        let layer = |t: &wgpu::Texture, i: u32| {
            t.create_view(&wgpu::TextureViewDescriptor {
                dimension: Some(wgpu::TextureViewDimension::D2),
                base_array_layer: i,
                array_layer_count: Some(1),
                ..Default::default()
            })
        };
        let light = Vector3::from_array(LIGHT);
        let face_projection = Matrix4::perspective_rh(PI / 2., 1., 0.5, 100.);
        let shadow_faces = (0..6)
            .map(|i| {
                let (direction, up) = FACES[i];
                let view = Matrix4::look_at_rh(
                    light,
                    light + Vector3::from_array(direction),
                    Vector3::from_array(up),
                );
                let mut data: Vec<f32> = face_projection.to_cols_array().map(|v| v as f32).to_vec();
                data.extend(view.to_cols_array().map(|v| v as f32));
                let buffer = init(
                    "ssgi shadow camera",
                    bytemuck::cast_slice(&data),
                    wgpu::BufferUsages::UNIFORM,
                );
                (
                    layer(&shadow_color, i as u32),
                    layer(&shadow_depth, i as u32),
                    bind(
                        r,
                        shadow_pipeline.get_bind_group_layout(0),
                        &[(0, buffer.as_entire_binding())],
                    ),
                )
            })
            .collect();
        let shadow_objects = meshes
            .iter()
            .map(|m| {
                bind(
                    r,
                    shadow_pipeline.get_bind_group_layout(1),
                    &[(0, m.shadow.as_entire_binding())],
                )
            })
            .collect();
        let shadow_cube = shadow_depth.create_view(&wgpu::TextureViewDescriptor {
            dimension: Some(wgpu::TextureViewDimension::Cube),
            ..Default::default()
        });
        // TRAANode's DepthTexture( 1, 1 ) placeholder, cleared to the far plane.
        let placeholder = view(&texture(r, (1, 1, 1), DEPTH));
        let mut encoder = r.device.create_command_encoder(&Default::default());
        encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("ssgi placeholder"),
            depth_stencil_attachment: depth(&placeholder),
            ..Default::default()
        });
        r.queue.submit([encoder.finish()]);
        Ok(Self {
            controls,
            params: [0., 2., 8., 12., 2., 1., 0., 1., 10., 0., 1., 1.],
            // The page renders its first frame on loading: it builds the TRAA
            // output node ( unjittered ) and starts the history.
            pending: true,
            loaded: false,
            frame_id: 0,
            jitter: 0,
            built: false,
            resolves: 0,
            motion: None,
            resolved: None,
            meshes,
            shadow_pipeline,
            shadow_faces,
            shadow_objects,
            shadow_cube,
            physical_render: uniform(
                r,
                "ssgi physical render",
                wgsl!("physical_fs"),
                "renderStruct",
            )?,
            basic_render: uniform(r, "ssgi basic render", wgsl!("basic_fs"), "renderStruct")?,
            ssgi_object: uniform(r, "ssgi object", wgsl!("ssgi_fs"), "objectStruct")?,
            traa_object: uniform(r, "ssgi traa", TRAA_FS, "objectStruct")?,
            quad_uv: init(
                "ssgi quad",
                bytemuck::cast_slice(&[0f32, -1., 0., 1., 2., 1.]),
                wgpu::BufferUsages::VERTEX,
            ),
            placeholder,
            linear: r.device.create_sampler(&wgpu::SamplerDescriptor {
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
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, _dt: f64, animate: bool) -> Result<()> {
        if animate {
            self.pending = true;
        }
        Ok(())
    }
    pub fn prepare(&mut self, _s: &mut Scene, _c: Object3D, _r: &Renderer) -> Result<()> {
        Ok(())
    }
    fn resize(&mut self, r: &Renderer, out: &RenderTarget) -> Result<()> {
        let (width, height) = (out.width, out.height);
        let size = (width, height, 1);
        let make = |format| {
            let t = texture(r, size, format);
            let v = view(&t);
            (t, v)
        };
        let scene = [make(HALF), make(BYTE), make(BYTE), make(HALF)];
        let depth_target = make(DEPTH);
        let ao = view(&texture(r, size, wgpu::TextureFormat::R8Unorm));
        let gi = view(&texture(r, size, wgpu::TextureFormat::Rg11b10Ufloat));
        let composite = make(HALF);
        let resolve = make(HALF);
        let history = texture(r, size, HALF);
        let history_depth = texture(r, size, DEPTH);
        let tex = wgpu::BindingResource::TextureView;
        let sampler = wgpu::BindingResource::Sampler;
        let mrt = [HALF, BYTE, BYTE, HALF];
        let attrs = [
            wgpu::vertex_attr_array![0 => Float32x3],
            wgpu::vertex_attr_array![1 => Float32x3],
        ];
        let lit = [0, 1].map(|i| wgpu::VertexBufferLayout {
            array_stride: 12,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &attrs[i],
        });
        let depth_state = Some((wgpu::CompareFunction::LessEqual, true));
        let triangles = (1, wgpu::PrimitiveTopology::TriangleList);
        let physical = sampled_pipeline(
            r,
            "ssgi physical",
            (wgsl!("physical_vs"), wgsl!("physical_fs")),
            &lit,
            &mrt,
            depth_state,
            (false, false),
            triangles,
        );
        let basic = sampled_pipeline(
            r,
            "ssgi basic",
            (wgsl!("basic_vs"), wgsl!("basic_fs")),
            &lit,
            &mrt,
            depth_state,
            (false, false),
            triangles,
        );
        let physical_render = bind(
            r,
            physical.get_bind_group_layout(0),
            &[(0, self.physical_render.as_entire_binding())],
        );
        let basic_render = bind(
            r,
            basic.get_bind_group_layout(0),
            &[(0, self.basic_render.as_entire_binding())],
        );
        let draws = self
            .meshes
            .iter()
            .map(|m| {
                if m.basic {
                    (
                        basic.clone(),
                        vec![
                            basic_render.clone(),
                            bind(
                                r,
                                basic.get_bind_group_layout(1),
                                &[(0, m.object.as_entire_binding())],
                            ),
                        ],
                    )
                } else {
                    (
                        physical.clone(),
                        vec![
                            physical_render.clone(),
                            bind(
                                r,
                                physical.get_bind_group_layout(1),
                                &[
                                    (0, m.object.as_entire_binding()),
                                    (1, sampler(&self.linear)),
                                    (2, tex(&r.dfg)),
                                    (3, sampler(&self.compare)),
                                    (4, tex(&self.shadow_cube)),
                                ],
                            ),
                        ],
                    )
                }
            })
            .collect();
        let quad_attrs = [wgpu::vertex_attr_array![0 => Float32x2]];
        let quad = [wgpu::VertexBufferLayout {
            array_stride: 8,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &quad_attrs[0],
        }];
        let screen_pass = |label, vs, fs, formats: &[wgpu::TextureFormat]| {
            sampled_pipeline(
                r,
                label,
                (vs, fs),
                &quad,
                formats,
                None,
                (false, false),
                triangles,
            )
        };
        let ssgi_pipeline = screen_pass(
            "ssgi",
            wgsl!("ssgi_vs"),
            wgsl!("ssgi_fs"),
            &[
                wgpu::TextureFormat::R8Unorm,
                wgpu::TextureFormat::Rg11b10Ufloat,
            ],
        );
        let ssgi = (
            ssgi_pipeline.clone(),
            vec![bind(
                r,
                ssgi_pipeline.get_bind_group_layout(0),
                &[
                    (0, tex(&depth_target.1)),
                    (1, self.ssgi_object.as_entire_binding()),
                    (2, sampler(&self.linear)),
                    (3, tex(&scene[2].1)),
                    (4, sampler(&self.linear)),
                    (5, tex(&scene[0].1)),
                ],
            )],
        );
        let composite_pipeline =
            screen_pass("ssgi composite", QUAD_VS, wgsl!("composite_fs"), &[HALF]);
        let four = |p: &wgpu::RenderPipeline| {
            bind(
                r,
                p.get_bind_group_layout(0),
                &[
                    (0, sampler(&self.linear)),
                    (1, tex(&scene[0].1)),
                    (2, sampler(&self.linear)),
                    (3, tex(&ao)),
                    (4, sampler(&self.linear)),
                    (5, tex(&scene[1].1)),
                    (6, sampler(&self.linear)),
                    (7, tex(&gi)),
                ],
            )
        };
        let composite_draw = (composite_pipeline.clone(), vec![four(&composite_pipeline)]);
        let traa_pipeline = screen_pass("ssgi traa", TRAA_VS, TRAA_FS, &[HALF]);
        let history_view = view(&history);
        let history_depth_view = view(&history_depth);
        let traa_groups = [&self.placeholder, &history_depth_view].map(|previous| {
            bind(
                r,
                traa_pipeline.get_bind_group_layout(0),
                &[
                    // Velocity is read with textureLoad: the layout drops its sampler.
                    (1, tex(&scene[3].1)),
                    (2, sampler(&self.linear)),
                    (3, tex(&composite.1)),
                    (4, tex(&depth_target.1)),
                    (5, self.traa_object.as_entire_binding()),
                    (6, tex(previous)),
                    (7, sampler(&self.linear)),
                    (8, tex(&history_view)),
                ],
            )
        });
        let format = out.options.format;
        let single = |fs, source: &wgpu::TextureView| {
            let p = screen_pass("ssgi output", OUTPUT_VS, fs, &[format]);
            let g = bind(
                r,
                p.get_bind_group_layout(0),
                &[
                    (0, sampler(&self.linear)),
                    (1, wgpu::BindingResource::TextureView(source)),
                ],
            );
            (p, vec![g])
        };
        let composite_output = screen_pass(
            "ssgi output",
            OUTPUT_VS,
            wgsl!("output_composite_fs"),
            &[format],
        );
        let outputs = [
            single(OUTPUT_FS, &resolve.1),
            single(wgsl!("output_ao_fs"), &ao),
            single(wgsl!("output_gi_fs"), &gi),
            single(wgsl!("output_direct_fs"), &scene[0].1),
            (composite_output.clone(), vec![four(&composite_output)]),
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
            format,
            scene,
            depth: depth_target,
            ao,
            gi,
            composite,
            resolve,
            history,
            history_depth,
            screen,
            draws,
            ssgi,
            composite_draw,
            traa: (traa_pipeline, traa_groups),
            outputs,
        });
        Ok(())
    }
    /// The output node: 0 the resolved composite, 1 AO, 2 GI, 3 the scene
    /// color, 4 the composite without temporal filtering.
    fn output_index(&self) -> usize {
        // The GUI lists Combined, Direct, AO and GI.
        match self.params[0].round() as usize {
            1 => 3,
            2 => 1,
            3 => 2,
            _ if self.params[11] < 0.5 => 4,
            _ => 0,
        }
    }
    fn present(&self, encoder: &mut wgpu::CommandEncoder, t: &Targets) {
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("ssgi output"),
            color_attachments: &[color(&t.screen.view, wgpu::Color::BLACK)],
            ..Default::default()
        });
        set(&mut pass, &t.outputs[self.output_index()]);
        pass.set_vertex_buffer(0, self.quad_uv.slice(..));
        pass.draw(0..3, 0..1);
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
            // New history targets: TRAA restarts from the beauty buffer.
            self.resolves = 0;
        }
        let t = self
            .targets
            .as_ref()
            .ok_or(Error::Invalid("ssgi targets"))?;
        if !std::mem::take(&mut self.pending) {
            let mut encoder = r.device.create_command_encoder(&Default::default());
            self.present(&mut encoder, t);
            r.queue.submit([encoder.finish()]);
            return Ok(true);
        }
        if std::mem::replace(&mut self.loaded, true) {
            self.frame_id += 1;
        }
        s.update()?;
        let (camera, world) = s.camera(c)?;
        let (projection, view) = (camera.projection_matrix()?, world.inverse());
        let output = self.output_index();
        let traa_on = output == 0;
        let ssgi_on = output != 3;
        // TRAANode.setViewOffset: the camera shifted by the Halton offset ( in
        // pixels ) for the whole pipeline.
        let jittered = if traa_on && self.built {
            let [x, y] = halton(self.jitter);
            let mut p = projection;
            p.z_axis.x += 2. * (x - 0.5) / t.width as f64;
            p.z_axis.y -= 2. * (y - 0.5) / t.height as f64;
            p
        } else {
            projection
        };
        // VelocityNode: last frame's projection and view ( this frame's on the first ).
        let motion = self.motion.unwrap_or(Motion { projection, view });
        self.motion = Some(Motion { projection, view });
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
        let light = Vector3::from_array(LIGHT);
        let ambient = Color::from_hex(0x0c0c0c).0.to_array().to_vec();
        write(
            &self.physical_render,
            wgsl!("physical_fs"),
            "renderStruct",
            &[
                ("nodeUniform30", m4(motion.projection)),
                ("cameraProjectionMatrix", m4(jittered)),
                ("cameraViewMatrix", m4(view)),
                ("nodeUniform14", vec![100.; 3]),
                ("nodeUniform25", vec![100.]),
                ("nodeUniform26", vec![2.]),
                ("nodeUniform27", ambient.clone()),
                (
                    "nodeUniform13",
                    view.transform_point3(light).to_array().to_vec(),
                ),
                ("nodeUniform15", m4(Matrix4::from_translation(-light))),
                ("nodeUniform19", vec![0.5]),
                ("nodeUniform18", vec![100.]),
                ("nodeUniform17", vec![0.]),
                ("nodeUniform20", vec![0.]),
                ("nodeUniform22", vec![1.]),
                ("nodeUniform23", vec![SHADOW as f64; 2]),
                ("nodeUniform24", vec![1.]),
            ],
        )?;
        write(
            &self.basic_render,
            wgsl!("basic_fs"),
            "renderStruct",
            &[
                ("nodeUniform8", m4(motion.projection)),
                ("cameraProjectionMatrix", m4(jittered)),
                ("cameraViewMatrix", m4(view)),
                ("nodeUniform2", ambient),
            ],
        )?;
        for m in &self.meshes {
            if m.basic {
                write(
                    &m.object,
                    wgsl!("basic_vs"),
                    "objectStruct",
                    &[
                        ("nodeUniform0", m.color.to_vec()),
                        ("nodeUniform1", vec![1.]),
                        ("nodeUniform4", m3(m.model.inverse().transpose())),
                        ("nodeUniform5", m4(projection)),
                        ("nodeUniform7", m4(m.model)),
                        ("nodeUniform9", m4(motion.view)),
                        ("nodeUniform10", m4(m.model)),
                        ("nodeUniform12", m4(m.model)),
                    ],
                )?;
            } else {
                write(
                    &m.object,
                    wgsl!("physical_fs"),
                    "objectStruct",
                    &[
                        ("nodeUniform0", m.color.to_vec()),
                        ("nodeUniform1", vec![1.]),
                        ("nodeUniform2", vec![0.]),
                        ("nodeUniform3", vec![1.]),
                        ("nodeUniform5", m3(m.model.inverse().transpose())),
                        ("nodeUniform6", vec![1.5]),
                        ("nodeUniform7", vec![1.; 3]),
                        ("nodeUniform8", vec![1.]),
                        ("nodeUniform9", vec![0.; 3]),
                        ("nodeUniform10", vec![1.]),
                        ("nodeUniform12", m4(m.model)),
                        ("nodeUniform28", m4(projection)),
                        ("nodeUniform29", m4(m.model)),
                        ("nodeUniform31", m4(motion.view)),
                        ("nodeUniform32", m4(m.model)),
                    ],
                )?;
            }
        }
        let [
            _,
            slices,
            steps,
            radius,
            exp_factor,
            thickness,
            backface,
            ao_intensity,
            gi_intensity,
            linear_thickness,
            screen_space,
            temporal,
        ] = self.params;
        let (direction, offset) = if temporal > 0.5 {
            (
                TEMPORAL_ROTATIONS[self.frame_id % 6] / 360.,
                SPATIAL_OFFSETS[self.frame_id % 4],
            )
        } else {
            (1., 1.)
        };
        let fov = match camera {
            Camera::Perspective(p) => p.fov,
            _ => 40.,
        };
        write(
            &self.ssgi_object,
            wgsl!("ssgi_fs"),
            "objectStruct",
            &[
                ("nodeUniform1", m4(jittered.inverse())),
                ("nodeUniform3", vec![slices.round()]),
                ("nodeUniform4", vec![steps.round()]),
                ("nodeUniform5", vec![ao_intensity]),
                ("nodeUniform6", vec![gi_intensity]),
                ("nodeUniform7", vec![radius]),
                ("nodeUniform8", vec![screen_space.round()]),
                ("nodeUniform9", vec![t.width as f64, t.height as f64]),
                (
                    "nodeUniform10",
                    vec![t.height as f64 / ((fov.to_radians() * 0.5).tan() * 2.) * 0.5],
                ),
                ("nodeUniform11", vec![direction]),
                ("nodeUniform12", vec![exp_factor]),
                ("nodeUniform13", vec![thickness]),
                ("nodeUniform14", vec![backface]),
                ("nodeUniform15", vec![offset]),
                ("nodeUniform16", vec![linear_thickness.round()]),
                ("nodeUniform17", vec![100.]),
                ("nodeUniform19", vec![1.]),
            ],
        )?;
        // TRAANode.updateBefore: the previous camera matrices ( identity before
        // the first resolve ) and this frame's.
        let previous = self.resolved.unwrap_or(Resolved {
            world: Matrix4::IDENTITY,
            projection_inverse: Matrix4::IDENTITY,
        });
        if traa_on {
            self.resolved = Some(Resolved {
                world,
                projection_inverse: jittered.inverse(),
            });
            write(
                &self.traa_object,
                TRAA_FS,
                "objectStruct",
                &[
                    ("nodeUniform3", vec![0.1, 100.]),
                    ("nodeUniform4", m4(view)),
                    ("nodeUniform5", m4(previous.world)),
                    ("nodeUniform6", m4(previous.projection_inverse)),
                    ("nodeUniform8", m3(Matrix4::IDENTITY)),
                    ("nodeUniform10", m3(Matrix4::IDENTITY)),
                    ("nodeUniform11", vec![1.]),
                ],
            )?;
        }
        let mut encoder = r.device.create_command_encoder(&Default::default());
        // The cube shadow, each face culling the casters.
        for (i, (color_view, depth_view, group)) in self.shadow_faces.iter().enumerate() {
            let (direction, up) = FACES[i];
            let face_view = Matrix4::look_at_rh(
                light,
                light + Vector3::from_array(direction),
                Vector3::from_array(up),
            );
            let frustum = Frustum::from_projection(
                Matrix4::perspective_rh(PI / 2., 1., 0.5, 100.) * face_view,
            );
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("ssgi shadow"),
                color_attachments: &[color(color_view, wgpu::Color::BLACK)],
                depth_stencil_attachment: depth(depth_view),
                ..Default::default()
            });
            pass.set_pipeline(&self.shadow_pipeline);
            pass.set_bind_group(0, group, &[]);
            for (k, m) in self.meshes.iter().enumerate() {
                if m.caster && m.visible(&frustum) {
                    pass.set_bind_group(1, &self.shadow_objects[k], &[]);
                    pass.set_vertex_buffer(0, m.positions.slice(..));
                    pass.set_index_buffer(m.index.slice(..), wgpu::IndexFormat::Uint32);
                    pass.draw_indexed(0..m.count, 0, 0..1);
                }
            }
        }
        // The scene MRT.
        let frustum = Frustum::from_projection(jittered * view);
        {
            let black = wgpu::Color::BLACK;
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("ssgi scene"),
                color_attachments: &[
                    color(
                        &t.scene[0].1,
                        wgpu::Color {
                            r: BACKGROUND,
                            g: BACKGROUND,
                            b: BACKGROUND,
                            a: 1.,
                        },
                    ),
                    color(&t.scene[1].1, black),
                    color(&t.scene[2].1, black),
                    color(&t.scene[3].1, black),
                ],
                depth_stencil_attachment: depth(&t.depth.1),
                ..Default::default()
            });
            for (m, draw) in self.meshes.iter().zip(&t.draws) {
                if m.visible(&frustum) {
                    set(&mut pass, draw);
                    pass.set_vertex_buffer(0, m.normals.slice(..));
                    pass.set_vertex_buffer(1, m.positions.slice(..));
                    pass.set_index_buffer(m.index.slice(..), wgpu::IndexFormat::Uint32);
                    pass.draw_indexed(0..m.count, 0, 0..1);
                }
            }
        }
        let quad = |encoder: &mut wgpu::CommandEncoder,
                    targets: &[Option<wgpu::RenderPassColorAttachment>],
                    draw: &Draw| {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("ssgi quad"),
                color_attachments: targets,
                ..Default::default()
            });
            set(&mut pass, draw);
            pass.set_vertex_buffer(0, self.quad_uv.slice(..));
            pass.draw(0..3, 0..1);
        };
        if ssgi_on {
            // AO clears white, GI black.
            quad(
                &mut encoder,
                &[
                    color(&t.ao, wgpu::Color::WHITE),
                    color(&t.gi, wgpu::Color::BLACK),
                ],
                &t.ssgi,
            );
        }
        if traa_on {
            quad(
                &mut encoder,
                &[color(&t.composite.1, wgpu::Color::BLACK)],
                &t.composite_draw,
            );
            let full = wgpu::Extent3d {
                width: t.width,
                height: t.height,
                depth_or_array_layers: 1,
            };
            // A new history starts from the beauty buffer.
            if self.resolves == 0 {
                encoder.copy_texture_to_texture(
                    t.composite.0.as_image_copy(),
                    t.history.as_image_copy(),
                    full,
                );
            }
            // The first two resolves read TRAANode's 1 × 1 placeholder depth
            // ( the far plane ); later ones the previous frame's.
            let (pipeline, groups) = &t.traa;
            let draw = (
                pipeline.clone(),
                vec![groups[usize::from(self.resolves >= 2)].clone()],
            );
            quad(
                &mut encoder,
                &[color(&t.resolve.1, wgpu::Color::BLACK)],
                &draw,
            );
            encoder.copy_texture_to_texture(
                t.resolve.0.as_image_copy(),
                t.history.as_image_copy(),
                full,
            );
            encoder.copy_texture_to_texture(
                t.depth.0.as_image_copy(),
                t.history_depth.as_image_copy(),
                full,
            );
            self.resolves += 1;
            self.jitter = (self.jitter + 1) % 32;
            self.built = true;
        }
        self.present(&mut encoder, t);
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
    /// output, slice count, step count, radius, exp factor, thickness,
    /// backface lighting, AO intensity, GI intensity, use linear thickness,
    /// screen-space sampling and temporal filtering.
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        // The output and the temporal filtering rebuild the output node.
        if index == 0 || index == 11 {
            self.built = false;
        }
        *self
            .params
            .get_mut(index)
            .ok_or(Error::Invalid("ssgi parameter"))? = value as f64;
        Ok(())
    }
    pub fn seek(&mut self, _t: f64) {
        self.pending = true;
    }
}
