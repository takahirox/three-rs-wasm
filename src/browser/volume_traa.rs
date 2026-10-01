//! webgpu_volume_lighting_traa: webgpu_volume_lighting's teapot, floor,
//! wandering point light and sweeping spot light, with the fog volume drawn
//! additively at full resolution inside the scene pass and resolved by TRAA.
//! A depth pre-pass of the opaque objects gives the volume its occlusion;
//! the scene pass writes color and velocity (MRT); the volume dithers its
//! ray offsets with interleaved gradient noise rotated by the frame's Halton
//! offset. TRAANode jitters the camera by the 32-sample Halton sequence,
//! reprojects its history by the velocity, rejects disoccluded history by
//! the previous frame's depth and blends with flicker reduction; the output
//! applies NeutralToneMapping (exposure 2). Every stage runs the WGSL
//! three.js r186 generates for the page (in `volume_traa/`; the shadows and
//! the pre-pass teapot are `volume_lighting/`'s).
use super::controls_attributes::{Controls, camera_state};
use super::lights_projector::{m3, m4, pack};
use super::volume_lighting::{
    DEPTH, HALF, Mesh, POINT_SIZE, SPOT_SIZE, Shadows, bind, noise_texture, perspective, sized,
    spot_map, texture, uniform, view,
};
use crate::{
    Error, Result, camera::*, geometry::*, math::*, render_target::*, renderer::*, scene::*,
};
use std::f64::consts::PI;
use wgpu::util::DeviceExt;

