//! webgpu_volume_caustics: the turning glass duck of webgpu_caustics on a
//! black floor, its caustic shadow from a half-float spot shadow, and a
//! volumetric pass. The scene renders with 4× MSAA: the duck's caustic
//! shadow (front and back faces), the floor, then the transmissive duck's
//! back and front faces, each over a fresh mipmapped copy of the frame. The
//! half-resolution volume pass ray-marches the 1.5 × 0.5 × 1.5 box through
//! the 128³ ImprovedNoise smoke, the spot light and its shadow, dithered by
//! bayer16 offset by the frame id, against the scene depth. BloomNode blurs
//! it ( a high pass and five H / V Gaussian levels, then the composite ) and
//! the output adds 0.7 of the bloom to the scene. Every stage runs the WGSL
//! three.js r186 generates for the page (in `volume_caustics/`).
use super::controls_attributes::{Controls, camera_state};
use super::gltf_viewer::{decode_texture_image, fetch, load_asset};
use super::lights_projector::{m3, m4, pack};
use super::pmrem_cube_uv::bind;
use super::shadowmap_opacity::{Mipmaps, mip_count};
use super::volume_lighting::{bayer_texture, noise_texture};
use crate::{
    Error, Result, camera::*, geometry::*, math::*, render_target::*, renderer::*, scene::*,
};
use std::f64::consts::PI;
use wgpu::util::DeviceExt;

