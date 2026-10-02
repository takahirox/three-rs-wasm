//! webgpu_generator_building: one SkyscraperGenerator tower, its seed-picked
//! masonry colour and its single partId-shaded material ( brick coursing,
//! weathering and interior-mapped rooms behind the glass ), under a SkyMesh
//! sky and the SunLight addon's two shadow cascades, with an invisible
//! shadow-catching ground, ACES ( exposure 0.25 ) and auto-rotating
//! OrbitControls. The generator runs on the CPU when a building parameter
//! changes, as the page's does, and bakes one non-indexed geometry drawn in
//! one call per pass. updateSun() places the sun for the time of day and
//! bakes the sky ( without its disc ) into the PMREM ( fromScene ). Each
//! frame the cascades are fitted to the view and the tower renders into
//! both, then the scene renders with 4× MSAA. Every stage runs the WGSL
//! three.js r186 generates for the page ( in `generator_building/`; the sky,
//! the PMREM capture and blur and the output are the custom_fog, retro and
//! cubemap_dynamic modules, byte-identical ).
mod skyscraper;
use super::controls_attributes::{Controls, camera_state};
use super::custom_fog::{
    Buffers, FACE_PROJECTION, FACE_VIEWS, TEXEL, layouts, pipeline, two_groups, upload,
};
use super::deferred::{Draw, culled_pipeline, set};
use super::fog_volume::{Cascade, cascades};
use super::lights_projector::{m3, m4, pack};
use super::pmrem_cube_uv::{GGX_8, Pmrem};
use super::retro::uniform;
use crate::{
    Error, Result, camera::*, geometry::*, math::*, render_target::*, renderer::*, scene::*,
};
use skyscraper::{Settings, build, ground, pick_building_color};
use std::f64::consts::PI;
use wgpu::util::DeviceExt;

