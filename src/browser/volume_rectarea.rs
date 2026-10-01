//! webgpu_volume_lighting_rectarea: a white torus knot over a checkered
//! floor, lit by three turning red, green and blue RectAreaLights (each with
//! an emissive front panel and a dark back panel), inside a 50 × 40 × 50 fog
//! volume. The scene pass is followed by the volumetric pass:
//! VolumeNodeMaterial ray-marches the box at a quarter resolution,
//! evaluating the three area lights with the LTC tables and the 128³
//! ImprovedNoise smoke texture, dithered by the Bayer texture and stopped at
//! the scene depth. A two-pass Gaussian blur denoises it and the output adds
//! it to the scene with NeutralToneMapping (exposure 2). Every stage runs the
//! WGSL three.js r186 generates for the page (in `volume_rectarea/`; the blur
//! and output are the same as `volume_lighting/`'s).
use super::controls_attributes::{Controls, camera_state};
use super::lights_projector::{m3, m4, pack};
use super::volume_lighting::{bayer_texture, noise_texture};
use crate::{
    Error, Result, camera::*, geometry::*, math::*, render_target::*, renderer::*, scene::*,
};
use std::f64::consts::PI;
use wgpu::util::DeviceExt;

const HALF: wgpu::TextureFormat = wgpu::TextureFormat::Rgba16Float;
const DEPTH: wgpu::TextureFormat = wgpu::TextureFormat::Depth24Plus;
macro_rules! wgsl {
    ($name:literal) => {
        include_str!(concat!("volume_rectarea/", $name, ".wgsl"))
    };
}
macro_rules! shared {
    ($name:literal) => {
        include_str!(concat!("volume_lighting/", $name, ".wgsl"))
    };
}
/// The three lights: color, x position and turning speed.
const LIGHTS: [(u32, f64, f64); 3] = [
    (0xff0000, -5., -1.),
    (0x00ff00, 0., 0.5),
    (0x0000ff, 5., 1.),
];
fn uniform(r: &Renderer, label: &str, size: u64) -> wgpu::Buffer {
    r.device.create_buffer(&wgpu::BufferDescriptor {
        label: Some(label),
        size,
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    })
}
fn sized(r: &Renderer, label: &str, source: &str, name: &str) -> Result<wgpu::Buffer> {
    Ok(uniform(r, label, pack(source, name, &[])?.len() as u64))
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
fn texture(r: &Renderer, size: (u32, u32), format: wgpu::TextureFormat) -> wgpu::TextureView {
    r.device
        .create_texture(&wgpu::TextureDescriptor {
            label: Some("rect area volume target"),
            size: wgpu::Extent3d {
                width: size.0,
                height: size.1,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        })
        .create_view(&Default::default())
}
/// RectAreaLightTexturesLib's LTC_FLOAT_1 and LTC_FLOAT_2 (half floats where
/// 32-bit float filtering is unavailable).
fn ltc_tables(r: &Renderer) -> [wgpu::TextureView; 2] {
    let float32 = r
        .device
        .features()
        .contains(wgpu::Features::FLOAT32_FILTERABLE);
    let texture = r.device.create_texture_with_data(
        &r.queue,
        &wgpu::TextureDescriptor {
            label: Some("LTC tables"),
            size: wgpu::Extent3d {
                width: 64,
                height: 64,
                depth_or_array_layers: 2,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: if float32 {
                wgpu::TextureFormat::Rgba32Float
            } else {
                wgpu::TextureFormat::Rgba16Float
            },
            usage: wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        },
        wgpu::util::TextureDataOrder::LayerMajor,
        if float32 {
            include_bytes!("../shaders/ltc-r186-f32.bin").as_slice()
        } else {
            include_bytes!("../shaders/ltc-r186.bin").as_slice()
        },
    );
    [0, 1].map(|layer| {
        texture.create_view(&wgpu::TextureViewDescriptor {
            dimension: Some(wgpu::TextureViewDimension::D2),
            base_array_layer: layer,
            array_layer_count: Some(1),
            ..Default::default()
        })
    })
}
/// A lit material's object group: its uniforms, the DFG table and both LTC
/// tables.
fn lit<'a>(
    object: &'a wgpu::Buffer,
    sampler: &'a wgpu::Sampler,
    dfg: &'a wgpu::TextureView,
    ltc: &'a [wgpu::TextureView; 2],
) -> Vec<(u32, wgpu::BindingResource<'a>)> {
    let sampler = wgpu::BindingResource::Sampler(sampler);
    vec![
        (0, object.as_entire_binding()),
        (1, sampler.clone()),
        (2, wgpu::BindingResource::TextureView(dfg)),
        (3, sampler.clone()),
        (4, wgpu::BindingResource::TextureView(&ltc[0])),
        (5, sampler),
        (6, wgpu::BindingResource::TextureView(&ltc[1])),
    ]
}
struct Mesh {
    positions: wgpu::Buffer,
    normals: wgpu::Buffer,
    uvs: wgpu::Buffer,
    index: wgpu::Buffer,
    count: u32,
    radius: f64,
}
struct Targets {
    width: u32,
    height: u32,
    format: wgpu::TextureFormat,
    color: wgpu::TextureView,
    depth: wgpu::TextureView,
    screen: RenderTarget,
    floor: (wgpu::RenderPipeline, [wgpu::BindGroup; 2]),
    /// The knot and the three back panels share the standard pipeline.
    standard: (wgpu::RenderPipeline, wgpu::BindGroup, Vec<wgpu::BindGroup>),
    basic: (wgpu::RenderPipeline, wgpu::BindGroup, Vec<wgpu::BindGroup>),
    volume_pass: (wgpu::RenderPipeline, [wgpu::BindGroup; 2]),
    blur: [wgpu::RenderPipeline; 2],
    output: (wgpu::RenderPipeline, wgpu::BindGroup),
    volumes: Vec<Volume>,
}
struct Volume {
    size: (u32, u32),
    target: wgpu::TextureView,
    blurred: [wgpu::TextureView; 2],
    blur: [wgpu::BindGroup; 2],
    output: [wgpu::BindGroup; 2],
}
pub(super) struct Demo {
    controls: Controls,
    time: f64,
    /// resolution, step count, denoise strength, denoise, fog intensity,
    /// smoke amount.
    params: [f64; 6],
    floor: Mesh,
    knot: Mesh,
    panel: Mesh,
    volume: Mesh,
    floor_render: wgpu::Buffer,
    floor_object: wgpu::Buffer,
    standard_render: wgpu::Buffer,
    /// The knot's, then each back panel's.
    standard_objects: Vec<wgpu::Buffer>,
    basic_render: wgpu::Buffer,
    basic_objects: Vec<wgpu::Buffer>,
    volume_render: wgpu::Buffer,
    volume_object: wgpu::Buffer,
    blur_objects: [wgpu::Buffer; 2],
    output_render: wgpu::Buffer,
    output_object: wgpu::Buffer,
    quad_uv: wgpu::Buffer,
    ltc: [wgpu::TextureView; 2],
    bayer: wgpu::TextureView,
    noise: wgpu::TextureView,
    linear: wgpu::Sampler,
    noise_sampler: wgpu::Sampler,
    targets: Option<Targets>,
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
            far: 250.,
            aspect,
            ..Default::default()
        }));
        s.get_mut(c)?.position = Vector3::new(0., 5., -15.);
        let mut controls = Controls::new(None, (5., 200.), PI, true);
        controls.set_target(Vector3::new(0., 5.5, 0.));
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
        let mesh = |g: BufferGeometry| -> Result<Mesh> {
            let f = |name: &str, n: usize| -> Result<Vec<f32>> {
                let a = g
                    .attributes
                    .get(name)
                    .ok_or(Error::Invalid("rect area attribute"))?;
                (0..a.count())
                    .flat_map(|i| (0..n).map(move |k| (i, k)))
                    .map(|(i, k)| a.get_component(i, k).map(|v| v as f32))
                    .collect()
            };
            let positions = f("position", 3)?;
            let index = g.index.clone().ok_or(Error::Invalid("rect area index"))?;
            Ok(Mesh {
                radius: positions
                    .chunks(3)
                    .map(|p| (p[0] as f64).hypot(p[1] as f64).hypot(p[2] as f64))
                    .fold(0., f64::max),
                positions: init(
                    "rect area positions",
                    bytemuck::cast_slice(&positions),
                    vertex,
                ),
                normals: init(
                    "rect area normals",
                    bytemuck::cast_slice(&f("normal", 3)?),
                    vertex,
                ),
                uvs: init("rect area uvs", bytemuck::cast_slice(&f("uv", 2)?), vertex),
                index: init(
                    "rect area index",
                    bytemuck::cast_slice(&index),
                    wgpu::BufferUsages::INDEX,
                ),
                count: index.len() as u32,
            })
        };
        let standard_objects = (0..4)
            .map(|_| sized(r, "standard object", wgsl!("standard_fs"), "objectStruct"))
            .collect::<Result<Vec<_>>>()?;
        let basic_objects = (0..3)
            .map(|_| sized(r, "basic object", wgsl!("basic_fs"), "objectStruct"))
            .collect::<Result<Vec<_>>>()?;
        Ok(Self {
            controls,
            time: 0.,
            params: [0.25, 12., 0.6, 1., 1., 2.],
            floor: mesh(BoxGeometry::build(2000., 0.1, 2000.)?)?,
            knot: mesh(TorusKnotGeometry::build(1.5, 0.5, 200, 16, 2, 3)?)?,
            panel: mesh(PlaneGeometry::build(4., 10., 1, 1)?)?,
            volume: mesh(BoxGeometry::build(50., 40., 50.)?)?,
            floor_render: sized(r, "floor render", wgsl!("floor_fs"), "renderStruct")?,
            floor_object: sized(r, "floor object", wgsl!("floor_fs"), "objectStruct")?,
            standard_render: sized(r, "standard render", wgsl!("standard_fs"), "renderStruct")?,
            standard_objects,
            basic_render: uniform(r, "basic render", 128),
            basic_objects,
            volume_render: sized(r, "volume render", wgsl!("volume_fs"), "renderStruct")?,
            volume_object: sized(r, "volume object", wgsl!("volume_fs"), "objectStruct")?,
            blur_objects: [0; 2].map(|_| uniform(r, "blur object", 16)),
            output_render: uniform(r, "output render", 16),
            output_object: uniform(r, "output object", 16),
            quad_uv: init(
                "quad uv",
                bytemuck::cast_slice(&[0f32, -1., 0., 1., 2., 1.]),
                vertex,
            ),
            ltc: ltc_tables(r),
            bayer: bayer_texture(r)?,
            noise: noise_texture(r),
            linear: r.device.create_sampler(&wgpu::SamplerDescriptor {
                mag_filter: wgpu::FilterMode::Linear,
                min_filter: wgpu::FilterMode::Linear,
                ..Default::default()
            }),
            // RepeatWrapping in s and t; r keeps the default clamp.
            noise_sampler: r.device.create_sampler(&wgpu::SamplerDescriptor {
                address_mode_u: wgpu::AddressMode::Repeat,
                address_mode_v: wgpu::AddressMode::Repeat,
                address_mode_w: wgpu::AddressMode::ClampToEdge,
                mag_filter: wgpu::FilterMode::Linear,
                min_filter: wgpu::FilterMode::Linear,
                ..Default::default()
            }),
            targets: None,
        })
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, dt: f64, animate: bool) -> Result<()> {
        if animate {
            self.time += dt;
        }
        Ok(())
    }
    pub fn prepare(&mut self, _s: &mut Scene, _c: Object3D, _r: &Renderer) -> Result<()> {
        Ok(())
    }
    fn resize(&mut self, r: &Renderer, out: &RenderTarget) -> Result<()> {
        let (width, height) = (out.width, out.height);
        let color = texture(r, (width, height), HALF);
        let depth = texture(r, (width, height), DEPTH);
        let module = |label, source: &str| {
            r.device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some(label),
                source: wgpu::ShaderSource::Wgsl(source.into()),
            })
        };
        let attributes = [
            wgpu::vertex_attr_array![0 => Float32x3],
            wgpu::vertex_attr_array![1 => Float32x3],
            wgpu::vertex_attr_array![0 => Float32x2],
            wgpu::vertex_attr_array![2 => Float32x3],
        ];
        let layout = |i: usize, stride| wgpu::VertexBufferLayout {
            array_stride: stride,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &attributes[i],
        };
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
        // (depth, front face clockwise, blended)
        let pipeline = |label,
                        vs: &str,
                        fs: &str,
                        buffers: &[wgpu::VertexBufferLayout],
                        format,
                        state: (bool, bool, bool)| {
            let (depth, cw, blended) = state;
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
                            blend: blended.then_some(blend),
                            write_mask: wgpu::ColorWrites::ALL,
                        })],
                    }),
                    primitive: wgpu::PrimitiveState {
                        front_face: if cw {
                            wgpu::FrontFace::Cw
                        } else {
                            wgpu::FrontFace::Ccw
                        },
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
        let tex = wgpu::BindingResource::TextureView;
        let lit = |object| lit(object, &self.linear, &r.dfg, &self.ltc);
        let floor = pipeline(
            "rect area floor",
            wgsl!("floor_vs"),
            wgsl!("floor_fs"),
            &[layout(2, 8), layout(1, 12), layout(3, 12)],
            HALF,
            (true, false, false),
        );
        let floor_binds = [
            bind(
                r,
                floor.get_bind_group_layout(0),
                &[(0, self.floor_render.as_entire_binding())],
            ),
            bind(r, floor.get_bind_group_layout(1), &lit(&self.floor_object)),
        ];
        let standard = pipeline(
            "rect area standard",
            wgsl!("standard_vs"),
            wgsl!("standard_fs"),
            &[layout(0, 12), layout(1, 12)],
            HALF,
            (true, false, false),
        );
        let standard_render = bind(
            r,
            standard.get_bind_group_layout(0),
            &[(0, self.standard_render.as_entire_binding())],
        );
        let standard_objects = self
            .standard_objects
            .iter()
            .map(|object| bind(r, standard.get_bind_group_layout(1), &lit(object)))
            .collect();
        // The front panels: MeshBasicMaterial on the back side.
        let basic = pipeline(
            "rect area panel",
            wgsl!("basic_vs"),
            wgsl!("basic_fs"),
            &[layout(0, 12)],
            HALF,
            (true, true, false),
        );
        let basic_render = bind(
            r,
            basic.get_bind_group_layout(0),
            &[(0, self.basic_render.as_entire_binding())],
        );
        let basic_objects = self
            .basic_objects
            .iter()
            .map(|object| {
                bind(
                    r,
                    basic.get_bind_group_layout(1),
                    &[(0, object.as_entire_binding())],
                )
            })
            .collect();
        // VolumeNodeMaterial: the box's back faces, blended, without depth.
        let volume = pipeline(
            "rect area volume",
            wgsl!("volume_vs"),
            wgsl!("volume_fs"),
            &[layout(0, 12)],
            HALF,
            (false, true, true),
        );
        let volume_binds = [
            bind(
                r,
                volume.get_bind_group_layout(0),
                &[(0, self.volume_render.as_entire_binding())],
            ),
            bind(
                r,
                volume.get_bind_group_layout(1),
                &[
                    (0, self.volume_object.as_entire_binding()),
                    (2, tex(&self.bayer)),
                    (3, tex(&depth)),
                    (4, wgpu::BindingResource::Sampler(&self.noise_sampler)),
                    (5, tex(&self.noise)),
                ],
            ),
        ];
        let blur = [shared!("blur_h_fs"), shared!("blur_v_fs")].map(|fs| {
            pipeline(
                "gaussian blur",
                shared!("blur_vs"),
                fs,
                &[layout(2, 8)],
                HALF,
                (false, false, false),
            )
        });
        let output = pipeline(
            "render output",
            shared!("output_vs"),
            shared!("output_fs"),
            &[layout(2, 8)],
            out.options.format,
            (false, false, false),
        );
        let output_render = bind(
            r,
            output.get_bind_group_layout(0),
            &[(0, self.output_render.as_entire_binding())],
        );
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
            color,
            depth,
            screen,
            floor: (floor, floor_binds),
            standard: (standard, standard_render, standard_objects),
            basic: (basic, basic_render, basic_objects),
            volume_pass: (volume, volume_binds),
            blur,
            output: (output, output_render),
            volumes: vec![],
        });
        Ok(())
    }
    /// The volumetric target for a size, created on first use.
    fn volume_targets(&mut self, r: &Renderer, size: (u32, u32)) {
        let Some(t) = self.targets.as_mut() else {
            return;
        };
        if t.volumes.iter().any(|v| v.size == size) {
            return;
        }
        let target = texture(r, size, HALF);
        let blurred = [0; 2].map(|_| texture(r, size, HALF));
        let tex = wgpu::BindingResource::TextureView;
        let sampler = wgpu::BindingResource::Sampler(&self.linear);
        let blur = [
            (&target, &self.blur_objects[0]),
            (&blurred[0], &self.blur_objects[1]),
        ]
        .iter()
        .zip(&t.blur)
        .map(|((source, object), pipeline)| {
            bind(
                r,
                pipeline.get_bind_group_layout(0),
                &[
                    (0, sampler.clone()),
                    (1, tex(source)),
                    (2, object.as_entire_binding()),
                ],
            )
        })
        .collect::<Vec<_>>();
        let output = [&blurred[1], &target].map(|volumetric| {
            bind(
                r,
                t.output.0.get_bind_group_layout(1),
                &[
                    (0, sampler.clone()),
                    (1, tex(&t.color)),
                    (2, sampler.clone()),
                    (3, tex(volumetric)),
                    (4, self.output_object.as_entire_binding()),
                ],
            )
        });
        let Ok(blur) = <[wgpu::BindGroup; 2]>::try_from(blur) else {
            return;
        };
        t.volumes.push(Volume {
            size,
            target,
            blurred,
            blur,
            output,
        });
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
        let volume_size = self.targets.as_ref().map_or((1, 1), |t| {
            (
                ((t.width as f64 * self.params[0]) as u32).max(1),
                ((t.height as f64 * self.params[0]) as u32).max(1),
            )
        });
        self.volume_targets(r, volume_size);
        s.update()?;
        let (camera, world) = s.camera(c)?;
        let (projection, view) = (camera.projection_matrix()?, world.inverse());
        let camera_position = world.transform_point3(Vector3::ZERO);
        let time = self.time;
        let [_, steps, strength, denoise, fog, smoke] = self.params;
        // animate(): the lights turn about y by the timer's deltas.
        let lights: Vec<(Vec<f64>, Matrix4)> = LIGHTS
            .iter()
            .map(|&(hex, x, speed)| {
                let world = Matrix4::from_translation(Vector3::new(x, 6., 5.))
                    * Matrix4::from_rotation_y(speed * time);
                (Color::from_hex(hex).0.to_array().to_vec(), world)
            })
            .collect();
        // RectAreaLightNode: the color × intensity, the view-space position
        // and the rotated half extents.
        let light_values = |names: [[&'static str; 4]; 3]| {
            let mut values = vec![];
            for (i, (color, world)) in lights.iter().enumerate() {
                let rotation = view * world;
                let rotate = |v: Vector3| rotation.transform_vector3(v).to_array().to_vec();
                let [c, p, w, h] = names[i];
                values.push((c, color.iter().map(|v| v * 5.).collect::<Vec<_>>()));
                values.push((
                    p,
                    view.transform_point3(world.w_axis.truncate())
                        .to_array()
                        .to_vec(),
                ));
                values.push((w, rotate(Vector3::new(2., 0., 0.))));
                values.push((h, rotate(Vector3::new(0., 5., 0.))));
            }
            values
        };
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
        let mut values = camera_values.clone();
        values.extend(light_values([
            [
                "nodeUniform14",
                "nodeUniform15",
                "nodeUniform16",
                "nodeUniform17",
            ],
            [
                "nodeUniform20",
                "nodeUniform21",
                "nodeUniform22",
                "nodeUniform23",
            ],
            [
                "nodeUniform26",
                "nodeUniform27",
                "nodeUniform28",
                "nodeUniform29",
            ],
        ]));
        write(
            &self.standard_render,
            wgsl!("standard_fs"),
            "renderStruct",
            &values,
        )?;
        let mut values = camera_values.clone();
        values.extend(light_values([
            [
                "nodeUniform13",
                "nodeUniform14",
                "nodeUniform15",
                "nodeUniform16",
            ],
            [
                "nodeUniform19",
                "nodeUniform20",
                "nodeUniform21",
                "nodeUniform22",
            ],
            [
                "nodeUniform25",
                "nodeUniform26",
                "nodeUniform27",
                "nodeUniform28",
            ],
        ]));
        write(
            &self.floor_render,
            wgsl!("floor_fs"),
            "renderStruct",
            &values,
        )?;
        let identity = m3(Matrix4::IDENTITY);
        let floor_model = Matrix4::IDENTITY;
        let mut values = vec![
            (
                "nodeUniform0",
                Color::from_hex(0x444444).0.to_array().to_vec(),
            ),
            ("nodeUniform1", vec![1.]),
            ("nodeUniform2", vec![0.]),
            ("nodeUniform4", m3(floor_model)),
            ("nodeUniform6", vec![1.]),
            ("nodeUniform8", m4(floor_model)),
        ];
        for name in [
            "nodeUniform10",
            "nodeUniform12",
            "nodeUniform17",
            "nodeUniform18",
            "nodeUniform23",
            "nodeUniform24",
        ] {
            values.push((name, identity.clone()));
        }
        write(
            &self.floor_object,
            wgsl!("floor_fs"),
            "objectStruct",
            &values,
        )?;
        let knot_model = Matrix4::from_translation(Vector3::new(0., 5.5, 0.));
        let panel =
            |world: Matrix4, z: f64| world * Matrix4::from_translation(Vector3::new(0., 0., z));
        // The knot (white, roughness 0), then the dark back panels
        // (roughness 1).
        let standard_models: Vec<(Matrix4, u32, f64)> = std::iter::once((knot_model, 0xffffff, 0.))
            .chain(
                lights
                    .iter()
                    .map(|(_, world)| (panel(*world, 0.08), 0x111111, 1.)),
            )
            .collect();
        for ((model, hex, roughness), buffer) in standard_models.iter().zip(&self.standard_objects)
        {
            let mut values = vec![
                ("nodeUniform0", Color::from_hex(*hex).0.to_array().to_vec()),
                ("nodeUniform1", vec![1.]),
                ("nodeUniform2", vec![0.]),
                ("nodeUniform3", vec![*roughness]),
                ("nodeUniform5", m3(model.inverse().transpose())),
                ("nodeUniform7", vec![1.]),
                ("nodeUniform9", m4(*model)),
            ];
            for name in [
                "nodeUniform11",
                "nodeUniform13",
                "nodeUniform18",
                "nodeUniform19",
                "nodeUniform24",
                "nodeUniform25",
            ] {
                values.push((name, identity.clone()));
            }
            write(buffer, wgsl!("standard_fs"), "objectStruct", &values)?;
        }
        let mut camera_data: Vec<f32> = projection.to_cols_array().map(|v| v as f32).to_vec();
        camera_data.extend(view.to_cols_array().map(|v| v as f32));
        r.queue
            .write_buffer(&self.basic_render, 0, bytemuck::cast_slice(&camera_data));
        for ((color, world), buffer) in lights.iter().zip(&self.basic_objects) {
            write(
                buffer,
                wgsl!("basic_fs"),
                "objectStruct",
                &[
                    ("nodeUniform0", color.clone()),
                    ("nodeUniform1", vec![1.]),
                    ("nodeUniform4", m4(panel(*world, 0.01))),
                ],
            )?;
        }
        let volume_model = Matrix4::from_translation(Vector3::new(0., 20., 0.));
        let mut values = camera_values;
        values.extend(light_values([
            [
                "nodeUniform11",
                "nodeUniform12",
                "nodeUniform13",
                "nodeUniform14",
            ],
            [
                "nodeUniform15",
                "nodeUniform16",
                "nodeUniform17",
                "nodeUniform18",
            ],
            [
                "nodeUniform19",
                "nodeUniform20",
                "nodeUniform21",
                "nodeUniform22",
            ],
        ]));
        values.extend([
            ("cameraNear", vec![0.1]),
            ("cameraFar", vec![250.]),
            ("nodeUniform24", vec![time]),
            ("cameraPosition", camera_position.to_array().to_vec()),
            (
                "nodeUniform10",
                vec![volume_size.0 as f64, volume_size.1 as f64],
            ),
        ]);
        write(
            &self.volume_render,
            wgsl!("volume_fs"),
            "renderStruct",
            &values,
        )?;
        write(
            &self.volume_object,
            wgsl!("volume_fs"),
            "objectStruct",
            &[
                ("nodeUniform0", vec![1.]),
                ("nodeUniform2", m4(volume_model)),
                ("nodeUniform3", vec![self.volume.radius]),
                ("nodeUniform4", vec![steps.trunc()]),
                ("nodeUniform25", vec![smoke]),
            ],
        )?;
        let texel = [1. / volume_size.0 as f32, 1. / volume_size.1 as f32];
        for object in &self.blur_objects {
            r.queue.write_buffer(
                object,
                0,
                bytemuck::cast_slice(&[strength as f32, 0., texel[0], texel[1]]),
            );
        }
        r.queue.write_buffer(
            &self.output_render,
            0,
            bytemuck::cast_slice(&[2f32, 0., 0., 0.]),
        );
        r.queue.write_buffer(
            &self.output_object,
            0,
            bytemuck::cast_slice(&[fog as f32, 0., 0., 0.]),
        );
        let t = self
            .targets
            .as_ref()
            .ok_or(Error::Invalid("rect area targets"))?;
        let v = t
            .volumes
            .iter()
            .find(|v| v.size == volume_size)
            .ok_or(Error::Invalid("rect area volume target"))?;
        let frustum = Frustum::from_projection(projection * view);
        let visible = |m: &Mesh, model: Matrix4| {
            let scale = model.x_axis.truncate().length();
            frustum.intersects_sphere(Sphere {
                center: model.transform_point3(Vector3::ZERO),
                radius: m.radius * scale,
            })
        };
        let draw = |pass: &mut wgpu::RenderPass, m: &Mesh, slots: &[&wgpu::Buffer]| {
            for (slot, buffer) in slots.iter().enumerate() {
                pass.set_vertex_buffer(slot as u32, buffer.slice(..));
            }
            pass.set_index_buffer(m.index.slice(..), wgpu::IndexFormat::Uint32);
            pass.draw_indexed(0..m.count, 0, 0..1);
        };
        let mut encoder = r.device.create_command_encoder(&Default::default());
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("rect area scene"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &t.color,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
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
            if visible(&self.floor, floor_model) {
                pass.set_pipeline(&t.floor.0);
                pass.set_bind_group(0, &t.floor.1[0], &[]);
                pass.set_bind_group(1, &t.floor.1[1], &[]);
                draw(
                    &mut pass,
                    &self.floor,
                    &[&self.floor.uvs, &self.floor.normals, &self.floor.positions],
                );
            }
            if visible(&self.knot, knot_model) {
                pass.set_pipeline(&t.standard.0);
                pass.set_bind_group(0, &t.standard.1, &[]);
                pass.set_bind_group(1, &t.standard.2[0], &[]);
                draw(
                    &mut pass,
                    &self.knot,
                    &[&self.knot.normals, &self.knot.positions],
                );
            }
            pass.set_pipeline(&t.basic.0);
            pass.set_bind_group(0, &t.basic.1, &[]);
            for ((_, world), group) in lights.iter().zip(&t.basic.2) {
                if visible(&self.panel, panel(*world, 0.01)) {
                    pass.set_bind_group(1, group, &[]);
                    draw(&mut pass, &self.panel, &[&self.panel.positions]);
                }
            }
            pass.set_pipeline(&t.standard.0);
            pass.set_bind_group(0, &t.standard.1, &[]);
            for ((model, _, _), group) in standard_models.iter().zip(&t.standard.2).skip(1) {
                if visible(&self.panel, *model) {
                    pass.set_bind_group(1, group, &[]);
                    draw(
                        &mut pass,
                        &self.panel,
                        &[&self.panel.normals, &self.panel.positions],
                    );
                }
            }
        }
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("volumetric lighting"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &v.target,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                ..Default::default()
            });
            if visible(&self.volume, volume_model) {
                pass.set_pipeline(&t.volume_pass.0);
                pass.set_bind_group(0, &t.volume_pass.1[0], &[]);
                pass.set_bind_group(1, &t.volume_pass.1[1], &[]);
                draw(&mut pass, &self.volume, &[&self.volume.positions]);
            }
        }
        let quad = |encoder: &mut wgpu::CommandEncoder,
                    target: &wgpu::TextureView,
                    pipeline: &wgpu::RenderPipeline,
                    groups: &[&wgpu::BindGroup]| {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("rect area post"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: target,
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
            for (i, group) in groups.iter().enumerate() {
                pass.set_bind_group(i as u32, *group, &[]);
            }
            pass.set_vertex_buffer(0, self.quad_uv.slice(..));
            pass.draw(0..3, 0..1);
        };
        let denoised = denoise > 0.5;
        if denoised {
            quad(&mut encoder, &v.blurred[0], &t.blur[0], &[&v.blur[0]]);
            quad(&mut encoder, &v.blurred[1], &t.blur[1], &[&v.blur[1]]);
        }
        quad(
            &mut encoder,
            &t.screen.view,
            &t.output.0,
            &[&t.output.1, &v.output[if denoised { 0 } else { 1 }]],
        );
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
    /// resolution, step count, denoise strength, denoise, fog intensity and
    /// smoke amount.
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        *self
            .params
            .get_mut(index)
            .ok_or(Error::Invalid("rect area parameter"))? = value as f64;
        Ok(())
    }
    pub fn seek(&mut self, t: f64) {
        self.time = t;
    }
}
