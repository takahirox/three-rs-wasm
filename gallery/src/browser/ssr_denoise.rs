//! webgpu_postprocessing_ssr_denoise: the dungeon_warkarma scene ( roughness
//! 0.3 throughout, no normal maps ) under a DirectionalLight with a 4096²
//! shadow ( rendered once, and again when the light moves, as the page's
//! shadow.autoUpdate = false and needsUpdate do ) and the quarry HDR
//! environment ( its r187 PMREM, pmrem.rs ), with stochastic SSRNode
//! reflections reprojected by TemporalReprojectNode and filtered by
//! RecurrentDenoiseNode, whose output feeds back as SSR's history; the
//! reflections are added to the scene color, graded ( AgX, contrast,
//! saturation, gamma ), resolved by TRAA and sharpened. TRAA jitters the
//! camera for the whole pipeline frame by the Halton sequence; the velocity
//! uses the unjittered projection. Each frame, as r187 orders the passes:
//! the shadow map when it needs an update, the scene MRT ( color, packed normal,
//! velocity, diffuse color with metalness ), SSR, its copy, the temporal
//! reprojection ( seeding its history on the first frame ), the denoise, the
//! graded sum, TRAA ( its history copied from the input on the first frame ),
//! the copy the sharpening reads, the sharpening and the output. The
//! histories copy the depth, normal and resolve after their passes, as the
//! nodes do. Frustum culling and the opaque order ( clip-space z of each
//! geometry's bounding sphere center ) are per camera. Every
//! stage runs the WGSL three.js r187 generates for the page ( in
//! `ssr_denoise/`; the shadow modules are the fog_volume ones, TRAA the
//! volume_traa ones and the blit vertex module the volume_caustics one, all
//! byte-identical ). The output modes, the step exponent and the binary
//! refinement ( build-time variants ) are not reproduced.
use super::controls_attributes::{Controls, camera_state};
use super::deferred::render_cube;
use super::gltf_viewer::{fetch, load_asset};
use super::lights_projector::{m3, m4, pack};
use super::pmrem::{Pmrem, from_equirect};
use super::retro::{mipmapped, mipmapped_raw, uniform};
use super::shadowmap_opacity::Mipmaps;
use super::trackball_sprites::parse_rgbe;
use super::wgsl_bind::{Spec, attribute, groups};
use crate::{
    Error, Result, camera::*, geometry::*, math::*, render_target::*, renderer::*, scene::*,
};
use wgpu::util::DeviceExt;