const HALF: wgpu::TextureFormat = wgpu::TextureFormat::Rgba16Float;
const BYTE: wgpu::TextureFormat = wgpu::TextureFormat::Rgba8Unorm;
const DEPTH: wgpu::TextureFormat = wgpu::TextureFormat::Depth24Plus;
/// The SunLight shadow atlas: two 2048² cascades side by side.
const MAP: u32 = 2048;
macro_rules! wgsl {
    ($name:literal) => {
        include_str!(concat!("generator_building/", $name, ".wgsl"))
    };
}
const BOX_VS: &str = include_str!("custom_fog/box_vs.wgsl");
const BOX_FS: &str = include_str!("custom_fog/box_fs.wgsl");
const SKY_VS: &str = include_str!("custom_fog/sky_vs.wgsl");
const SKY_FS: &str = include_str!("custom_fog/sky_fs.wgsl");
const OUTPUT_VS: &str = include_str!("cubemap_dynamic/output_vs.wgsl");
const OUTPUT_FS: &str = include_str!("cubemap_dynamic/output_fs.wgsl");
/// The baked tower's vertex buffers ( partId, roomCenter, roomSize,
/// position, normal, uv ) and its bounding sphere.
struct Tower {
    buffers: Vec<wgpu::Buffer>,
    count: u32,
    sphere: Sphere,
}
impl Tower {
    fn new(r: &Renderer, settings: &Settings) -> Self {
        let b = build(settings);
        let init = |label, data: &[f32]| {
            r.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some(label),
                    contents: bytemuck::cast_slice(data),
                    usage: wgpu::BufferUsages::VERTEX,
                })
        };
        Self {
            buffers: vec![
                init("skyscraper partId", &b.part_id),
                init("skyscraper roomCenter", &b.room_center),
                init("skyscraper roomSize", &b.room_size),
                init("skyscraper position", &b.position),
                init("skyscraper normal", &b.normal),
                init("skyscraper uv", &b.uv),
            ],
            count: b.part_id.len() as u32,
            sphere: Sphere {
                center: Vector3::from_array(b.center),
                radius: b.radius,
            },
        }
    }
    fn draw(&self, pass: &mut wgpu::RenderPass) {
        for (i, b) in self.buffers.iter().enumerate() {
            pass.set_vertex_buffer(i as u32, b.slice(..));
        }
        pass.draw(0..self.count, 0..1);
    }
}
fn tower_attributes() -> Vec<Vec<wgpu::VertexAttribute>> {
    vec![
        wgpu::vertex_attr_array![0 => Float32].to_vec(),
        wgpu::vertex_attr_array![1 => Float32x3].to_vec(),
        wgpu::vertex_attr_array![2 => Float32x2].to_vec(),
        wgpu::vertex_attr_array![3 => Float32x3].to_vec(),
        wgpu::vertex_attr_array![4 => Float32x3].to_vec(),
        wgpu::vertex_attr_array![5 => Float32x2].to_vec(),
    ]
}
const TOWER_STRIDES: [(u64, bool); 6] = [
    (4, false),
    (12, false),
    (8, false),
    (12, false),
    (12, false),
    (8, false),
];
struct Targets {
    width: u32,
    height: u32,
    format: wgpu::TextureFormat,
    samples: u32,
    color: wgpu::TextureView,
    resolve: Option<wgpu::TextureView>,
    depth: wgpu::TextureView,
    tower: Draw,
    sky: Draw,
    ground: Draw,
    output: Draw,
    screen: RenderTarget,
}
pub(super) struct Demo {
    controls: Controls,
    pending: bool,
    time: f64,
    /// seed, height, width, depth, floorHeight, bayWidth, chamfer, setback
    /// and timeOfDay.
    params: [f64; 9],
    rebuild: bool,
    pmrem_pending: Option<f64>,
    tower: Tower,
    ground: (wgpu::Buffer, wgpu::Buffer),
    sky_box: Buffers,
    pmrem: Pmrem,
    pmrem_depth: wgpu::TextureView,
    box_draw: Draw,
    bake_draws: Vec<Draw>,
    bake_renders: Vec<wgpu::Buffer>,
    bake_object: wgpu::Buffer,
    shadow_color: wgpu::TextureView,
    shadow_depth: wgpu::TextureView,
    shadow_draws: Vec<Draw>,
    shadow_renders: Vec<wgpu::Buffer>,
    shadow_object: wgpu::Buffer,
    tower_render: wgpu::Buffer,
    tower_object: wgpu::Buffer,
    sky_render: wgpu::Buffer,
    sky_object: wgpu::Buffer,
    ground_render: wgpu::Buffer,
    ground_object: wgpu::Buffer,
    output_render: wgpu::Buffer,
    output_object: wgpu::Buffer,
    output_quad: wgpu::Buffer,
    linear: wgpu::Sampler,
    compare: wgpu::Sampler,
    targets: Option<Targets>,
}
fn write(
    r: &Renderer,
    buffer: &wgpu::Buffer,
    source: &str,
    name: &str,
    values: &[(&str, Vec<f64>)],
) -> Result<()> {
    let values: Vec<(&str, &[f64])> = values.iter().map(|(n, v)| (*n, &v[..])).collect();
    r.queue
        .write_buffer(buffer, 0, &pack(source, name, &values)?);
    Ok(())
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, _id: u32, r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 45.,
            near: 1.,
            far: 20000.,
            aspect,
            ..Default::default()
        }));
        s.get_mut(c)?.position = Vector3::new(120., 95., 155.);
        let mut controls = Controls::new(Some(0.05), (30., 600.), PI * 0.495, true);
        controls.set_target(Vector3::new(0., 58., 0.));
        controls.auto_rotate = Some(0.3);
        // init()'s controls.update() already turns by one auto-rotation step.
        controls.frame_update(s, c)?;
        let params = [7., 100., 34., 28., 4., 2.6, 5., 1.5, 17.];
        let tower = Tower::new(r, &settings(&params));
        let g = ground(2000.);
        let vertex = |label, data: &[f32]| {
            r.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some(label),
                    contents: bytemuck::cast_slice(data),
                    usage: wgpu::BufferUsages::VERTEX,
                })
        };
        let ground_buffers = (
            vertex("ground position", &g.position),
            vertex("ground normal", &g.normal),
        );
        let unit = BoxGeometry::build(1., 1., 1.)?;
        let positions: Vec<f32> = {
            let a = unit
                .attributes
                .get("position")
                .ok_or(Error::Invalid("sky box position"))?;
            (0..a.count())
                .flat_map(|i| (0..3).map(move |k| (i, k)))
                .map(|(i, k)| a.get_component(i, k).map(|v| v as f32))
                .collect::<Result<_>>()?
        };
        let index = unit.index.clone().ok_or(Error::Invalid("sky box index"))?;
        let sky_box = upload(r, &[bytemuck::cast_slice(&positions)], &index);
        let sampler = |compare| {
            r.device.create_sampler(&wgpu::SamplerDescriptor {
                mag_filter: wgpu::FilterMode::Linear,
                min_filter: wgpu::FilterMode::Linear,
                compare,
                ..Default::default()
            })
        };
        let linear = sampler(None);
        let compare = sampler(Some(wgpu::CompareFunction::LessEqual));
        let texture = |size: (u32, u32), format, usage| {
            r.device
                .create_texture(&wgpu::TextureDescriptor {
                    label: Some("generator building"),
                    size: wgpu::Extent3d {
                        width: size.0,
                        height: size.1,
                        depth_or_array_layers: 1,
                    },
                    mip_level_count: 1,
                    sample_count: 1,
                    dimension: wgpu::TextureDimension::D2,
                    format,
                    usage,
                    view_formats: &[],
                })
                .create_view(&Default::default())
        };
        let attach = wgpu::TextureUsages::RENDER_ATTACHMENT;
        let atlas = (MAP * 2, MAP);
        let shadow_color = texture(atlas, BYTE, attach);
        let shadow_depth = texture(atlas, DEPTH, attach | wgpu::TextureUsages::TEXTURE_BINDING);
        // PMREMGenerator.fromScene( envScene ): the cube camera's faces into
        // level 0 of the cubeUV target, then the GGX levels.
        let pmrem = Pmrem::new(r, &linear, 8, GGX_8)?;
        let pmrem_depth = texture((768, 1024), DEPTH, attach);
        let position_only = [wgpu::vertex_attr_array![0 => Float32x3].to_vec()];
        let box_layouts = layouts(&position_only, &[(12, false)]);
        let box_render = uniform(r, "PMREM box", BOX_VS, "renderStruct")?;
        let box_object = uniform(r, "PMREM box", BOX_FS, "objectStruct")?;
        let box_draw = two_groups(
            r,
            pipeline(
                r,
                "PMREM box",
                (BOX_VS, BOX_FS),
                &box_layouts,
                HALF,
                1,
                (wgpu::CompareFunction::Always, false),
                true,
            ),
            &box_render,
            &[(0, box_object.as_entire_binding())],
        );
        write(
            r,
            &box_render,
            BOX_VS,
            "renderStruct",
            &[
                ("cameraProjectionMatrix", FACE_PROJECTION.to_vec()),
                ("cameraViewMatrix", m4(Matrix4::IDENTITY)),
            ],
        )?;
        write(
            r,
            &box_object,
            BOX_FS,
            "objectStruct",
            &[
                ("nodeUniform0", vec![0.; 3]),
                ("nodeUniform1", vec![1.]),
                ("nodeUniform4", m4(Matrix4::IDENTITY)),
            ],
        )?;
        let bake_object = uniform(r, "sky bake", SKY_FS, "objectStruct")?;
        let bake_pipeline = pipeline(
            r,
            "sky bake",
            (SKY_VS, SKY_FS),
            &box_layouts,
            HALF,
            1,
            (wgpu::CompareFunction::LessEqual, false),
            true,
        );
        let mut bake_renders = vec![];
        let mut bake_draws = vec![];
        for _ in 0..6 {
            let render = uniform(r, "sky bake", SKY_FS, "renderStruct")?;
            bake_draws.push(two_groups(
                r,
                bake_pipeline.clone(),
                &render,
                &[(0, bake_object.as_entire_binding())],
            ));
            bake_renders.push(render);
        }
        let attributes = tower_attributes();
        let tower_layouts = layouts(&attributes, &TOWER_STRIDES);
        let shadow_object = uniform(r, "tower shadow", wgsl!("shadow_fs"), "objectStruct")?;
        let shadow_pipeline = pipeline(
            r,
            "tower shadow",
            (wgsl!("shadow_vs"), wgsl!("shadow_fs")),
            &tower_layouts,
            BYTE,
            1,
            (wgpu::CompareFunction::LessEqual, true),
            true,
        );
        let mut shadow_renders = vec![];
        let mut shadow_draws = vec![];
        for _ in 0..2 {
            let render = uniform(r, "tower shadow", wgsl!("shadow_fs"), "renderStruct")?;
            shadow_draws.push(two_groups(
                r,
                shadow_pipeline.clone(),
                &render,
                &[(0, shadow_object.as_entire_binding())],
            ));
            shadow_renders.push(render);
        }
        Ok(Self {
            controls,
            pending: true,
            time: 0.,
            params,
            rebuild: false,
            pmrem_pending: Some(0.),
            tower,
            ground: ground_buffers,
            sky_box,
            pmrem,
            pmrem_depth,
            box_draw,
            bake_draws,
            bake_renders,
            bake_object,
            shadow_color,
            shadow_depth,
            shadow_draws,
            shadow_renders,
            shadow_object,
            tower_render: uniform(r, "tower", wgsl!("building_fs"), "renderStruct")?,
            tower_object: uniform(r, "tower", wgsl!("building_fs"), "objectStruct")?,
            sky_render: uniform(r, "sky", SKY_FS, "renderStruct")?,
            sky_object: uniform(r, "sky", SKY_FS, "objectStruct")?,
            ground_render: uniform(r, "ground", wgsl!("ground_fs"), "renderStruct")?,
            ground_object: uniform(r, "ground", wgsl!("ground_fs"), "objectStruct")?,
            output_render: uniform(r, "building output", OUTPUT_FS, "renderStruct")?,
            output_object: uniform(r, "building output", OUTPUT_VS, "objectStruct")?,
            output_quad: vertex("building quad", &[-1., 3., 0., -1., -1., 0., 3., -1., 0.]),
            linear,
            compare,
            targets: None,
        })
    }
    /// updateSun(): the sun's direction for the time of day, and the key
    /// light's colour × intensity ( warmer and dimmer near the horizon ).
    fn sun(&self) -> (Vector3, Vec<f64>) {
        let t = self.params[8];
        let u = (t - 12.) / 6.;
        let elevation = (1. - u * u).max(0.) * 72.;
        let azimuth = 90. - u * 55.;
        let (phi, theta) = ((90. - elevation).to_radians(), azimuth.to_radians());
        let sun = Vector3::new(phi.sin() * theta.sin(), phi.cos(), phi.sin() * theta.cos());
        let transmittance = elevation.to_radians().sin().max(0.).sqrt();
        let (a, b) = (Color::from_hex(0xffb072).0, Color::from_hex(0xfff4e8).0);
        let color = a + (b - a) * transmittance;
        (sun, (color * (6. * transmittance)).to_array().to_vec())
    }
    /// The SkyMesh uniforms ( turbidity 8, rayleigh 3, mieCoefficient 0.008,
    /// mieDirectionalG 0.88 ) with or without the sun disc.
    fn sky_values(&self, disc: bool) -> Vec<(&'static str, Vec<f64>)> {
        let (sun, _) = self.sun();
        vec![
            (
                "nodeUniform0",
                m4(Matrix4::from_scale(Vector3::splat(10000.))),
            ),
            ("nodeUniform2", vec![0.88]),
            ("nodeUniform3", vec![if disc { 1. } else { 0. }]),
            ("nodeUniform4", vec![0.4]),
            ("nodeUniform5", vec![0.5]),
            ("nodeUniform6", vec![0.0002]),
            ("nodeUniform8", vec![0.00002]),
            ("nodeUniform9", vec![0.4]),
            ("nodeUniform10", vec![1.]),
            ("nodeUniform11", sun.to_array().to_vec()),
            ("nodeUniform12", vec![3.]),
            ("nodeUniform13", vec![8.]),
            ("nodeUniform14", vec![0.008]),
        ]
    }
    fn resize(&mut self, r: &Renderer, out: &RenderTarget) -> Result<()> {
        let size = (out.width, out.height);
        let samples = if out.options.samples > 1 { 4 } else { 1 };
        let target = |format, samples| {
            r.device
                .create_texture(&wgpu::TextureDescriptor {
                    label: Some("generator building target"),
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
                        | wgpu::TextureUsages::TEXTURE_BINDING,
                    view_formats: &[],
                })
                .create_view(&Default::default())
        };
        let color = target(HALF, samples);
        let resolve = (samples > 1).then(|| target(HALF, 1));
        let depth = target(DEPTH, samples);
        let tex = wgpu::BindingResource::TextureView;
        let sampler = wgpu::BindingResource::Sampler;
        let attributes = tower_attributes();
        let tower_layouts = layouts(&attributes, &TOWER_STRIDES);
        let less = (wgpu::CompareFunction::LessEqual, true);
        let tower = two_groups(
            r,
            pipeline(
                r,
                "tower",
                (wgsl!("building_vs"), wgsl!("building_fs")),
                &tower_layouts,
                HALF,
                samples,
                less,
                false,
            ),
            &self.tower_render,
            &[
                (0, self.tower_object.as_entire_binding()),
                (1, sampler(&self.linear)),
                (2, tex(&r.dfg)),
                (3, sampler(&self.compare)),
                (4, tex(&self.shadow_depth)),
                (5, sampler(&self.linear)),
                (6, tex(&self.pmrem.view)),
            ],
        );
        let position_only = [wgpu::vertex_attr_array![0 => Float32x3].to_vec()];
        let box_layouts = layouts(&position_only, &[(12, false)]);
        let sky = two_groups(
            r,
            pipeline(
                r,
                "sky",
                (SKY_VS, SKY_FS),
                &box_layouts,
                HALF,
                samples,
                (wgpu::CompareFunction::LessEqual, false),
                true,
            ),
            &self.sky_render,
            &[(0, self.sky_object.as_entire_binding())],
        );
        let ground_attributes = [
            wgpu::vertex_attr_array![0 => Float32x3].to_vec(),
            wgpu::vertex_attr_array![1 => Float32x3].to_vec(),
        ];
        let ground_pipeline = culled_pipeline(
            r,
            "ground",
            (wgsl!("ground_vs"), wgsl!("ground_fs")),
            &layouts(&ground_attributes, &[(12, false), (12, false)]),
            &[HALF],
            Some(less),
            (false, true),
            (samples, wgpu::PrimitiveTopology::TriangleList),
            Some(wgpu::Face::Back),
        );
        let ground = two_groups(
            r,
            ground_pipeline,
            &self.ground_render,
            &[
                (0, self.ground_object.as_entire_binding()),
                (1, sampler(&self.compare)),
                (2, tex(&self.shadow_depth)),
            ],
        );
        let quad = [wgpu::vertex_attr_array![0 => Float32x3].to_vec()];
        let output_pipeline = culled_pipeline(
            r,
            "building output",
            (OUTPUT_VS, OUTPUT_FS),
            &layouts(&quad, &[(12, false)]),
            &[out.options.format],
            None,
            (false, false),
            (1, wgpu::PrimitiveTopology::TriangleList),
            Some(wgpu::Face::Back),
        );
        let output = two_groups(
            r,
            output_pipeline,
            &self.output_render,
            &[
                (0, sampler(&self.linear)),
                (1, tex(resolve.as_ref().unwrap_or(&color))),
                (2, self.output_object.as_entire_binding()),
            ],
        );
        self.targets = Some(Targets {
            width: size.0,
            height: size.1,
            format: out.options.format,
            samples,
            color,
            resolve,
            depth,
            tower,
            sky,
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
    fn present(&self, encoder: &mut wgpu::CommandEncoder, t: &Targets) {
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("building output"),
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
        set(&mut pass, &t.output);
        pass.set_vertex_buffer(0, self.output_quad.slice(..));
        pass.draw(0..3, 0..1);
    }
    /// updateSun()'s bake: the sky without its disc into the PMREM.
    fn bake(&self, r: &Renderer, encoder: &mut wgpu::CommandEncoder, time: f64) -> Result<()> {
        write(
            r,
            &self.bake_object,
            SKY_FS,
            "objectStruct",
            &self.sky_values(false),
        )?;
        for (render, view) in self.bake_renders.iter().zip(FACE_VIEWS) {
            write(
                r,
                render,
                SKY_FS,
                "renderStruct",
                &[
                    ("nodeUniform7", vec![time]),
                    ("cameraProjectionMatrix", FACE_PROJECTION.to_vec()),
                    ("cameraViewMatrix", view.to_vec()),
                    ("cameraPosition", vec![0.; 3]),
                ],
            )?;
        }
        let pass = |encoder: &mut wgpu::CommandEncoder, clear: bool| {
            let load = |c| {
                if clear {
                    wgpu::LoadOp::Clear(c)
                } else {
                    wgpu::LoadOp::Load
                }
            };
            encoder
                .begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("PMREM fromScene"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &self.pmrem.view,
                        depth_slice: None,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: load(wgpu::Color::TRANSPARENT),
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                    depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                        view: &self.pmrem_depth,
                        depth_ops: Some(wgpu::Operations {
                            load: if clear {
                                wgpu::LoadOp::Clear(1.)
                            } else {
                                wgpu::LoadOp::Load
                            },
                            store: wgpu::StoreOp::Store,
                        }),
                        stencil_ops: None,
                    }),
                    ..Default::default()
                })
                .forget_lifetime()
        };
        drop(pass(encoder, true));
        {
            let mut p = pass(encoder, false);
            p.set_viewport(0., 0., 768., 1024., 0., 1.);
            set(&mut p, &self.box_draw);
            self.sky_box.draw(&mut p, 1);
        }
        for (i, draw) in self.bake_draws.iter().enumerate() {
            let mut p = pass(encoder, false);
            p.set_viewport(
                (i % 3) as f32 * 256.,
                (i / 3) as f32 * 256.,
                256.,
                256.,
                0.,
                1.,
            );
            set(&mut p, draw);
            self.sky_box.draw(&mut p, 1);
        }
        self.pmrem.encode_levels(encoder);
        Ok(())
    }
    pub fn render(
        &mut self,
        r: &Renderer,
        s: &mut Scene,
        c: Object3D,
        out: &RenderTarget,
    ) -> Result<bool> {
        let samples = if out.options.samples > 1 { 4 } else { 1 };
        // A resize redraws the frame without advancing the animation.
        let mut resized = false;
        if self.targets.as_ref().is_none_or(|t| {
            t.width != out.width
                || t.height != out.height
                || t.format != out.options.format
                || t.samples != samples
        }) {
            self.resize(r, out)?;
            resized = true;
        }
        let mut encoder = r.device.create_command_encoder(&Default::default());
        let advance = std::mem::take(&mut self.pending);
        if !advance && !resized {
            let t = self
                .targets
                .as_ref()
                .ok_or(Error::Invalid("building targets"))?;
            self.present(&mut encoder, t);
            r.queue.submit([encoder.finish()]);
            return Ok(true);
        }
        // generate(): the tower rebuilt from the parameters.
        if std::mem::take(&mut self.rebuild) {
            self.tower = Tower::new(r, &settings(&self.params));
        }
        if let Some(time) = self.pmrem_pending.take() {
            self.bake(r, &mut encoder, time)?;
        }
        let t = self
            .targets
            .as_ref()
            .ok_or(Error::Invalid("building targets"))?;
        // animate(): controls.update(), then render.
        if advance {
            self.controls.frame_update(s, c)?;
        }
        s.update()?;
        let (camera, world) = s.camera(c)?;
        let (projection, view) = (camera.projection_matrix()?, world.inverse());
        let position = world.w_axis.truncate();
        let (sun, light_color) = self.sun();
        let fitted: [Cascade; 2] = cascades(sun, world, projection, 1., 20000., 1000.);
        let id3 = m3(Matrix4::IDENTITY);
        let id4 = m4(Matrix4::IDENTITY);
        let base = Color::from_hex(pick_building_color(self.params[0])).0;
        let base = base.to_array().to_vec();
        write(
            r,
            &self.shadow_object,
            wgsl!("shadow_fs"),
            "objectStruct",
            &[
                ("nodeUniform0", id4.clone()),
                ("nodeUniform2", id4.clone()),
                ("nodeUniform3", base.clone()),
                ("nodeUniform5", id3.clone()),
                ("nodeUniform6", vec![1.]),
            ],
        )?;
        for (render, cascade) in self.shadow_renders.iter().zip(&fitted) {
            write(
                r,
                render,
                wgsl!("shadow_fs"),
                "renderStruct",
                &[
                    ("cameraProjectionMatrix", m4(cascade.projection)),
                    ("cameraViewMatrix", m4(cascade.view)),
                    (
                        "cameraPosition",
                        cascade.view.inverse().w_axis.truncate().to_array().to_vec(),
                    ),
                ],
            )?;
        }
        let atlas = vec![(MAP * 2) as f64, MAP as f64];
        let mut tower_render = vec![
            ("cameraProjectionMatrix", m4(projection)),
            ("cameraViewMatrix", m4(view)),
            ("cameraPosition", position.to_array().to_vec()),
            ("nodeUniform10", light_color),
            ("nodeUniform9", sun.to_array().to_vec()),
            ("nodeUniform13", m4(fitted[1].matrix)),
            ("nodeUniform12", fitted[1].data.to_vec()),
            ("nodeUniform19", m4(fitted[0].matrix)),
            ("nodeUniform18", fitted[0].data.to_vec()),
            ("nodeUniform11", vec![0.]),
            ("nodeUniform14", vec![-0.0004]),
            ("nodeUniform16", vec![1.]),
            ("nodeUniform17", atlas.clone()),
            ("nodeUniform20", vec![-0.0004]),
            ("nodeUniform21", vec![1.]),
            ("nodeUniform22", atlas.clone()),
            ("nodeUniform23", vec![1.]),
        ];
        tower_render.push(("cameraWorldMatrix", m4(world)));
        write(
            r,
            &self.tower_render,
            wgsl!("building_fs"),
            "renderStruct",
            &tower_render,
        )?;
        write(
            r,
            &self.tower_object,
            wgsl!("building_fs"),
            "objectStruct",
            &[
                ("nodeUniform0", id4.clone()),
                ("nodeUniform2", id4.clone()),
                ("nodeUniform3", base),
                ("nodeUniform5", id3.clone()),
                ("nodeUniform6", vec![1.]),
                ("nodeUniform24", vec![8.]),
                ("nodeUniform25", id4.clone()),
                ("nodeUniform27", vec![TEXEL[0]]),
                ("nodeUniform28", vec![TEXEL[1]]),
                ("nodeUniform30", vec![0.25]),
            ],
        )?;
        write(
            r,
            &self.sky_render,
            SKY_FS,
            "renderStruct",
            &[
                ("nodeUniform7", vec![self.time]),
                ("cameraProjectionMatrix", m4(projection)),
                ("cameraViewMatrix", m4(view)),
                ("cameraPosition", position.to_array().to_vec()),
            ],
        )?;
        write(
            r,
            &self.sky_object,
            SKY_FS,
            "objectStruct",
            &self.sky_values(true),
        )?;
        write(
            r,
            &self.ground_render,
            wgsl!("ground_fs"),
            "renderStruct",
            &[
                ("cameraProjectionMatrix", m4(projection)),
                ("cameraViewMatrix", m4(view)),
                ("nodeUniform8", m4(fitted[1].matrix)),
                ("nodeUniform7", fitted[1].data.to_vec()),
                ("nodeUniform14", m4(fitted[0].matrix)),
                ("nodeUniform13", fitted[0].data.to_vec()),
                ("nodeUniform6", vec![0.]),
                ("nodeUniform9", vec![-0.0004]),
                ("nodeUniform11", vec![1.]),
                ("nodeUniform12", atlas.clone()),
                ("nodeUniform15", vec![-0.0004]),
                ("nodeUniform16", vec![1.]),
                ("nodeUniform17", atlas),
                ("nodeUniform18", vec![1.]),
            ],
        )?;
        write(
            r,
            &self.ground_object,
            wgsl!("ground_fs"),
            "objectStruct",
            &[
                ("nodeUniform0", vec![0.; 3]),
                ("nodeUniform1", vec![0.35]),
                ("nodeUniform2", id4.clone()),
                ("nodeUniform4", id3),
            ],
        )?;
        write(
            r,
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
                ("cameraViewMatrix", id4.clone()),
                ("nodeUniform1", vec![t.width as f64, t.height as f64]),
                ("nodeUniform2", vec![0.25]),
            ],
        )?;
        write(
            r,
            &self.output_object,
            OUTPUT_VS,
            "objectStruct",
            &[("nodeUniform5", id4)],
        )?;
        // The SunLight shadow: the atlas cleared, then each cascade's tile.
        let shadow_pass = |encoder: &mut wgpu::CommandEncoder, clear: bool| {
            encoder
                .begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("sun shadow"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &self.shadow_color,
                        depth_slice: None,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: if clear {
                                wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT)
                            } else {
                                wgpu::LoadOp::Load
                            },
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                    depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                        view: &self.shadow_depth,
                        depth_ops: Some(wgpu::Operations {
                            load: if clear {
                                wgpu::LoadOp::Clear(1.)
                            } else {
                                wgpu::LoadOp::Load
                            },
                            store: wgpu::StoreOp::Store,
                        }),
                        stencil_ops: None,
                    }),
                    ..Default::default()
                })
                .forget_lifetime()
        };
        drop(shadow_pass(&mut encoder, true));
        for (draw, cascade) in self.shadow_draws.iter().zip(&fitted) {
            let mut pass = shadow_pass(&mut encoder, false);
            let [x, y, w, h] = cascade.viewport;
            pass.set_viewport(x, y, w, h, 0., 1.);
            if Frustum::from_projection(cascade.projection * cascade.view)
                .intersects_sphere(self.tower.sphere)
            {
                set(&mut pass, draw);
                self.tower.draw(&mut pass);
            }
        }
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("building scene"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &t.color,
                    depth_slice: None,
                    resolve_target: t.resolve.as_ref(),
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
            if Frustum::from_projection(projection * view).intersects_sphere(self.tower.sphere) {
                set(&mut pass, &t.tower);
                self.tower.draw(&mut pass);
            }
            set(&mut pass, &t.sky);
            self.sky_box.draw(&mut pass, 1);
            set(&mut pass, &t.ground);
            pass.set_vertex_buffer(0, self.ground.0.slice(..));
            pass.set_vertex_buffer(1, self.ground.1.slice(..));
            pass.draw(0..6, 0..1);
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
            // The wheel handler updates at once: the auto-rotation turns a
            // step with the dolly.
            self.controls.dolly(wheel, &camera, Vector2::ZERO);
            self.controls.frame_update(s, c)?;
        } else if pan {
            self.controls.pan(&camera, dx, dy, height);
        } else {
            self.controls.rotate(dx, dy, height);
        }
        Ok(())
    }
    /// The building parameters ( generate() ) and the time of day
    /// ( updateSun() ).
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        *self
            .params
            .get_mut(index)
            .ok_or(Error::Invalid("building parameter"))? = value as f64;
        if index == 8 {
            self.pmrem_pending = Some(self.time);
        } else {
            self.rebuild = true;
        }
        Ok(())
    }
    pub fn seek(&mut self, t: f64) {
        self.time = t;
        self.pending = true;
    }
}
fn settings(p: &[f64; 9]) -> Settings {
    Settings {
        seed: p[0],
        height: p[1],
        width: p[2],
        depth: p[3],
        floor_height: p[4],
        bay_width: p[5],
        chamfer: p[6],
        setback: p[7],
    }
}