const HALF: wgpu::TextureFormat = wgpu::TextureFormat::Rgba16Float;
const DEPTH: wgpu::TextureFormat = wgpu::TextureFormat::Depth24Plus;
const SHADOW_SIZE: u32 = 1024;
/// BloomNode's mip levels.
const LEVELS: usize = 5;
/// nodeFrame.frameId before the first requested frame. The original's
/// renderer loop advances it on every display frame; the fixture stops that
/// loop and restarts the count when the page is ready, and each requested
/// frame advances it ( the first renders frame 1 ).
const FRAME_ID_START: u32 = 0;
macro_rules! wgsl {
    ($name:literal) => {
        include_str!(concat!("volume_caustics/", $name, ".wgsl"))
    };
}
fn sized(r: &Renderer, label: &str, source: &str, name: &str) -> Result<wgpu::Buffer> {
    Ok(r.device.create_buffer(&wgpu::BufferDescriptor {
        label: Some(label),
        size: pack(source, name, &[])?.len() as u64,
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    }))
}
fn texture(
    r: &Renderer,
    size: (u32, u32),
    format: wgpu::TextureFormat,
    samples: u32,
    mips: u32,
    usage: wgpu::TextureUsages,
) -> wgpu::Texture {
    r.device.create_texture(&wgpu::TextureDescriptor {
        label: Some("volume caustics texture"),
        size: wgpu::Extent3d {
            width: size.0,
            height: size.1,
            depth_or_array_layers: 1,
        },
        mip_level_count: mips,
        sample_count: samples,
        dimension: wgpu::TextureDimension::D2,
        format,
        usage,
        view_formats: &[],
    })
}
fn view(t: &wgpu::Texture) -> wgpu::TextureView {
    t.create_view(&Default::default())
}
/// Vertex buffers of a mesh ( position and normal ), its index and bounding sphere.
struct Mesh {
    positions: wgpu::Buffer,
    normals: wgpu::Buffer,
    index: wgpu::Buffer,
    count: u32,
    center: Vector3,
    radius: f64,
}
type Draw = (wgpu::RenderPipeline, [wgpu::BindGroup; 2]);
/// The bloom chain's targets and draws for one size.
struct Bloom {
    bright: wgpu::TextureView,
    /// Per level: the horizontal and vertical targets.
    levels: Vec<(wgpu::TextureView, wgpu::TextureView, (u32, u32))>,
    high: (wgpu::RenderPipeline, wgpu::BindGroup),
    /// Per level: the horizontal and vertical draws with their uniforms.
    blurs: Vec<[(wgpu::RenderPipeline, wgpu::BindGroup, wgpu::Buffer); 2]>,
    composite: (wgpu::RenderPipeline, wgpu::BindGroup),
}
struct Targets {
    width: u32,
    height: u32,
    format: wgpu::TextureFormat,
    samples: u32,
    color: wgpu::TextureView,
    resolve: Option<wgpu::Texture>,
    resolve_view: Option<wgpu::TextureView>,
    single: Option<wgpu::Texture>,
    depth: wgpu::TextureView,
    transmission: wgpu::Texture,
    transmission_levels: Vec<(wgpu::TextureView, wgpu::BindGroup)>,
    volume_target: wgpu::TextureView,
    volume: Draw,
    bloom: Bloom,
    screen: RenderTarget,
    floor: Draw,
    /// Back and front faces.
    duck: [Draw; 2],
    output: (wgpu::RenderPipeline, wgpu::BindGroup),
}
pub(super) struct Demo {
    controls: Controls,
    time: f64,
    last: f64,
    rotation: f64,
    /// Requested frames: nodeFrame.frameId.
    frame_id: u32,
    /// caustic occlusion and material color.
    params: [f64; 2],
    duck: Mesh,
    floor: Mesh,
    volume_box: Mesh,
    caustic_map: wgpu::TextureView,
    bayer: wgpu::TextureView,
    noise: wgpu::TextureView,
    shadow_color: wgpu::TextureView,
    shadow_depth: wgpu::TextureView,
    /// Front and back faces.
    shadow: [Draw; 2],
    shadow_render: wgpu::Buffer,
    shadow_object: wgpu::Buffer,
    floor_render: wgpu::Buffer,
    floor_object: wgpu::Buffer,
    duck_render: wgpu::Buffer,
    duck_object: wgpu::Buffer,
    volume_render: wgpu::Buffer,
    volume_object: wgpu::Buffer,
    high_object: wgpu::Buffer,
    composite_object: wgpu::Buffer,
    tints: wgpu::Buffer,
    output_object: wgpu::Buffer,
    quad_uv: wgpu::Buffer,
    repeat: wgpu::Sampler,
    linear: wgpu::Sampler,
    transmission_sampler: wgpu::Sampler,
    noise_sampler: wgpu::Sampler,
    compare: wgpu::Sampler,
    mipmaps: Mipmaps,
    targets: Option<Targets>,
}
/// A triangle pipeline: `depth` None for color-only passes.
#[allow(clippy::too_many_arguments)]
fn pipeline(
    r: &Renderer,
    label: &str,
    (vs, fs): (&str, &str),
    buffers: &[wgpu::VertexBufferLayout],
    format: wgpu::TextureFormat,
    samples: u32,
    depth: Option<(wgpu::CompareFunction, bool)>,
    (cw, blended): (bool, bool),
) -> wgpu::RenderPipeline {
    super::deferred::sampled_pipeline(
        r,
        label,
        (vs, fs),
        buffers,
        &[format],
        depth,
        (cw, blended),
        (samples, wgpu::PrimitiveTopology::TriangleList),
    )
}
const P0: [wgpu::VertexAttribute; 1] = wgpu::vertex_attr_array![0 => Float32x3];
const P1: [wgpu::VertexAttribute; 1] = wgpu::vertex_attr_array![1 => Float32x3];
const UV0: [wgpu::VertexAttribute; 1] = wgpu::vertex_attr_array![0 => Float32x2];
fn layouts<'a>(
    a: &'a [wgpu::VertexAttribute; 1],
    b: &'a [wgpu::VertexAttribute; 1],
) -> [wgpu::VertexBufferLayout<'a>; 2] {
    [a, b].map(|attributes| wgpu::VertexBufferLayout {
        array_stride: 12,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes,
    })
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, _id: u32, r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 25.,
            near: 0.025,
            far: 5.,
            aspect,
            ..Default::default()
        }));
        s.get_mut(c)?.position = Vector3::new(-0.7, 0.2, 0.2);
        let mut controls = Controls::new(None, (0., 1.), PI, true);
        controls.set_target(Vector3::new(0., 0.02, -0.05));
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
        let mesh = |positions: &[f32], normals: &[f32], index: &[u32]| {
            let points: Vec<Vector3> = positions
                .chunks(3)
                .map(|p| Vector3::new(p[0] as f64, p[1] as f64, p[2] as f64))
                .collect();
            let (lo, hi) = points.iter().fold(
                (Vector3::splat(f64::MAX), Vector3::splat(f64::MIN)),
                |(lo, hi), p| (lo.min(*p), hi.max(*p)),
            );
            let center = (lo + hi) / 2.;
            Mesh {
                positions: init(
                    "volume caustics positions",
                    bytemuck::cast_slice(positions),
                    vertex,
                ),
                normals: init(
                    "volume caustics normals",
                    bytemuck::cast_slice(normals),
                    vertex,
                ),
                index: init(
                    "volume caustics index",
                    bytemuck::cast_slice(index),
                    wgpu::BufferUsages::INDEX,
                ),
                count: index.len() as u32,
                center,
                radius: points
                    .iter()
                    .map(|p| (*p - center).length())
                    .fold(0., f64::max),
            }
        };
        let (asset, buffers, _) = load_asset("/web/gallery/assets/gltf/duck.glb").await?;
        let primitive = asset
            .meshes()
            .next()
            .and_then(|m| m.primitives().next())
            .ok_or(Error::Invalid("duck mesh"))?;
        let reader = primitive.reader(|b| buffers.get(b.index()).map(Vec::as_slice));
        let positions: Vec<f32> = reader
            .read_positions()
            .ok_or(Error::Invalid("duck positions"))?
            .flatten()
            .collect();
        let normals: Vec<f32> = reader
            .read_normals()
            .ok_or(Error::Invalid("duck normals"))?
            .flatten()
            .collect();
        let index: Vec<u32> = reader
            .read_indices()
            .ok_or(Error::Invalid("duck index"))?
            .into_u32()
            .collect();
        let duck = mesh(&positions, &normals, &index);
        let geometry_mesh = |g: BufferGeometry| -> Result<Mesh> {
            let f = |name: &str| -> Result<Vec<f32>> {
                let a = g
                    .attributes
                    .get(name)
                    .ok_or(Error::Invalid("volume caustics attribute"))?;
                (0..a.count())
                    .flat_map(|i| (0..3).map(move |k| (i, k)))
                    .map(|(i, k)| a.get_component(i, k).map(|v| v as f32))
                    .collect()
            };
            let index = g
                .index
                .clone()
                .ok_or(Error::Invalid("volume caustics index"))?;
            Ok(mesh(&f("position")?, &f("normal")?, &index))
        };
        let floor = geometry_mesh(PlaneGeometry::build(2., 2., 1, 1)?)?;
        let volume_box = geometry_mesh(BoxGeometry::build(1.5, 0.5, 1.5)?)?;
        // Caustic_Free.jpg: sRGB, flipped on upload, mipmapped on the GPU.
        let mut mipmaps = Mipmaps::new(r);
        let image =
            decode_texture_image(&fetch("/web/gallery/assets/opengameart/Caustic_Free.jpg").await?)
                .await?;
        let size = (image.width, image.height);
        let format = wgpu::TextureFormat::Rgba8UnormSrgb;
        let t = texture(
            r,
            size,
            format,
            1,
            mip_count(size),
            wgpu::TextureUsages::TEXTURE_BINDING
                | wgpu::TextureUsages::COPY_DST
                | wgpu::TextureUsages::RENDER_ATTACHMENT,
        );
        let row = image.width as usize * 4;
        let flipped: Vec<u8> = image.rgba.chunks(row).rev().flatten().copied().collect();
        r.queue.write_texture(
            t.as_image_copy(),
            &flipped,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(row as u32),
                rows_per_image: Some(image.height),
            },
            t.size(),
        );
        let mut encoder = r.device.create_command_encoder(&Default::default());
        let levels = mipmaps.levels(r, &t);
        mipmaps.encode(&mut encoder, format, &levels);
        r.queue.submit([encoder.finish()]);
        let caustic_map = view(&t);
        let sampler = |address, mag, mipmap| {
            r.device.create_sampler(&wgpu::SamplerDescriptor {
                address_mode_u: address,
                address_mode_v: address,
                mag_filter: mag,
                min_filter: wgpu::FilterMode::Linear,
                mipmap_filter: mipmap,
                ..Default::default()
            })
        };
        use wgpu::{AddressMode::*, FilterMode::*};
        let repeat = sampler(Repeat, Linear, Linear);
        let linear = sampler(ClampToEdge, Linear, Nearest);
        let transmission_sampler = sampler(ClampToEdge, Nearest, Linear);
        let noise_sampler = sampler(Repeat, Linear, Nearest);
        let compare = r.device.create_sampler(&wgpu::SamplerDescriptor {
            mag_filter: Linear,
            min_filter: Linear,
            compare: Some(wgpu::CompareFunction::LessEqual),
            ..Default::default()
        });
        let attachment =
            wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING;
        let shadow_size = (SHADOW_SIZE, SHADOW_SIZE);
        let shadow_render = sized(r, "shadow render", wgsl!("shadow_vs"), "renderStruct")?;
        let shadow_object = sized(r, "shadow object", wgsl!("shadow_fs"), "objectStruct")?;
        let duck_layouts = layouts(&P0, &P1);
        let shadow =
            [(wgsl!("shadow_fs"), false), (wgsl!("shadow_back_fs"), true)].map(|(fs, cw)| {
                let p = pipeline(
                    r,
                    "volume caustics shadow",
                    (wgsl!("shadow_vs"), fs),
                    &duck_layouts,
                    HALF,
                    1,
                    Some((wgpu::CompareFunction::LessEqual, true)),
                    (cw, false),
                );
                let groups = [
                    bind(
                        r,
                        p.get_bind_group_layout(0),
                        &[(0, shadow_render.as_entire_binding())],
                    ),
                    bind(
                        r,
                        p.get_bind_group_layout(1),
                        &[
                            (0, wgpu::BindingResource::Sampler(&repeat)),
                            (1, wgpu::BindingResource::TextureView(&caustic_map)),
                            (2, shadow_object.as_entire_binding()),
                        ],
                    ),
                ];
                (p, groups)
            });
        let tints: Vec<f32> = (0..LEVELS).flat_map(|_| [1f32, 1., 1., 0.]).collect();
        Ok(Self {
            controls,
            time: 0.,
            last: 0.,
            rotation: 0.,
            frame_id: FRAME_ID_START,
            params: [1., 0xffd700 as f64],
            duck,
            floor,
            volume_box,
            caustic_map,
            bayer: bayer_texture(r)?,
            noise: noise_texture(r),
            shadow_color: view(&texture(r, shadow_size, HALF, 1, 1, attachment)),
            shadow_depth: view(&texture(r, shadow_size, DEPTH, 1, 1, attachment)),
            shadow,
            shadow_render,
            shadow_object,
            floor_render: sized(r, "floor render", wgsl!("floor_fs"), "renderStruct")?,
            floor_object: sized(r, "floor object", wgsl!("floor_fs"), "objectStruct")?,
            duck_render: sized(r, "duck render", wgsl!("duck_fs"), "renderStruct")?,
            duck_object: sized(r, "duck object", wgsl!("duck_fs"), "objectStruct")?,
            volume_render: sized(r, "volume render", wgsl!("volume_fs"), "renderStruct")?,
            volume_object: sized(r, "volume object", wgsl!("volume_fs"), "objectStruct")?,
            high_object: sized(r, "bloom high pass", wgsl!("high_fs"), "objectStruct")?,
            composite_object: sized(r, "bloom composite", wgsl!("composite_fs"), "objectStruct")?,
            tints: init(
                "bloom tints",
                bytemuck::cast_slice(&tints),
                wgpu::BufferUsages::UNIFORM,
            ),
            output_object: sized(
                r,
                "volume caustics output",
                wgsl!("output_fs"),
                "objectStruct",
            )?,
            quad_uv: init(
                "quad uv",
                bytemuck::cast_slice(&[0f32, -1., 0., 1., 2., 1.]),
                vertex,
            ),
            repeat,
            linear,
            transmission_sampler,
            noise_sampler,
            compare,
            mipmaps,
            targets: None,
        })
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, dt: f64, animate: bool) -> Result<()> {
        if animate {
            self.time += dt;
            self.frame_id += 1;
        }
        Ok(())
    }
    /// animate(): the duck turns 0.01 rad per frame ( 60 fps steps ), then
    /// controls.update().
    pub fn prepare(&mut self, s: &mut Scene, c: Object3D, _r: &Renderer) -> Result<()> {
        self.rotation -= 0.01 * (self.time - self.last) * 60.;
        self.last = self.time;
        self.controls.update(s, c)
    }
    fn resize(&mut self, r: &Renderer, out: &RenderTarget) -> Result<()> {
        let size = (out.width, out.height);
        let samples = if out.options.samples > 1 { 4 } else { 1 };
        let attachment = wgpu::TextureUsages::RENDER_ATTACHMENT;
        let sampled =
            attachment | wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_SRC;
        let (color, resolve, single) = if samples > 1 {
            (
                view(&texture(r, size, HALF, samples, 1, attachment)),
                Some(texture(r, size, HALF, 1, 1, sampled)),
                None,
            )
        } else {
            let t = texture(r, size, HALF, 1, 1, sampled);
            (view(&t), None, Some(t))
        };
        // The scene depth: the volume reads it ( multisampled with MSAA ).
        let depth = view(&texture(
            r,
            size,
            DEPTH,
            samples,
            1,
            attachment | wgpu::TextureUsages::TEXTURE_BINDING,
        ));
        let transmission = texture(
            r,
            size,
            HALF,
            1,
            mip_count(size),
            wgpu::TextureUsages::TEXTURE_BINDING
                | wgpu::TextureUsages::COPY_DST
                | wgpu::TextureUsages::RENDER_ATTACHMENT,
        );
        let transmission_levels = self.mipmaps.levels(r, &transmission);
        let transmission_view = view(&transmission);
        let sampler = wgpu::BindingResource::Sampler;
        let tex = wgpu::BindingResource::TextureView;
        let less = Some((wgpu::CompareFunction::LessEqual, true));
        let mesh_layouts = layouts(&P0, &P1);
        let floor = pipeline(
            r,
            "volume caustics floor",
            (wgsl!("floor_vs"), wgsl!("floor_fs")),
            &mesh_layouts,
            HALF,
            samples,
            less,
            (false, false),
        );
        let floor_groups = [
            bind(
                r,
                floor.get_bind_group_layout(0),
                &[(0, self.floor_render.as_entire_binding())],
            ),
            bind(
                r,
                floor.get_bind_group_layout(1),
                &[
                    (0, self.floor_object.as_entire_binding()),
                    (1, sampler(&self.linear)),
                    (2, tex(&r.dfg)),
                    (3, sampler(&self.linear)),
                    (4, tex(&self.shadow_color)),
                    (5, sampler(&self.compare)),
                    (6, tex(&self.shadow_depth)),
                ],
            ),
        ];
        let duck = [(wgsl!("duck_back_fs"), true), (wgsl!("duck_fs"), false)].map(|(fs, cw)| {
            let p = pipeline(
                r,
                "volume caustics duck",
                (wgsl!("duck_vs"), fs),
                &mesh_layouts,
                HALF,
                samples,
                less,
                (cw, true),
            );
            let groups = [
                bind(
                    r,
                    p.get_bind_group_layout(0),
                    &[(0, self.duck_render.as_entire_binding())],
                ),
                bind(
                    r,
                    p.get_bind_group_layout(1),
                    &[
                        (0, self.duck_object.as_entire_binding()),
                        (1, sampler(&self.repeat)),
                        (2, tex(&self.caustic_map)),
                        (3, sampler(&self.linear)),
                        (4, tex(&r.dfg)),
                        (5, sampler(&self.transmission_sampler)),
                        (6, tex(&transmission_view)),
                    ],
                ),
            ];
            (p, groups)
        });
        // The volumetric pass at half resolution, its bloom chain and the output.
        let half = (
            ((size.0 as f64 / 2.).round() as u32).max(1),
            ((size.1 as f64 / 2.).round() as u32).max(1),
        );
        let target = |size: (u32, u32)| {
            view(&texture(
                r,
                size,
                HALF,
                1,
                1,
                wgpu::TextureUsages::RENDER_ATTACHMENT
                    | wgpu::TextureUsages::TEXTURE_BINDING
                    | wgpu::TextureUsages::COPY_SRC,
            ))
        };
        let volume_target = target(half);
        let volume_fs = if samples > 1 {
            wgsl!("volume_fs")
        } else {
            wgsl!("volume_single_fs")
        };
        let volume = pipeline(
            r,
            "volume caustics volume",
            (wgsl!("volume_vs"), volume_fs),
            &layouts(&P0, &P1),
            HALF,
            1,
            None,
            (true, true),
        );
        let volume_groups = [
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
                    // bayer16 is read with textureLoad: its sampler is unused.
                    (2, tex(&self.bayer)),
                    (3, tex(&depth)),
                    (4, sampler(&self.linear)),
                    (5, tex(&self.shadow_color)),
                    (6, sampler(&self.compare)),
                    (7, tex(&self.shadow_depth)),
                    (8, sampler(&self.noise_sampler)),
                    (9, tex(&self.noise)),
                ],
            ),
        ];
        let uv = [wgpu::VertexBufferLayout {
            array_stride: 8,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &UV0,
        }];
        let quad = |label, vs, fs, format| {
            pipeline(r, label, (vs, fs), &uv, format, 1, None, (false, false))
        };
        let bright = target(half);
        let high = quad("bloom high pass", wgsl!("high_vs"), wgsl!("high_fs"), HALF);
        let high_group = bind(
            r,
            high.get_bind_group_layout(0),
            &[
                (0, sampler(&self.linear)),
                (1, tex(&volume_target)),
                (2, self.high_object.as_entire_binding()),
            ],
        );
        // BloomNode.setSize: each level halves the last, rounded.
        let mut levels = vec![];
        let mut level_size = half;
        for _ in 0..LEVELS {
            levels.push((target(level_size), target(level_size), level_size));
            level_size = (
                ((level_size.0 as f64 / 2.).round() as u32).max(1),
                ((level_size.1 as f64 / 2.).round() as u32).max(1),
            );
        }
        let blur_shaders = [
            wgsl!("blur0_fs"),
            wgsl!("blur1_fs"),
            wgsl!("blur2_fs"),
            wgsl!("blur3_fs"),
            wgsl!("blur4_fs"),
        ];
        let mut blurs = vec![];
        for (i, fs) in blur_shaders.iter().enumerate() {
            let p = quad("bloom blur", wgsl!("blur_vs"), fs, HALF);
            let input = if i == 0 { &bright } else { &levels[i - 1].1 };
            let pass = |input: &wgpu::TextureView| -> Result<_> {
                let object = sized(r, "bloom blur", fs, "objectStruct")?;
                let group = bind(
                    r,
                    p.get_bind_group_layout(0),
                    &[
                        (0, wgpu::BindingResource::Sampler(&self.linear)),
                        (1, wgpu::BindingResource::TextureView(input)),
                        (2, object.as_entire_binding()),
                    ],
                );
                Ok((p.clone(), group, object))
            };
            blurs.push([pass(input)?, pass(&levels[i].0)?]);
        }
        let composite = quad(
            "bloom composite",
            wgsl!("composite_vs"),
            wgsl!("composite_fs"),
            HALF,
        );
        let mut entries = vec![
            (0, self.composite_object.as_entire_binding()),
            (1, self.tints.as_entire_binding()),
        ];
        for (i, (_, vertical, _)) in levels.iter().enumerate() {
            entries.push((2 + i as u32 * 2, sampler(&self.linear)));
            entries.push((3 + i as u32 * 2, tex(vertical)));
        }
        let composite_group = bind(r, composite.get_bind_group_layout(0), &entries);
        let scene_view = match (&resolve, &single) {
            (Some(t), _) | (None, Some(t)) => view(t),
            _ => return Err(Error::Invalid("volume caustics target")),
        };
        let output = quad(
            "volume caustics output",
            wgsl!("composite_vs"),
            wgsl!("output_fs"),
            out.options.format,
        );
        let output_group = bind(
            r,
            output.get_bind_group_layout(0),
            &[
                (0, sampler(&self.linear)),
                (1, tex(&scene_view)),
                (2, sampler(&self.linear)),
                (3, tex(&levels[0].0)),
                (4, self.output_object.as_entire_binding()),
            ],
        );
        self.targets = Some(Targets {
            width: size.0,
            height: size.1,
            format: out.options.format,
            samples,
            color,
            resolve_view: resolve.as_ref().map(view),
            resolve,
            single,
            depth,
            transmission,
            transmission_levels,
            volume_target,
            volume: (volume, volume_groups),
            bloom: Bloom {
                bright,
                levels,
                high: (high, high_group),
                blurs,
                composite: (composite, composite_group),
            },
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
            floor: (floor, floor_groups),
            duck,
            output: (output, output_group),
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
        s.update()?;
        let (camera, world) = s.camera(c)?;
        let (projection, view) = (camera.projection_matrix()?, world.inverse());
        let camera_position = world.transform_point3(Vector3::ZERO);
        // The spot light at ( 0.2, 0.3, 0.2 ) aimed at the origin; its shadow
        // camera spans 2 × angle, near 0.1, far 1.
        let spot = Vector3::new(0.2, 0.3, 0.2);
        let (near, far) = (0.1, 1.);
        let top = near * (PI / 6.).tan();
        let shadow_projection = Matrix4::from_cols_array(&[
            near / top,
            0.,
            0.,
            0.,
            0.,
            near / top,
            0.,
            0.,
            0.,
            0.,
            -far / (far - near),
            -1.,
            0.,
            0.,
            -far * near / (far - near),
            0.,
        ]);
        let shadow_view = Matrix4::look_at_rh(spot, Vector3::ZERO, Vector3::Y);
        let bias = Matrix4::from_cols_array(&[
            0.5, 0., 0., 0., 0., 0.5, 0., 0., 0., 0., 1., 0., 0.5, 0.5, 0., 1.,
        ]);
        let t = self
            .targets
            .as_ref()
            .ok_or(Error::Invalid("volume caustics targets"))?;
        let spot_view = view.transform_point3(spot).to_array().to_vec();
        let color = Color::from_hex(self.params[1] as u32).0.to_array().to_vec();
        let occlusion = self.params[0];
        let duck_model =
            Matrix4::from_scale(Vector3::splat(0.5)) * Matrix4::from_rotation_y(self.rotation);
        let duck_normal = m3(duck_model.inverse().transpose());
        let floor_model = Matrix4::from_rotation_x(-PI / 2.);
        let volume_model = Matrix4::from_translation(Vector3::new(0., 0.25, 0.));
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
        // SpotLightNode: cone and penumbra cosines, distance, decay, color,
        // view position, world position and target.
        let spot_values = |names: [&'static str; 8]| {
            vec![
                (names[0], vec![(PI / 6.).cos()]),
                (names[1], vec![1.]),
                (names[2], vec![0.]),
                (names[3], vec![2.]),
                (names[4], vec![1.; 3]),
                (names[5], spot_view.clone()),
                (names[6], spot.to_array().to_vec()),
                (names[7], vec![0.; 3]),
            ]
        };
        let shadow_matrix = m4(bias * shadow_projection * shadow_view);
        let mut floor_render = camera_values.clone();
        floor_render.extend(spot_values([
            "nodeUniform21",
            "nodeUniform22",
            "nodeUniform25",
            "nodeUniform26",
            "nodeUniform11",
            "nodeUniform10",
            "nodeUniform23",
            "nodeUniform24",
        ]));
        floor_render.extend([
            ("nodeUniform13", shadow_matrix.clone()),
            ("nodeUniform15", vec![0.]),
            ("nodeUniform16", vec![0.]),
            ("nodeUniform18", vec![1.]),
            ("nodeUniform19", vec![SHADOW_SIZE as f64; 2]),
            ("nodeUniform20", vec![0.95]),
        ]);
        write(
            &self.floor_render,
            wgsl!("floor_fs"),
            "renderStruct",
            &floor_render,
        )?;
        write(
            &self.floor_object,
            wgsl!("floor_fs"),
            "objectStruct",
            &[
                ("nodeUniform0", vec![0.; 3]),
                ("nodeUniform1", vec![1.]),
                ("nodeUniform2", vec![0.]),
                ("nodeUniform3", vec![1.]),
                ("nodeUniform5", m3(floor_model.inverse().transpose())),
                ("nodeUniform6", vec![0.; 3]),
                ("nodeUniform7", vec![1.]),
                ("nodeUniform9", m4(floor_model)),
            ],
        )?;
        let mut duck_render = camera_values.clone();
        duck_render.extend(spot_values([
            "nodeUniform26",
            "nodeUniform27",
            "nodeUniform30",
            "nodeUniform31",
            "nodeUniform25",
            "nodeUniform17",
            "nodeUniform28",
            "nodeUniform29",
        ]));
        duck_render.push(("cameraPosition", camera_position.to_array().to_vec()));
        duck_render.push(("nodeUniform23", vec![t.width as f64, t.height as f64]));
        write(
            &self.duck_render,
            wgsl!("duck_fs"),
            "renderStruct",
            &duck_render,
        )?;
        write(
            &self.duck_object,
            wgsl!("duck_fs"),
            "objectStruct",
            &[
                ("nodeUniform0", color.clone()),
                ("nodeUniform1", vec![1.]),
                ("nodeUniform2", vec![0.]),
                ("nodeUniform3", vec![0.1]),
                ("nodeUniform5", duck_normal.clone()),
                ("nodeUniform6", vec![1.5]),
                ("nodeUniform7", vec![1.; 3]),
                ("nodeUniform8", vec![1.]),
                ("nodeUniform9", vec![1.]),
                ("nodeUniform10", vec![0.25]),
                ("nodeUniform11", vec![f64::INFINITY]),
                ("nodeUniform12", vec![1.; 3]),
                ("nodeUniform14", m4(duck_model)),
                ("nodeUniform15", vec![occlusion]),
                ("nodeUniform16", color.clone()),
                ("nodeUniform21", m4(duck_model)),
            ],
        )?;
        write(
            &self.shadow_render,
            wgsl!("shadow_vs"),
            "renderStruct",
            &[
                ("cameraProjectionMatrix", m4(shadow_projection)),
                ("cameraViewMatrix", m4(shadow_view)),
            ],
        )?;
        write(
            &self.shadow_object,
            wgsl!("shadow_fs"),
            "objectStruct",
            &[
                ("nodeUniform2", m4(duck_model)),
                ("nodeUniform3", duck_normal),
                ("nodeUniform4", vec![occlusion]),
                ("nodeUniform5", color),
                ("nodeUniform6", vec![1.]),
            ],
        )?;
        let mut volume_render = camera_values.clone();
        volume_render.extend(spot_values([
            "nodeUniform23",
            "nodeUniform24",
            "nodeUniform28",
            "nodeUniform29",
            "nodeUniform12",
            "nodeUniform25",
            "nodeUniform26",
            "nodeUniform27",
        ]));
        volume_render.extend([
            ("cameraNear", vec![0.025]),
            ("cameraFar", vec![5.]),
            ("nodeUniform31", vec![self.time]),
            ("nodeUniform6", vec![self.frame_id as f64]),
            ("nodeUniform14", shadow_matrix),
            ("nodeUniform17", vec![0.]),
            ("nodeUniform18", vec![0.]),
            ("nodeUniform22", vec![0.95]),
            ("cameraPosition", camera_position.to_array().to_vec()),
            (
                "nodeUniform11",
                vec![(t.width / 2).max(1) as f64, (t.height / 2).max(1) as f64],
            ),
            ("nodeUniform20", vec![1.]),
            ("nodeUniform21", vec![SHADOW_SIZE as f64; 2]),
        ]);
        write(
            &self.volume_render,
            wgsl!("volume_fs"),
            "renderStruct",
            &volume_render,
        )?;
        let radius = self.volume_box.radius;
        write(
            &self.volume_object,
            wgsl!("volume_fs"),
            "objectStruct",
            &[
                ("nodeUniform0", vec![1.]),
                ("nodeUniform2", m4(volume_model)),
                ("nodeUniform3", vec![radius]),
                ("nodeUniform4", vec![20.]),
                ("nodeUniform16", m3(Matrix4::IDENTITY)),
                ("nodeUniform32", vec![3.]),
            ],
        )?;
        write(
            &self.high_object,
            wgsl!("high_fs"),
            "objectStruct",
            &[("nodeUniform1", vec![0.]), ("nodeUniform2", vec![0.01])],
        )?;
        let identity = m3(Matrix4::IDENTITY);
        for (blur, (_, _, level)) in t.bloom.blurs.iter().zip(&t.bloom.levels) {
            for ((_, _, object), direction) in blur.iter().zip([[1., 0.], [0., 1.]]) {
                write(
                    object,
                    wgsl!("blur0_fs"),
                    "objectStruct",
                    &[
                        ("nodeUniform1", identity.clone()),
                        ("nodeUniform2", identity.clone()),
                        ("nodeUniform3", direction.to_vec()),
                        (
                            "nodeUniform4",
                            vec![1. / level.0 as f64, 1. / level.1 as f64],
                        ),
                        ("nodeUniform5", identity.clone()),
                    ],
                )?;
            }
        }
        write(
            &self.composite_object,
            wgsl!("composite_fs"),
            "objectStruct",
            &[
                ("nodeUniform0", vec![1.]),
                ("nodeUniform3", identity.clone()),
                ("nodeUniform5", identity.clone()),
                ("nodeUniform7", identity.clone()),
                ("nodeUniform9", identity.clone()),
                ("nodeUniform11", identity),
                ("nodeUniform12", vec![1.]),
            ],
        )?;
        write(
            &self.output_object,
            wgsl!("output_fs"),
            "objectStruct",
            &[("nodeUniform2", vec![0.7])],
        )?;
        let mut encoder = r.device.create_command_encoder(&Default::default());
        let draw = |pass: &mut wgpu::RenderPass, m: &Mesh, normal_first: bool| {
            let (a, b) = if normal_first {
                (&m.normals, &m.positions)
            } else {
                (&m.positions, &m.normals)
            };
            pass.set_vertex_buffer(0, a.slice(..));
            pass.set_vertex_buffer(1, b.slice(..));
            pass.set_index_buffer(m.index.slice(..), wgpu::IndexFormat::Uint32);
            pass.draw_indexed(0..m.count, 0, 0..1);
        };
        let set = |pass: &mut wgpu::RenderPass, (pipeline, groups): &Draw| {
            pass.set_pipeline(pipeline);
            pass.set_bind_group(0, &groups[0], &[]);
            pass.set_bind_group(1, &groups[1], &[]);
        };
        // The duck's caustic shadow, front faces then back faces.
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("volume caustics shadow"),
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
            for d in &self.shadow {
                set(&mut pass, d);
                draw(&mut pass, &self.duck, false);
            }
        }
        let frustum = Frustum::from_projection(projection * view);
        let visible = |model: Matrix4, m: &Mesh, scale: f64| {
            frustum.intersects_sphere(Sphere {
                center: model.transform_point3(m.center),
                radius: m.radius * scale,
            })
        };
        let scene_pass = |encoder: &mut wgpu::CommandEncoder, load| {
            encoder
                .begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("volume caustics scene"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &t.color,
                        depth_slice: None,
                        resolve_target: t.resolve_view.as_ref(),
                        ops: wgpu::Operations {
                            load,
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                    depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                        view: &t.depth,
                        depth_ops: Some(wgpu::Operations {
                            load: if matches!(load, wgpu::LoadOp::Load) {
                                wgpu::LoadOp::Load
                            } else {
                                wgpu::LoadOp::Clear(1.)
                            },
                            store: wgpu::StoreOp::Store,
                        }),
                        stencil_ops: None,
                    }),
                    ..Default::default()
                })
                .forget_lifetime()
        };
        {
            let mut pass = scene_pass(&mut encoder, wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT));
            if visible(floor_model, &self.floor, 1.) {
                set(&mut pass, &t.floor);
                draw(&mut pass, &self.floor, true);
            }
        }
        let source = t
            .resolve
            .as_ref()
            .or(t.single.as_ref())
            .ok_or(Error::Invalid("volume caustics target"))?;
        // The transmissive duck: back faces, then front faces, each over a
        // fresh mipmapped copy of the frame.
        if visible(duck_model, &self.duck, 0.5) {
            for d in &t.duck {
                encoder.copy_texture_to_texture(
                    source.as_image_copy(),
                    t.transmission.as_image_copy(),
                    source.size(),
                );
                self.mipmaps
                    .encode(&mut encoder, HALF, &t.transmission_levels);
                let mut pass = scene_pass(&mut encoder, wgpu::LoadOp::Load);
                set(&mut pass, d);
                draw(&mut pass, &self.duck, true);
            }
        }
        let quad_pass =
            |encoder: &mut wgpu::CommandEncoder, target: &wgpu::TextureView, clear: wgpu::Color| {
                encoder
                    .begin_render_pass(&wgpu::RenderPassDescriptor {
                        label: Some("volume caustics quad"),
                        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                            view: target,
                            depth_slice: None,
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Clear(clear),
                                store: wgpu::StoreOp::Store,
                            },
                        })],
                        ..Default::default()
                    })
                    .forget_lifetime()
            };
        // The volumetric layer: the box's ray march over the scene depth.
        {
            let mut pass = quad_pass(&mut encoder, &t.volume_target, wgpu::Color::BLACK);
            if visible(volume_model, &self.volume_box, 1.) {
                set(&mut pass, &t.volume);
                draw(&mut pass, &self.volume_box, false);
            }
        }
        let fullscreen = |pass: &mut wgpu::RenderPass| {
            pass.set_vertex_buffer(0, self.quad_uv.slice(..));
            pass.draw(0..3, 0..1);
        };
        let b = &t.bloom;
        {
            let mut pass = quad_pass(&mut encoder, &b.bright, wgpu::Color::BLACK);
            pass.set_pipeline(&b.high.0);
            pass.set_bind_group(0, &b.high.1, &[]);
            fullscreen(&mut pass);
        }
        for (blur, (horizontal, vertical, _)) in b.blurs.iter().zip(&b.levels) {
            for ((pipeline, group, _), target) in blur.iter().zip([horizontal, vertical]) {
                let mut pass = quad_pass(&mut encoder, target, wgpu::Color::BLACK);
                pass.set_pipeline(pipeline);
                pass.set_bind_group(0, group, &[]);
                fullscreen(&mut pass);
            }
        }
        {
            let mut pass = quad_pass(&mut encoder, &b.levels[0].0, wgpu::Color::BLACK);
            pass.set_pipeline(&b.composite.0);
            pass.set_bind_group(0, &b.composite.1, &[]);
            fullscreen(&mut pass);
        }
        {
            let mut pass = quad_pass(&mut encoder, &t.screen.view, wgpu::Color::TRANSPARENT);
            pass.set_pipeline(&t.output.0);
            pass.set_bind_group(0, &t.output.1, &[]);
            fullscreen(&mut pass);
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
    /// caustic occlusion and material color.
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        *self
            .params
            .get_mut(index)
            .ok_or(Error::Invalid("volume caustics parameter"))? = value as f64;
        Ok(())
    }
    /// A requested frame: nodeFrame.update() advances the frame id.
    pub fn seek(&mut self, t: f64) {
        self.time = t;
        self.frame_id += 1;
    }
}
