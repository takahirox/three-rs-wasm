//! webgpu_postprocessing_retro: the baked coffee mug with its rising smoke
//! (or the Damaged Helmet) under the starry PS1 sky, through RetroPassNode
//! and the CRT chain. The retro pass renders the scene at a quarter of the
//! drawing-buffer size with 4× MSAA, each material replaced by its retro
//! variant (vertex snapping to the pass resolution, affine texture mapping,
//! level-0 texture reads); barrelUV reads it back nearest-neighbour; the
//! output applies color bleeding, the Bayer dither, posterize, the vignette
//! and the scanlines before the sRGB output. Every stage runs the WGSL
//! three.js r186 generates for the page (in `retro/`). The helmet loads when
//! selected, with venice_sunset_1k.hdr: the retro helmet reads it through
//! CubeMapNode's 512² cube, the plain one through scene.environment's PMREM.
use super::controls_attributes::{Controls, camera_state};
use super::gltf_viewer::{decode_texture_image, fetch, load_asset};
use super::lights_projector::{m3, m4, pack};
use super::pmrem_cube_uv::{GGX_8, Pmrem, bind, flat_camera, raw_pipeline, source_pipeline};
use super::shadowmap_opacity::{Mipmaps, mip_count};
use super::trackball_sprites::parse_rgbe;
use crate::{
    Error, Result, camera::*, geometry::*, math::*, render_target::*, renderer::*, scene::*,
};
use std::cell::RefCell;
use std::f64::consts::PI;
use std::rc::Rc;
use wgpu::util::DeviceExt;

