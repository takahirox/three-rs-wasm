//! webgpu_custom_fog: TerrainGenerator's eroded 512² terrain and
//! ForestGenerator's 500,000 instanced tree blobs, lit by a sun whose
//! colour, shadow and SkyMesh image-based light follow its elevation and
//! azimuth, inside the page's custom fog node: a triNoise3D band settling
//! into the valley plus a distance haze. The generators run once on the
//! CPU, as the page's do; the SkyMesh is captured by a cube camera into the
//! PMREM ( fromScene ) when the sun moves; the 4096² shadow renders once
//! and again only when the sun moves or the terrain is regenerated, as the
//! page's on-demand shadow does. FirstPersonControls walk the camera and
//! the forest's distance cull follows it. Every stage runs the WGSL three.js
//! r186 generates for the page (in `custom_fog/`; the PMREM blur, the
//! background vertex stage and the output are the retro, backdrop_water
//! and cubemap_dynamic modules, byte-identical).
mod generators;
use super::deferred::{Draw, culled_pipeline, set};
use super::lights_projector::{m3, m4, pack};
use super::pmrem_cube_uv::{GGX_8, Pmrem, bind};
use super::retro::uniform;
use super::trackball_sprites::FirstPerson;
use crate::{
    Error, Result, camera::*, geometry::*, math::*, render_target::*, renderer::*, scene::*,
};
use generators::{Forest, Terrain, TerrainParams};
use wgpu::util::DeviceExt;