macro_rules! wgsl {
    ($name:literal) => {
        include_str!(concat!("volume_traa/", $name, ".wgsl"))
    };
}
macro_rules! shared {
    ($name:literal) => {
        include_str!(concat!("volume_lighting/", $name, ".wgsl"))
    };
}
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
/// The previous frame's matrices VelocityNode reads.
#[derive(Clone, Copy)]
struct Motion {
    projection: Matrix4,
    view: Matrix4,
    teapot: Matrix4,
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
    pre_color: wgpu::TextureView,
    pre_depth: (wgpu::Texture, wgpu::TextureView),
    color: (wgpu::Texture, wgpu::TextureView),
    velocity: wgpu::TextureView,
    depth: wgpu::TextureView,
    resolve: (wgpu::Texture, wgpu::TextureView),
    history: wgpu::Texture,
    history_depth: wgpu::Texture,
    screen: RenderTarget,
    pre_teapot: (wgpu::RenderPipeline, [wgpu::BindGroup; 2]),
    pre_floor: (wgpu::RenderPipeline, [wgpu::BindGroup; 2]),
    teapot: (wgpu::RenderPipeline, [wgpu::BindGroup; 2]),
    floor: (wgpu::RenderPipeline, [wgpu::BindGroup; 2]),
    volume: (wgpu::RenderPipeline, [wgpu::BindGroup; 2]),
    /// Resolve with the placeholder, or the history, previous depth.
    traa: (wgpu::RenderPipeline, [wgpu::BindGroup; 2]),
    /// Output of the resolved, or the raw scene, color.
    output: (wgpu::RenderPipeline, wgpu::BindGroup, [wgpu::BindGroup; 2]),
}
pub(super) struct Demo {
    controls: Controls,
    /// The fixture clock and the page's accumulated animation time.
    clock: Option<f64>,
    animation: f64,
    /// animated, TRAA, step count, light intensity, spot intensity,
    /// volumetric intensity (unused by the page), smoke amount.
    params: [f64; 7],
    /// A frame was requested (the page's animate()); other redraws only
    /// present the last frame again.
    pending: bool,
    frames: usize,
    jitter: usize,
    /// The output node with TRAA has been built: the frame that builds it
    /// renders the scene unjittered (its before-pipeline hook registers
    /// during that frame).
    built: bool,
    /// TRAA resolves since the history (re)started.
    resolves: usize,
    motion: Option<Motion>,
    resolved: Option<Resolved>,
    teapot: Mesh,
    floor: Mesh,
    volume: Mesh,
    shadows: Shadows,
    teapot_render: wgpu::Buffer,
    teapot_object: wgpu::Buffer,
    floor_render: wgpu::Buffer,
    floor_object: wgpu::Buffer,
    pre_teapot_render: wgpu::Buffer,
    pre_teapot_object: wgpu::Buffer,
    pre_floor_render: wgpu::Buffer,
    pre_floor_object: wgpu::Buffer,
    volume_render: wgpu::Buffer,
    volume_object: wgpu::Buffer,
    traa_object: wgpu::Buffer,
    output_render: wgpu::Buffer,
    quad_uv: wgpu::Buffer,
    colors: wgpu::TextureView,
    noise: wgpu::TextureView,
    /// TRAANode's 1 × 1 DepthTexture, bound until the history depth is.
    placeholder: wgpu::TextureView,
    linear: wgpu::Sampler,
    mipmapped: wgpu::Sampler,
    noise_sampler: wgpu::Sampler,
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
            fov: 60.,
            near: 0.1,
            far: 100.,
            aspect,
            ..Default::default()
        }));
        s.get_mut(c)?.position = Vector3::new(-8., 1., -6.);
        let mut controls = Controls::new(None, (2., 40.), PI, true);
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
        let vertex = wgpu::BufferUsages::VERTEX;
        let mesh = |g: BufferGeometry| -> Result<Mesh> {
            let f = |name: &str| -> Result<Vec<f32>> {
                let a = g
                    .attributes
                    .get(name)
                    .ok_or(Error::Invalid("traa attribute"))?;
                (0..a.count())
                    .flat_map(|i| (0..3).map(move |k| (i, k)))
                    .map(|(i, k)| a.get_component(i, k).map(|v| v as f32))
                    .collect()
            };
            let positions = f("position")?;
            let index = g.index.clone().ok_or(Error::Invalid("traa index"))?;
            Ok(Mesh {
                radius: positions
                    .chunks(3)
                    .map(|p| (p[0] as f64).hypot(p[1] as f64).hypot(p[2] as f64))
                    .fold(0., f64::max),
                positions: init("traa positions", bytemuck::cast_slice(&positions), vertex),
                normals: init("traa normals", bytemuck::cast_slice(&f("normal")?), vertex),
                index: init(
                    "traa index",
                    bytemuck::cast_slice(&index),
                    wgpu::BufferUsages::INDEX,
                ),
                count: index.len() as u32,
            })
        };
        let sampler = |mipmap| {
            r.device.create_sampler(&wgpu::SamplerDescriptor {
                mag_filter: wgpu::FilterMode::Linear,
                min_filter: wgpu::FilterMode::Linear,
                mipmap_filter: mipmap,
                ..Default::default()
            })
        };
        Ok(Self {
            controls,
            clock: None,
            animation: 0.,
            params: [1., 1., 12., 3., 100., 1., 2.],
            pending: true,
            frames: 0,
            jitter: 0,
            built: false,
            resolves: 0,
            motion: None,
            resolved: None,
            teapot: mesh(super::models_modifiers::teapot(
                0.8, 18, true, true, true, true, true,
            )?)?,
            floor: mesh(PlaneGeometry::build(100., 100., 1, 1)?)?,
            volume: mesh(BoxGeometry::build(20., 10., 20.)?)?,
            shadows: Shadows::new(r),
            teapot_render: sized(r, "teapot render", wgsl!("teapot_fs"), "renderStruct")?,
            teapot_object: sized(r, "teapot object", wgsl!("teapot_fs"), "objectStruct")?,
            floor_render: sized(r, "floor render", wgsl!("floor_fs"), "renderStruct")?,
            floor_object: sized(r, "floor object", wgsl!("floor_fs"), "objectStruct")?,
            pre_teapot_render: sized(r, "pre teapot render", shared!("teapot_fs"), "renderStruct")?,
            pre_teapot_object: sized(r, "pre teapot object", shared!("teapot_fs"), "objectStruct")?,
            pre_floor_render: sized(r, "pre floor render", wgsl!("floor_pre_fs"), "renderStruct")?,
            pre_floor_object: sized(r, "pre floor object", wgsl!("floor_pre_fs"), "objectStruct")?,
            volume_render: sized(r, "volume render", wgsl!("volume_fs"), "renderStruct")?,
            volume_object: sized(r, "volume object", wgsl!("volume_fs"), "objectStruct")?,
            traa_object: sized(r, "traa object", wgsl!("traa_fs"), "objectStruct")?,
            output_render: uniform(r, "output render", 16),
            quad_uv: init(
                "quad uv",
                bytemuck::cast_slice(&[0f32, -1., 0., 1., 2., 1.]),
                vertex,
            ),
            colors: spot_map(r).await?,
            noise: noise_texture(r),
            placeholder: placeholder(r),
            linear: sampler(wgpu::FilterMode::Nearest),
            mipmapped: sampler(wgpu::FilterMode::Linear),
            // RepeatWrapping in s and t; r keeps the default clamp.
            noise_sampler: r.device.create_sampler(&wgpu::SamplerDescriptor {
                address_mode_u: wgpu::AddressMode::Repeat,
                address_mode_v: wgpu::AddressMode::Repeat,
                address_mode_w: wgpu::AddressMode::ClampToEdge,
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
    /// animate(): the page's clock delta advances the animation while
    /// animated.
    fn advance(&mut self, now: f64) {
        self.pending = true;
        let delta = now - self.clock.unwrap_or(0.);
        self.clock = Some(now);
        if self.params[0] > 0.5 {
            self.animation += delta;
        }
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, dt: f64, animate: bool) -> Result<()> {
        if animate {
            let now = self.clock.unwrap_or(0.) + dt;
            self.advance(now);
        }
        Ok(())
    }
    pub fn prepare(&mut self, _s: &mut Scene, _c: Object3D, _r: &Renderer) -> Result<()> {
        Ok(())
    }
    fn resize(&mut self, r: &Renderer, out: &RenderTarget) -> Result<()> {
        let (width, height) = (out.width, out.height);
        let size = (width, height, 1);
        let target = |format, copy: bool| {
            r.device.create_texture(&wgpu::TextureDescriptor {
                label: Some("traa target"),
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
                    | wgpu::TextureUsages::TEXTURE_BINDING
                    | if copy {
                        wgpu::TextureUsages::COPY_SRC | wgpu::TextureUsages::COPY_DST
                    } else {
                        wgpu::TextureUsages::empty()
                    },
                view_formats: &[],
            })
        };
        let pre_color = view(&texture(r, size, HALF, true));
        let pre_depth = target(DEPTH, true);
        let color = target(HALF, true);
        let velocity = view(&texture(r, size, HALF, true));
        let depth = view(&texture(r, size, DEPTH, true));
        let resolve = target(HALF, true);
        let history = target(HALF, true);
        let history_depth = target(DEPTH, true);
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
        ];
        let layout = |i: usize, stride| wgpu::VertexBufferLayout {
            array_stride: stride,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &attributes[i],
        };
        // AdditiveBlending.
        let additive = wgpu::BlendState {
            color: wgpu::BlendComponent {
                src_factor: wgpu::BlendFactor::SrcAlpha,
                dst_factor: wgpu::BlendFactor::One,
                operation: wgpu::BlendOperation::Add,
            },
            alpha: wgpu::BlendComponent {
                src_factor: wgpu::BlendFactor::One,
                dst_factor: wgpu::BlendFactor::One,
                operation: wgpu::BlendOperation::Add,
            },
        };
        // (targets, depth: None or (compare, write), cull, clockwise, blended)
        let pipeline = |label,
                        vs: &str,
                        fs: &str,
                        buffers: &[wgpu::VertexBufferLayout],
                        formats: &[wgpu::TextureFormat],
                        depth: Option<(wgpu::CompareFunction, bool)>,
                        state: (Option<wgpu::Face>, bool, bool)| {
            let (cull, cw, blended) = state;
            let targets: Vec<_> = formats
                .iter()
                .map(|&format| {
                    Some(wgpu::ColorTargetState {
                        format,
                        blend: blended.then_some(additive),
                        write_mask: wgpu::ColorWrites::ALL,
                    })
                })
                .collect();
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
                        targets: &targets,
                    }),
                    primitive: wgpu::PrimitiveState {
                        front_face: if cw {
                            wgpu::FrontFace::Cw
                        } else {
                            wgpu::FrontFace::Ccw
                        },
                        cull_mode: cull,
                        ..Default::default()
                    },
                    depth_stencil: depth.map(|(compare, write)| wgpu::DepthStencilState {
                        format: DEPTH,
                        depth_write_enabled: write,
                        depth_compare: compare,
                        stencil: Default::default(),
                        bias: Default::default(),
                    }),
                    multisample: Default::default(),
                    multiview: None,
                    cache: None,
                })
        };
        let less = Some((wgpu::CompareFunction::LessEqual, true));
        let back = Some(wgpu::Face::Back);
        let tex = wgpu::BindingResource::TextureView;
        let sampler = wgpu::BindingResource::Sampler;
        let render_group = |p: &wgpu::RenderPipeline, buffer: &wgpu::Buffer| {
            bind(
                r,
                p.get_bind_group_layout(0),
                &[(0, buffer.as_entire_binding())],
            )
        };
        let teapot_group = |p: &wgpu::RenderPipeline, object: &wgpu::Buffer| {
            bind(
                r,
                p.get_bind_group_layout(1),
                &[
                    (0, object.as_entire_binding()),
                    (1, sampler(&self.linear)),
                    (2, tex(&r.dfg)),
                    (3, sampler(&self.mipmapped)),
                    (4, tex(&self.colors)),
                ],
            )
        };
        let floor_group = |p: &wgpu::RenderPipeline, object: &wgpu::Buffer| {
            bind(
                r,
                p.get_bind_group_layout(1),
                &[
                    (0, object.as_entire_binding()),
                    (1, sampler(&self.linear)),
                    (2, tex(&r.dfg)),
                    (3, sampler(&self.compare)),
                    (4, tex(&self.shadows.point_cube)),
                    (5, sampler(&self.compare)),
                    (6, tex(&self.shadows.spot.1)),
                    (7, sampler(&self.mipmapped)),
                    (8, tex(&self.colors)),
                ],
            )
        };
        let mrt = [HALF, HALF];
        let pre_teapot = pipeline(
            "traa pre teapot",
            shared!("teapot_vs"),
            shared!("teapot_fs"),
            &[layout(0, 12), layout(1, 12)],
            &[HALF],
            less,
            (None, false, false),
        );
        let pre_teapot_binds = [
            render_group(&pre_teapot, &self.pre_teapot_render),
            teapot_group(&pre_teapot, &self.pre_teapot_object),
        ];
        let pre_floor = pipeline(
            "traa pre floor",
            wgsl!("floor_pre_vs"),
            wgsl!("floor_pre_fs"),
            &[layout(0, 12), layout(1, 12)],
            &[HALF],
            less,
            (back, false, false),
        );
        let pre_floor_binds = [
            render_group(&pre_floor, &self.pre_floor_render),
            floor_group(&pre_floor, &self.pre_floor_object),
        ];
        let teapot = pipeline(
            "traa teapot",
            wgsl!("teapot_vs"),
            wgsl!("teapot_fs"),
            &[layout(0, 12), layout(1, 12)],
            &mrt,
            less,
            (None, false, false),
        );
        let teapot_binds = [
            render_group(&teapot, &self.teapot_render),
            teapot_group(&teapot, &self.teapot_object),
        ];
        let floor = pipeline(
            "traa floor",
            wgsl!("floor_vs"),
            wgsl!("floor_fs"),
            &[layout(0, 12), layout(1, 12)],
            &mrt,
            less,
            (back, false, false),
        );
        let floor_binds = [
            render_group(&floor, &self.floor_render),
            floor_group(&floor, &self.floor_object),
        ];
        // The volume: back faces, added over the scene, tested always and
        // writing no depth.
        let volume = pipeline(
            "traa volume",
            wgsl!("volume_vs"),
            wgsl!("volume_fs"),
            &[layout(0, 12), layout(1, 12)],
            &mrt,
            Some((wgpu::CompareFunction::Always, false)),
            (back, true, true),
        );
        let pre_depth_view = view(&pre_depth);
        let volume_binds = [
            render_group(&volume, &self.volume_render),
            bind(
                r,
                volume.get_bind_group_layout(1),
                &[
                    (0, self.volume_object.as_entire_binding()),
                    (1, tex(&pre_depth_view)),
                    (2, sampler(&self.compare)),
                    (3, tex(&self.shadows.point_cube)),
                    (4, sampler(&self.compare)),
                    (5, tex(&self.shadows.spot.1)),
                    (6, sampler(&self.mipmapped)),
                    (7, tex(&self.colors)),
                    (8, sampler(&self.noise_sampler)),
                    (9, tex(&self.noise)),
                ],
            ),
        ];
        let quad = [layout(2, 8)];
        let traa = pipeline(
            "traa resolve",
            wgsl!("traa_vs"),
            wgsl!("traa_fs"),
            &quad,
            &[HALF],
            None,
            (back, false, false),
        );
        let (color_view, history_view, history_depth_view) =
            (view(&color), view(&history), view(&history_depth));
        let traa_binds = [&self.placeholder, &history_depth_view].map(|previous| {
            bind(
                r,
                traa.get_bind_group_layout(0),
                &[
                    // Velocity is read with textureLoad: the layout drops its sampler.
                    (1, tex(&velocity)),
                    (2, sampler(&self.linear)),
                    (3, tex(&color_view)),
                    (4, tex(&pre_depth_view)),
                    (5, self.traa_object.as_entire_binding()),
                    (6, tex(previous)),
                    (7, sampler(&self.linear)),
                    (8, tex(&history_view)),
                ],
            )
        });
        let output = pipeline(
            "render output",
            wgsl!("output_vs"),
            wgsl!("output_fs"),
            &quad,
            &[out.options.format],
            None,
            (back, false, false),
        );
        let resolve_view = view(&resolve);
        let output_binds = [&resolve_view, &color_view].map(|source| {
            bind(
                r,
                output.get_bind_group_layout(1),
                &[(0, sampler(&self.linear)), (1, tex(source))],
            )
        });
        let output_render = render_group(&output, &self.output_render);
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
            pre_color,
            pre_depth: (pre_depth, pre_depth_view),
            color: (color, color_view),
            velocity,
            depth,
            resolve: (resolve, resolve_view),
            history,
            history_depth,
            screen,
            pre_teapot: (pre_teapot, pre_teapot_binds),
            pre_floor: (pre_floor, pre_floor_binds),
            teapot: (teapot, teapot_binds),
            floor: (floor, floor_binds),
            volume: (volume, volume_binds),
            traa: (traa, traa_binds),
            output: (output, output_render, output_binds),
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
            // New history targets: TRAA restarts from the beauty buffer.
            self.resolves = 0;
        }
        let t = self
            .targets
            .as_ref()
            .ok_or(Error::Invalid("traa targets"))?;
        if !std::mem::take(&mut self.pending) {
            let mut encoder = r.device.create_command_encoder(&Default::default());
            self.present(&mut encoder, t);
            r.queue.submit([encoder.finish()]);
            return Ok(true);
        }
        s.update()?;
        let (camera, world) = s.camera(c)?;
        let (projection, view) = (camera.projection_matrix()?, world.inverse());
        let camera_position = world.transform_point3(Vector3::ZERO);
        let traa_on = self.params[1] > 0.5;
        // TRAANode.setViewOffset: the camera shifted by the Halton offset
        // (in pixels) for the whole pipeline.
        let jittered = if traa_on && self.built {
            let [x, y] = halton(self.jitter);
            let mut p = projection;
            p.z_axis.x += 2. * (x - 0.5) / t.width as f64;
            p.z_axis.y -= 2. * (y - 0.5) / t.height as f64;
            p
        } else {
            projection
        };
        let temporal = halton(self.frames);
        self.frames += 1;
        let time = self.animation;
        let scale = 2.4;
        let point = Vector3::new(
            (time * 0.7).sin() * scale,
            (time * 0.5).cos() * scale,
            (time * 0.3).cos() * scale,
        );
        let spot = Vector3::new((time * 0.3).cos() * scale, 5., 2.5);
        let teapot_model = Matrix4::from_rotation_y(time * 0.2);
        let floor_model = Matrix4::from_translation(Vector3::new(0., -3., 0.))
            * Matrix4::from_rotation_x(-PI / 2.);
        let volume_model = Matrix4::from_translation(Vector3::new(0., 2., 0.));
        let [_, _, steps, point_intensity, spot_intensity, _, smoke] = self.params;
        // VelocityNode: last frame's projection, view and model matrices (the
        // current ones on the first frame).
        let motion = self.motion.unwrap_or(Motion {
            projection,
            view,
            teapot: teapot_model,
        });
        self.motion = Some(Motion {
            projection,
            view,
            teapot: teapot_model,
        });
        let spot_projection = perspective(60., 1., 15.);
        let spot_view = Matrix4::look_at_rh(spot, Vector3::ZERO, Vector3::Y);
        let bias = Matrix4::from_cols_array(&[
            0.5, 0., 0., 0., 0., 0.5, 0., 0., 0., 0., 1., 0., 0.5, 0.5, 0., 1.,
        ]);
        let spot_matrix = bias * spot_projection * spot_view;
        let point_matrix = Matrix4::from_translation(-point);
        let point_color = Color::from_hex(0xf9bb50)
            .0
            .to_array()
            .map(|v| v * point_intensity)
            .to_vec();
        let spot_color = vec![spot_intensity; 3];
        let cone = (PI / 6.).cos();
        let point_view = view.transform_point3(point).to_array().to_vec();
        let spot_view_position = view.transform_point3(spot).to_array().to_vec();
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
            ("cameraProjectionMatrix", m4(jittered)),
            ("cameraViewMatrix", m4(view)),
        ];
        let mut teapot_values = camera_values.clone();
        teapot_values.extend([
            ("nodeUniform11", point_color.clone()),
            ("nodeUniform12", vec![100.]),
            ("nodeUniform13", vec![2.]),
            ("nodeUniform17", vec![cone]),
            ("nodeUniform18", vec![1.]),
            ("nodeUniform22", vec![0.]),
            ("nodeUniform23", vec![2.]),
            ("nodeUniform16", spot_color.clone()),
            ("nodeUniform10", point_view.clone()),
            ("nodeUniform14", spot_view_position.clone()),
            ("nodeUniform20", spot.to_array().to_vec()),
            ("nodeUniform21", vec![0.; 3]),
            ("nodeUniform15", m4(spot_matrix)),
        ]);
        write(
            &self.pre_teapot_render,
            shared!("teapot_fs"),
            "renderStruct",
            &teapot_values,
        )?;
        teapot_values.push(("nodeUniform27", m4(motion.projection)));
        write(
            &self.teapot_render,
            wgsl!("teapot_fs"),
            "renderStruct",
            &teapot_values,
        )?;
        let standard = |model: Matrix4| {
            vec![
                ("nodeUniform0", vec![1.; 3]),
                ("nodeUniform1", vec![1.]),
                ("nodeUniform2", vec![0.]),
                ("nodeUniform3", vec![1.]),
                ("nodeUniform5", m3(model.inverse().transpose())),
                ("nodeUniform7", vec![1.]),
                ("nodeUniform9", m4(model)),
            ]
        };
        write(
            &self.pre_teapot_object,
            shared!("teapot_fs"),
            "objectStruct",
            &standard(teapot_model),
        )?;
        let mut values = standard(teapot_model);
        values.extend([
            ("nodeUniform25", m4(projection)),
            ("nodeUniform26", m4(teapot_model)),
            ("nodeUniform28", m4(motion.view)),
            ("nodeUniform29", m4(motion.teapot)),
        ]);
        write(
            &self.teapot_object,
            wgsl!("teapot_fs"),
            "objectStruct",
            &values,
        )?;
        write(
            &self.pre_floor_object,
            wgsl!("floor_pre_fs"),
            "objectStruct",
            &standard(floor_model),
        )?;
        let mut values = standard(floor_model);
        values.extend([
            ("nodeUniform40", m4(projection)),
            ("nodeUniform41", m4(floor_model)),
            ("nodeUniform43", m4(motion.view)),
            ("nodeUniform44", m4(floor_model)),
        ]);
        write(
            &self.floor_object,
            wgsl!("floor_fs"),
            "objectStruct",
            &values,
        )?;
        let mut floor_values = camera_values.clone();
        floor_values.extend([
            ("nodeUniform11", point_color.clone()),
            ("nodeUniform22", vec![100.]),
            ("nodeUniform23", vec![2.]),
            ("nodeUniform33", vec![cone]),
            ("nodeUniform34", vec![1.]),
            ("nodeUniform37", vec![0.]),
            ("nodeUniform38", vec![2.]),
            ("nodeUniform26", spot_color.clone()),
            ("nodeUniform10", point_view.clone()),
            ("nodeUniform24", spot_view_position.clone()),
            ("nodeUniform35", spot.to_array().to_vec()),
            ("nodeUniform36", vec![0.; 3]),
            ("nodeUniform25", m4(spot_matrix)),
            ("nodeUniform12", m4(point_matrix)),
            ("nodeUniform16", vec![0.5]),
            ("nodeUniform15", vec![100.]),
            ("nodeUniform14", vec![0.]),
            ("nodeUniform17", vec![0.]),
            ("nodeUniform19", vec![1.]),
            ("nodeUniform20", vec![POINT_SIZE as f64; 2]),
            ("nodeUniform21", vec![1.]),
            ("nodeUniform27", vec![0.]),
            ("nodeUniform28", vec![0.]),
            ("nodeUniform30", vec![1.]),
            ("nodeUniform31", vec![SPOT_SIZE as f64; 2]),
            ("nodeUniform32", vec![0.98]),
        ]);
        write(
            &self.pre_floor_render,
            wgsl!("floor_pre_fs"),
            "renderStruct",
            &floor_values,
        )?;
        floor_values.push(("nodeUniform42", m4(motion.projection)));
        write(
            &self.floor_render,
            wgsl!("floor_fs"),
            "renderStruct",
            &floor_values,
        )?;
        let mut volume_values = camera_values;
        volume_values.extend([
            ("cameraNear", vec![0.1]),
            ("cameraFar", vec![100.]),
            ("nodeUniform12", point_color),
            ("nodeUniform24", vec![100.]),
            ("nodeUniform26", vec![2.]),
            ("nodeUniform35", vec![cone]),
            ("nodeUniform36", vec![1.]),
            ("nodeUniform40", vec![0.]),
            ("nodeUniform41", vec![2.]),
            ("nodeUniform28", spot_color),
            ("nodeUniform25", point_view),
            ("nodeUniform37", spot_view_position),
            ("nodeUniform38", spot.to_array().to_vec()),
            ("nodeUniform39", vec![0.; 3]),
            ("nodeUniform27", m4(spot_matrix)),
            ("nodeUniform13", m4(point_matrix)),
            ("nodeUniform16", vec![0.]),
            ("nodeUniform17", vec![100.]),
            ("nodeUniform18", vec![0.5]),
            ("nodeUniform19", vec![0.]),
            ("nodeUniform21", vec![1.]),
            ("nodeUniform22", vec![POINT_SIZE as f64; 2]),
            ("nodeUniform23", vec![1.]),
            ("nodeUniform29", vec![0.]),
            ("nodeUniform30", vec![0.]),
            ("nodeUniform32", vec![1.]),
            ("nodeUniform33", vec![SPOT_SIZE as f64; 2]),
            ("nodeUniform34", vec![0.98]),
            ("cameraPosition", camera_position.to_array().to_vec()),
            ("nodeUniform11", vec![t.width as f64, t.height as f64]),
            ("nodeUniform48", m4(motion.projection)),
        ]);
        write(
            &self.volume_render,
            wgsl!("volume_fs"),
            "renderStruct",
            &volume_values,
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
                ("nodeUniform5", vec![temporal[0]]),
                ("nodeUniform6", vec![temporal[1]]),
                ("nodeUniform15", m3(volume_model.inverse().transpose())),
                ("nodeUniform44", vec![time]),
                ("nodeUniform45", vec![smoke]),
                ("nodeUniform46", m4(projection)),
                ("nodeUniform47", m4(volume_model)),
                ("nodeUniform49", m4(motion.view)),
                ("nodeUniform50", m4(volume_model)),
            ],
        )?;
        // TRAANode.updateBefore: the previous camera matrices (identity
        // before the first resolve) and this frame's.
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
                wgsl!("traa_fs"),
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
        r.queue.write_buffer(
            &self.output_render,
            0,
            bytemuck::cast_slice(&[2f32, 0., 0., 0.]),
        );
        let mut encoder = r.device.create_command_encoder(&Default::default());
        self.shadows.encode(
            r,
            &mut encoder,
            &self.teapot,
            teapot_model,
            point,
            spot_projection,
            spot_view,
        );
        let frustum = Frustum::from_projection(jittered * view);
        let visible = |m: &Mesh, model: Matrix4| {
            frustum.intersects_sphere(Sphere {
                center: model.transform_point3(Vector3::ZERO),
                radius: m.radius,
            })
        };
        let (teapot_visible, floor_visible) = (
            visible(&self.teapot, teapot_model),
            visible(&self.floor, floor_model),
        );
        let draw = |pass: &mut wgpu::RenderPass, m: &Mesh, slots: &[&wgpu::Buffer]| {
            for (slot, buffer) in slots.iter().enumerate() {
                pass.set_vertex_buffer(slot as u32, buffer.slice(..));
            }
            pass.set_index_buffer(m.index.slice(..), wgpu::IndexFormat::Uint32);
            pass.draw_indexed(0..m.count, 0, 0..1);
        };
        // The depth pre-pass of the opaque objects.
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("traa pre-pass"),
                color_attachments: &[color_attachment(&t.pre_color)],
                depth_stencil_attachment: depth_attachment(&t.pre_depth.1),
                ..Default::default()
            });
            if teapot_visible {
                pass.set_pipeline(&t.pre_teapot.0);
                pass.set_bind_group(0, &t.pre_teapot.1[0], &[]);
                pass.set_bind_group(1, &t.pre_teapot.1[1], &[]);
                draw(
                    &mut pass,
                    &self.teapot,
                    &[&self.teapot.normals, &self.teapot.positions],
                );
            }
            if floor_visible {
                pass.set_pipeline(&t.pre_floor.0);
                pass.set_bind_group(0, &t.pre_floor.1[0], &[]);
                pass.set_bind_group(1, &t.pre_floor.1[1], &[]);
                draw(
                    &mut pass,
                    &self.floor,
                    &[&self.floor.normals, &self.floor.positions],
                );
            }
        }
        // The scene pass: color and velocity.
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("traa scene"),
                color_attachments: &[color_attachment(&t.color.1), color_attachment(&t.velocity)],
                depth_stencil_attachment: depth_attachment(&t.depth),
                ..Default::default()
            });
            if teapot_visible {
                pass.set_pipeline(&t.teapot.0);
                pass.set_bind_group(0, &t.teapot.1[0], &[]);
                pass.set_bind_group(1, &t.teapot.1[1], &[]);
                draw(
                    &mut pass,
                    &self.teapot,
                    &[&self.teapot.normals, &self.teapot.positions],
                );
            }
            if floor_visible {
                pass.set_pipeline(&t.floor.0);
                pass.set_bind_group(0, &t.floor.1[0], &[]);
                pass.set_bind_group(1, &t.floor.1[1], &[]);
                draw(
                    &mut pass,
                    &self.floor,
                    &[&self.floor.normals, &self.floor.positions],
                );
            }
            if visible(&self.volume, volume_model) {
                pass.set_pipeline(&t.volume.0);
                pass.set_bind_group(0, &t.volume.1[0], &[]);
                pass.set_bind_group(1, &t.volume.1[1], &[]);
                draw(
                    &mut pass,
                    &self.volume,
                    &[&self.volume.positions, &self.volume.normals],
                );
            }
        }
        let full = wgpu::Extent3d {
            width: t.width,
            height: t.height,
            depth_or_array_layers: 1,
        };
        let quad = |encoder: &mut wgpu::CommandEncoder,
                    target: &wgpu::TextureView,
                    pipeline: &wgpu::RenderPipeline,
                    groups: &[&wgpu::BindGroup]| {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("traa quad"),
                color_attachments: &[color_attachment(target)],
                ..Default::default()
            });
            pass.set_pipeline(pipeline);
            for (i, group) in groups.iter().enumerate() {
                pass.set_bind_group(i as u32, *group, &[]);
            }
            pass.set_vertex_buffer(0, self.quad_uv.slice(..));
            pass.draw(0..3, 0..1);
        };
        if traa_on {
            // A new history starts from the beauty buffer.
            if self.resolves == 0 {
                encoder.copy_texture_to_texture(
                    t.color.0.as_image_copy(),
                    t.history.as_image_copy(),
                    full,
                );
            }
            // The first two resolves read TRAANode's 1 × 1 placeholder depth
            // (the far plane); later ones the previous frame's.
            let group = &t.traa.1[usize::from(self.resolves >= 2)];
            quad(&mut encoder, &t.resolve.1, &t.traa.0, &[group]);
            encoder.copy_texture_to_texture(
                t.resolve.0.as_image_copy(),
                t.history.as_image_copy(),
                full,
            );
            encoder.copy_texture_to_texture(
                t.pre_depth.0.as_image_copy(),
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
    /// The output pass: tone mapping of the resolved, or the scene, color.
    fn present(&self, encoder: &mut wgpu::CommandEncoder, t: &Targets) {
        let source = usize::from(self.params[1] < 0.5);
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("traa output"),
            color_attachments: &[color_attachment(&t.screen.view)],
            ..Default::default()
        });
        pass.set_pipeline(&t.output.0);
        pass.set_bind_group(0, &t.output.1, &[]);
        pass.set_bind_group(1, &t.output.2[source], &[]);
        pass.set_vertex_buffer(0, self.quad_uv.slice(..));
        pass.draw(0..3, 0..1);
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
    /// animated, TRAA, step count, light intensity, spot intensity,
    /// volumetric intensity and smoke amount.
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        // Switching TRAA rebuilds the output node.
        if index == 1 {
            self.built = false;
        }
        *self
            .params
            .get_mut(index)
            .ok_or(Error::Invalid("traa parameter"))? = value as f64;
        Ok(())
    }
    pub fn seek(&mut self, t: f64) {
        self.advance(t);
    }
}
fn color_attachment(view: &wgpu::TextureView) -> Option<wgpu::RenderPassColorAttachment<'_>> {
    Some(wgpu::RenderPassColorAttachment {
        view,
        depth_slice: None,
        resolve_target: None,
        ops: wgpu::Operations {
            load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
            store: wgpu::StoreOp::Store,
        },
    })
}
fn depth_attachment(
    view: &wgpu::TextureView,
) -> Option<wgpu::RenderPassDepthStencilAttachment<'_>> {
    Some(wgpu::RenderPassDepthStencilAttachment {
        view,
        depth_ops: Some(wgpu::Operations {
            load: wgpu::LoadOp::Clear(1.),
            store: wgpu::StoreOp::Store,
        }),
        stencil_ops: None,
    })
}
/// TRAANode's DepthTexture( 1, 1 ) placeholder, cleared to the far plane.
fn placeholder(r: &Renderer) -> wgpu::TextureView {
    let depth = view(&texture(r, (1, 1, 1), DEPTH, true));
    let mut encoder = r.device.create_command_encoder(&Default::default());
    encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: Some("traa placeholder"),
        depth_stencil_attachment: depth_attachment(&depth),
        ..Default::default()
    });
    r.queue.submit([encoder.finish()]);
    depth
}