const HALF: wgpu::TextureFormat = wgpu::TextureFormat::Rgba16Float;
const DEPTH: wgpu::TextureFormat = wgpu::TextureFormat::Depth24Plus;
macro_rules! wgsl {
    ($name:literal) => {
        include_str!(concat!("retro/", $name, ".wgsl"))
    };
}
pub(super) fn uniform(r: &Renderer, label: &str, source: &str, name: &str) -> Result<wgpu::Buffer> {
    Ok(r.device.create_buffer(&wgpu::BufferDescriptor {
        label: Some(label),
        size: pack(source, name, &[])?.len() as u64,
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    }))
}
pub(super) fn target(
    r: &Renderer,
    size: (u32, u32),
    format: wgpu::TextureFormat,
    samples: u32,
) -> wgpu::TextureView {
    r.device
        .create_texture(&wgpu::TextureDescriptor {
            label: Some("retro target"),
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
                | wgpu::TextureUsages::TEXTURE_BINDING
                | wgpu::TextureUsages::COPY_SRC,
            view_formats: &[],
        })
        .create_view(&Default::default())
}
/// Positions, a second attribute and the index, with a bounding radius.
struct Mesh {
    buffers: [wgpu::Buffer; 2],
    index: wgpu::Buffer,
    count: u32,
    radius: f64,
    model: Matrix4,
}
/// A texture uploaded with its mip chain generated on the GPU.
pub(super) fn mipmapped(
    r: &Renderer,
    mipmaps: &mut Mipmaps,
    image: &crate::material::Texture,
    format: wgpu::TextureFormat,
) -> wgpu::TextureView {
    mipmapped_raw(
        r,
        mipmaps,
        &image.rgba,
        (image.width, image.height),
        4,
        format,
    )
}
/// Texels of `bytes` bytes each, uploaded as level 0 of a mipmapped texture.
pub(super) fn mipmapped_raw(
    r: &Renderer,
    mipmaps: &mut Mipmaps,
    data: &[u8],
    size: (u32, u32),
    bytes: u32,
    format: wgpu::TextureFormat,
) -> wgpu::TextureView {
    let texture = r.device.create_texture(&wgpu::TextureDescriptor {
        label: Some("retro map"),
        size: wgpu::Extent3d {
            width: size.0,
            height: size.1,
            depth_or_array_layers: 1,
        },
        mip_level_count: mip_count(size),
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format,
        usage: wgpu::TextureUsages::TEXTURE_BINDING
            | wgpu::TextureUsages::COPY_DST
            | wgpu::TextureUsages::RENDER_ATTACHMENT,
        view_formats: &[],
    });
    // Textures load flipped ( flipY ), as TextureLoader's are; glTF images
    // are not, and the caller passes them already in their final row order.
    r.queue.write_texture(
        texture.as_image_copy(),
        data,
        wgpu::TexelCopyBufferLayout {
            offset: 0,
            bytes_per_row: Some(size.0 * bytes),
            rows_per_image: Some(size.1),
        },
        wgpu::Extent3d {
            width: size.0,
            height: size.1,
            depth_or_array_layers: 1,
        },
    );
    let mut encoder = r.device.create_command_encoder(&Default::default());
    let levels = mipmaps.levels(r, &texture);
    mipmaps.encode(&mut encoder, format, &levels);
    r.queue.submit([encoder.finish()]);
    texture.create_view(&Default::default())
}
struct Targets {
    width: u32,
    height: u32,
    format: wgpu::TextureFormat,
    retro_size: (u32, u32),
    samples: u32,
    /// The retro pass ( quarter size ) and the plain scene pass.
    retro: ScenePass,
    plain: ScenePass,
    low: wgpu::TextureView,
    full: wgpu::TextureView,
    full_depth: wgpu::TextureView,
    screen: RenderTarget,
    /// The retro mug with level-0 reads, and with filtered ones.
    filtered_mug: (wgpu::RenderPipeline, [wgpu::BindGroup; 2]),
    barrel: (wgpu::RenderPipeline, wgpu::BindGroup),
    output: (wgpu::RenderPipeline, [wgpu::BindGroup; 2]),
    plain_output: (wgpu::RenderPipeline, wgpu::BindGroup),
    helmet: HelmetDraws,
}
/// One scene pass: its multisampled targets ( None without MSAA ) and the
/// background, mug and double-sided smoke draws.
struct ScenePass {
    msaa: Option<(wgpu::TextureView, wgpu::TextureView)>,
    depth: wgpu::TextureView,
    background: (wgpu::RenderPipeline, [wgpu::BindGroup; 2]),
    mug: (wgpu::RenderPipeline, [wgpu::BindGroup; 2]),
    /// Back faces, then front faces.
    smoke: [(wgpu::RenderPipeline, [wgpu::BindGroup; 2]); 2],
}
pub(super) struct Demo {
    controls: Controls,
    time: f64,
    /// Retro pipeline, curvature, color depth, scanlines, scanline density,
    /// scanline speed, vignette, color bleeding, affine distortion.
    params: [f64; 9],
    /// RetroPassNode.filterTextures.
    filter: bool,
    /// The model: 0 the coffee mug, 1 the Damaged Helmet.
    model: usize,
    /// The helmet and its environment, loaded on first selection.
    helmet_load: Option<HelmetLoad>,
    helmet: Option<Helmet>,
    mipmaps: Mipmaps,
    background: Mesh,
    mug: Mesh,
    smoke: Mesh,
    mug_map: wgpu::TextureView,
    noise: wgpu::TextureView,
    background_render: wgpu::Buffer,
    background_object: wgpu::Buffer,
    mug_render: wgpu::Buffer,
    mug_object: wgpu::Buffer,
    smoke_render: wgpu::Buffer,
    smoke_object: wgpu::Buffer,
    plain_background_render: wgpu::Buffer,
    plain_background_object: wgpu::Buffer,
    plain_mug_render: wgpu::Buffer,
    /// The filtered retro mug's render struct orders its uniforms differently.
    filtered_mug_render: wgpu::Buffer,
    plain_mug_object: wgpu::Buffer,
    barrel_object: wgpu::Buffer,
    output_render: wgpu::Buffer,
    output_object: wgpu::Buffer,
    quad_uv: wgpu::Buffer,
    repeat: wgpu::Sampler,
    clamp: wgpu::Sampler,
    targets: Option<Targets>,
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, _id: u32, r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 25.,
            near: 0.1,
            far: 100.,
            aspect,
            ..Default::default()
        }));
        s.get_mut(c)?.position = Vector3::new(8., 5., 20.);
        let mut controls = Controls::new(Some(0.05), (0.1, 50.), PI, true);
        controls.set_target(Vector3::new(0., 1., 0.));
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
        let mesh = |a: &[f32], b: &[f32], index: &[u32], model: Matrix4| Mesh {
            radius: a
                .chunks(3)
                .map(|p| (p[0] as f64).hypot(p[1] as f64).hypot(p[2] as f64))
                .fold(0., f64::max),
            buffers: [
                init("retro attribute", bytemuck::cast_slice(a), vertex),
                init("retro attribute", bytemuck::cast_slice(b), vertex),
            ],
            index: init(
                "retro index",
                bytemuck::cast_slice(index),
                wgpu::BufferUsages::INDEX,
            ),
            count: index.len() as u32,
            model,
        };
        let read = |g: &BufferGeometry, name: &str, n: usize| -> Result<Vec<f32>> {
            let a = g
                .attributes
                .get(name)
                .ok_or(Error::Invalid("retro attribute"))?;
            (0..a.count())
                .flat_map(|i| (0..n).map(move |k| (i, k)))
                .map(|(i, k)| a.get_component(i, k).map(|v| v as f32))
                .collect()
        };
        // The background: three's 32 × 32 unit sphere, normals then positions.
        let sphere = SphereGeometry::build(1., 32, 32)?;
        let background = mesh(
            &read(&sphere, "normal", 3)?,
            &read(&sphere, "position", 3)?,
            &sphere.index.clone().ok_or(Error::Invalid("sphere index"))?,
            Matrix4::IDENTITY,
        );
        // The smoke: PlaneGeometry( 1, 1, 16, 64 ) translated and scaled.
        let plane = PlaneGeometry::build(1., 1., 16, 64)?;
        let positions: Vec<f32> = read(&plane, "position", 3)?
            .chunks(3)
            .flat_map(|p| [p[0] * 1.5, (p[1] + 0.5) * 6., p[2] * 1.5])
            .collect();
        let smoke = mesh(
            &positions,
            &read(&plane, "uv", 2)?,
            &plane.index.clone().ok_or(Error::Invalid("plane index"))?,
            Matrix4::from_translation(Vector3::new(0., 1.83, 0.)),
        );
        let (asset, buffers, images) =
            load_asset("/web/gallery/assets/tsl-next/models/gltf/coffeeMug.glb").await?;
        let node = asset
            .nodes()
            .find(|n| n.mesh().is_some())
            .ok_or(Error::Invalid("mug node"))?;
        let primitive = node
            .mesh()
            .and_then(|m| m.primitives().next())
            .ok_or(Error::Invalid("mug primitive"))?;
        let reader = primitive.reader(|b| buffers.get(b.index()).map(Vec::as_slice));
        let uvs: Vec<f32> = reader
            .read_tex_coords(0)
            .ok_or(Error::Invalid("mug uv"))?
            .into_f32()
            .flatten()
            .collect();
        let positions: Vec<f32> = reader
            .read_positions()
            .ok_or(Error::Invalid("mug positions"))?
            .flatten()
            .collect();
        let index: Vec<u32> = reader
            .read_indices()
            .ok_or(Error::Invalid("mug index"))?
            .into_u32()
            .collect();
        let t = node.transform().decomposed().0;
        let mut mug = mesh(
            &uvs,
            &positions,
            &index,
            Matrix4::from_translation(Vector3::new(t[0] as f64, t[1] as f64, t[2] as f64)),
        );
        // The bounding radius of the positions (the second buffer).
        mug.radius = positions
            .chunks(3)
            .map(|p| (p[0] as f64).hypot(p[1] as f64).hypot(p[2] as f64))
            .fold(0., f64::max);
        let mut mipmaps = Mipmaps::new(r);
        let image = images.first().ok_or(Error::Invalid("mug image"))?;
        let mug_map = mipmapped(r, &mut mipmaps, image, wgpu::TextureFormat::Rgba8UnormSrgb);
        let noise = decode_texture_image(
            &fetch("/web/gallery/assets/tsl-next/textures/noises/perlin/128x128.png").await?,
        )
        .await?;
        let row = noise.width as usize * 4;
        let flipped = crate::material::Texture {
            rgba: noise.rgba.chunks(row).rev().flatten().copied().collect(),
            ..noise
        };
        let noise = mipmapped(r, &mut mipmaps, &flipped, wgpu::TextureFormat::Rgba8Unorm);
        let sampler = |address, mipmap| {
            r.device.create_sampler(&wgpu::SamplerDescriptor {
                address_mode_u: address,
                address_mode_v: address,
                mag_filter: wgpu::FilterMode::Linear,
                min_filter: wgpu::FilterMode::Linear,
                mipmap_filter: mipmap,
                ..Default::default()
            })
        };
        Ok(Self {
            controls,
            time: 0.,
            params: [1., 0.02, 32., 0.3, 1., 0., 0.3, 0.001, 0.],
            filter: false,
            model: 0,
            helmet_load: None,
            helmet: None,
            mipmaps,
            background,
            mug,
            smoke,
            mug_map,
            noise,
            background_render: uniform(
                r,
                "background render",
                wgsl!("background_fs"),
                "renderStruct",
            )?,
            background_object: uniform(
                r,
                "background object",
                wgsl!("background_fs"),
                "objectStruct",
            )?,
            mug_render: uniform(r, "mug render", wgsl!("mug_fs"), "renderStruct")?,
            mug_object: uniform(r, "mug object", wgsl!("mug_fs"), "objectStruct")?,
            smoke_render: uniform(r, "smoke render", wgsl!("smoke_fs"), "renderStruct")?,
            smoke_object: uniform(r, "smoke object", wgsl!("smoke_fs"), "objectStruct")?,
            plain_background_render: uniform(
                r,
                "background render",
                wgsl!("plain_background_fs"),
                "renderStruct",
            )?,
            plain_background_object: uniform(
                r,
                "background object",
                wgsl!("plain_background_fs"),
                "objectStruct",
            )?,
            plain_mug_render: uniform(r, "mug render", wgsl!("plain_mug_fs"), "renderStruct")?,
            filtered_mug_render: uniform(r, "mug render", wgsl!("filter_mug_fs"), "renderStruct")?,
            plain_mug_object: uniform(r, "mug object", wgsl!("plain_mug_fs"), "objectStruct")?,
            barrel_object: uniform(r, "barrel object", wgsl!("retro_fs"), "objectStruct")?,
            output_render: uniform(r, "output render", wgsl!("output_fs"), "renderStruct")?,
            output_object: uniform(r, "output object", wgsl!("output_fs"), "objectStruct")?,
            quad_uv: init(
                "quad uv",
                bytemuck::cast_slice(&[0f32, -1., 0., 1., 2., 1.]),
                vertex,
            ),
            repeat: sampler(wgpu::AddressMode::Repeat, wgpu::FilterMode::Linear),
            clamp: sampler(wgpu::AddressMode::ClampToEdge, wgpu::FilterMode::Nearest),
            targets: None,
        })
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, dt: f64, animate: bool) -> Result<()> {
        if animate {
            self.time += dt;
        }
        Ok(())
    }
    pub fn prepare(&mut self, s: &mut Scene, c: Object3D, _r: &Renderer) -> Result<()> {
        self.controls.frame_update(s, c)
    }
    fn resize(&mut self, r: &Renderer, out: &RenderTarget) -> Result<()> {
        let size = (out.width, out.height);
        // PassNode takes the renderer's samples: 4× with antialias.
        let samples = if out.options.samples > 1 { 4 } else { 1 };
        // PassNode at resolution scale 0.25.
        let retro_size = (
            ((size.0 as f64 * 0.25) as u32).max(1),
            ((size.1 as f64 * 0.25) as u32).max(1),
        );
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
            wgpu::vertex_attr_array![1 => Float32x2],
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
        // (format, samples, depth: (compare, write), cull, clockwise, blended)
        #[allow(clippy::type_complexity)]
        let pipeline = |label,
                        vs: &str,
                        fs: &str,
                        buffers: &[wgpu::VertexBufferLayout],
                        state: (
            wgpu::TextureFormat,
            u32,
            (wgpu::CompareFunction, bool),
            Option<wgpu::Face>,
            bool,
            bool,
        )| {
            let (format, samples, (compare, write), cull, cw, blended) = state;
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
                        cull_mode: cull,
                        ..Default::default()
                    },
                    depth_stencil: Some(wgpu::DepthStencilState {
                        format: DEPTH,
                        depth_write_enabled: write,
                        depth_compare: compare,
                        stencil: Default::default(),
                        bias: Default::default(),
                    }),
                    multisample: wgpu::MultisampleState {
                        count: samples,
                        ..Default::default()
                    },
                    multiview: None,
                    cache: None,
                })
        };
        let less = (wgpu::CompareFunction::LessEqual, true);
        let back = Some(wgpu::Face::Back);
        let tex = wgpu::BindingResource::TextureView;
        let sampler = wgpu::BindingResource::Sampler;
        let groups = |p: &wgpu::RenderPipeline,
                      render: &wgpu::Buffer,
                      object: Vec<(u32, wgpu::BindingResource)>| {
            [
                bind(
                    r,
                    p.get_bind_group_layout(0),
                    &[(0, render.as_entire_binding())],
                ),
                bind(r, p.get_bind_group_layout(1), &object),
            ]
        };
        let mug_object = |plain: bool| {
            let object = if plain {
                &self.plain_mug_object
            } else {
                &self.mug_object
            };
            vec![
                (0, object.as_entire_binding()),
                (1, sampler(&self.repeat)),
                (2, tex(&self.mug_map)),
            ]
        };
        let smoke_object = || {
            vec![
                (0, sampler(&self.repeat)),
                (1, tex(&self.noise)),
                (2, self.smoke_object.as_entire_binding()),
                (3, sampler(&self.repeat)),
                (4, tex(&self.noise)),
            ]
        };
        let scene_pass = |retro: bool, size: (u32, u32)| -> ScenePass {
            let (bvs, bfs, mvs, mfs, brender, bobject, mrender) = if retro {
                (
                    wgsl!("background_vs"),
                    wgsl!("background_fs"),
                    wgsl!("mug_vs"),
                    wgsl!("mug_fs"),
                    &self.background_render,
                    &self.background_object,
                    &self.mug_render,
                )
            } else {
                (
                    wgsl!("plain_background_vs"),
                    wgsl!("plain_background_fs"),
                    wgsl!("plain_mug_vs"),
                    wgsl!("plain_mug_fs"),
                    &self.plain_background_render,
                    &self.plain_background_object,
                    &self.plain_mug_render,
                )
            };
            let background = pipeline(
                "retro background",
                bvs,
                bfs,
                &[layout(0, 12), layout(1, 12)],
                (
                    HALF,
                    samples,
                    (wgpu::CompareFunction::Always, false),
                    back,
                    true,
                    false,
                ),
            );
            let background_groups =
                groups(&background, brender, vec![(0, bobject.as_entire_binding())]);
            let mug = pipeline(
                "retro mug",
                mvs,
                mfs,
                &[layout(2, 8), layout(1, 12)],
                (HALF, samples, less, None, false, false),
            );
            let mug_groups = groups(&mug, mrender, mug_object(!retro));
            let smoke = [true, false].map(|cw| {
                let p = pipeline(
                    "retro smoke",
                    wgsl!("smoke_vs"),
                    wgsl!("smoke_fs"),
                    &[layout(0, 12), layout(3, 8)],
                    (
                        HALF,
                        samples,
                        (wgpu::CompareFunction::LessEqual, false),
                        back,
                        cw,
                        true,
                    ),
                );
                let g = groups(&p, &self.smoke_render, smoke_object());
                (p, g)
            });
            ScenePass {
                msaa: (samples > 1).then(|| {
                    (
                        target(r, size, HALF, samples),
                        target(r, size, DEPTH, samples),
                    )
                }),
                depth: target(r, size, DEPTH, 1),
                background: (background, background_groups),
                mug: (mug, mug_groups),
                smoke,
            }
        };
        let retro = scene_pass(true, retro_size);
        let plain = scene_pass(false, size);
        let filtered = pipeline(
            "retro mug",
            wgsl!("filter_mug_vs"),
            wgsl!("filter_mug_fs"),
            &[layout(2, 8), layout(1, 12)],
            (HALF, samples, less, None, false, false),
        );
        let filtered_groups = groups(&filtered, &self.filtered_mug_render, mug_object(false));
        let low = target(r, retro_size, HALF, 1);
        let full = target(r, size, HALF, 1);
        let full_depth = target(r, size, DEPTH, 1);
        let quad = [layout(2, 8)];
        let barrel = pipeline(
            "retro barrel",
            wgsl!("retro_vs"),
            wgsl!("retro_fs"),
            &quad,
            (HALF, 1, less, back, false, false),
        );
        let barrel_group = bind(
            r,
            barrel.get_bind_group_layout(0),
            &[(0, tex(&low)), (1, self.barrel_object.as_entire_binding())],
        );
        let output = pipeline(
            "retro output",
            wgsl!("output_vs"),
            wgsl!("output_fs"),
            &quad,
            (out.options.format, 1, less, back, false, false),
        );
        let output_groups = [
            bind(
                r,
                output.get_bind_group_layout(0),
                &[(0, self.output_render.as_entire_binding())],
            ),
            bind(
                r,
                output.get_bind_group_layout(1),
                &[
                    (0, sampler(&self.clamp)),
                    (1, tex(&full)),
                    (2, self.output_object.as_entire_binding()),
                ],
            ),
        ];
        let plain_output = pipeline(
            "retro plain output",
            wgsl!("plain_output_vs"),
            wgsl!("plain_output_fs"),
            &quad,
            (out.options.format, 1, less, back, false, false),
        );
        let plain_output_group = bind(
            r,
            plain_output.get_bind_group_layout(0),
            &[(0, sampler(&self.clamp)), (1, tex(&full))],
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
            retro_size,
            samples,
            retro,
            plain,
            low,
            full,
            full_depth,
            screen,
            filtered_mug: (filtered, filtered_groups),
            barrel: (barrel, barrel_group),
            output: (output, output_groups),
            plain_output: (plain_output, plain_output_group),
            helmet: HelmetDraws::default(),
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
        let retro_on = self.params[0] > 0.5;
        let helmet_on = self.model == 1;
        if helmet_on
            && self.helmet.is_none()
            && let Some(load) = &self.helmet_load
            && let Some(result) = load.borrow_mut().take()
        {
            self.helmet = Some(Helmet::new(r, &mut self.mipmaps, result?)?);
        }
        // The smoke hides once the helmet has loaded; until then neither model draws.
        let helmet_shown = helmet_on && self.helmet.is_some();
        if helmet_shown {
            self.prepare_helmet(r, retro_on)?;
        }
        let t = self
            .targets
            .as_ref()
            .ok_or(Error::Invalid("retro targets"))?;
        s.update()?;
        let (camera, world) = s.camera(c)?;
        let (projection, view) = (camera.projection_matrix()?, world.inverse());
        let time = self.time;
        let [
            _,
            curvature,
            depth,
            scanlines,
            density,
            speed,
            vignette,
            bleeding,
            affine,
        ] = self.params;
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
        // AmbientLight( 0x404040, 2 ).
        let ambient = Color::from_hex(0x404040)
            .0
            .to_array()
            .map(|v| v * 2.)
            .to_vec();
        let mut values = camera_values.clone();
        values.push(("nodeUniform3", vec![1.]));
        write(
            &self.background_render,
            wgsl!("background_fs"),
            "renderStruct",
            &values,
        )?;
        write(
            &self.background_object,
            wgsl!("background_fs"),
            "objectStruct",
            &[
                ("nodeUniform1", m3(Matrix4::IDENTITY)),
                ("nodeUniform4", vec![1.]),
                ("nodeUniform5", vec![30.]),
                (
                    "nodeUniform6",
                    Color::from_hex(0x111111).0.to_array().to_vec(),
                ),
                ("nodeUniform8", vec![1.]),
                ("nodeUniform10", m4(Matrix4::IDENTITY)),
            ],
        )?;
        let mut values = camera_values.clone();
        values.push(("nodeUniform3", vec![1.]));
        write(
            &self.plain_background_render,
            wgsl!("plain_background_fs"),
            "renderStruct",
            &values,
        )?;
        write(
            &self.plain_background_object,
            wgsl!("plain_background_fs"),
            "objectStruct",
            &[
                ("nodeUniform1", m3(Matrix4::IDENTITY)),
                ("nodeUniform4", vec![1.]),
                ("nodeUniform6", m4(Matrix4::IDENTITY)),
            ],
        )?;
        let mut values = camera_values.clone();
        values.push(("nodeUniform4", ambient.clone()));
        write(
            &self.plain_mug_render,
            wgsl!("plain_mug_fs"),
            "renderStruct",
            &values,
        )?;
        write(
            &self.plain_mug_object,
            wgsl!("plain_mug_fs"),
            "objectStruct",
            &[
                ("nodeUniform0", vec![1.; 3]),
                ("nodeUniform2", m3(Matrix4::IDENTITY)),
                ("nodeUniform3", vec![1.]),
                ("nodeUniform7", m4(self.mug.model)),
            ],
        )?;
        let mut values = camera_values.clone();
        values.extend([
            (
                "nodeUniform9",
                vec![t.retro_size.0 as f64, t.retro_size.1 as f64],
            ),
            ("nodeUniform5", ambient.clone()),
        ]);
        write(&self.mug_render, wgsl!("mug_fs"), "renderStruct", &values)?;
        write(
            &self.filtered_mug_render,
            wgsl!("filter_mug_fs"),
            "renderStruct",
            &values,
        )?;
        write(
            &self.mug_object,
            wgsl!("mug_fs"),
            "objectStruct",
            &[
                ("nodeUniform0", vec![1.; 3]),
                ("nodeUniform2", m3(Matrix4::IDENTITY)),
                ("nodeUniform3", vec![affine]),
                ("nodeUniform4", vec![1.]),
                ("nodeUniform8", m4(self.mug.model)),
            ],
        )?;
        let mut values = camera_values.clone();
        values.extend([
            ("nodeUniform1", vec![time]),
            ("nodeUniform3", vec![time]),
            ("nodeUniform5", ambient.clone()),
        ]);
        write(
            &self.smoke_render,
            wgsl!("smoke_fs"),
            "renderStruct",
            &values,
        )?;
        write(
            &self.smoke_object,
            wgsl!("smoke_fs"),
            "objectStruct",
            &[
                ("nodeUniform4", vec![1.]),
                ("nodeUniform8", m4(self.smoke.model)),
            ],
        )?;
        if let Some(h) = self.helmet.as_ref().filter(|_| helmet_shown) {
            // AmbientLight( 0x404040, 2 ), DirectionalLight( 0xffffff, 3 ) at
            // ( 5, 10, 5 ) and PointLight( 0xff6600, 5, 20 ) at ( -3, 3, 2 ).
            let point = Color::from_hex(0xff6600)
                .0
                .to_array()
                .map(|v| v * 5.)
                .to_vec();
            let point_view = view
                .transform_point3(Vector3::new(-3., 3., 2.))
                .to_array()
                .to_vec();
            let normal = m3(h.model.inverse().transpose());
            let identity = m3(Matrix4::IDENTITY);
            let mut values = camera_values.clone();
            values.extend([
                ("nodeUniform29", ambient.clone()),
                ("nodeUniform33", vec![3.; 3]),
                ("nodeUniform35", point.clone()),
                ("nodeUniform36", vec![20.]),
                ("nodeUniform37", vec![2.]),
                ("nodeUniform31", vec![5., 10., 5.]),
                ("nodeUniform32", vec![0.; 3]),
                ("nodeUniform34", point_view.clone()),
                ("cameraProjectionMatrixInverse", m4(projection.inverse())),
                (
                    "nodeUniform11",
                    vec![t.retro_size.0 as f64, t.retro_size.1 as f64],
                ),
                ("cameraWorldMatrix", m4(world)),
            ]);
            write(&h.render, wgsl!("helmet_fs"), "renderStruct", &values)?;
            write(
                &h.filtered_render,
                wgsl!("filter_helmet_fs"),
                "renderStruct",
                &values,
            )?;
            write(
                &h.object,
                wgsl!("helmet_fs"),
                "objectStruct",
                &[
                    ("nodeUniform0", vec![1.; 3]),
                    ("nodeUniform2", identity.clone()),
                    ("nodeUniform3", vec![affine]),
                    ("nodeUniform5", m4(Matrix4::IDENTITY)),
                    ("nodeUniform8", m4(h.model)),
                    ("nodeUniform12", normal.clone()),
                    ("nodeUniform14", identity.clone()),
                    ("nodeUniform15", vec![1., -1.]),
                    ("nodeUniform16", vec![1.]),
                    ("nodeUniform18", identity.clone()),
                    ("nodeUniform19", vec![1.]),
                    ("nodeUniform21", identity.clone()),
                    ("nodeUniform22", vec![1.]),
                    ("nodeUniform23", vec![30.]),
                    (
                        "nodeUniform24",
                        Color::from_hex(0x111111).0.to_array().to_vec(),
                    ),
                    ("nodeUniform25", vec![1.; 3]),
                    ("nodeUniform26", vec![1.]),
                    ("nodeUniform28", identity.clone()),
                ],
            )?;
            let mut values = camera_values.clone();
            values.extend([
                ("nodeUniform23", ambient.clone()),
                ("nodeUniform27", vec![3.; 3]),
                ("nodeUniform29", point),
                ("nodeUniform30", vec![20.]),
                ("nodeUniform31", vec![2.]),
                ("nodeUniform25", vec![5., 10., 5.]),
                ("nodeUniform26", vec![0.; 3]),
                ("nodeUniform28", point_view),
                ("cameraWorldMatrix", m4(world)),
            ]);
            write(
                &h.plain_render,
                wgsl!("plain_helmet_fs"),
                "renderStruct",
                &values,
            )?;
            // The PMREM's lodMax and texel size, and the environment rotation and intensity.
            write(
                &h.plain_object,
                wgsl!("plain_helmet_fs"),
                "objectStruct",
                &[
                    ("nodeUniform0", vec![1.; 3]),
                    ("nodeUniform2", identity.clone()),
                    ("nodeUniform3", vec![1.]),
                    ("nodeUniform5", identity.clone()),
                    ("nodeUniform6", vec![1.]),
                    ("nodeUniform7", vec![1.]),
                    ("nodeUniform9", identity.clone()),
                    ("nodeUniform10", vec![1.]),
                    ("nodeUniform11", identity.clone()),
                    ("nodeUniform13", normal),
                    ("nodeUniform14", vec![1.; 3]),
                    ("nodeUniform15", vec![1.]),
                    ("nodeUniform17", identity.clone()),
                    ("nodeUniform19", m4(h.model)),
                    ("nodeUniform21", identity),
                    ("nodeUniform22", vec![1., -1.]),
                    ("nodeUniform32", vec![8.]),
                    ("nodeUniform33", m4(Matrix4::IDENTITY)),
                    ("nodeUniform35", vec![1. / 768.]),
                    ("nodeUniform36", vec![1. / 1024.]),
                    ("nodeUniform38", vec![1.]),
                ],
            )?;
        }
        write(
            &self.barrel_object,
            wgsl!("retro_fs"),
            "objectStruct",
            &[("nodeUniform1", vec![curvature])],
        )?;
        write(
            &self.output_render,
            wgsl!("output_fs"),
            "renderStruct",
            &[
                ("nodeUniform6", vec![time]),
                ("nodeUniform1", vec![t.width as f64, t.height as f64]),
            ],
        )?;
        write(
            &self.output_object,
            wgsl!("output_fs"),
            "objectStruct",
            &[
                ("nodeUniform2", vec![bleeding]),
                ("nodeUniform3", vec![curvature]),
                ("nodeUniform4", vec![depth]),
                ("nodeUniform5", vec![vignette]),
                ("nodeUniform7", vec![speed]),
                ("nodeUniform8", vec![density]),
                ("nodeUniform9", vec![scanlines]),
            ],
        )?;
        let frustum = Frustum::from_projection(projection * view);
        let visible = |m: &Mesh| {
            frustum.intersects_sphere(Sphere {
                center: m.model.transform_point3(Vector3::ZERO),
                radius: m.radius,
            })
        };
        let draw = |pass: &mut wgpu::RenderPass, m: &Mesh| {
            pass.set_vertex_buffer(0, m.buffers[0].slice(..));
            pass.set_vertex_buffer(1, m.buffers[1].slice(..));
            pass.set_index_buffer(m.index.slice(..), wgpu::IndexFormat::Uint32);
            pass.draw_indexed(0..m.count, 0, 0..1);
        };
        let mut encoder = r.device.create_command_encoder(&Default::default());
        let (scene, resolve, clear) = if retro_on {
            (&t.retro, &t.low, wgpu::Color::BLACK)
        } else {
            (&t.plain, &t.full, wgpu::Color::TRANSPARENT)
        };
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("retro scene"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: scene.msaa.as_ref().map_or(resolve, |m| &m.0),
                    depth_slice: None,
                    resolve_target: scene.msaa.as_ref().map(|_| resolve),
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(clear),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: scene.msaa.as_ref().map_or(&scene.depth, |m| &m.1),
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.),
                        store: wgpu::StoreOp::Discard,
                    }),
                    stencil_ops: None,
                }),
                ..Default::default()
            });
            pass.set_pipeline(&scene.background.0);
            pass.set_bind_group(0, &scene.background.1[0], &[]);
            pass.set_bind_group(1, &scene.background.1[1], &[]);
            draw(&mut pass, &self.background);
            if helmet_on {
                if let Some(h) = self
                    .helmet
                    .as_ref()
                    .filter(|h| frustum.intersects_sphere(h.sphere))
                    && let Some((pipeline, groups)) = if !retro_on {
                        &t.helmet.plain
                    } else if self.filter {
                        &t.helmet.filtered
                    } else {
                        &t.helmet.retro
                    }
                {
                    pass.set_pipeline(pipeline);
                    pass.set_bind_group(0, &groups[0], &[]);
                    pass.set_bind_group(1, &groups[1], &[]);
                    // The retro variants take uv, position, normal; the plain one uv, normal, position.
                    let [uv, position, normal] = &h.buffers;
                    let order = if retro_on {
                        [uv, position, normal]
                    } else {
                        [uv, normal, position]
                    };
                    for (slot, buffer) in order.into_iter().enumerate() {
                        pass.set_vertex_buffer(slot as u32, buffer.slice(..));
                    }
                    pass.set_index_buffer(h.index.slice(..), wgpu::IndexFormat::Uint32);
                    pass.draw_indexed(0..h.count, 0, 0..1);
                }
            } else if visible(&self.mug) {
                let mug = if retro_on && self.filter {
                    &t.filtered_mug
                } else {
                    &scene.mug
                };
                pass.set_pipeline(&mug.0);
                pass.set_bind_group(0, &mug.1[0], &[]);
                pass.set_bind_group(1, &mug.1[1], &[]);
                draw(&mut pass, &self.mug);
            }
            // The smoke's positionNode moves it out of its bounds; three culls
            // it against the geometry's bounding sphere all the same.
            if !helmet_shown && visible(&self.smoke) {
                for (pipeline, groups) in &scene.smoke {
                    pass.set_pipeline(pipeline);
                    pass.set_bind_group(0, &groups[0], &[]);
                    pass.set_bind_group(1, &groups[1], &[]);
                    draw(&mut pass, &self.smoke);
                }
            }
        }
        let quad = |encoder: &mut wgpu::CommandEncoder,
                    view: &wgpu::TextureView,
                    depth: Option<&wgpu::TextureView>,
                    pipeline: &wgpu::RenderPipeline,
                    groups: &[&wgpu::BindGroup]| {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("retro quad"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: depth.map(|view| {
                    wgpu::RenderPassDepthStencilAttachment {
                        view,
                        depth_ops: Some(wgpu::Operations {
                            load: wgpu::LoadOp::Clear(1.),
                            store: wgpu::StoreOp::Discard,
                        }),
                        stencil_ops: None,
                    }
                }),
                ..Default::default()
            });
            pass.set_pipeline(pipeline);
            for (i, g) in groups.iter().enumerate() {
                pass.set_bind_group(i as u32, *g, &[]);
            }
            pass.set_vertex_buffer(0, self.quad_uv.slice(..));
            pass.draw(0..3, 0..1);
        };
        if retro_on {
            quad(
                &mut encoder,
                &t.full,
                Some(&t.full_depth),
                &t.barrel.0,
                &[&t.barrel.1],
            );
            quad(
                &mut encoder,
                &t.screen.view,
                Some(&t.full_depth),
                &t.output.0,
                &[&t.output.1[0], &t.output.1[1]],
            );
        } else {
            quad(
                &mut encoder,
                &t.screen.view,
                Some(&t.full_depth),
                &t.plain_output.0,
                &[&t.plain_output.1],
            );
        }
        r.queue.submit([encoder.finish()]);
        Ok(true)
    }
    /// Renders the cube or the PMREM the helmet's current variant needs and
    /// builds its draw in the current targets.
    fn prepare_helmet(&mut self, r: &Renderer, retro_on: bool) -> Result<()> {
        let (Some(h), Some(t)) = (self.helmet.as_mut(), self.targets.as_mut()) else {
            return Ok(());
        };
        let attributes = [
            wgpu::vertex_attr_array![0 => Float32x2],
            wgpu::vertex_attr_array![1 => Float32x3],
            wgpu::vertex_attr_array![2 => Float32x3],
        ];
        let layouts = [(0, 8), (1, 12), (2, 12)].map(|(i, stride)| wgpu::VertexBufferLayout {
            array_stride: stride,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &attributes[i],
        });
        let tex = wgpu::BindingResource::TextureView;
        let sampler = wgpu::BindingResource::Sampler;
        let repeat = || sampler(&self.repeat);
        let build =
            |label, vs, fs, render: &wgpu::Buffer, object: Vec<(u32, wgpu::BindingResource)>| {
                let p = raw_pipeline(
                    r,
                    label,
                    vs,
                    fs,
                    &layouts,
                    HALF,
                    t.samples,
                    Some((wgpu::CompareFunction::LessEqual, true)),
                    false,
                );
                let groups = [
                    bind(
                        r,
                        p.get_bind_group_layout(0),
                        &[(0, render.as_entire_binding())],
                    ),
                    bind(r, p.get_bind_group_layout(1), &object),
                ];
                (p, groups)
            };
        if retro_on {
            if h.cube.is_none() {
                h.render_cube(r, &self.clamp)?;
            }
            let cube = h.cube.as_ref().ok_or(Error::Invalid("helmet cube"))?;
            let [albedo, metal_roughness, emissive, occlusion, normal] = &h.maps;
            let object = || {
                vec![
                    (0, h.object.as_entire_binding()),
                    (1, repeat()),
                    (2, tex(albedo)),
                    (3, sampler(&self.clamp)),
                    (4, tex(cube)),
                    (5, repeat()),
                    (6, tex(normal)),
                    (7, repeat()),
                    (8, tex(metal_roughness)),
                    (9, repeat()),
                    (10, tex(occlusion)),
                    (11, repeat()),
                    (12, tex(emissive)),
                ]
            };
            if self.filter && t.helmet.filtered.is_none() {
                t.helmet.filtered = Some(build(
                    "retro helmet",
                    wgsl!("filter_helmet_vs"),
                    wgsl!("filter_helmet_fs"),
                    &h.filtered_render,
                    object(),
                ));
            }
            if !self.filter && t.helmet.retro.is_none() {
                t.helmet.retro = Some(build(
                    "retro helmet",
                    wgsl!("helmet_vs"),
                    wgsl!("helmet_fs"),
                    &h.render,
                    object(),
                ));
            }
        } else {
            if h.pmrem.is_none() {
                h.generate_pmrem(r, &self.clamp)?;
            }
            if t.helmet.plain.is_none() {
                let pmrem = h.pmrem.as_ref().ok_or(Error::Invalid("helmet PMREM"))?;
                let [albedo, metal_roughness, emissive, occlusion, normal] = &h.maps;
                t.helmet.plain = Some(build(
                    "helmet",
                    wgsl!("plain_helmet_vs"),
                    wgsl!("plain_helmet_fs"),
                    &h.plain_render,
                    vec![
                        (0, h.plain_object.as_entire_binding()),
                        (1, repeat()),
                        (2, tex(albedo)),
                        (3, repeat()),
                        (4, tex(occlusion)),
                        (5, repeat()),
                        (6, tex(metal_roughness)),
                        (7, repeat()),
                        (8, tex(emissive)),
                        (9, sampler(&self.clamp)),
                        (10, tex(&r.dfg)),
                        (11, repeat()),
                        (12, tex(normal)),
                        (13, sampler(&self.clamp)),
                        (14, tex(&pmrem.view)),
                    ],
                ));
            }
        }
        Ok(())
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
    /// The model; then the retro pipeline switch,
    /// curvature, color depth, scanlines, scanline density, scanline speed,
    /// vignette, color bleeding and affine distortion.
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        match index {
            // loadModel(): the helmet loads once, then its environment.
            0 => {
                self.model = (value > 0.5) as usize;
                if self.model == 1 && self.helmet_load.is_none() {
                    let slot: HelmetLoad = Rc::default();
                    let result = slot.clone();
                    wasm_bindgen_futures::spawn_local(async move {
                        *result.borrow_mut() = Some(load_helmet().await);
                    });
                    self.helmet_load = Some(slot);
                }
            }
            1..=9 => self.params[index - 1] = value as f64,
            // retro.dispose(): the materials rebuild with filtered reads.
            10 => self.filter = value > 0.5,
            _ => return Err(Error::Invalid("retro parameter")),
        }
        Ok(())
    }
    pub fn seek(&mut self, t: f64) {
        self.time = t;
    }
}
/// The Damaged Helmet as loaded: uv, position, normal and index, the maps
/// (albedo, metal-roughness, emissive, AO, normal), the model matrix and
/// venice_sunset_1k.hdr (rows flipped, as HDRLoader's texture is).
struct HelmetData {
    attributes: [Vec<f32>; 3],
    index: Vec<u32>,
    images: Vec<crate::material::Texture>,
    model: Matrix4,
    hdr: (u32, u32, Vec<u16>),
}
type HelmetLoad = Rc<RefCell<Option<Result<HelmetData>>>>;
async fn load_helmet() -> Result<HelmetData> {
    let (asset, buffers, images) =
        load_asset("/web/models/DamagedHelmet/glTF/DamagedHelmet.gltf").await?;
    let node = asset
        .nodes()
        .find(|n| n.mesh().is_some())
        .ok_or(Error::Invalid("helmet node"))?;
    let primitive = node
        .mesh()
        .and_then(|m| m.primitives().next())
        .ok_or(Error::Invalid("helmet primitive"))?;
    let reader = primitive.reader(|b| buffers.get(b.index()).map(Vec::as_slice));
    let uv = reader
        .read_tex_coords(0)
        .ok_or(Error::Invalid("helmet uv"))?
        .into_f32()
        .flatten()
        .collect();
    let position = reader
        .read_positions()
        .ok_or(Error::Invalid("helmet positions"))?
        .flatten()
        .collect();
    let normal = reader
        .read_normals()
        .ok_or(Error::Invalid("helmet normals"))?
        .flatten()
        .collect();
    let index = reader
        .read_indices()
        .ok_or(Error::Invalid("helmet index"))?
        .into_u32()
        .collect();
    // gltf.scene at scale 3 and y = 1, with the node's rotation.
    let q = node.transform().decomposed().1;
    let model = Matrix4::from_scale_rotation_translation(
        Vector3::splat(3.),
        Quaternion::from_xyzw(q[0] as f64, q[1] as f64, q[2] as f64, q[3] as f64),
        Vector3::new(0., 1., 0.),
    );
    let (width, height, texels) =
        parse_rgbe(&fetch("/web/environments/venice_sunset_1k.hdr").await?)?;
    let row = width as usize * 4;
    let texels = texels.chunks(row).rev().flatten().copied().collect();
    Ok(HelmetData {
        attributes: [uv, position, normal],
        index,
        images,
        model,
        hdr: (width, height, texels),
    })
}
fn float3(location: u32) -> [wgpu::VertexAttribute; 1] {
    [wgpu::VertexAttribute {
        format: wgpu::VertexFormat::Float32x3,
        offset: 0,
        shader_location: location,
    }]
}
/// The helmet's resident resources.
struct Helmet {
    /// uv, position and normal.
    buffers: [wgpu::Buffer; 3],
    index: wgpu::Buffer,
    count: u32,
    model: Matrix4,
    /// The geometry's bounding sphere in world space.
    sphere: Sphere,
    /// Albedo, metal-roughness, emissive, AO and normal.
    maps: [wgpu::TextureView; 5],
    equirect: wgpu::TextureView,
    /// CubeMapNode's 512² cube, rendered the first time the retro helmet draws.
    cube: Option<wgpu::TextureView>,
    /// scene.environment's PMREM, generated the first time the plain pass draws the helmet.
    pmrem: Option<Pmrem>,
    render: wgpu::Buffer,
    /// The filtered retro helmet's render struct orders its uniforms differently.
    filtered_render: wgpu::Buffer,
    object: wgpu::Buffer,
    plain_render: wgpu::Buffer,
    plain_object: wgpu::Buffer,
}
impl Helmet {
    fn new(r: &Renderer, mipmaps: &mut Mipmaps, data: HelmetData) -> Result<Self> {
        let init = |label, data: &[u8], usage| {
            r.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some(label),
                    contents: data,
                    usage,
                })
        };
        let vertex = wgpu::BufferUsages::VERTEX;
        let [uv, position, normal] = &data.attributes;
        // Geometry.computeBoundingSphere: the box center and the farthest point.
        let points: Vec<Vector3> = position
            .chunks(3)
            .map(|p| Vector3::new(p[0] as f64, p[1] as f64, p[2] as f64))
            .collect();
        let (min, max) = points.iter().fold(
            (
                Vector3::splat(f64::INFINITY),
                Vector3::splat(f64::NEG_INFINITY),
            ),
            |(a, b), p| (a.min(*p), b.max(*p)),
        );
        let center = (min + max) * 0.5;
        let radius = points.iter().map(|p| p.distance(center)).fold(0., f64::max);
        let image = |i: usize| data.images.get(i).ok_or(Error::Invalid("helmet image"));
        let srgb = wgpu::TextureFormat::Rgba8UnormSrgb;
        let linear = wgpu::TextureFormat::Rgba8Unorm;
        let maps = [
            mipmapped(r, mipmaps, image(0)?, srgb),
            mipmapped(r, mipmaps, image(1)?, linear),
            mipmapped(r, mipmaps, image(2)?, srgb),
            mipmapped(r, mipmaps, image(3)?, linear),
            mipmapped(r, mipmaps, image(4)?, linear),
        ];
        let (width, height, texels) = &data.hdr;
        let equirect = mipmapped_raw(
            r,
            mipmaps,
            bytemuck::cast_slice(texels),
            (*width, *height),
            8,
            HALF,
        );
        Ok(Self {
            buffers: [
                init("helmet uv", bytemuck::cast_slice(uv), vertex),
                init("helmet position", bytemuck::cast_slice(position), vertex),
                init("helmet normal", bytemuck::cast_slice(normal), vertex),
            ],
            index: init(
                "helmet index",
                bytemuck::cast_slice(&data.index),
                wgpu::BufferUsages::INDEX,
            ),
            count: data.index.len() as u32,
            model: data.model,
            sphere: Sphere {
                center: data.model.transform_point3(center),
                radius: radius * 3.,
            },
            maps,
            equirect,
            cube: None,
            pmrem: None,
            render: uniform(r, "helmet render", wgsl!("helmet_fs"), "renderStruct")?,
            filtered_render: uniform(
                r,
                "helmet render",
                wgsl!("filter_helmet_fs"),
                "renderStruct",
            )?,
            object: uniform(r, "helmet object", wgsl!("helmet_fs"), "objectStruct")?,
            plain_render: uniform(r, "helmet render", wgsl!("plain_helmet_fs"), "renderStruct")?,
            plain_object: uniform(r, "helmet object", wgsl!("plain_helmet_fs"), "objectStruct")?,
        })
    }
    /// CubeMapNode: the equirect texture rendered into a 512² cube by a
    /// CubeCamera ( near 1, far 10 ) inside a 5 × 5 × 5 back-faced box.
    fn render_cube(&mut self, r: &Renderer, clamp: &wgpu::Sampler) -> Result<()> {
        const SIZE: u32 = 512;
        let cube = r.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("helmet cube"),
            size: wgpu::Extent3d {
                width: SIZE,
                height: SIZE,
                depth_or_array_layers: 6,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: HALF,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });
        let depth = target(r, (SIZE, SIZE), DEPTH, 1);
        let geometry = BoxGeometry::build(5., 5., 5.)?;
        let a = geometry
            .attributes
            .get("position")
            .ok_or(Error::Invalid("box position"))?;
        let positions: Vec<f32> = (0..a.count())
            .flat_map(|i| (0..3).map(move |k| (i, k)))
            .map(|(i, k)| a.get_component(i, k).map(|v| v as f32))
            .collect::<Result<_>>()?;
        let index = geometry.index.clone().ok_or(Error::Invalid("box index"))?;
        let init = |data: &[u8], usage| {
            r.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("helmet cube"),
                    contents: data,
                    usage,
                })
        };
        let vertices = init(bytemuck::cast_slice(&positions), wgpu::BufferUsages::VERTEX);
        let indices = init(bytemuck::cast_slice(&index), wgpu::BufferUsages::INDEX);
        let attribute = float3(0);
        let pipeline = raw_pipeline(
            r,
            "helmet cube",
            wgsl!("cube_vs"),
            wgsl!("cube_fs"),
            &[wgpu::VertexBufferLayout {
                array_stride: 12,
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes: &attribute,
            }],
            HALF,
            1,
            Some((wgpu::CompareFunction::LessEqual, true)),
            true,
        );
        let object = init(
            &pack(
                wgsl!("cube_vs"),
                "objectStruct",
                &[
                    ("nodeUniform1", &m4(Matrix4::IDENTITY)),
                    ("nodeUniform2", &[1.]),
                    ("nodeUniform3", &[30.]),
                    ("nodeUniform4", &Color::from_hex(0x111111).0.to_array()),
                    ("nodeUniform6", &[1.]),
                ],
            )?,
            wgpu::BufferUsages::UNIFORM,
        );
        let object_group = bind(
            r,
            pipeline.get_bind_group_layout(1),
            &[
                (0, wgpu::BindingResource::Sampler(clamp)),
                (1, wgpu::BindingResource::TextureView(&self.equirect)),
                (2, object.as_entire_binding()),
            ],
        );
        // CubeCamera's WebGPU projection and the six face views, in layer order.
        let projection = [
            -1.,
            0.,
            0.,
            0.,
            0.,
            -1.,
            0.,
            0.,
            0.,
            0.,
            -10. / 9.,
            -1.,
            0.,
            0.,
            -10. / 9.,
            0.,
        ];
        let views: [[f64; 16]; 6] = [
            [
                0., 0., 1., 0., 0., -1., 0., 0., 1., 0., 0., 0., 0., 0., 0., 1.,
            ],
            [
                0., 0., -1., 0., 0., -1., 0., 0., -1., 0., 0., 0., 0., 0., 0., 1.,
            ],
            [
                1., 0., 0., 0., 0., 0., -1., 0., 0., 1., 0., 0., 0., 0., 0., 1.,
            ],
            [
                1., 0., 0., 0., 0., 0., 1., 0., 0., -1., 0., 0., 0., 0., 0., 1.,
            ],
            [
                1., 0., 0., 0., 0., -1., 0., 0., 0., 0., -1., 0., 0., 0., 0., 1.,
            ],
            [
                -1., 0., 0., 0., 0., -1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1.,
            ],
        ];
        let mut encoder = r.device.create_command_encoder(&Default::default());
        let mut renders = vec![];
        for (layer, view) in views.iter().enumerate() {
            let render = init(
                &pack(
                    wgsl!("cube_vs"),
                    "renderStruct",
                    &[
                        ("cameraProjectionMatrix", &projection),
                        ("cameraViewMatrix", view),
                    ],
                )?,
                wgpu::BufferUsages::UNIFORM,
            );
            let render_group = bind(
                r,
                pipeline.get_bind_group_layout(0),
                &[(0, render.as_entire_binding())],
            );
            let face = cube.create_view(&wgpu::TextureViewDescriptor {
                dimension: Some(wgpu::TextureViewDimension::D2),
                base_array_layer: layer as u32,
                array_layer_count: Some(1),
                ..Default::default()
            });
            {
                let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("helmet cube"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &face,
                        depth_slice: None,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                    depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                        view: &depth,
                        depth_ops: Some(wgpu::Operations {
                            load: wgpu::LoadOp::Clear(1.),
                            store: wgpu::StoreOp::Discard,
                        }),
                        stencil_ops: None,
                    }),
                    ..Default::default()
                });
                pass.set_pipeline(&pipeline);
                pass.set_bind_group(0, &render_group, &[]);
                pass.set_bind_group(1, &object_group, &[]);
                pass.set_vertex_buffer(0, vertices.slice(..));
                pass.set_index_buffer(indices.slice(..), wgpu::IndexFormat::Uint32);
                pass.draw_indexed(0..index.len() as u32, 0, 0..1);
            }
            renders.push(render);
        }
        r.queue.submit([encoder.finish()]);
        self.cube = Some(cube.create_view(&wgpu::TextureViewDescriptor {
            dimension: Some(wgpu::TextureViewDimension::Cube),
            ..Default::default()
        }));
        Ok(())
    }
    /// PMREMGenerator.fromEquirectangular: the equirect texture into the
    /// cubeUV target, then the GGX levels.
    fn generate_pmrem(&mut self, r: &Renderer, clamp: &wgpu::Sampler) -> Result<()> {
        let pmrem = Pmrem::new(r, clamp, 8, GGX_8)?;
        let equirect = source_pipeline(
            r,
            "helmet PMREM equirect",
            wgsl!("pmrem_equirect_vs"),
            wgsl!("pmrem_equirect_fs"),
        );
        let object = r
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("helmet PMREM"),
                contents: &pack(
                    wgsl!("pmrem_equirect_vs"),
                    "objectStruct",
                    &[("nodeUniform3", &m4(Matrix4::IDENTITY))],
                )?,
                usage: wgpu::BufferUsages::UNIFORM,
            });
        let groups = [
            bind(
                r,
                equirect.get_bind_group_layout(0),
                &[(
                    0,
                    flat_camera(r, wgsl!("pmrem_equirect_vs"))?.as_entire_binding(),
                )],
            ),
            bind(
                r,
                equirect.get_bind_group_layout(1),
                &[
                    (0, wgpu::BindingResource::Sampler(clamp)),
                    (1, wgpu::BindingResource::TextureView(&self.equirect)),
                    (2, object.as_entire_binding()),
                ],
            ),
        ];
        let mut encoder = r.device.create_command_encoder(&Default::default());
        pmrem.encode(&mut encoder, &equirect, [&groups[0], &groups[1]]);
        r.queue.submit([encoder.finish()]);
        self.pmrem = Some(pmrem);
        Ok(())
    }
}
/// The helmet's draws in the current targets: retro, filtered retro and plain.
#[derive(Default)]
struct HelmetDraws {
    retro: Option<(wgpu::RenderPipeline, [wgpu::BindGroup; 2])>,
    filtered: Option<(wgpu::RenderPipeline, [wgpu::BindGroup; 2])>,
    plain: Option<(wgpu::RenderPipeline, [wgpu::BindGroup; 2])>,
}