const HALF: wgpu::TextureFormat = wgpu::TextureFormat::Rgba16Float;
const BYTE: wgpu::TextureFormat = wgpu::TextureFormat::Rgba8Unorm;
const DEPTH: wgpu::TextureFormat = wgpu::TextureFormat::Depth24Plus;
/// The light's shadow.mapSize and position ( it shines towards its target
/// at the origin ).
const MAP: u32 = 4096;
const SUN: [f64; 3] = [-10.9, 2.2, 10.75];
/// The shadow camera: OrthographicCamera( -1.75, 1.75, 1.75, -1.75, 0.1, 50 ).
const SHADOW_CAMERA: (f64, f64, f64) = (1.75, 0.1, 50.);
/// The light's shadow.bias.
const SHADOW_BIAS: f64 = -0.0005;
macro_rules! wgsl {
    ($name:literal) => {
        include_str!(concat!("ssr_denoise/", $name, ".wgsl"))
    };
}
const BACKGROUND: (&str, &str) = (wgsl!("background_vs"), wgsl!("background_fs"));
const MODEL: (&str, &str) = (wgsl!("model_vs"), wgsl!("model_fs"));
const SINGLE: (&str, &str) = (wgsl!("single_vs"), wgsl!("single_fs"));
const SHADOW: (&str, &str) = (
    include_str!("fog_volume/shadow_vs.wgsl"),
    include_str!("fog_volume/shadow_fs.wgsl"),
);
const BLIT_VS: &str = include_str!("volume_caustics/high_vs.wgsl");
const SSR: (&str, &str) = (wgsl!("ssr_vs"), wgsl!("ssr_fs"));
const COPY: (&str, &str) = (BLIT_VS, wgsl!("copy_fs"));
const SEED: (&str, &str) = (wgsl!("seed_vs"), wgsl!("seed_fs"));
const REPROJECT: (&str, &str) = (wgsl!("reproject_vs"), wgsl!("reproject_fs"));
const DENOISE: (&str, &str) = (wgsl!("denoise_vs"), wgsl!("denoise_fs"));
const GRADE: (&str, &str) = (wgsl!("grade_vs"), wgsl!("grade_fs"));
const TRAA: (&str, &str) = (
    include_str!("volume_traa/traa_vs.wgsl"),
    include_str!("volume_traa/traa_fs.wgsl"),
);
const BLIT: (&str, &str) = (BLIT_VS, wgsl!("blit_fs"));
const SHARPEN: (&str, &str) = (include_str!("retro/output_vs.wgsl"), wgsl!("sharpen_fs"));
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
/// A dungeon mesh: its interleaved position, normal and uv, index, world
/// matrix, bounding sphere, material and uniforms.
struct Mesh {
    vertex: wgpu::Buffer,
    index: wgpu::Buffer,
    count: u32,
    world: Matrix4,
    center: Vector3,
    radius: f64,
    material: usize,
    double_sided: bool,
    object: wgpu::Buffer,
    shadow_object: wgpu::Buffer,
}
/// A glTF material: its factors and metal-roughness ( and occlusion ) map.
struct Material {
    color: [f64; 3],
    emissive: [f64; 3],
    metalness: f64,
    map: usize,
}
type Draw = (wgpu::RenderPipeline, Vec<wgpu::BindGroup>);
/// A size-dependent texture and its view.
struct Tex {
    texture: wgpu::Texture,
    view: wgpu::TextureView,
}
struct Targets {
    width: u32,
    height: u32,
    format: wgpu::TextureFormat,
    /// Scene MRT: color, normal, velocity, diffuse; depth.
    mrt: [Tex; 4],
    depth: Tex,
    ssr: Tex,
    ssr_copy: Tex,
    reproject: Tex,
    reproject_history: Tex,
    reproject_depth: Tex,
    previous_normal: Tex,
    denoise: Tex,
    graded: Tex,
    traa: Tex,
    traa_history: Tex,
    traa_depth: Tex,
    sharpen_input: Tex,
    sharpened: Tex,
    /// Per mesh: the scene draw.
    model_draws: Vec<Draw>,
    background: Draw,
    ssr_draw: Draw,
    copy: Draw,
    seed: Draw,
    /// The reprojection reading its seeded history or the denoise feedback.
    reproject_draws: [Draw; 2],
    denoise_draw: Draw,
    grade: Draw,
    traa_draw: Draw,
    sharpen_copy: Draw,
    sharpen: Draw,
    output: Draw,
    screen: RenderTarget,
}
/// The previous frame's camera, as each node keeps it.
#[derive(Clone, Copy)]
struct Previous {
    view: Matrix4,
    world: Matrix4,
    projection: Matrix4,
    jittered: Matrix4,
}
pub(super) struct Demo {
    controls: Controls,
    /// The page's GUI values ( see `parameter` ).
    params: [f64; 26],
    pending: bool,
    /// frameId ( SSR's noise index and the denoise's frame count ).
    frame: usize,
    /// Whether the next frame reseeds the histories ( the first frame and
    /// every resize, as the nodes' resized targets do ).
    restart: bool,
    jitter: usize,
    previous: Option<Previous>,
    meshes: Vec<Mesh>,
    materials: Vec<Material>,
    maps: Vec<wgpu::TextureView>,
    sphere: (wgpu::Buffer, wgpu::Buffer, wgpu::Buffer, u32),
    quad: wgpu::Buffer,
    pmrem: Pmrem,
    cube: wgpu::TextureView,
    equirect: wgpu::TextureView,
    shadow_color: wgpu::TextureView,
    shadow_depth: wgpu::TextureView,
    shadow_pipelines: [wgpu::RenderPipeline; 2],
    shadow_groups: Vec<Vec<wgpu::BindGroup>>,
    shadow_render: wgpu::Buffer,
    /// The light position the shadow map was last rendered for.
    shadow_light: Option<[f64; 3]>,
    model_render: wgpu::Buffer,
    single_render: wgpu::Buffer,
    background_render: wgpu::Buffer,
    background_object: wgpu::Buffer,
    ssr_object: wgpu::Buffer,
    seed_object: wgpu::Buffer,
    reproject_object: wgpu::Buffer,
    denoise_object: wgpu::Buffer,
    grade_render: wgpu::Buffer,
    grade_object: wgpu::Buffer,
    traa_object: wgpu::Buffer,
    clamp: wgpu::Sampler,
    repeat: wgpu::Sampler,
    env_sampler: wgpu::Sampler,
    /// The PMREM's LinearMipmapLinearFilter.
    trilinear: wgpu::Sampler,
    compare: wgpu::Sampler,
    targets: Option<Targets>,
}
fn texture(
    r: &Renderer,
    label: &str,
    (width, height): (u32, u32),
    format: wgpu::TextureFormat,
) -> Tex {
    let texture = r.device.create_texture(&wgpu::TextureDescriptor {
        label: Some(label),
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
            | wgpu::TextureUsages::COPY_SRC
            | wgpu::TextureUsages::COPY_DST,
        view_formats: &[],
    });
    let view = texture.create_view(&Default::default());
    Tex { texture, view }
}
fn copy(encoder: &mut wgpu::CommandEncoder, from: &Tex, to: &Tex) {
    let size = from.texture.size();
    encoder.copy_texture_to_texture(
        from.texture.as_image_copy(),
        to.texture.as_image_copy(),
        size,
    );
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
            far: 8.,
            aspect,
            ..Default::default()
        }));
        // The page's initial camera: its position and the controls' target
        // ( OrbitControls with damping, updated each frame ).
        let n = s.get_mut(c)?;
        n.position = Vector3::new(1.259878548682251, 0.5391287340899181, -0.27217301481427114);
        let mut controls =
            Controls::new(Some(0.05), (0., f64::INFINITY), std::f64::consts::PI, true);
        controls.set_target(Vector3::new(
            1.0258536154689288,
            0.2746440590977971,
            -1.0815876858987743,
        ));
        controls.update(s, c)?;
        let init = |label, data: &[u8], usage| {
            r.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some(label),
                    contents: data,
                    usage,
                })
        };
        let vertex_usage = wgpu::BufferUsages::VERTEX;
        let index_usage = wgpu::BufferUsages::INDEX;
        // The dungeon at a tenth of its size.
        let (asset, buffers, images) = load_asset("assets/probes-hdr/dungeon_warkarma.glb").await?;
        let mut mipmaps = Mipmaps::new(r);
        let mut maps = vec![];
        let mut map_index = std::collections::HashMap::new();
        let mut materials = vec![];
        for m in asset.materials() {
            let pbr = m.pbr_metallic_roughness();
            let t = pbr
                .metallic_roughness_texture()
                .ok_or(Error::Invalid("dungeon map"))?
                .texture();
            let image = t.source().index();
            let map = *map_index.entry(image).or_insert_with(|| maps.len());
            if map == maps.len() {
                let img = images.get(image).ok_or(Error::Invalid("dungeon image"))?;
                maps.push(mipmapped(r, &mut mipmaps, img, BYTE));
            }
            let color = pbr.base_color_factor();
            let emissive = m.emissive_factor();
            materials.push(Material {
                color: [color[0] as f64, color[1] as f64, color[2] as f64],
                emissive: emissive.map(f64::from),
                metalness: pbr.metallic_factor() as f64,
                map,
            });
        }
        let mut meshes = vec![];
        let root = Matrix4::from_scale(Vector3::splat(0.1));
        fn walk<'a>(n: gltf::Node<'a>, parent: Matrix4, out: &mut Vec<(gltf::Node<'a>, Matrix4)>) {
            let world = parent
                * Matrix4::from_cols_array_2d(&n.transform().matrix().map(|c| c.map(f64::from)));
            out.push((n.clone(), world));
            for c in n.children() {
                walk(c, world, out);
            }
        }
        let mut nodes = vec![];
        for n in asset
            .default_scene()
            .or_else(|| asset.scenes().next())
            .ok_or(Error::Invalid("dungeon scene"))?
            .nodes()
        {
            walk(n, root, &mut nodes);
        }
        let object = |source| uniform(r, "dungeon object", source, "objectStruct");
        for (node, world) in nodes {
            let Some(mesh) = node.mesh() else {
                continue;
            };
            for primitive in mesh.primitives() {
                let reader = primitive.reader(|b| buffers.get(b.index()).map(Vec::as_slice));
                let positions: Vec<[f32; 3]> = reader
                    .read_positions()
                    .ok_or(Error::Invalid("dungeon positions"))?
                    .collect();
                let normals: Vec<[f32; 3]> = reader
                    .read_normals()
                    .ok_or(Error::Invalid("dungeon normals"))?
                    .collect();
                let uvs: Vec<[f32; 2]> = reader
                    .read_tex_coords(0)
                    .ok_or(Error::Invalid("dungeon uv"))?
                    .into_f32()
                    .collect();
                let index: Vec<u32> = reader
                    .read_indices()
                    .ok_or(Error::Invalid("dungeon index"))?
                    .into_u32()
                    .collect();
                let mut data = Vec::with_capacity(positions.len() * 8);
                for ((p, n), uv) in positions.iter().zip(&normals).zip(&uvs) {
                    data.extend_from_slice(p);
                    data.extend_from_slice(n);
                    data.extend_from_slice(uv);
                }
                // GLTFLoader's computeBounds: the POSITION accessor's min and
                // max ( the page's primitives have no morph targets ), and the
                // sphere around that box.
                let accessor = primitive
                    .get(&gltf::Semantic::Positions)
                    .ok_or(Error::Invalid("dungeon positions"))?;
                let bound = |v: Option<gltf::json::Value>| -> Result<Vector3> {
                    let v = v.ok_or(Error::Invalid("dungeon bounds"))?;
                    let c = |k: usize| {
                        v.get(k)
                            .and_then(|x| x.as_f64())
                            .ok_or(Error::Invalid("dungeon bounds"))
                    };
                    Ok(Vector3::new(c(0)?, c(1)?, c(2)?))
                };
                let (lo, hi) = (bound(accessor.min())?, bound(accessor.max())?);
                let center = (lo + hi) * 0.5;
                let radius = lo.distance(hi) / 2.;
                let material = primitive.material();
                let double_sided = material.double_sided();
                meshes.push(Mesh {
                    vertex: init(
                        "dungeon vertices",
                        bytemuck::cast_slice(&data),
                        vertex_usage,
                    ),
                    count: index.len() as u32,
                    index: init("dungeon index", bytemuck::cast_slice(&index), index_usage),
                    world,
                    center,
                    radius,
                    material: material.index().unwrap_or(0),
                    double_sided,
                    object: object(if double_sided { MODEL.1 } else { SINGLE.1 })?,
                    shadow_object: object(SHADOW.0)?,
                });
            }
        }
        // The quarry HDR ( HDRLoader, mipmapped ): the background cube, the
        // environment's PMREM and SSR's environment.
        let (width, height, texels) =
            parse_rgbe(&fetch("assets/draco-variants/quarry_01_1k.hdr").await?)?;
        let row = width as usize * 4;
        let texels: Vec<u16> = texels.chunks(row).rev().flatten().copied().collect();
        let equirect = mipmapped_raw(
            r,
            &mut mipmaps,
            bytemuck::cast_slice(&texels),
            (width, height),
            8,
            HALF,
        );
        let clamp = r.device.create_sampler(&wgpu::SamplerDescriptor {
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        // scene.environment = hdrTexture: PMREMNode's fromEquirectangular.
        let pmrem = from_equirect(r, &equirect, width)?;
        let cube = render_cube(r, &clamp, &equirect, height)?;
        let sphere = SphereGeometry::build(1., 32, 32)?;
        let read = |name: &str| -> Result<Vec<f32>> {
            match sphere.attributes.get(name) {
                Some(Attribute::F32(a)) => Ok(a.array().to_vec()),
                _ => Err(Error::Invalid("background attribute")),
            }
        };
        let sphere_index = sphere.index.clone().ok_or(Error::Invalid("sphere index"))?;
        let shadow_size = (MAP, MAP);
        let shadow_color = texture(r, "sun shadow color", shadow_size, BYTE).view;
        let shadow_depth = texture(r, "sun shadow depth", shadow_size, DEPTH).view;
        let p0 = [wgpu::VertexAttribute {
            format: wgpu::VertexFormat::Float32x3,
            offset: 0,
            shader_location: 0,
        }];
        let shadow_pipelines = [None, Some(wgpu::Face::Back)].map(|cull| {
            Spec {
                label: "sun shadow",
                shaders: SHADOW,
                buffers: &[attribute(&p0, 32)],
                format: BYTE,
                blend: false,
                depth: Some((DEPTH, wgpu::CompareFunction::LessEqual, true, (0, 0.))),
                cw: cull.is_some(),
                cull,
                samples: 1,
            }
            .build(r)
        });
        let shadow_render = uniform(r, "shadow render", SHADOW.0, "renderStruct")?;
        let mut shadow_groups = vec![];
        for m in &meshes {
            let p = &shadow_pipelines[usize::from(!m.double_sided)];
            shadow_groups.push(groups(
                r,
                |g| p.get_bind_group_layout(g),
                &[SHADOW.0, SHADOW.1],
                |name| match name {
                    "render" => Some(shadow_render.as_entire_binding()),
                    "object" => Some(m.shadow_object.as_entire_binding()),
                    _ => None,
                },
            )?);
        }
        let repeat = r.device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::Repeat,
            address_mode_v: wgpu::AddressMode::Repeat,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        let env_sampler = r.device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::Repeat,
            address_mode_v: wgpu::AddressMode::Repeat,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        let trilinear = r.device.create_sampler(&wgpu::SamplerDescriptor {
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        let compare = r.device.create_sampler(&wgpu::SamplerDescriptor {
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            compare: Some(wgpu::CompareFunction::LessEqual),
            ..Default::default()
        });
        // The page's environmentIntensity is 3.14, not π.
        #[allow(clippy::approx_constant)]
        let params = [
            0., 0.25, 0., 3., 0., 0.4, 1., 0.1, 3.14, 0.3, 1., 0.75, 20., 0.3, 5., 1.5, 0.725, 0.5,
            16., 0.25, 1., 1., SUN[0], SUN[1], SUN[2], 20.,
        ];
        Ok(Self {
            controls,
            params,
            pending: true,
            frame: 0,
            restart: true,
            jitter: 0,
            previous: None,
            meshes,
            materials,
            maps,
            sphere: (
                init(
                    "background normal",
                    bytemuck::cast_slice(&read("normal")?),
                    vertex_usage,
                ),
                init(
                    "background position",
                    bytemuck::cast_slice(&read("position")?),
                    vertex_usage,
                ),
                init(
                    "background index",
                    bytemuck::cast_slice(&sphere_index),
                    index_usage,
                ),
                sphere_index.len() as u32,
            ),
            quad: init(
                "quad uv",
                bytemuck::cast_slice(&[0f32, -1., 0., 1., 2., 1.]),
                vertex_usage,
            ),
            pmrem,
            cube,
            equirect,
            shadow_color,
            shadow_depth,
            shadow_pipelines,
            shadow_groups,
            shadow_render,
            shadow_light: None,
            model_render: uniform(r, "dungeon render", MODEL.1, "renderStruct")?,
            single_render: uniform(r, "dungeon render", SINGLE.1, "renderStruct")?,
            background_render: uniform(r, "background", BACKGROUND.1, "renderStruct")?,
            background_object: uniform(r, "background", BACKGROUND.1, "objectStruct")?,
            ssr_object: uniform(r, "ssr", SSR.1, "objectStruct")?,
            seed_object: uniform(r, "reproject seed", SEED.1, "objectStruct")?,
            reproject_object: uniform(r, "reproject", REPROJECT.1, "objectStruct")?,
            denoise_object: uniform(r, "denoise", DENOISE.1, "objectStruct")?,
            grade_render: uniform(r, "grade", GRADE.1, "renderStruct")?,
            grade_object: uniform(r, "grade", GRADE.1, "objectStruct")?,
            traa_object: uniform(r, "traa", TRAA.1, "objectStruct")?,
            clamp,
            repeat,
            env_sampler,
            trilinear,
            compare,
            targets: None,
        })
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, _dt: f64, animate: bool) -> Result<()> {
        self.pending |= animate;
        Ok(())
    }
    pub fn prepare(&mut self, _s: &mut Scene, _c: Object3D, _r: &Renderer) -> Result<()> {
        Ok(())
    }
    fn resize(&mut self, r: &Renderer, out: &RenderTarget) -> Result<()> {
        let size = (out.width, out.height);
        let t = |label, format| texture(r, label, size, format);
        let mrt = [
            t("scene color", HALF),
            t("scene normal", BYTE),
            t("scene velocity", HALF),
            t("scene diffuse", BYTE),
        ];
        let depth = t("scene depth", DEPTH);
        let ssr = t("ssr", HALF);
        let ssr_copy = t("ssr copy", HALF);
        let reproject = t("reproject", HALF);
        let reproject_history = t("reproject history", HALF);
        let reproject_depth = t("reproject depth", DEPTH);
        let previous_normal = t("previous normal", BYTE);
        let denoise = t("denoise", HALF);
        let graded = t("graded", HALF);
        let traa = t("traa", HALF);
        let traa_history = t("traa history", HALF);
        let traa_depth = t("traa depth", DEPTH);
        let sharpen_input = t("sharpen input", HALF);
        let sharpened = t("sharpened", HALF);
        let tex = wgpu::BindingResource::TextureView;
        let sampler = wgpu::BindingResource::Sampler;
        let uv0 = [wgpu::VertexAttribute {
            format: wgpu::VertexFormat::Float32x2,
            offset: 0,
            shader_location: 0,
        }];
        let quad_layout = [attribute(&uv0, 8)];
        let fullscreen = |label: &'static str, shaders: (&'static str, &'static str), format| {
            Spec {
                label,
                shaders,
                buffers: if shaders.0.contains("@location( 0 ) uv") {
                    &quad_layout
                } else {
                    &[]
                },
                format,
                blend: false,
                depth: None,
                cw: false,
                cull: Some(wgpu::Face::Back),
                samples: 1,
            }
            .build(r)
        };
        let mrt_targets = [HALF, BYTE, HALF, BYTE];
        let mrt_pipeline = |label,
                            shaders: (&'static str, &'static str),
                            layouts: &[wgpu::VertexBufferLayout],
                            depth,
                            cw,
                            cull| {
            let module = |source: &str| {
                r.device.create_shader_module(wgpu::ShaderModuleDescriptor {
                    label: Some(label),
                    source: wgpu::ShaderSource::Wgsl(source.into()),
                })
            };
            r.device
                .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    label: Some(label),
                    layout: None,
                    vertex: wgpu::VertexState {
                        module: &module(shaders.0),
                        entry_point: Some("main"),
                        compilation_options: Default::default(),
                        buffers: layouts,
                    },
                    fragment: Some(wgpu::FragmentState {
                        module: &module(shaders.1),
                        entry_point: Some("main"),
                        compilation_options: Default::default(),
                        targets: &mrt_targets.map(|f| Some(f.into())),
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
                    depth_stencil: Some(depth),
                    multisample: Default::default(),
                    multiview: None,
                    cache: None,
                })
        };
        let less = wgpu::DepthStencilState {
            format: DEPTH,
            depth_write_enabled: true,
            depth_compare: wgpu::CompareFunction::LessEqual,
            stencil: Default::default(),
            bias: Default::default(),
        };
        let interleaved = [
            wgpu::VertexAttribute {
                format: wgpu::VertexFormat::Float32x2,
                offset: 24,
                shader_location: 0,
            },
            wgpu::VertexAttribute {
                format: wgpu::VertexFormat::Float32x3,
                offset: 12,
                shader_location: 1,
            },
            wgpu::VertexAttribute {
                format: wgpu::VertexFormat::Float32x3,
                offset: 0,
                shader_location: 2,
            },
        ];
        let model_layout = [wgpu::VertexBufferLayout {
            array_stride: 32,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &interleaved,
        }];
        let model_pipelines = [
            mrt_pipeline("dungeon", MODEL, &model_layout, less.clone(), false, None),
            mrt_pipeline(
                "dungeon",
                SINGLE,
                &model_layout,
                less.clone(),
                false,
                Some(wgpu::Face::Back),
            ),
        ];
        let mut model_draws = vec![];
        for m in &self.meshes {
            let k = usize::from(!m.double_sided);
            let pipeline = model_pipelines[k].clone();
            let shaders = if k == 0 { MODEL } else { SINGLE };
            let render = if k == 0 {
                &self.model_render
            } else {
                &self.single_render
            };
            let map = &self.maps[self.materials[m.material].map];
            model_draws.push(draw(r, pipeline, shaders, |name| match name {
                "render" => Some(render.as_entire_binding()),
                "object" => Some(m.object.as_entire_binding()),
                "nodeUniform2" => Some(tex(map)),
                "nodeUniform2_sampler" => Some(sampler(&self.repeat)),
                "nodeUniform13" => Some(tex(&r.dfg)),
                "nodeUniform22" => Some(tex(&self.shadow_depth)),
                "nodeUniform22_sampler" => Some(sampler(&self.compare)),
                "nodeUniform26" => Some(tex(&self.pmrem.view)),
                "nodeUniform26_sampler" => Some(sampler(&self.trilinear)),
                _ => Some(sampler(&self.clamp)),
            })?);
        }
        let n0 = [wgpu::VertexAttribute {
            format: wgpu::VertexFormat::Float32x3,
            offset: 0,
            shader_location: 0,
        }];
        let p1 = [wgpu::VertexAttribute {
            format: wgpu::VertexFormat::Float32x3,
            offset: 0,
            shader_location: 1,
        }];
        let background_pipeline = mrt_pipeline(
            "dungeon background",
            BACKGROUND,
            &[attribute(&n0, 12), attribute(&p1, 12)],
            wgpu::DepthStencilState {
                depth_write_enabled: false,
                depth_compare: wgpu::CompareFunction::Always,
                ..less.clone()
            },
            true,
            Some(wgpu::Face::Back),
        );
        let background = draw(r, background_pipeline, BACKGROUND, |name| match name {
            "render" => Some(self.background_render.as_entire_binding()),
            "object" => Some(self.background_object.as_entire_binding()),
            "nodeUniform0" => Some(tex(&self.cube)),
            _ => Some(sampler(&self.clamp)),
        })?;
        let [color, normal, velocity, diffuse] = &mrt;
        let ssr_draw = draw(r, fullscreen("ssr", SSR, HALF), SSR, |name| match name {
            "object" => Some(self.ssr_object.as_entire_binding()),
            "nodeUniform2" => Some(tex(&depth.view)),
            "nodeUniform5" => Some(tex(&normal.view)),
            "nodeUniform7" => Some(tex(&diffuse.view)),
            "nodeUniform14" => Some(tex(&color.view)),
            "nodeUniform15" => Some(tex(&denoise.view)),
            "nodeUniform16" => Some(tex(&velocity.view)),
            "nodeUniform18" => Some(tex(&self.equirect)),
            "nodeUniform18_sampler" => Some(sampler(&self.env_sampler)),
            _ => Some(sampler(&self.clamp)),
        })?;
        let copy_draw = draw(
            r,
            fullscreen("ssr copy", COPY, HALF),
            COPY,
            |name| match name {
                "nodeUniform0" => Some(tex(&ssr.view)),
                _ => Some(sampler(&self.clamp)),
            },
        )?;
        let seed = draw(
            r,
            fullscreen("reproject seed", SEED, HALF),
            SEED,
            |name| match name {
                "object" => Some(self.seed_object.as_entire_binding()),
                "nodeUniform0" => Some(tex(&ssr_copy.view)),
                _ => Some(sampler(&self.clamp)),
            },
        )?;
        let reproject_pipeline = fullscreen("reproject", REPROJECT, HALF);
        let mut reproject_draws = vec![];
        for history in [&reproject_history.view, &denoise.view] {
            reproject_draws.push(draw(
                r,
                reproject_pipeline.clone(),
                REPROJECT,
                |name| match name {
                    "object" => Some(self.reproject_object.as_entire_binding()),
                    "nodeUniform0" => Some(tex(&depth.view)),
                    "nodeUniform1" => Some(tex(&ssr_copy.view)),
                    "nodeUniform3" => Some(tex(&normal.view)),
                    "nodeUniform8" => Some(tex(&velocity.view)),
                    "nodeUniform11" => Some(tex(&reproject_depth.view)),
                    "nodeUniform13" => Some(tex(history)),
                    "nodeUniform15" => Some(tex(&previous_normal.view)),
                    _ => Some(sampler(&self.clamp)),
                },
            )?);
        }
        let [seeded, fed_back] = <[Draw; 2]>::try_from(reproject_draws)
            .map_err(|_| Error::Invalid("reproject draws"))?;
        let denoise_draw = draw(
            r,
            fullscreen("denoise", DENOISE, HALF),
            DENOISE,
            |name| match name {
                "object" => Some(self.denoise_object.as_entire_binding()),
                "nodeUniform0" => Some(tex(&depth.view)),
                "nodeUniform1" => Some(tex(&normal.view)),
                "nodeUniform3" => Some(tex(&reproject.view)),
                "nodeUniform5" => Some(tex(&diffuse.view)),
                "nodeUniform8" => Some(tex(&ssr.view)),
                _ => Some(sampler(&self.clamp)),
            },
        )?;
        let grade = draw(
            r,
            fullscreen("grade", GRADE, HALF),
            GRADE,
            |name| match name {
                "render" => Some(self.grade_render.as_entire_binding()),
                "object" => Some(self.grade_object.as_entire_binding()),
                "nodeUniform0" => Some(tex(&color.view)),
                "nodeUniform1" => Some(tex(&denoise.view)),
                "nodeUniform2" => Some(tex(&ssr.view)),
                _ => Some(sampler(&self.clamp)),
            },
        )?;
        let traa_draw = draw(r, fullscreen("traa", TRAA, HALF), TRAA, |name| match name {
            "object" => Some(self.traa_object.as_entire_binding()),
            "nodeUniform0" => Some(tex(&velocity.view)),
            "nodeUniform1" => Some(tex(&graded.view)),
            "nodeUniform2" => Some(tex(&depth.view)),
            "nodeUniform7" => Some(tex(&traa_depth.view)),
            "nodeUniform9" => Some(tex(&traa_history.view)),
            _ => Some(sampler(&self.clamp)),
        })?;
        let sharpen_copy = draw(
            r,
            fullscreen("sharpen input", BLIT, HALF),
            BLIT,
            |name| match name {
                "nodeUniform0" => Some(tex(&traa.view)),
                _ => Some(sampler(&self.clamp)),
            },
        )?;
        let sharpen = draw(
            r,
            fullscreen("sharpen", SHARPEN, HALF),
            SHARPEN,
            |name| match name {
                "nodeUniform0" => Some(tex(&sharpen_input.view)),
                _ => Some(sampler(&self.clamp)),
            },
        )?;
        let output = draw(
            r,
            fullscreen("output", BLIT, out.options.format),
            BLIT,
            |name| match name {
                "nodeUniform0" => Some(tex(&sharpened.view)),
                _ => Some(sampler(&self.clamp)),
            },
        )?;
        let screen = RenderTarget::with_options(
            &r.device,
            out.width,
            out.height,
            RenderTargetOptions {
                samples: 0,
                depth_buffer: false,
                ..out.options.clone()
            },
        )?;
        self.targets = Some(Targets {
            width: out.width,
            height: out.height,
            format: out.options.format,
            mrt,
            depth,
            ssr,
            ssr_copy,
            reproject,
            reproject_history,
            reproject_depth,
            previous_normal,
            denoise,
            graded,
            traa,
            traa_history,
            traa_depth,
            sharpen_input,
            sharpened,
            model_draws,
            background,
            ssr_draw,
            copy: copy_draw,
            seed,
            reproject_draws: [seeded, fed_back],
            denoise_draw,
            grade,
            traa_draw,
            sharpen_copy,
            sharpen,
            output,
            screen,
        });
        // A new size restarts the histories.
        self.restart = true;
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
        }
        // Only animation frames advance the pipeline ( a resize alone presents
        // the cleared targets until the next one, as the page renders nothing ).
        if !std::mem::take(&mut self.pending) {
            return self.present(r);
        }
        // animate(): controls.update(), then the pipeline.
        self.controls.frame_update(s, c)?;
        s.update()?;
        let t = self
            .targets
            .as_ref()
            .ok_or(Error::Invalid("ssr denoise targets"))?;
        let restart = std::mem::take(&mut self.restart);
        self.frame += 1;
        let (camera, world) = s.camera(c)?;
        let (projection, view) = (camera.projection_matrix()?, world.inverse());
        let (near, far, fov) = match camera {
            Camera::Perspective(p) => (p.near, p.far, p.fov),
            _ => (0.1, 8., 35.),
        };
        let (w, h) = (f64::from(t.width), f64::from(t.height));
        // TRAANode.setViewOffset: the Halton jitter for the whole frame.
        let [jx, jy] = halton(self.jitter);
        let mut jittered = projection;
        jittered.z_axis.x += 2. * (jx - 0.5) / w;
        jittered.z_axis.y -= 2. * (jy - 0.5) / h;
        let previous = self.previous.unwrap_or(Previous {
            view,
            world,
            projection,
            jittered,
        });
        self.previous = Some(Previous {
            view,
            world,
            projection,
            jittered,
        });
        let p = self.params;
        let identity = m4(Matrix4::IDENTITY);
        let id3 = m3(Matrix4::IDENTITY);
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
        // The light's shadow camera looks from the light at its target; the
        // map is rendered again only when the light has moved ( the page's
        // GUI sets shadow.needsUpdate ).
        let light = Vector3::new(p[22], p[23], p[24]);
        let (half, shadow_near, shadow_far) = SHADOW_CAMERA;
        let shadow_projection = Matrix4::from_cols_array(&[
            1. / half,
            0.,
            0.,
            0.,
            0.,
            1. / half,
            0.,
            0.,
            0.,
            0.,
            -1. / (shadow_far - shadow_near),
            0.,
            0.,
            0.,
            -shadow_near / (shadow_far - shadow_near),
            1.,
        ]);
        let shadow_view = Matrix4::look_at_rh(light, Vector3::ZERO, Vector3::Y);
        let bias = Matrix4::from_cols_array(&[
            0.5, 0., 0., 0., 0., 0.5, 0., 0., 0., 0., 1., 0., 0.5, 0.5, 0., 1.,
        ]);
        let shadow_update = self.shadow_light != Some(light.to_array());
        if shadow_update {
            self.shadow_light = Some(light.to_array());
            write(
                &self.shadow_render,
                SHADOW.0,
                "renderStruct",
                &[
                    ("cameraViewMatrix", m4(shadow_view)),
                    ("cameraProjectionMatrix", m4(shadow_projection)),
                ],
            )?;
        }
        // The velocity's current and previous projections and previous view,
        // the light's colour, position and target, and its shadow's matrix,
        // normal bias, bias, radius, map size and intensity.
        let camera_values = vec![
            ("nodeUniform31", m4(projection)),
            ("nodeUniform33", m4(previous.projection)),
            ("nodeUniform34", m4(previous.view)),
            ("cameraProjectionMatrix", m4(jittered)),
            ("cameraViewMatrix", m4(view)),
            ("nodeUniform18", vec![p[25]; 3]),
            ("nodeUniform16", light.to_array().to_vec()),
            ("nodeUniform17", vec![0.; 3]),
            ("nodeUniform19", m4(bias * shadow_projection * shadow_view)),
            ("nodeUniform20", vec![0.]),
            ("nodeUniform21", vec![SHADOW_BIAS]),
            ("nodeUniform23", vec![1.]),
            ("nodeUniform24", vec![f64::from(MAP); 2]),
            ("nodeUniform25", vec![1.]),
            ("cameraWorldMatrix", m4(world)),
        ];
        write(&self.model_render, MODEL.1, "renderStruct", &camera_values)?;
        write(
            &self.single_render,
            SINGLE.1,
            "renderStruct",
            &camera_values,
        )?;
        for m in &self.meshes {
            let material = &self.materials[m.material];
            write(
                &m.object,
                if m.double_sided { MODEL.1 } else { SINGLE.1 },
                "objectStruct",
                &[
                    ("nodeUniform0", material.color.to_vec()),
                    ("nodeUniform1", vec![1.]),
                    ("nodeUniform3", id3.clone()),
                    ("nodeUniform4", vec![1.]),
                    ("nodeUniform5", vec![material.metalness]),
                    ("nodeUniform6", id3.clone()),
                    ("nodeUniform7", vec![p[9]]),
                    ("nodeUniform8", id3.clone()),
                    ("nodeUniform10", m3(m.world.inverse().transpose())),
                    ("nodeUniform11", material.emissive.to_vec()),
                    ("nodeUniform12", vec![1.]),
                    ("nodeUniform14", m4(m.world)),
                    // The environment's rotation, maxLod and intensity, then
                    // the model and previous model matrices.
                    ("nodeUniform27", identity.clone()),
                    ("nodeUniform29", vec![f64::from(self.pmrem.max_lod)]),
                    ("nodeUniform30", vec![1.]),
                    ("nodeUniform32", m4(m.world)),
                    ("nodeUniform35", m4(m.world)),
                ],
            )?;
            write(
                &m.shadow_object,
                SHADOW.0,
                "objectStruct",
                &[("nodeUniform0", vec![1.]), ("nodeUniform3", m4(m.world))],
            )?;
        }
        write(
            &self.background_render,
            BACKGROUND.1,
            "renderStruct",
            &[
                ("nodeUniform4", vec![0.]),
                ("nodeUniform5", vec![1.]),
                ("nodeUniform1", identity.clone()),
                ("nodeUniform8", m4(projection)),
                ("nodeUniform11", m4(previous.projection)),
                ("nodeUniform12", m4(previous.view)),
                ("cameraProjectionMatrix", m4(jittered)),
                ("cameraViewMatrix", m4(view)),
            ],
        )?;
        write(
            &self.background_object,
            BACKGROUND.1,
            "objectStruct",
            &[
                ("nodeUniform3", id3.clone()),
                ("nodeUniform6", vec![1.]),
                ("nodeUniform7", vec![f64::NAN]),
                ("nodeUniform10", identity.clone()),
                ("nodeUniform13", identity.clone()),
                ("nodeUniform14", vec![f64::NAN]),
                ("nodeUniform16", identity.clone()),
            ],
        )?;
        write(
            &self.ssr_object,
            SSR.1,
            "objectStruct",
            &[
                ("nodeUniform0", vec![w, h]),
                ("nodeUniform1", vec![self.frame as f64]),
                ("nodeUniform3", m4(jittered.inverse())),
                ("nodeUniform4", m4(world)),
                ("nodeUniform6", vec![p[2]]),
                ("nodeUniform8", vec![p[5]]),
                ("nodeUniform9", vec![near]),
                ("nodeUniform10", m4(jittered)),
                ("nodeUniform11", vec![p[1]]),
                ("nodeUniform12", vec![far]),
                ("nodeUniform13", vec![p[7]]),
                ("nodeUniform17", vec![0.2]),
                ("nodeUniform19", vec![1.]),
                ("nodeUniform20", vec![p[8]]),
                ("nodeUniform21", vec![35.]),
                ("nodeUniform22", vec![p[6]]),
            ],
        )?;
        write(
            &self.seed_object,
            SEED.1,
            "objectStruct",
            &[("nodeUniform1", vec![w, h])],
        )?;
        write(
            &self.reproject_object,
            REPROJECT.1,
            "objectStruct",
            &[
                ("nodeUniform2", vec![w, h]),
                ("nodeUniform4", vec![p[20]]),
                ("nodeUniform5", m4(view)),
                ("nodeUniform6", m4(jittered.inverse())),
                ("nodeUniform7", m4(world)),
                ("nodeUniform9", m4(previous.world)),
                ("nodeUniform10", m4(previous.jittered.inverse())),
                ("nodeUniform12", id3.clone()),
                ("nodeUniform14", id3.clone()),
                ("nodeUniform16", id3.clone()),
                ("nodeUniform17", m4(previous.view)),
                ("nodeUniform18", world.w_axis.truncate().to_array().to_vec()),
                ("nodeUniform19", m4(previous.jittered)),
                ("nodeUniform20", vec![p[21]]),
                ("nodeUniform21", vec![p[19]]),
                ("nodeUniform22", vec![p[18]]),
            ],
        )?;
        write(
            &self.denoise_object,
            DENOISE.1,
            "objectStruct",
            &[
                ("nodeUniform2", m4(view)),
                ("nodeUniform4", m4(jittered.inverse())),
                ("nodeUniform6", vec![w, h]),
                ("nodeUniform7", vec![self.frame as f64]),
                ("nodeUniform9", vec![1.]),
                ("nodeUniform10", vec![fov.to_radians()]),
                ("nodeUniform11", vec![if p[10] > 0.5 { p[15] } else { 0. }]),
                ("nodeUniform12", vec![p[16]]),
                ("nodeUniform13", vec![p[13]]),
                ("nodeUniform14", vec![p[17]]),
                ("nodeUniform15", m4(jittered)),
                ("nodeUniform16", vec![p[11]]),
                ("nodeUniform17", vec![p[14]]),
                ("nodeUniform18", vec![100.]),
                ("nodeUniform19", vec![p[12]]),
                ("nodeUniform20", vec![1.]),
                ("nodeUniform21", vec![1.]),
            ],
        )?;
        write(
            &self.grade_render,
            GRADE.1,
            "renderStruct",
            &[("nodeUniform3", vec![1.57])],
        )?;
        write(
            &self.grade_object,
            GRADE.1,
            "objectStruct",
            &[
                ("nodeUniform4", vec![1.31]),
                ("nodeUniform5", vec![1.]),
                ("nodeUniform6", vec![0.89]),
            ],
        )?;
        write(
            &self.traa_object,
            TRAA.1,
            "objectStruct",
            &[
                ("nodeUniform3", vec![near, far]),
                ("nodeUniform4", m4(view)),
                ("nodeUniform5", m4(previous.world)),
                ("nodeUniform6", m4(previous.jittered.inverse())),
                ("nodeUniform8", id3.clone()),
                ("nodeUniform10", id3),
                ("nodeUniform11", vec![1.]),
            ],
        )?;
        // Culling and r187's opaque order per camera.
        let order = |pv: Matrix4| -> Vec<usize> {
            let frustum = Frustum::from_projection(pv);
            let mut items: Vec<(f64, usize)> = self
                .meshes
                .iter()
                .enumerate()
                .filter(|(_, m)| {
                    let (scale, _, _) = m.world.to_scale_rotation_translation();
                    frustum.intersects_sphere(Sphere {
                        center: m.world.transform_point3(m.center),
                        radius: m.radius * scale.abs().max_element(),
                    })
                })
                .map(|(k, m)| ((pv * m.world * m.center.extend(1.)).z, k))
                .collect();
            items.sort_by(|a, b| a.0.total_cmp(&b.0).then(a.1.cmp(&b.1)));
            items.into_iter().map(|(_, k)| k).collect()
        };
        let mut encoder = r.device.create_command_encoder(&Default::default());
        // The shadow map, when it needs an update.
        if shadow_update {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("light shadow"),
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
            for i in order(shadow_projection * shadow_view) {
                let m = &self.meshes[i];
                pass.set_pipeline(&self.shadow_pipelines[usize::from(!m.double_sided)]);
                for (g, group) in self.shadow_groups[i].iter().enumerate() {
                    pass.set_bind_group(g as u32, group, &[]);
                }
                pass.set_vertex_buffer(0, m.vertex.slice(..));
                pass.set_index_buffer(m.index.slice(..), wgpu::IndexFormat::Uint32);
                pass.draw_indexed(0..m.count, 0, 0..1);
            }
        }
        {
            // The scene MRT: the background, then the dungeon.
            let attachments: Vec<Option<wgpu::RenderPassColorAttachment>> = t
                .mrt
                .iter()
                .map(|target| {
                    Some(wgpu::RenderPassColorAttachment {
                        view: &target.view,
                        depth_slice: None,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                            store: wgpu::StoreOp::Store,
                        },
                    })
                })
                .collect();
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("dungeon scene"),
                color_attachments: &attachments,
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &t.depth.view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                ..Default::default()
            });
            set(&mut pass, &t.background);
            pass.set_vertex_buffer(0, self.sphere.0.slice(..));
            pass.set_vertex_buffer(1, self.sphere.1.slice(..));
            pass.set_index_buffer(self.sphere.2.slice(..), wgpu::IndexFormat::Uint32);
            pass.draw_indexed(0..self.sphere.3, 0, 0..1);
            for i in order(jittered * view) {
                let m = &self.meshes[i];
                set(&mut pass, &t.model_draws[i]);
                pass.set_vertex_buffer(0, m.vertex.slice(..));
                pass.set_index_buffer(m.index.slice(..), wgpu::IndexFormat::Uint32);
                pass.draw_indexed(0..m.count, 0, 0..1);
            }
        }
        let quad = &self.quad;
        let full = |encoder: &mut wgpu::CommandEncoder,
                    target: &wgpu::TextureView,
                    d: &Draw,
                    clear: wgpu::Color| {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("ssr denoise pass"),
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
            });
            set(&mut pass, d);
            pass.set_vertex_buffer(0, quad.slice(..));
            pass.draw(0..3, 0..1);
        };
        full(
            &mut encoder,
            &t.ssr.view,
            &t.ssr_draw,
            wgpu::Color::TRANSPARENT,
        );
        full(&mut encoder, &t.ssr_copy.view, &t.copy, wgpu::Color::BLACK);
        if restart {
            // TemporalReprojectNode's restart: its history seeded from the input.
            full(
                &mut encoder,
                &t.reproject_history.view,
                &t.seed,
                wgpu::Color::BLACK,
            );
        }
        full(
            &mut encoder,
            &t.reproject.view,
            &t.reproject_draws[usize::from(!restart)],
            wgpu::Color::BLACK,
        );
        copy(&mut encoder, &t.depth, &t.reproject_depth);
        copy(&mut encoder, &t.mrt[1], &t.previous_normal);
        full(
            &mut encoder,
            &t.denoise.view,
            &t.denoise_draw,
            wgpu::Color::BLACK,
        );
        full(&mut encoder, &t.graded.view, &t.grade, wgpu::Color::BLACK);
        if restart {
            // TRAANode's restart: the history from the beauty; the previous
            // depth is the resized ( cleared ) history depth.
            copy(&mut encoder, &t.graded, &t.traa_history);
        }
        full(&mut encoder, &t.traa.view, &t.traa_draw, wgpu::Color::BLACK);
        copy(&mut encoder, &t.traa, &t.traa_history);
        copy(&mut encoder, &t.depth, &t.traa_depth);
        full(
            &mut encoder,
            &t.sharpen_input.view,
            &t.sharpen_copy,
            wgpu::Color::BLACK,
        );
        full(
            &mut encoder,
            &t.sharpened.view,
            &t.sharpen,
            wgpu::Color::BLACK,
        );
        r.queue.submit([encoder.finish()]);
        // TRAANode.clearViewOffset: the next Halton offset.
        self.jitter = (self.jitter + 1) % 32;
        self.present(r)
    }
    fn present(&self, r: &Renderer) -> Result<bool> {
        let t = self
            .targets
            .as_ref()
            .ok_or(Error::Invalid("ssr denoise targets"))?;
        let mut encoder = r.device.create_command_encoder(&Default::default());
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("ssr denoise output"),
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
            pass.set_vertex_buffer(0, self.quad.slice(..));
            pass.draw(0..3, 0..1);
        }
        r.queue.submit([encoder.finish()]);
        Ok(true)
    }
    pub fn output(&self) -> Option<&RenderTarget> {
        self.targets.as_ref().map(|t| &t.screen)
    }
    /// The target `output` returns, to render into the canvas ( see
    /// RenderTarget::alias_color ).
    pub fn output_mut(&mut self) -> Option<&mut RenderTarget> {
        self.targets.as_mut().map(|t| &mut t.screen)
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
    /// The GUI in the page's order: output, quality, mirror bias, step
    /// exponent, binary refine, max distance, intensity, thickness, env
    /// intensity, roughness; denoise enabled, luma, depth, normal and ray
    /// length phis, radius, strength, adapt; the reprojection's max frames,
    /// clamp intensity, flicker suppression, hit point reprojection; the
    /// light's x, y, z and intensity. The output modes, step exponent and
    /// binary refinement rebuild shaders and are not reproduced ( the
    /// gallery leaves them out ).
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        *self
            .params
            .get_mut(index)
            .ok_or(Error::Invalid("ssr denoise parameter"))? = f64::from(value);
        Ok(())
    }
    pub fn seek(&mut self, _t: f64) {
        self.pending = true;
    }
}
/// Bind by name: a pass's resources in its generated slots.
fn draw<'a>(
    r: &Renderer,
    pipeline: wgpu::RenderPipeline,
    shaders: (&str, &str),
    resolve: impl Fn(&str) -> Option<wgpu::BindingResource<'a>>,
) -> Result<Draw> {
    let g = groups(
        r,
        |i| pipeline.get_bind_group_layout(i),
        &[shaders.0, shaders.1],
        resolve,
    )?;
    Ok((pipeline, g))
}
fn set(pass: &mut wgpu::RenderPass, (pipeline, groups): &Draw) {
    pass.set_pipeline(pipeline);
    for (i, g) in groups.iter().enumerate() {
        pass.set_bind_group(i as u32, g, &[]);
    }
}