const HALF: wgpu::TextureFormat = wgpu::TextureFormat::Rgba16Float;
const BYTE: wgpu::TextureFormat = wgpu::TextureFormat::Rgba8Unorm;
const DEPTH: wgpu::TextureFormat = wgpu::TextureFormat::Depth24Plus;
const SHADOW: u32 = 4096;
/// The cube-UV texel sizes at lodMax 8.
pub(super) const TEXEL: [f64; 2] = [1. / 768., 1. / 1024.];
macro_rules! wgsl {
    ($name:literal) => {
        include_str!(concat!("custom_fog/", $name, ".wgsl"))
    };
}
const BACKGROUND_VS: &str = include_str!("backdrop_water/background_vs.wgsl");
const OUTPUT_VS: &str = include_str!("cubemap_dynamic/output_vs.wgsl");
const OUTPUT_FS: &str = include_str!("cubemap_dynamic/output_fs.wgsl");
/// The cube camera's six view matrices, in fromScene's face order.
pub(super) const FACE_VIEWS: [[f64; 16]; 6] = [
    [
        0., 0., -1., 0., 0., 1., 0., 0., 1., 0., 0., 0., 0., 0., 0., 1.,
    ],
    [
        -1., 0., 0., 0., 0., 0., 1., 0., 0., 1., 0., 0., 0., 0., 0., 1.,
    ],
    [
        -1., 0., 0., 0., 0., 1., 0., 0., 0., 0., -1., 0., 0., 0., 0., 1.,
    ],
    [
        0., 0., 1., 0., 0., 1., 0., 0., -1., 0., 0., 0., 0., 0., 0., 1.,
    ],
    [
        -1., 0., 0., 0., 0., 0., -1., 0., 0., -1., 0., 0., 0., 0., 0., 1.,
    ],
    [
        1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1.,
    ],
];
/// The cube camera's projection ( fov 90, near 0.1, far 100 ).
pub(super) const FACE_PROJECTION: [f64; 16] = [
    1.,
    0.,
    0.,
    0.,
    0.,
    1.,
    0.,
    0.,
    0.,
    0.,
    -1.001001001001001,
    -1.,
    0.,
    0.,
    -0.1001001001001001,
    0.,
];
pub(super) struct Buffers {
    vertex: Vec<wgpu::Buffer>,
    index: wgpu::Buffer,
    count: u32,
}
impl Buffers {
    pub(super) fn draw(&self, pass: &mut wgpu::RenderPass, instances: u32) {
        for (i, b) in self.vertex.iter().enumerate() {
            pass.set_vertex_buffer(i as u32, b.slice(..));
        }
        pass.set_index_buffer(self.index.slice(..), wgpu::IndexFormat::Uint32);
        pass.draw_indexed(0..self.count, 0, 0..instances);
    }
}
pub(super) fn upload(r: &Renderer, vertex: &[&[u8]], index: &[u32]) -> Buffers {
    let init = |data: &[u8], usage| {
        r.device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("custom fog"),
                contents: data,
                usage,
            })
    };
    Buffers {
        vertex: vertex
            .iter()
            .map(|d| init(d, wgpu::BufferUsages::VERTEX))
            .collect(),
        index: init(bytemuck::cast_slice(index), wgpu::BufferUsages::INDEX),
        count: index.len() as u32,
    }
}
/// The baked terrain and forest on the GPU, with the forest's instance count.
struct Landscape {
    terrain: Buffers,
    forest: Buffers,
    trees: u32,
    min_y: f64,
    max_y: f64,
    sphere: Sphere,
}
fn landscape(r: &Renderer, params: TerrainParams) -> Landscape {
    let terrain = Terrain::build(params);
    let forest = Forest::build(&terrain);
    let half = params.size / 2.;
    let (lo, hi) = (terrain.min_y as f64, terrain.max_y as f64);
    Landscape {
        terrain: upload(
            r,
            &[
                bytemuck::cast_slice(&terrain.positions),
                bytemuck::cast_slice(&terrain.normals),
            ],
            &terrain.index,
        ),
        forest: upload(
            r,
            &[
                bytemuck::cast_slice(&forest.positions),
                bytemuck::cast_slice(&forest.normals),
                bytemuck::cast_slice(&forest.cull),
                bytemuck::cast_slice(&forest.region),
                bytemuck::cast_slice(&forest.ao),
                bytemuck::cast_slice(&forest.matrices),
            ],
            &forest.index,
        ),
        trees: forest.count as u32,
        min_y: lo,
        max_y: hi,
        sphere: Sphere {
            center: Vector3::new(0., (lo + hi) / 2., 0.),
            radius: (2. * half * half + ((hi - lo) / 2.).powi(2)).sqrt(),
        },
    }
}
struct Targets {
    width: u32,
    height: u32,
    format: wgpu::TextureFormat,
    samples: u32,
    color: wgpu::TextureView,
    resolve: Option<wgpu::TextureView>,
    depth: wgpu::TextureView,
    background: Draw,
    terrain: Draw,
    forest: Draw,
    output: Draw,
    screen: RenderTarget,
}
pub(super) struct Demo {
    walker: FirstPerson,
    pending: bool,
    time: f64,
    last: f64,
    /// elevation, azimuth, fog base, fog top, haze, cull from, cull to,
    /// erosion and valley bias.
    params: [f64; 9],
    seed: u32,
    land: Landscape,
    shadow_pending: bool,
    /// generate() with the next seed, on the next frame.
    regenerate: bool,
    pmrem_pending: Option<f64>,
    background_mesh: Buffers,
    sky_box: Buffers,
    pmrem: Pmrem,
    pmrem_depth: wgpu::TextureView,
    box_draw: Draw,
    sky_draws: Vec<Draw>,
    sky_renders: Vec<wgpu::Buffer>,
    sky_object: wgpu::Buffer,
    shadow_color: wgpu::TextureView,
    shadow_depth: wgpu::TextureView,
    terrain_shadow: Draw,
    forest_shadow: Draw,
    background_render: wgpu::Buffer,
    background_object: wgpu::Buffer,
    terrain_shadow_render: wgpu::Buffer,
    terrain_shadow_object: wgpu::Buffer,
    forest_shadow_render: wgpu::Buffer,
    forest_shadow_object: wgpu::Buffer,
    terrain_render: wgpu::Buffer,
    terrain_object: wgpu::Buffer,
    forest_render: wgpu::Buffer,
    forest_object: wgpu::Buffer,
    output_render: wgpu::Buffer,
    output_object: wgpu::Buffer,
    output_quad: wgpu::Buffer,
    linear: wgpu::Sampler,
    compare: wgpu::Sampler,
    targets: Option<Targets>,
}
pub(super) fn layouts<'a>(
    attributes: &'a [Vec<wgpu::VertexAttribute>],
    strides: &[(u64, bool)],
) -> Vec<wgpu::VertexBufferLayout<'a>> {
    attributes
        .iter()
        .zip(strides)
        .map(|(a, &(stride, instance))| wgpu::VertexBufferLayout {
            array_stride: stride,
            step_mode: if instance {
                wgpu::VertexStepMode::Instance
            } else {
                wgpu::VertexStepMode::Vertex
            },
            attributes: a,
        })
        .collect()
}
fn mesh_attributes() -> Vec<Vec<wgpu::VertexAttribute>> {
    vec![
        wgpu::vertex_attr_array![0 => Float32x3].to_vec(),
        wgpu::vertex_attr_array![1 => Float32x3].to_vec(),
    ]
}
fn forest_attributes() -> Vec<Vec<wgpu::VertexAttribute>> {
    vec![
        wgpu::vertex_attr_array![0 => Float32x3].to_vec(),
        wgpu::vertex_attr_array![1 => Float32x3].to_vec(),
        wgpu::vertex_attr_array![2 => Float32x4].to_vec(),
        wgpu::vertex_attr_array![3 => Float32].to_vec(),
        wgpu::vertex_attr_array![4 => Float32].to_vec(),
        wgpu::vertex_attr_array![5 => Float32x4, 6 => Float32x4, 7 => Float32x4, 8 => Float32x4]
            .to_vec(),
    ]
}
const FOREST_STRIDES: [(u64, bool); 6] = [
    (12, false),
    (12, false),
    (16, true),
    (4, true),
    (4, false),
    (64, true),
];
#[allow(clippy::too_many_arguments)]
pub(super) fn pipeline(
    r: &Renderer,
    label: &str,
    shaders: (&str, &str),
    buffers: &[wgpu::VertexBufferLayout],
    format: wgpu::TextureFormat,
    samples: u32,
    depth: (wgpu::CompareFunction, bool),
    cw: bool,
) -> wgpu::RenderPipeline {
    culled_pipeline(
        r,
        label,
        shaders,
        buffers,
        &[format],
        Some(depth),
        (cw, false),
        (samples, wgpu::PrimitiveTopology::TriangleList),
        Some(wgpu::Face::Back),
    )
}
pub(super) fn two_groups(
    r: &Renderer,
    p: wgpu::RenderPipeline,
    render: &wgpu::Buffer,
    object: &[(u32, wgpu::BindingResource)],
) -> Draw {
    let groups = vec![
        bind(
            r,
            p.get_bind_group_layout(0),
            &[(0, render.as_entire_binding())],
        ),
        bind(r, p.get_bind_group_layout(1), object),
    ];
    (p, groups)
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
        s.get_mut(c)?.position = Vector3::new(-50., 88., 230.);
        // controls.lookAt( 0, 5, -120 ): the latitude and longitude of the look.
        s.look_at(c, Vector3::new(0., 5., -120.))?;
        let mut walker = FirstPerson::default();
        walker.speed = 20.;
        let look = s.get(c)?.quaternion * -Vector3::Z;
        walker.lat = 90. - look.y.clamp(-1., 1.).acos().to_degrees();
        walker.lon = look.x.atan2(look.z).to_degrees();
        let land = landscape(r, TerrainParams::page());
        let mesh = |g: &BufferGeometry, names: &[&str]| -> Result<Buffers> {
            let read = |name: &str| -> Result<Vec<f32>> {
                let a = g
                    .attributes
                    .get(name)
                    .ok_or(Error::Invalid("custom fog attribute"))?;
                (0..a.count())
                    .flat_map(|i| (0..3).map(move |k| (i, k)))
                    .map(|(i, k)| a.get_component(i, k).map(|v| v as f32))
                    .collect()
            };
            let data: Vec<Vec<f32>> = names.iter().map(|n| read(n)).collect::<Result<_>>()?;
            let slices: Vec<&[u8]> = data.iter().map(|d| bytemuck::cast_slice(d)).collect();
            let index = g.index.clone().ok_or(Error::Invalid("custom fog index"))?;
            Ok(upload(r, &slices, &index))
        };
        let background_mesh = mesh(&SphereGeometry::build(1., 32, 32)?, &["normal", "position"])?;
        let sky_box = mesh(&BoxGeometry::build(1., 1., 1.)?, &["position"])?;
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
        let texture = |size: u32, format, usage| {
            r.device
                .create_texture(&wgpu::TextureDescriptor {
                    label: Some("custom fog"),
                    size: wgpu::Extent3d {
                        width: size,
                        height: size,
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
        let shadow_color = texture(SHADOW, BYTE, attach);
        let shadow_depth = texture(SHADOW, DEPTH, attach | wgpu::TextureUsages::TEXTURE_BINDING);
        // PMREMGenerator.fromScene( envScene, 0, 0.1, 100 ): the cube camera's
        // faces into level 0 of the cubeUV target, then the GGX levels.
        let pmrem = Pmrem::new(r, &linear, 8, GGX_8)?;
        let pmrem_depth = r
            .device
            .create_texture(&wgpu::TextureDescriptor {
                label: Some("PMREM depth"),
                size: wgpu::Extent3d {
                    width: 768,
                    height: 1024,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: DEPTH,
                usage: attach,
                view_formats: &[],
            })
            .create_view(&Default::default());
        let position_only = [wgpu::vertex_attr_array![0 => Float32x3].to_vec()];
        let box_layouts = layouts(&position_only, &[(12, false)]);
        let box_render = uniform(r, "PMREM box", wgsl!("box_vs"), "renderStruct")?;
        let box_object = uniform(r, "PMREM box", wgsl!("box_fs"), "objectStruct")?;
        let box_pipeline = pipeline(
            r,
            "PMREM box",
            (wgsl!("box_vs"), wgsl!("box_fs")),
            &box_layouts,
            HALF,
            1,
            (wgpu::CompareFunction::Always, false),
            true,
        );
        let box_draw = two_groups(
            r,
            box_pipeline,
            &box_render,
            &[(0, box_object.as_entire_binding())],
        );
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
        write(
            &box_render,
            wgsl!("box_vs"),
            "renderStruct",
            &[
                ("cameraProjectionMatrix", FACE_PROJECTION.to_vec()),
                ("cameraViewMatrix", m4(Matrix4::IDENTITY)),
            ],
        )?;
        write(
            &box_object,
            wgsl!("box_fs"),
            "objectStruct",
            &[
                ("nodeUniform0", vec![0.; 3]),
                ("nodeUniform1", vec![1.]),
                ("nodeUniform4", m4(Matrix4::IDENTITY)),
            ],
        )?;
        let sky_object = uniform(r, "sky", wgsl!("sky_fs"), "objectStruct")?;
        let sky_pipeline = pipeline(
            r,
            "sky",
            (wgsl!("sky_vs"), wgsl!("sky_fs")),
            &box_layouts,
            HALF,
            1,
            (wgpu::CompareFunction::LessEqual, false),
            true,
        );
        let mut sky_renders = vec![];
        let mut sky_draws = vec![];
        for _ in 0..6 {
            let render = uniform(r, "sky", wgsl!("sky_fs"), "renderStruct")?;
            sky_draws.push(two_groups(
                r,
                sky_pipeline.clone(),
                &render,
                &[(0, sky_object.as_entire_binding())],
            ));
            sky_renders.push(render);
        }
        let mesh_attrs = mesh_attributes();
        let mesh_layouts = layouts(&mesh_attrs, &[(12, false), (12, false)]);
        let forest_attrs = forest_attributes();
        let forest_layouts = layouts(&forest_attrs, &FOREST_STRIDES);
        let less = (wgpu::CompareFunction::LessEqual, true);
        let terrain_shadow_render = uniform(
            r,
            "terrain shadow",
            wgsl!("terrain_shadow_fs"),
            "renderStruct",
        )?;
        let terrain_shadow_object = uniform(
            r,
            "terrain shadow",
            wgsl!("terrain_shadow_fs"),
            "objectStruct",
        )?;
        let terrain_shadow = two_groups(
            r,
            pipeline(
                r,
                "terrain shadow",
                (wgsl!("terrain_shadow_vs"), wgsl!("terrain_shadow_fs")),
                &mesh_layouts,
                BYTE,
                1,
                less,
                true,
            ),
            &terrain_shadow_render,
            &[(0, terrain_shadow_object.as_entire_binding())],
        );
        let forest_shadow_render = uniform(
            r,
            "forest shadow",
            wgsl!("forest_shadow_vs"),
            "renderStruct",
        )?;
        let forest_shadow_object = uniform(
            r,
            "forest shadow",
            wgsl!("forest_shadow_vs"),
            "objectStruct",
        )?;
        let forest_shadow = two_groups(
            r,
            pipeline(
                r,
                "forest shadow",
                (wgsl!("forest_shadow_vs"), wgsl!("forest_shadow_fs")),
                &forest_layouts,
                BYTE,
                1,
                less,
                true,
            ),
            &forest_shadow_render,
            &[(0, forest_shadow_object.as_entire_binding())],
        );
        let demo = Self {
            walker,
            pending: true,
            time: 0.,
            last: 0.,
            params: [11., 150., -20., 55., 0.0012, 300., 620., 0.7, 1.2],
            seed: 1,
            land,
            shadow_pending: true,
            regenerate: false,
            pmrem_pending: Some(0.),
            background_mesh,
            sky_box,
            pmrem,
            pmrem_depth,
            box_draw,
            sky_draws,
            sky_renders,
            sky_object,
            shadow_color,
            shadow_depth,
            terrain_shadow,
            forest_shadow,
            background_render: uniform(r, "background", wgsl!("background_fs"), "renderStruct")?,
            background_object: uniform(r, "background", wgsl!("background_fs"), "objectStruct")?,
            terrain_shadow_render,
            terrain_shadow_object,
            forest_shadow_render,
            forest_shadow_object,
            terrain_render: uniform(r, "terrain", wgsl!("terrain_fs"), "renderStruct")?,
            terrain_object: uniform(r, "terrain", wgsl!("terrain_fs"), "objectStruct")?,
            forest_render: uniform(r, "forest", wgsl!("forest_fs"), "renderStruct")?,
            forest_object: uniform(r, "forest", wgsl!("forest_fs"), "objectStruct")?,
            output_render: uniform(r, "fog output", OUTPUT_FS, "renderStruct")?,
            output_object: uniform(r, "fog output", OUTPUT_VS, "objectStruct")?,
            output_quad: r
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("fog quad"),
                    contents: bytemuck::cast_slice(&[-1f32, 3., 0., -1., -1., 0., 3., -1., 0.]),
                    usage: wgpu::BufferUsages::VERTEX,
                }),
            linear,
            compare,
            targets: None,
        };
        Ok(demo)
    }
    /// The sun: its direction, the light's colour ( warmer and dimmer near
    /// the horizon ) and intensity.
    fn sun(&self) -> (Vector3, Vec<f64>) {
        let (elevation, azimuth) = (self.params[0], self.params[1]);
        let phi = (90. - elevation).to_radians();
        let theta = azimuth.to_radians();
        let sun = Vector3::new(phi.sin() * theta.sin(), phi.cos(), phi.sin() * theta.cos());
        let transmittance = elevation.to_radians().sin().max(0.).sqrt();
        let (a, b) = (Color::from_hex(0xff7a2f).0, Color::from_hex(0xfff2e0).0);
        let color = a + (b - a) * transmittance;
        let intensity = 11. * transmittance + 0.3;
        (sun, (color * intensity).to_array().to_vec())
    }
    fn resize(&mut self, r: &Renderer, out: &RenderTarget) -> Result<()> {
        let size = (out.width, out.height);
        let samples = if out.options.samples > 1 { 4 } else { 1 };
        let target = |format, samples| {
            r.device
                .create_texture(&wgpu::TextureDescriptor {
                    label: Some("custom fog target"),
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
        let mesh_attrs = mesh_attributes();
        let mesh_layouts = layouts(&mesh_attrs, &[(12, false), (12, false)]);
        let forest_attrs = forest_attributes();
        let forest_layouts = layouts(&forest_attrs, &FOREST_STRIDES);
        let less = (wgpu::CompareFunction::LessEqual, true);
        let background = two_groups(
            r,
            pipeline(
                r,
                "fog background",
                (BACKGROUND_VS, wgsl!("background_fs")),
                &mesh_layouts,
                HALF,
                samples,
                (wgpu::CompareFunction::Always, false),
                true,
            ),
            &self.background_render,
            &[(0, self.background_object.as_entire_binding())],
        );
        macro_rules! lit {
            ($object:expr) => {
                [
                    (0, $object.as_entire_binding()),
                    (1, sampler(&self.linear)),
                    (2, tex(&r.dfg)),
                    (3, sampler(&self.compare)),
                    (4, tex(&self.shadow_depth)),
                    (5, sampler(&self.linear)),
                    (6, tex(&self.pmrem.view)),
                ]
            };
        }
        let terrain = two_groups(
            r,
            pipeline(
                r,
                "terrain",
                (wgsl!("terrain_vs"), wgsl!("terrain_fs")),
                &mesh_layouts,
                HALF,
                samples,
                less,
                false,
            ),
            &self.terrain_render,
            &lit!(self.terrain_object),
        );
        let forest = two_groups(
            r,
            pipeline(
                r,
                "forest",
                (wgsl!("forest_vs"), wgsl!("forest_fs")),
                &forest_layouts,
                HALF,
                samples,
                less,
                false,
            ),
            &self.forest_render,
            &lit!(self.forest_object),
        );
        let quad = [wgpu::vertex_attr_array![0 => Float32x3].to_vec()];
        let output_pipeline = culled_pipeline(
            r,
            "fog output",
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
            background,
            terrain,
            forest,
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
            label: Some("fog output"),
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
    /// updateSun(): the sky's sun and the PMREM of the sky ( its clouds at
    /// the frame time ).
    fn bake(&self, r: &Renderer, encoder: &mut wgpu::CommandEncoder, time: f64) -> Result<()> {
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
        let (sun, _) = self.sun();
        write(
            &self.sky_object,
            wgsl!("sky_fs"),
            "objectStruct",
            &[
                (
                    "nodeUniform0",
                    m4(Matrix4::from_scale(Vector3::splat(10000.))),
                ),
                ("nodeUniform2", vec![0.88]),
                ("nodeUniform3", vec![0.]),
                ("nodeUniform4", vec![0.4]),
                ("nodeUniform5", vec![0.5]),
                ("nodeUniform6", vec![0.0002]),
                ("nodeUniform8", vec![0.00002]),
                ("nodeUniform9", vec![0.4]),
                ("nodeUniform10", vec![1.]),
                ("nodeUniform11", sun.to_array().to_vec()),
                ("nodeUniform12", vec![2.]),
                ("nodeUniform13", vec![12.]),
                ("nodeUniform14", vec![0.005]),
            ],
        )?;
        for (render, view) in self.sky_renders.iter().zip(FACE_VIEWS) {
            write(
                render,
                wgsl!("sky_fs"),
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
            encoder
                .begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("PMREM fromScene"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &self.pmrem.view,
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
        for (i, draw) in self.sky_draws.iter().enumerate() {
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
        if self.targets.as_ref().is_none_or(|t| {
            t.width != out.width
                || t.height != out.height
                || t.format != out.options.format
                || t.samples != samples
        }) {
            self.resize(r, out)?;
            self.pending = true;
        }
        let t = self
            .targets
            .as_ref()
            .ok_or(Error::Invalid("custom fog targets"))?;
        let mut encoder = r.device.create_command_encoder(&Default::default());
        if !std::mem::take(&mut self.pending) {
            self.present(&mut encoder, t);
            r.queue.submit([encoder.finish()]);
            return Ok(true);
        }
        if std::mem::take(&mut self.regenerate) {
            let mut params = TerrainParams::page();
            params.seed = self.seed;
            params.erosion = self.params[7];
            params.valley_bias = self.params[8];
            self.land = landscape(r, params);
        }
        let t = self
            .targets
            .as_ref()
            .ok_or(Error::Invalid("custom fog targets"))?;
        if let Some(time) = self.pmrem_pending.take() {
            self.bake(r, &mut encoder, time)?;
        }
        // animate(): the controls by the timer's delta, then the forest's
        // camera position.
        let delta = (self.time - self.last).max(0.);
        self.last = self.time;
        self.walker.update(s, c, delta)?;
        s.update()?;
        let (camera, world) = s.camera(c)?;
        let (projection, view) = (camera.projection_matrix()?, world.inverse());
        let position = world.w_axis.truncate();
        let (sun, light_color) = self.sun();
        let light = sun * 900.;
        // The shadow camera: OrthographicCamera( -420, 420, 420, -420, 200, 1800 ).
        let (near, far, extent) = (200., 1800., 420.);
        let shadow_projection = Matrix4::from_cols_array(&[
            1. / extent,
            0.,
            0.,
            0.,
            0.,
            1. / extent,
            0.,
            0.,
            0.,
            0.,
            -1. / (far - near),
            0.,
            0.,
            0.,
            -near / (far - near),
            1.,
        ]);
        let shadow_view = Matrix4::look_at_rh(light, Vector3::ZERO, Vector3::Y);
        let bias = Matrix4::from_cols_array(&[
            0.5, 0., 0., 0., 0., 0.5, 0., 0., 0., 0., 1., 0., 0.5, 0.5, 0., 1.,
        ]);
        let shadow_matrix = m4(bias * shadow_projection * shadow_view);
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
        let id3 = m3(Matrix4::IDENTITY);
        let id4 = m4(Matrix4::IDENTITY);
        let camera_position = position.to_array().to_vec();
        let [_, _, fog_base, fog_top, haze, from, to, _, _] = self.params;
        let (min_y, max_y) = (self.land.min_y, self.land.max_y);
        // The on-demand shadow: rendered when needsUpdate is set, with the
        // forest culled around the camera of that frame.
        if std::mem::take(&mut self.shadow_pending) {
            let camera_values = vec![
                ("cameraProjectionMatrix", m4(shadow_projection)),
                ("cameraViewMatrix", m4(shadow_view)),
                ("cameraPosition", light.to_array().to_vec()),
            ];
            write(
                &self.terrain_shadow_render,
                wgsl!("terrain_shadow_fs"),
                "renderStruct",
                &camera_values,
            )?;
            write(
                &self.terrain_shadow_object,
                wgsl!("terrain_shadow_fs"),
                "objectStruct",
                &[
                    ("nodeUniform0", id4.clone()),
                    ("nodeUniform1", vec![min_y]),
                    ("nodeUniform2", vec![max_y]),
                    ("nodeUniform4", id3.clone()),
                    ("nodeUniform7", vec![1.]),
                ],
            )?;
            write(
                &self.forest_shadow_render,
                wgsl!("forest_shadow_vs"),
                "renderStruct",
                &camera_values,
            )?;
            write(
                &self.forest_shadow_object,
                wgsl!("forest_shadow_vs"),
                "objectStruct",
                &[
                    ("nodeUniform4", camera_position.clone()),
                    ("nodeUniform5", vec![from]),
                    ("nodeUniform6", vec![to]),
                    ("nodeUniform7", id4.clone()),
                    ("nodeUniform8", camera_position.clone()),
                    ("nodeUniform9", vec![1.]),
                ],
            )?;
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("fog shadow"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &self.shadow_color,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.shadow_depth,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                ..Default::default()
            });
            set(&mut pass, &self.terrain_shadow);
            self.land.terrain.draw(&mut pass, 1);
            set(&mut pass, &self.forest_shadow);
            self.land.forest.draw(&mut pass, self.land.trees);
        }
        let camera_values = vec![
            ("cameraProjectionMatrix", m4(projection)),
            ("cameraViewMatrix", m4(view)),
        ];
        let mut bg = camera_values.clone();
        bg.push(("nodeUniform3", vec![1.]));
        write(
            &self.background_render,
            wgsl!("background_fs"),
            "renderStruct",
            &bg,
        )?;
        write(
            &self.background_object,
            wgsl!("background_fs"),
            "objectStruct",
            &[
                ("nodeUniform1", id3.clone()),
                ("nodeUniform4", vec![1.]),
                ("nodeUniform6", id4.clone()),
            ],
        )?;
        let light_values = |names: [&'static str; 9]| {
            vec![
                (names[0], light_color.clone()),
                (names[1], light.to_array().to_vec()),
                (names[2], vec![0.; 3]),
                (names[3], shadow_matrix.clone()),
                (names[4], vec![0.15]),
                (names[5], vec![-0.0004]),
                (names[6], vec![1.]),
                (names[7], vec![SHADOW as f64; 2]),
                (names[8], vec![1.]),
            ]
        };
        let mut terrain_render = camera_values.clone();
        terrain_render.extend(light_values([
            "nodeUniform14",
            "nodeUniform12",
            "nodeUniform13",
            "nodeUniform15",
            "nodeUniform16",
            "nodeUniform17",
            "nodeUniform19",
            "nodeUniform20",
            "nodeUniform21",
        ]));
        terrain_render.push(("cameraPosition", camera_position.clone()));
        terrain_render.push(("cameraWorldMatrix", m4(world)));
        write(
            &self.terrain_render,
            wgsl!("terrain_fs"),
            "renderStruct",
            &terrain_render,
        )?;
        let fog = |names: [&'static str; 5]| {
            vec![
                (names[0], vec![0.16]),
                (names[1], vec![fog_top]),
                (names[2], vec![self.time]),
                (names[3], vec![fog_base]),
                (names[4], vec![haze]),
            ]
        };
        let mut terrain_object = vec![
            ("nodeUniform0", id4.clone()),
            ("nodeUniform1", vec![min_y]),
            ("nodeUniform2", vec![max_y]),
            ("nodeUniform4", id3.clone()),
            ("nodeUniform7", vec![1.]),
            ("nodeUniform8", vec![0.]),
            ("nodeUniform9", vec![0.; 3]),
            ("nodeUniform10", vec![1.]),
            ("nodeUniform22", vec![8.]),
            ("nodeUniform23", id4.clone()),
            ("nodeUniform25", vec![TEXEL[0]]),
            ("nodeUniform26", vec![TEXEL[1]]),
        ];
        terrain_object.extend(fog([
            "nodeUniform28",
            "nodeUniform29",
            "nodeUniform30",
            "nodeUniform31",
            "nodeUniform32",
        ]));
        write(
            &self.terrain_object,
            wgsl!("terrain_fs"),
            "objectStruct",
            &terrain_object,
        )?;
        let mut forest_render = camera_values.clone();
        forest_render.extend(light_values([
            "nodeUniform20",
            "nodeUniform18",
            "nodeUniform19",
            "nodeUniform21",
            "nodeUniform22",
            "nodeUniform23",
            "nodeUniform25",
            "nodeUniform26",
            "nodeUniform27",
        ]));
        forest_render.push(("cameraWorldMatrix", m4(world)));
        write(
            &self.forest_render,
            wgsl!("forest_fs"),
            "renderStruct",
            &forest_render,
        )?;
        let mut forest_object = vec![
            ("nodeUniform4", camera_position.clone()),
            ("nodeUniform5", vec![from]),
            ("nodeUniform6", vec![to]),
            ("nodeUniform7", id4.clone()),
            ("nodeUniform8", camera_position.clone()),
            ("nodeUniform9", vec![1.]),
            ("nodeUniform10", vec![0.]),
            ("nodeUniform11", vec![0.88]),
            ("nodeUniform13", id3.clone()),
            ("nodeUniform14", vec![0.; 3]),
            ("nodeUniform15", vec![1.]),
            ("nodeUniform28", vec![8.]),
            ("nodeUniform29", id4.clone()),
            ("nodeUniform31", vec![TEXEL[0]]),
            ("nodeUniform32", vec![TEXEL[1]]),
        ];
        forest_object.extend(fog([
            "nodeUniform34",
            "nodeUniform35",
            "nodeUniform36",
            "nodeUniform37",
            "nodeUniform38",
        ]));
        write(
            &self.forest_object,
            wgsl!("forest_fs"),
            "objectStruct",
            &forest_object,
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
                ("cameraViewMatrix", id4.clone()),
                ("nodeUniform1", vec![t.width as f64, t.height as f64]),
                ("nodeUniform2", vec![0.62]),
            ],
        )?;
        write(
            &self.output_object,
            OUTPUT_VS,
            "objectStruct",
            &[("nodeUniform5", id4)],
        )?;
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("fog scene"),
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
            set(&mut pass, &t.background);
            self.background_mesh.draw(&mut pass, 1);
            if Frustum::from_projection(projection * view).intersects_sphere(self.land.sphere) {
                set(&mut pass, &t.terrain);
                self.land.terrain.draw(&mut pass, 1);
                set(&mut pass, &t.forest);
                self.land.forest.draw(&mut pass, self.land.trees);
            }
        }
        self.present(&mut encoder, t);
        r.queue.submit([encoder.finish()]);
        Ok(true)
    }
    pub fn output(&self) -> Option<&RenderTarget> {
        self.targets.as_ref().map(|t| &t.screen)
    }
    /// FirstPersonControls' pointer: down ( 10 + button ), move, up ( 20 + button ).
    pub fn draw(&mut self, kind: u32, x: f64, y: f64) {
        self.walker.pointer(kind, x, y);
        self.pending = true;
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
    /// elevation, azimuth ( updateSun ), fog base, top and haze, the cull
    /// range, erosion and valley bias, and regenerate ( a new seed ).
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        let value = value as f64;
        match index {
            0 | 1 => {
                self.params[index] = value;
                self.pmrem_pending = Some(self.time);
                self.shadow_pending = true;
            }
            2..=8 => self.params[index] = value,
            9 => {
                self.seed += 1;
                self.regenerate = true;
                self.shadow_pending = true;
            }
            _ => return Err(Error::Invalid("custom fog parameter")),
        }
        self.pending = true;
        Ok(())
    }
    pub fn seek(&mut self, t: f64) {
        self.time = t;
        self.pending = true;
    }
}
