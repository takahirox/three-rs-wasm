//! webgpu_postprocessing_ssr: the steampunk_camera.glb model over a metallic
//! floor, lit by the RoomEnvironment PMREM, with SSRNode's screen-space
//! reflections added to the beauty pass and SMAA on the result. The scene
//! pass writes color, packed view normals and metalness / roughness; the SSR
//! pass marches each reflective pixel's ray through the depth; a blur chain
//! of five mips widens it by roughness; the composite adds it to the color;
//! SMAA's edge, weight and blend passes antialias it; the output applies
//! ACES. Without SSR the output reads the scene color. Every stage runs the
//! WGSL three.js r186 generates for the page (in `ssr/`; the quad vertex
//! modules are the hdr and volume_traa ones, byte-identical). The blur
//! quality and binary refinement are build-time constants: their variants
//! are the page's shaders for each value.
use super::controls_attributes::{Controls, camera_state};
use super::deferred::{Draw, sampled_pipeline, set};
use super::gltf_viewer::{fetch, load_asset};
use super::lights_projector::{m3, m4, pack};
use super::pmrem_cube_uv::bind;
use super::retro::{mipmapped, uniform};
use super::shadowmap_opacity::Mipmaps;
use crate::{
    Error, Result, camera::*, geometry::*, math::*, render_target::*, renderer::*, scene::*,
};
use std::f64::consts::PI;
use wgpu::util::DeviceExt;