#[allow(unused_variables)]
impl super::example::Example for Demo {
    fn on_update(
        &mut self,
        scene: &mut crate::scene::Scene,
        camera: crate::scene::Object3D,
        delta: f64,
        animate: bool,
    ) -> Option<crate::Result<()>> {
        Some(self.update(scene, camera, delta, animate))
    }
    fn on_render(
        &mut self,
        renderer: &crate::renderer::Renderer,
        scene: &mut crate::scene::Scene,
        camera: crate::scene::Object3D,
        target: &crate::renderer::RenderTarget,
    ) -> Option<crate::Result<bool>> {
        Some(self.render(renderer, scene, camera, target))
    }
    fn on_output_target(&self) -> Option<Option<&crate::renderer::RenderTarget>> {
        if let Some(target) = self.output() {
            return Some(Some(target));
        }
        None
    }
    fn on_output_target_mut(&mut self) -> Option<Option<&mut crate::renderer::RenderTarget>> {
        Some(self.output_mut())
    }
    fn on_prepare(
        &mut self,
        renderer: &crate::renderer::Renderer,
        scene: &mut crate::scene::Scene,
        camera: crate::scene::Object3D,
        aspect: f64,
    ) -> crate::Result<()> {
        self.prepare(scene, camera, renderer)?;
        Ok(())
    }
    fn on_tsl_parameter(&mut self, index: usize, value: f32) -> Option<crate::Result<()>> {
        Some(self.parameter(index, value))
    }
    fn on_key(&mut self, code: u32, down: bool) {
        self.key(code, down);
    }
    fn on_draw(&mut self, kind: u32, x: f64, y: f64) -> Option<crate::Result<()>> {
        self.draw(kind, x, y);
        Some(Ok(()))
    }
    fn on_seek(&mut self, seconds: f64) {
        self.seek(seconds);
    }
    fn on_input(
        &mut self,
        scene: &mut crate::scene::Scene,
        camera: crate::scene::Object3D,
        dx: f64,
        dy: f64,
        wheel: f64,
        pan: bool,
        height: f64,
    ) -> Option<crate::Result<()>> {
        Some(self.input(scene, camera, dx, dy, wheel, pan, height))
    }
}

/// The program of three.js examples/webgpu_postprocessing_ssr_denoise.html: its renderer, its scene
/// and its frame loop.
#[wasm_bindgen::prelude::wasm_bindgen]
pub async fn webgpu_postprocessing_ssr_denoise(
    canvas: web_sys::HtmlCanvasElement,
    animate: bool,
) -> std::result::Result<super::BrowserApp, wasm_bindgen::JsValue> {
    super::program(
        canvas,
        animate,
        super::PageOptions {
            encode_srgb: true,
            format: super::TargetFormat::Canvas,
            raw_output: true,
            ..super::PageOptions::GALLERY
        },
        async |page| {
            let (scene, camera, renderer, _aspect) = page.gallery_scene()?;
            Ok(super::expanded::module(
                super::gltf_viewer::OrbitViewer::from_camera(crate::math::Vector3::ZERO, 1.),
                0.1,
                8.,
                Box::new(Demo::create(scene, camera, 417, renderer).await?),
            ))
        },
    )
    .await
}