const HALF: wgpu::TextureFormat = wgpu::TextureFormat::Rgba16Float;
const BYTE: wgpu::TextureFormat = wgpu::TextureFormat::Rgba8Unorm;
const DEPTH: wgpu::TextureFormat = wgpu::TextureFormat::Depth24Plus;
/// The blur chain's mip count.
const MIPS: u32 = 5;
macro_rules! wgsl {
    ($name:literal) => {
        include_str!(concat!("ssr/", $name, ".wgsl"))
    };
}
const MIP_VS: &str = include_str!("hdr/output_vs.wgsl");
const QUAD_VS: &str = include_str!("volume_traa/output_vs.wgsl");
/// The model's materials share one module; the lens casing's is transparent.
const MODEL: (&str, &str) = (wgsl!("model_vs"), wgsl!("model_fs"));
const LENS: (&str, &str) = (wgsl!("model_vs"), wgsl!("lens_fs"));
const FLOOR: (&str, &str) = (wgsl!("floor_vs"), wgsl!("floor_fs"));
const BACKGROUND: (&str, &str) = (wgsl!("background_vs"), wgsl!("background_fs"));
/// The blur kernel's loop bounds as r186 inlines blurQuality.
const BLUR_LOOP: [&str; 2] = [
    "for ( var i : i32 = i32( ( - 1.0 ) ); i <= 1; i ++ )",
    "for ( var j : i32 = i32( ( - 1.0 ) ); j <= 1; j ++ )",
];
fn texture(
    r: &Renderer,
    size: (u32, u32),
    format: wgpu::TextureFormat,
    mips: u32,
) -> wgpu::Texture {
    r.device.create_texture(&wgpu::TextureDescriptor {
        label: Some("ssr target"),
        size: wgpu::Extent3d {
            width: size.0,
            height: size.1,
            depth_or_array_layers: 1,
        },
        mip_level_count: mips,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT
            | wgpu::TextureUsages::TEXTURE_BINDING
            | wgpu::TextureUsages::COPY_SRC,
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
const OPAQUE_BLACK: wgpu::Color = wgpu::Color {
    r: 0.,
    g: 0.,
    b: 0.,
    a: 1.,
};
/// One of the model's three meshes: the interleaved normal, position,
/// tangent and uv ( 48-byte stride ), the 16-bit index, the local bounding
/// sphere and its material's maps ( color, normal, metalness / roughness /
/// occlusion ) and object struct.
struct Part {
    vertices: wgpu::Buffer,
    index: wgpu::Buffer,
    count: u32,
    sphere: Sphere,
    maps: [wgpu::TextureView; 3],
    object: wgpu::Buffer,
    transparent: bool,
}
struct Targets {
    width: u32,
    height: u32,
    format: wgpu::TextureFormat,
    scene: wgpu::TextureView,
    normal: wgpu::TextureView,
    metal_rough: wgpu::TextureView,
    depth: wgpu::TextureView,
    ssr: wgpu::TextureView,
    /// The blur chain's mip views.
    mips: Vec<wgpu::TextureView>,
    composite: wgpu::TextureView,
    edges: wgpu::TextureView,
    weights: wgpu::TextureView,
    blended: wgpu::TextureView,
    screen: RenderTarget,
    background: Draw,
    parts: Vec<Draw>,
    floor: Draw,
    /// Without and with binary refinement.
    ssr_draws: [Draw; 2],
    blur: Draw,
    /// Per blur quality, the draws of mips 1 to 4.
    mip_draws: [Vec<Draw>; 3],
    composite_draw: Draw,
    smaa: [Draw; 3],
    /// The output of SMAA's blend, and of the scene color without SSR.
    outputs: [Draw; 2],
}
pub(super) struct Demo {
    controls: Controls,
    /// quality, blur quality, max distance, intensity, thickness, binary
    /// refine, enabled, model roughness.
    params: [f64; 8],
    pending: bool,
    parts: Vec<Part>,
    floor: (wgpu::Buffer, wgpu::Buffer, wgpu::Buffer, u32),
    floor_object: wgpu::Buffer,
    background: (wgpu::Buffer, wgpu::Buffer, wgpu::Buffer, u32),
    background_render: wgpu::Buffer,
    background_object: wgpu::Buffer,
    pmrem: wgpu::TextureView,
    smaa_textures: [wgpu::TextureView; 2],
    model_render: wgpu::Buffer,
    floor_render: wgpu::Buffer,
    ssr_object: wgpu::Buffer,
    blur_object: wgpu::Buffer,
    mip_objects: Vec<wgpu::Buffer>,
    edges_object: wgpu::Buffer,
    weights_object: wgpu::Buffer,
    blend_object: wgpu::Buffer,
    output_render: wgpu::Buffer,
    quad_uv: wgpu::Buffer,
    clamp: wgpu::Sampler,
    mip_linear: wgpu::Sampler,
    repeat: wgpu::Sampler,
    ssr_pipelines: [wgpu::RenderPipeline; 2],
    mip_pipelines: [wgpu::RenderPipeline; 3],
    targets: Option<Targets>,
}
fn bounds(points: impl Iterator<Item = Vector3>) -> Sphere {
    let points: Vec<Vector3> = points.collect();
    let (lo, hi) = points.iter().fold(
        (
            Vector3::splat(f64::INFINITY),
            Vector3::splat(f64::NEG_INFINITY),
        ),
        |(lo, hi), p| (lo.min(*p), hi.max(*p)),
    );
    let center = (lo + hi) * 0.5;
    Sphere {
        center,
        radius: points.iter().map(|p| p.distance(center)).fold(0., f64::max),
    }
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, _id: u32, r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 35.,
            near: 0.1,
            far: 50.,
            aspect,
            ..Default::default()
        }));
        s.get_mut(c)?.position = Vector3::new(3., 2., 3.);
        let mut controls = Controls::new(Some(0.05), (0., f64::INFINITY), PI, true);
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
        let index_usage = wgpu::BufferUsages::INDEX;
        let (asset, buffers, images) =
            load_asset("/web/gallery/assets/gltf/steampunk_camera.glb").await?;
        let mut mipmaps = Mipmaps::new(r);
        let image = |i: usize| images.get(i).ok_or(Error::Invalid("steampunk image"));
        let texture_image = |t: gltf::Texture| -> usize {
            // EXT_texture_webp sources replace the ( absent ) core source.
            t.extension_value("EXT_texture_webp")
                .and_then(|e| e.get("source"))
                .and_then(|s| s.as_u64())
                .map_or(t.source().index(), |s| s as usize)
        };
        let mut parts = vec![];
        for mesh in asset.meshes() {
            let primitive = mesh
                .primitives()
                .next()
                .ok_or(Error::Invalid("steampunk primitive"))?;
            let reader = primitive.reader(|b| buffers.get(b.index()).map(Vec::as_slice));
            let normals: Vec<[f32; 3]> = reader
                .read_normals()
                .ok_or(Error::Invalid("steampunk normals"))?
                .collect();
            let positions: Vec<[f32; 3]> = reader
                .read_positions()
                .ok_or(Error::Invalid("steampunk positions"))?
                .collect();
            let tangents: Vec<[f32; 4]> = reader
                .read_tangents()
                .ok_or(Error::Invalid("steampunk tangents"))?
                .collect();
            let uv: Vec<[f32; 2]> = reader
                .read_tex_coords(0)
                .ok_or(Error::Invalid("steampunk uv"))?
                .into_f32()
                .collect();
            let mut interleaved = Vec::with_capacity(positions.len() * 12);
            for i in 0..positions.len() {
                interleaved.extend_from_slice(&normals[i]);
                interleaved.extend_from_slice(&positions[i]);
                interleaved.extend_from_slice(&tangents[i]);
                interleaved.extend_from_slice(&uv[i]);
            }
            let mut index: Vec<u16> = reader
                .read_indices()
                .ok_or(Error::Invalid("steampunk index"))?
                .into_u32()
                .map(|i| i as u16)
                .collect();
            let count = index.len() as u32;
            if index.len() % 2 == 1 {
                index.push(0);
            }
            let material = primitive.material();
            let pbr = material.pbr_metallic_roughness();
            let map = |t: Option<gltf::Texture>| -> Result<usize> {
                t.map(texture_image)
                    .ok_or(Error::Invalid("steampunk material map"))
            };
            let base = map(pbr.base_color_texture().map(|t| t.texture()))?;
            let normal = map(material.normal_texture().map(|t| t.texture()))?;
            let orm = map(pbr.metallic_roughness_texture().map(|t| t.texture()))?;
            parts.push(Part {
                vertices: init("steampunk", bytemuck::cast_slice(&interleaved), vertex),
                index: init("steampunk index", bytemuck::cast_slice(&index), index_usage),
                count,
                sphere: bounds(
                    positions
                        .iter()
                        .map(|p| Vector3::new(p[0] as f64, p[1] as f64, p[2] as f64)),
                ),
                // The shader's bindings: the color map, the metalness /
                // roughness / occlusion map and the normal map.
                maps: [
                    mipmapped(
                        r,
                        &mut mipmaps,
                        image(base)?,
                        wgpu::TextureFormat::Rgba8UnormSrgb,
                    ),
                    mipmapped(r, &mut mipmaps, image(orm)?, BYTE),
                    mipmapped(r, &mut mipmaps, image(normal)?, BYTE),
                ],
                object: uniform(r, "ssr model object", MODEL.1, "objectStruct")?,
                transparent: material.name() == Some("Lense_Casing"),
            });
        }
        let geometry_buffers =
            |g: &BufferGeometry, label: &'static str, names: &[&str]| -> Result<_> {
                let read = |name: &str| -> Result<Vec<f32>> {
                    let a = g
                        .attributes
                        .get(name)
                        .ok_or(Error::Invalid("ssr attribute"))?;
                    (0..a.count())
                        .flat_map(|i| (0..3).map(move |k| (i, k)))
                        .map(|(i, k)| a.get_component(i, k).map(|v| v as f32))
                        .collect()
                };
                let index = g.index.clone().ok_or(Error::Invalid("ssr index"))?;
                Ok((
                    init(label, bytemuck::cast_slice(&read(names[0])?), vertex),
                    init(label, bytemuck::cast_slice(&read(names[1])?), vertex),
                    init(label, bytemuck::cast_slice(&index), index_usage),
                    index.len() as u32,
                ))
            };
        // CircleGeometry( 2, 64 ) and the background's SphereGeometry( 1, 32,
        // 32 ): normal and position.
        let floor = geometry_buffers(
            &CircleGeometry::build(2., 64, 0., 2. * PI)?,
            "ssr floor",
            &["normal", "position"],
        )?;
        let background = geometry_buffers(
            &SphereGeometry::build(1., 32, 32)?,
            "ssr background",
            &["normal", "position"],
        )?;
        // RoomEnvironment through PMREMGenerator.fromScene( sigma 0.04 ):
        // the resident cube-UV atlas ( lodMax 8, 768 × 1024 ).
        let room = super::room_environment::environment(r)?;
        let pmrem = room
            .gpu
            .as_ref()
            .ok_or(Error::Invalid("room environment"))?
            .view
            .clone();
        let mut smaa = vec![];
        for name in ["smaa-area.png", "smaa-search.png"] {
            let image = crate::material::Texture::from_image(
                &fetch(&format!("/web/gallery/assets/tsl-next/{name}")).await?,
            )?;
            let t = r.device.create_texture_with_data(
                &r.queue,
                &wgpu::TextureDescriptor {
                    label: Some(name),
                    size: wgpu::Extent3d {
                        width: image.width,
                        height: image.height,
                        depth_or_array_layers: 1,
                    },
                    mip_level_count: 1,
                    sample_count: 1,
                    dimension: wgpu::TextureDimension::D2,
                    format: BYTE,
                    usage: wgpu::TextureUsages::TEXTURE_BINDING,
                    view_formats: &[],
                },
                wgpu::util::TextureDataOrder::LayerMajor,
                &image.rgba,
            );
            smaa.push(view(&t));
        }
        let [area, search]: [wgpu::TextureView; 2] =
            smaa.try_into().map_err(|_| Error::Invalid("smaa"))?;
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
        let (ssr_pipelines, mip_pipelines) = Self::variants(r);
        Ok(Self {
            controls,
            params: [0.5, 1., 1., 1., 0.03, 0., 1., 1.],
            pending: true,
            parts,
            floor,
            floor_object: uniform(r, "ssr floor object", FLOOR.1, "objectStruct")?,
            background,
            background_render: uniform(r, "ssr background", BACKGROUND.0, "renderStruct")?,
            background_object: uniform(r, "ssr background", BACKGROUND.0, "objectStruct")?,
            pmrem,
            smaa_textures: [area, search],
            model_render: uniform(r, "ssr model render", MODEL.1, "renderStruct")?,
            floor_render: uniform(r, "ssr floor render", FLOOR.1, "renderStruct")?,
            ssr_object: uniform(r, "ssr pass", wgsl!("ssr_fs"), "objectStruct")?,
            blur_object: uniform(r, "ssr blur", wgsl!("blur_fs"), "objectStruct")?,
            mip_objects: (1..MIPS)
                .map(|_| uniform(r, "ssr mip", wgsl!("mip_fs"), "objectStruct"))
                .collect::<Result<_>>()?,
            edges_object: uniform(r, "ssr edges", wgsl!("edges_vs"), "objectStruct")?,
            weights_object: uniform(r, "ssr weights", wgsl!("weights_fs"), "objectStruct")?,
            blend_object: uniform(r, "ssr blend", wgsl!("blend_fs"), "objectStruct")?,
            output_render: uniform(r, "ssr output", wgsl!("output_fs"), "renderStruct")?,
            quad_uv: init(
                "ssr quad",
                bytemuck::cast_slice(&[0f32, -1., 0., 1., 2., 1.]),
                vertex,
            ),
            clamp: sampler(wgpu::AddressMode::ClampToEdge, wgpu::FilterMode::Nearest),
            mip_linear: sampler(wgpu::AddressMode::ClampToEdge, wgpu::FilterMode::Linear),
            repeat: sampler(wgpu::AddressMode::Repeat, wgpu::FilterMode::Linear),
            ssr_pipelines,
            mip_pipelines,
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
    fn quad() -> [wgpu::VertexBufferLayout<'static>; 1] {
        const ATTRS: [wgpu::VertexAttribute; 1] = wgpu::vertex_attr_array![0 => Float32x2];
        [wgpu::VertexBufferLayout {
            array_stride: 8,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &ATTRS,
        }]
    }
    fn screen_pipeline(
        r: &Renderer,
        label: &str,
        shaders: (&str, &str),
        format: wgpu::TextureFormat,
    ) -> wgpu::RenderPipeline {
        sampled_pipeline(
            r,
            label,
            shaders,
            &Self::quad(),
            &[format],
            None,
            (false, false),
            (1, wgpu::PrimitiveTopology::TriangleList),
        )
    }
    /// The SSR pipelines without and with binary refinement and the mip
    /// blur pipelines for blur qualities 1 to 3: build-time constants the
    /// page's shaders inline.
    fn variants(r: &Renderer) -> ([wgpu::RenderPipeline; 2], [wgpu::RenderPipeline; 3]) {
        let ssr = [wgsl!("ssr_fs"), wgsl!("ssr_refine_fs")]
            .map(|fs| Self::screen_pipeline(r, "ssr", (wgsl!("ssr_vs"), fs), HALF));
        let mips = [1, 2, 3].map(|q| {
            let fs = wgsl!("mip_fs")
                .replace(
                    BLUR_LOOP[0],
                    &format!("for ( var i : i32 = i32( ( - {q}.0 ) ); i <= {q}; i ++ )"),
                )
                .replace(
                    BLUR_LOOP[1],
                    &format!("for ( var j : i32 = i32( ( - {q}.0 ) ); j <= {q}; j ++ )"),
                );
            Self::screen_pipeline(r, "ssr mip", (MIP_VS, &fs), HALF)
        });
        (ssr, mips)
    }
    fn resize(&mut self, r: &Renderer, out: &RenderTarget) -> Result<()> {
        let (width, height) = (out.width, out.height);
        let size = (width, height);
        let make = |format| view(&texture(r, size, format, 1));
        let scene = make(HALF);
        let normal = make(BYTE);
        let metal_rough = make(BYTE);
        let depth = make(DEPTH);
        let ssr = make(HALF);
        let chain = texture(r, size, HALF, MIPS);
        let mips = (0..MIPS)
            .map(|level| {
                chain.create_view(&wgpu::TextureViewDescriptor {
                    base_mip_level: level,
                    mip_level_count: Some(1),
                    ..Default::default()
                })
            })
            .collect();
        let chain_view = view(&chain);
        let composite = make(HALF);
        let edges = make(HALF);
        let weights = make(HALF);
        let blended = make(HALF);
        let tex = wgpu::BindingResource::TextureView;
        let sampler = wgpu::BindingResource::Sampler;
        let mrt = [HALF, BYTE, BYTE];
        let triangles = (1, wgpu::PrimitiveTopology::TriangleList);
        let two = [
            wgpu::vertex_attr_array![0 => Float32x3],
            wgpu::vertex_attr_array![1 => Float32x3],
        ];
        let two_layouts = [0, 1].map(|i| wgpu::VertexBufferLayout {
            array_stride: 12,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &two[i],
        });
        let model_attrs = wgpu::vertex_attr_array![
            1 => Float32x3,
            3 => Float32x3,
            2 => Float32x4,
            0 => Float32x2
        ];
        let model_layout = [wgpu::VertexBufferLayout {
            array_stride: 48,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &model_attrs,
        }];
        // The background mesh: back faces of the sphere ( clockwise front
        // faces ), no depth write, always passing.
        let background_pipeline = sampled_pipeline(
            r,
            "ssr background",
            BACKGROUND,
            &two_layouts,
            &mrt,
            Some((wgpu::CompareFunction::Always, false)),
            (true, false),
            triangles,
        );
        let background = (
            background_pipeline.clone(),
            vec![
                bind(
                    r,
                    background_pipeline.get_bind_group_layout(0),
                    &[(0, self.background_render.as_entire_binding())],
                ),
                bind(
                    r,
                    background_pipeline.get_bind_group_layout(1),
                    &[(0, self.background_object.as_entire_binding())],
                ),
            ],
        );
        let lit = |shaders, layouts: &[wgpu::VertexBufferLayout], blended| {
            sampled_pipeline(
                r,
                "ssr lit",
                shaders,
                layouts,
                &mrt,
                Some((wgpu::CompareFunction::LessEqual, true)),
                (false, blended),
                triangles,
            )
        };
        let opaque = lit(MODEL, &model_layout, false);
        let lens = lit(LENS, &model_layout, true);
        let parts = self
            .parts
            .iter()
            .map(|p| {
                let pipeline = if p.transparent { &lens } else { &opaque };
                (
                    pipeline.clone(),
                    vec![
                        bind(
                            r,
                            pipeline.get_bind_group_layout(0),
                            &[(0, self.model_render.as_entire_binding())],
                        ),
                        bind(
                            r,
                            pipeline.get_bind_group_layout(1),
                            &[
                                (0, p.object.as_entire_binding()),
                                (1, sampler(&self.repeat)),
                                (2, tex(&p.maps[0])),
                                (3, sampler(&self.repeat)),
                                (4, tex(&p.maps[1])),
                                (5, sampler(&self.clamp)),
                                (6, tex(&r.dfg)),
                                (7, sampler(&self.repeat)),
                                (8, tex(&p.maps[2])),
                                (9, sampler(&self.clamp)),
                                (10, tex(&self.pmrem)),
                            ],
                        ),
                    ],
                )
            })
            .collect();
        let floor_pipeline = lit(FLOOR, &two_layouts, false);
        let floor = (
            floor_pipeline.clone(),
            vec![
                bind(
                    r,
                    floor_pipeline.get_bind_group_layout(0),
                    &[(0, self.floor_render.as_entire_binding())],
                ),
                bind(
                    r,
                    floor_pipeline.get_bind_group_layout(1),
                    &[
                        (0, self.floor_object.as_entire_binding()),
                        (1, sampler(&self.clamp)),
                        (2, tex(&r.dfg)),
                        (3, sampler(&self.clamp)),
                        (4, tex(&self.pmrem)),
                    ],
                ),
            ],
        );
        let single = |pipeline: &wgpu::RenderPipeline, entries: &[(u32, wgpu::BindingResource)]| {
            (
                pipeline.clone(),
                vec![bind(r, pipeline.get_bind_group_layout(0), entries)],
            )
        };
        let ssr_entries = [
            (0, tex(&depth)),
            (1, self.ssr_object.as_entire_binding()),
            (2, sampler(&self.clamp)),
            (3, tex(&normal)),
            (4, sampler(&self.clamp)),
            (5, tex(&metal_rough)),
            (6, sampler(&self.clamp)),
            (7, tex(&scene)),
        ];
        let ssr_draws = self
            .ssr_pipelines
            .each_ref()
            .map(|p| single(p, &ssr_entries));
        let blur_pipeline =
            Self::screen_pipeline(r, "ssr blur", (wgsl!("blur_vs"), wgsl!("blur_fs")), HALF);
        let blur = single(
            &blur_pipeline,
            &[
                (0, sampler(&self.clamp)),
                (1, tex(&ssr)),
                (2, self.blur_object.as_entire_binding()),
            ],
        );
        let mip_draws = self.mip_pipelines.each_ref().map(|p| {
            self.mip_objects
                .iter()
                .map(|object| {
                    single(
                        p,
                        &[
                            (0, sampler(&self.clamp)),
                            (1, tex(&ssr)),
                            (2, object.as_entire_binding()),
                        ],
                    )
                })
                .collect()
        });
        let composite_pipeline =
            Self::screen_pipeline(r, "ssr composite", (QUAD_VS, wgsl!("composite_fs")), HALF);
        let composite_draw = single(
            &composite_pipeline,
            &[
                (0, sampler(&self.clamp)),
                (1, tex(&scene)),
                (2, sampler(&self.mip_linear)),
                (3, tex(&chain_view)),
                (4, sampler(&self.clamp)),
                (5, tex(&metal_rough)),
            ],
        );
        let edges_pipeline = Self::screen_pipeline(
            r,
            "ssr smaa edges",
            (wgsl!("edges_vs"), wgsl!("edges_fs")),
            HALF,
        );
        let weights_pipeline = Self::screen_pipeline(
            r,
            "ssr smaa weights",
            (wgsl!("weights_vs"), wgsl!("weights_fs")),
            HALF,
        );
        let blend_pipeline = Self::screen_pipeline(
            r,
            "ssr smaa blend",
            (wgsl!("blend_vs"), wgsl!("blend_fs")),
            HALF,
        );
        let [area, search] = &self.smaa_textures;
        let smaa = [
            single(
                &edges_pipeline,
                &[
                    (0, sampler(&self.clamp)),
                    (1, tex(&composite)),
                    (2, self.edges_object.as_entire_binding()),
                ],
            ),
            single(
                &weights_pipeline,
                &[
                    (0, sampler(&self.clamp)),
                    (1, tex(&edges)),
                    (2, self.weights_object.as_entire_binding()),
                    (3, tex(search)),
                    (4, sampler(&self.clamp)),
                    (5, tex(area)),
                ],
            ),
            single(
                &blend_pipeline,
                &[
                    (0, sampler(&self.clamp)),
                    (1, tex(&weights)),
                    (2, self.blend_object.as_entire_binding()),
                    (3, sampler(&self.clamp)),
                    (4, tex(&composite)),
                ],
            ),
        ];
        let format = out.options.format;
        let output_pipeline =
            Self::screen_pipeline(r, "ssr output", (QUAD_VS, wgsl!("output_fs")), format);
        let output = |source: &wgpu::TextureView| {
            (
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
                            (1, wgpu::BindingResource::TextureView(source)),
                        ],
                    ),
                ],
            )
        };
        let outputs = [output(&blended), output(&scene)];
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
            normal,
            metal_rough,
            depth,
            ssr,
            mips,
            composite,
            edges,
            weights,
            blended,
            screen,
            background,
            parts,
            floor,
            ssr_draws,
            blur,
            mip_draws,
            composite_draw,
            smaa,
            outputs,
        });
        Ok(())
    }
    fn present(&self, encoder: &mut wgpu::CommandEncoder, t: &Targets) {
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("ssr output"),
            color_attachments: &[color(&t.screen.view, wgpu::Color::BLACK)],
            ..Default::default()
        });
        set(&mut pass, &t.outputs[usize::from(self.params[6] < 0.5)]);
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
        }
        let t = self.targets.as_ref().ok_or(Error::Invalid("ssr targets"))?;
        if !std::mem::take(&mut self.pending) {
            let mut encoder = r.device.create_command_encoder(&Default::default());
            self.present(&mut encoder, t);
            r.queue.submit([encoder.finish()]);
            return Ok(true);
        }
        // animate(): the damped controls advance once per requested frame.
        self.controls.frame_update(s, c)?;
        s.update()?;
        let (camera, world) = s.camera(c)?;
        let (projection, view_matrix) = (camera.projection_matrix()?, world.inverse());
        let [
            quality,
            _,
            max_distance,
            intensity,
            thickness,
            _,
            enabled,
            roughness,
        ] = self.params;
        let size = vec![t.width as f64, t.height as f64];
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
        let identity3 = m3(Matrix4::IDENTITY);
        let camera_values = [
            ("cameraProjectionMatrix", m4(projection)),
            ("cameraViewMatrix", m4(view_matrix)),
            ("cameraWorldMatrix", m4(world)),
        ];
        write(
            &self.background_render,
            BACKGROUND.0,
            "renderStruct",
            &[
                ("nodeUniform1", vec![1.]),
                camera_values[0].clone(),
                camera_values[1].clone(),
                ("nodeUniform0", size.clone()),
            ],
        )?;
        write(
            &self.background_object,
            BACKGROUND.0,
            "objectStruct",
            &[
                ("nodeUniform2", vec![1.]),
                ("nodeUniform4", identity3.clone()),
                ("nodeUniform6", m4(Matrix4::IDENTITY)),
            ],
        )?;
        write(&self.model_render, MODEL.1, "renderStruct", &camera_values)?;
        write(&self.floor_render, FLOOR.1, "renderStruct", &camera_values)?;
        // The PMREM's lodMax and texel size and the environment's rotation
        // and intensity.
        let environment = |values: &mut Vec<(&str, Vec<f64>)>, names: [&'static str; 4]| {
            values.extend([
                (names[0], vec![8.]),
                (names[1], vec![1. / 768.]),
                (names[2], vec![1. / 1024.]),
                (names[3], vec![1.25]),
            ]);
        };
        let model = Matrix4::from_translation(Vector3::new(0., 0.1, 0.));
        for p in &self.parts {
            let mut values = vec![
                ("nodeUniform0", vec![1.; 3]),
                ("nodeUniform2", identity3.clone()),
                ("nodeUniform3", vec![1.]),
                ("nodeUniform5", identity3.clone()),
                ("nodeUniform6", vec![1.]),
                ("nodeUniform7", vec![1.]),
                ("nodeUniform8", identity3.clone()),
                ("nodeUniform9", vec![roughness]),
                ("nodeUniform10", identity3.clone()),
                ("nodeUniform12", identity3.clone()),
                ("nodeUniform13", vec![0.; 3]),
                ("nodeUniform14", vec![1.]),
                ("nodeUniform16", m4(model)),
                ("nodeUniform18", identity3.clone()),
                ("nodeUniform19", vec![1., 1.]),
                ("nodeUniform21", m4(Matrix4::IDENTITY)),
            ];
            environment(
                &mut values,
                [
                    "nodeUniform20",
                    "nodeUniform23",
                    "nodeUniform24",
                    "nodeUniform26",
                ],
            );
            write(&p.object, MODEL.1, "objectStruct", &values)?;
        }
        let floor_model = Matrix4::from_translation(Vector3::new(0., -0.8, 0.))
            * Matrix4::from_rotation_x(-PI / 2.);
        let mut floor_values = vec![
            ("nodeUniform0", vec![1.; 3]),
            ("nodeUniform1", vec![1.]),
            ("nodeUniform2", vec![1.]),
            ("nodeUniform3", vec![0.5]),
            ("nodeUniform5", m3(floor_model.inverse().transpose())),
            ("nodeUniform6", vec![0.; 3]),
            ("nodeUniform7", vec![1.]),
            ("nodeUniform9", m4(floor_model)),
            ("nodeUniform11", m4(Matrix4::IDENTITY)),
        ];
        environment(
            &mut floor_values,
            [
                "nodeUniform10",
                "nodeUniform13",
                "nodeUniform14",
                "nodeUniform16",
            ],
        );
        write(&self.floor_object, FLOOR.1, "objectStruct", &floor_values)?;
        let texel = vec![1. / t.width as f64, 1. / t.height as f64];
        if enabled > 0.5 {
            write(
                &self.ssr_object,
                wgsl!("ssr_fs"),
                "objectStruct",
                &[
                    ("nodeUniform1", m4(projection.inverse())),
                    ("nodeUniform2", m4(world)),
                    ("nodeUniform5", vec![max_distance]),
                    ("nodeUniform6", vec![0.1]),
                    ("nodeUniform7", size.clone()),
                    ("nodeUniform8", m4(projection)),
                    ("nodeUniform9", vec![quality]),
                    ("nodeUniform10", vec![50.]),
                    ("nodeUniform11", vec![thickness]),
                    ("nodeUniform13", vec![10.]),
                    ("nodeUniform14", vec![intensity]),
                ],
            )?;
            write(
                &self.blur_object,
                wgsl!("blur_fs"),
                "objectStruct",
                &[("nodeUniform1", identity3.clone())],
            )?;
            for (i, b) in self.mip_objects.iter().enumerate() {
                write(
                    b,
                    wgsl!("mip_fs"),
                    "objectStruct",
                    &[
                        ("nodeUniform1", identity3.clone()),
                        ("nodeUniform2", vec![(i + 1) as f64]),
                    ],
                )?;
            }
            write(
                &self.edges_object,
                wgsl!("edges_vs"),
                "objectStruct",
                &[("nodeUniform1", texel.clone())],
            )?;
            let mut weights = vec![
                ("nodeUniform2", texel.clone()),
                ("nodeUniform4", texel.clone()),
            ];
            for n in [1, 3, 6, 7, 8, 9, 10, 12, 13, 14, 15, 16, 17, 18, 19] {
                weights.push((NAMES[n], identity3.clone()));
            }
            write(
                &self.weights_object,
                wgsl!("weights_fs"),
                "objectStruct",
                &weights,
            )?;
            write(
                &self.blend_object,
                wgsl!("blend_fs"),
                "objectStruct",
                &[
                    ("nodeUniform1", identity3.clone()),
                    ("nodeUniform2", identity3.clone()),
                    ("nodeUniform3", texel.clone()),
                    ("nodeUniform4", identity3.clone()),
                    ("nodeUniform6", texel.clone()),
                ],
            )?;
        }
        write(
            &self.output_render,
            wgsl!("output_fs"),
            "renderStruct",
            &[("nodeUniform1", vec![1.])],
        )?;
        let mut encoder = r.device.create_command_encoder(&Default::default());
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("ssr scene"),
                color_attachments: &[
                    color(&t.scene, OPAQUE_BLACK),
                    color(&t.normal, OPAQUE_BLACK),
                    color(&t.metal_rough, OPAQUE_BLACK),
                ],
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
            pass.set_vertex_buffer(0, self.background.0.slice(..));
            pass.set_vertex_buffer(1, self.background.1.slice(..));
            pass.set_index_buffer(self.background.2.slice(..), wgpu::IndexFormat::Uint32);
            pass.draw_indexed(0..self.background.3, 0, 0..1);
            // The opaque meshes front to back by their bounding sphere
            // centers' clip depth, then the transparent lens casing.
            let view_projection = projection * view_matrix;
            let frustum = Frustum::from_projection(view_projection);
            let depth_of = |center: Vector3| view_projection.project_point3(center).z;
            let floor_sphere = Sphere {
                center: Vector3::new(0., -0.8, 0.),
                radius: 2.,
            };
            let mut opaque: Vec<(f64, Option<usize>)> = vec![];
            let mut transparent = vec![];
            for (i, p) in self.parts.iter().enumerate() {
                let center = model.transform_point3(p.sphere.center);
                if !frustum.intersects_sphere(Sphere {
                    center,
                    radius: p.sphere.radius,
                }) {
                    continue;
                }
                if p.transparent {
                    transparent.push((depth_of(center), i));
                } else {
                    opaque.push((depth_of(center), Some(i)));
                }
            }
            if frustum.intersects_sphere(floor_sphere) {
                opaque.push((depth_of(floor_sphere.center), None));
            }
            opaque.sort_by(|a, b| a.0.total_cmp(&b.0));
            transparent.sort_by(|a, b| b.0.total_cmp(&a.0));
            let order = opaque
                .into_iter()
                .map(|(_, i)| i)
                .chain(transparent.into_iter().map(|(_, i)| Some(i)));
            for item in order {
                match item {
                    Some(i) => {
                        let p = &self.parts[i];
                        set(&mut pass, &t.parts[i]);
                        pass.set_vertex_buffer(0, p.vertices.slice(..));
                        pass.set_index_buffer(p.index.slice(..), wgpu::IndexFormat::Uint16);
                        pass.draw_indexed(0..p.count, 0, 0..1);
                    }
                    None => {
                        set(&mut pass, &t.floor);
                        pass.set_vertex_buffer(0, self.floor.0.slice(..));
                        pass.set_vertex_buffer(1, self.floor.1.slice(..));
                        pass.set_index_buffer(self.floor.2.slice(..), wgpu::IndexFormat::Uint32);
                        pass.draw_indexed(0..self.floor.3, 0, 0..1);
                    }
                }
            }
        }
        let quad = |encoder: &mut wgpu::CommandEncoder,
                    target: &wgpu::TextureView,
                    clear: wgpu::Color,
                    draw: &Draw| {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("ssr quad"),
                color_attachments: &[color(target, clear)],
                ..Default::default()
            });
            set(&mut pass, draw);
            pass.set_vertex_buffer(0, self.quad_uv.slice(..));
            pass.draw(0..3, 0..1);
        };
        if enabled > 0.5 {
            let refine = usize::from(self.params[5] > 0.5);
            let blur_quality = (self.params[1].round() as usize).clamp(1, 3) - 1;
            quad(
                &mut encoder,
                &t.ssr,
                wgpu::Color::TRANSPARENT,
                &t.ssr_draws[refine],
            );
            quad(&mut encoder, &t.mips[0], wgpu::Color::TRANSPARENT, &t.blur);
            for (i, draw) in t.mip_draws[blur_quality].iter().enumerate() {
                quad(&mut encoder, &t.mips[i + 1], wgpu::Color::TRANSPARENT, draw);
            }
            quad(&mut encoder, &t.composite, OPAQUE_BLACK, &t.composite_draw);
            quad(&mut encoder, &t.edges, OPAQUE_BLACK, &t.smaa[0]);
            quad(&mut encoder, &t.weights, OPAQUE_BLACK, &t.smaa[1]);
            quad(&mut encoder, &t.blended, OPAQUE_BLACK, &t.smaa[2]);
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
        Ok(())
    }
    /// quality, blur quality, max distance, intensity, thickness, binary
    /// refine, enabled and the model roughness.
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        *self
            .params
            .get_mut(index)
            .ok_or(Error::Invalid("ssr parameter"))? = value as f64;
        Ok(())
    }
    pub fn seek(&mut self, _t: f64) {
        self.pending = true;
    }
}
const NAMES: [&str; 20] = [
    "nodeUniform0",
    "nodeUniform1",
    "nodeUniform2",
    "nodeUniform3",
    "nodeUniform4",
    "nodeUniform5",
    "nodeUniform6",
    "nodeUniform7",
    "nodeUniform8",
    "nodeUniform9",
    "nodeUniform10",
    "nodeUniform11",
    "nodeUniform12",
    "nodeUniform13",
    "nodeUniform14",
    "nodeUniform15",
    "nodeUniform16",
    "nodeUniform17",
    "nodeUniform18",
    "nodeUniform19",
];
