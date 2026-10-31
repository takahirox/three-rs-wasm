//! webgpu_lightprobes_sponza ( three.js r187 ): Sponza, fetched at run time
//! from glTF-Sample-Assets as the page does ( its license does not grant
//! redistribution ), lit by a shadowed SunLight ( two cascades ) under
//! SkyMesh with ACES, with FirstPersonControls and diffuse GI from a
//! LightProbeGrid baked 16 probes per frame in two passes. The page draws
//! through MeshletDrawingPipeline ( `meshlet.rs` ): the meshes are processed
//! into clusters one per projection, then the main view and the shadow
//! cascades cull them on the GPU and draw each material's visible clusters
//! with one drawIndirect of pulled vertices; the bake's light shadow too.
//! The double-sided, alpha-tested materials, the sky, the probe helper and
//! the probes' ArrayCamera captures stay with the regular pipeline, as the
//! pipeline leaves them. The GUI's Pipeline switches to the default drawing
//! pipeline ( every mesh drawn directly, culled on the CPU ).
//!
//! The bake follows r187's LightProbeGrid ( lightprobes.rs ): per batch of 8
//! probes one render of the 48 90° sub-cameras into the face atlas, the
//! solid-angle SH projection into the batch rows and the repack of the probe
//! range into the padded RGBA16F atlas; the shadow-casting SunLight is
//! replaced by a directional bake light whose 2048² shadow covers the
//! casters, rendered once per bake call, and the direct pass adds the unlit
//! bounce grid. Every stage runs the WGSL three.js r187 generates for the
//! page ( `lightprobes_sponza/`, whose uniforms and bindings
//! `lightprobes_sponza/shaders.rs` lists; the SH, repack and helper passes
//! are lightprobes_complex's and the output pass cubemap_dynamic's, which
//! are the same ).
use super::gltf_viewer::{fetch, load_asset_bytes};
use super::lights_projector::{m3, m4, pack};
use super::meshlet::{self, BatchDraw, GeometrySource, MeshletObject, MeshletPipeline, View};
use super::meshopt::Meshopt;
use super::trackball_sprites::FirstPerson;
use crate::{
    Error, Result, camera::*, geometry::*, material::Material as EngineMaterial, math::*,
    render_target::*, renderer::*, scene::*,
};
use std::collections::HashMap;
use std::f64::consts::PI;
use std::sync::Arc;
use wgpu::util::DeviceExt;

#[path = "lightprobes_sponza/shaders.rs"]
mod shaders;

const MODEL_INDEX_URL: &str = "https://raw.githubusercontent.com/KhronosGroup/glTF-Sample-Assets/main/Models/model-index.json";
const SAMPLE_ASSETS_BASE_URL: &str =
    "https://raw.githubusercontent.com/KhronosGroup/glTF-Sample-Assets/main/Models/";
/// three's bundled meshoptimizer modules ( examples/jsm/libs/ ), which
/// MeshletGeometryUtils loads.
const MESHOPT_SIMPLIFIER: &str = "assets/meshopt/meshopt_simplifier.module.js";
const MESHOPT_CLUSTERIZER: &str = "assets/meshopt/meshopt_clusterizer.module.js";

const HALF: wgpu::TextureFormat = wgpu::TextureFormat::Rgba16Float;
const FLOAT: wgpu::TextureFormat = wgpu::TextureFormat::Rgba32Float;
const BYTE: wgpu::TextureFormat = wgpu::TextureFormat::Rgba8Unorm;
const DEPTH: wgpu::TextureFormat = wgpu::TextureFormat::Depth24Plus;
/// The SunLight's shadow map size; its two cascades side by side.
const SHADOW: u32 = 2048;
/// The bake's cubemapSize and near.
const CUBE: u32 = 32;
const CUBE_NEAR: f64 = 0.05;
/// LightProbeGrid's probes per capture render, and the SH columns per probe
/// ( 9 coefficients × 6 faces × 4 row blocks ).
const BATCH: u32 = 8;
const COLUMNS: u32 = 9 * 6 * 4;
/// The probes the page bakes per frame.
const PROBES_PER_FRAME: u32 = 16;
/// The render lists of the meshlet pipeline: the camera, the two cascades
/// and the bake light.
const MAIN: u32 = 0;
const CASCADES: [u32; 2] = [1, 2];
const BAKE: u32 = 3;
/// LightProbeGrid's capture cameras per face: forward and up.
const CAPTURE_FACES: [([f64; 3], [f64; 3]); 6] = [
    ([1., 0., 0.], [0., 1., 0.]),
    ([-1., 0., 0.], [0., 1., 0.]),
    ([0., 1., 0.], [0., 0., -1.]),
    ([0., -1., 0.], [0., 0., 1.]),
    ([0., 0., 1.], [0., 1., 0.]),
    ([0., 0., -1.], [0., 1., 0.]),
];

macro_rules! shared {
    ($name:literal) => {
        include_str!(concat!("lightprobes_complex/", $name, ".wgsl"))
    };
}
const PROBE_VS: &str = shared!("probe_vs");
const PROBE_SH_FS: &str = shared!("probe_sh_fs");
const REPACK_FS: [&str; 7] = [
    shared!("probe_repack_0_fs"),
    shared!("probe_repack_1_fs"),
    shared!("probe_repack_2_fs"),
    shared!("probe_repack_3_fs"),
    shared!("probe_repack_4_fs"),
    shared!("probe_repack_5_fs"),
    shared!("probe_repack_6_fs"),
];
const HELPER_VS: &str = shared!("helper_vs");
const HELPER_FS: &str = shared!("helper_fs");
const OUTPUT_VS: &str = include_str!("cubemap_dynamic/output_vs.wgsl");
const OUTPUT_FS: &str = include_str!("cubemap_dynamic/output_fs.wgsl");

/// A captured pipeline: its WGSL, its uniforms by meaning and its bindings.
pub(super) struct Shader {
    vs: &'static str,
    fs: &'static str,
    names: &'static [(&'static str, u32)],
    bindings: &'static [(u32, u32, B)],
}
/// The material textures a binding reads.
#[derive(Clone, Copy, PartialEq, Debug)]
pub(super) enum Tex {
    Map,
    MetalRough,
    Normal,
    Dfg,
}
/// What a captured pipeline binds.
#[derive(Clone, Copy, PartialEq, Debug)]
pub(super) enum B {
    Render,
    Object,
    Sampler(Tex),
    Texture(Tex),
    ShadowSampler,
    Shadow,
    GridSampler,
    Grid,
    Views,
    Projections,
    Positions,
    CameraIndex,
    Vertices,
    Triangles,
    Queue,
    World,
}

/// Sponza's materials: MeshStandardMaterial with base color, metal-rough and
/// normal maps ( Std ), with only a base color map and no tangents ( Base ),
/// or double-sided and alpha-tested ( Mask, which MeshletDrawingPipeline
/// leaves to the regular pipeline ).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Kind {
    Std,
    Base,
    Mask,
}
impl Kind {
    fn index(self) -> usize {
        self as usize
    }
}
/// A main view build: GI ( the grid visible ) and shadows.
fn main_shader(kind: Kind, pulled: bool, gi: bool, shadows: bool) -> &'static Shader {
    use shaders::*;
    match (kind, pulled, gi, shadows) {
        (Kind::Std, false, true, true) => &STD_DIRECT_GI_SH,
        (Kind::Std, false, false, true) => &STD_DIRECT_SH,
        (Kind::Std, false, true, false) => &STD_DIRECT_GI,
        (Kind::Std, false, false, false) => &STD_DIRECT,
        (Kind::Mask, _, true, true) => &MASK_DIRECT_GI_SH,
        (Kind::Mask, _, false, true) => &MASK_DIRECT_SH,
        (Kind::Mask, _, true, false) => &MASK_DIRECT_GI,
        (Kind::Mask, _, false, false) => &MASK_DIRECT,
        (Kind::Base, false, true, true) => &BASE_DIRECT_GI_SH,
        (Kind::Base, false, false, true) => &BASE_DIRECT_SH,
        (Kind::Base, false, true, false) => &BASE_DIRECT_GI,
        (Kind::Base, false, false, false) => &BASE_DIRECT,
        (Kind::Std, true, true, true) => &STD_PULLED_GI_SH,
        (Kind::Std, true, false, true) => &STD_PULLED_SH,
        (Kind::Std, true, true, false) => &STD_PULLED_GI,
        (Kind::Std, true, false, false) => &STD_PULLED,
        (Kind::Base, true, true, true) => &BASE_PULLED_GI_SH,
        (Kind::Base, true, false, true) => &BASE_PULLED_SH,
        (Kind::Base, true, true, false) => &BASE_PULLED_GI,
        (Kind::Base, true, false, false) => &BASE_PULLED,
    }
}
/// A capture build ( the bounce grid always lights it ).
fn capture_shader(kind: Kind, shadows: bool) -> &'static Shader {
    use shaders::*;
    match (kind, shadows) {
        (Kind::Std, true) => &STD_CAPTURE_SH,
        (Kind::Base, true) => &BASE_CAPTURE_SH,
        (Kind::Mask, true) => &MASK_CAPTURE_SH,
        (Kind::Std, false) => &STD_CAPTURE,
        (Kind::Base, false) => &BASE_CAPTURE,
        (Kind::Mask, false) => &MASK_CAPTURE,
    }
}
/// The variants' indices: the main view's GI × shadows builds.
fn variant(gi: bool, shadows: bool) -> usize {
    usize::from(gi) * 2 + usize::from(shadows)
}

/// Values by meaning, written into a build's struct by its uniform names.
type Values = Vec<(&'static str, Vec<f64>)>;
fn write(
    r: &Renderer,
    buffer: &wgpu::Buffer,
    shader: &Shader,
    name: &str,
    values: &[(&str, Vec<f64>)],
) -> Result<()> {
    let source = if shader.fs.contains(&format!("struct {name} {{")) {
        shader.fs
    } else {
        shader.vs
    };
    let names: Vec<(String, &[f64])> = values
        .iter()
        .filter_map(|(meaning, v)| {
            if meaning.starts_with("camera") {
                return Some((meaning.to_string(), &v[..]));
            }
            shader
                .names
                .iter()
                .find(|(m, _)| m == meaning)
                .map(|(_, n)| (format!("nodeUniform{n}"), &v[..]))
        })
        .collect();
    let refs: Vec<(&str, &[f64])> = names.iter().map(|(n, v)| (n.as_str(), *v)).collect();
    r.queue.write_buffer(buffer, 0, &pack(source, name, &refs)?);
    Ok(())
}
fn struct_buffer(r: &Renderer, label: &str, shader: &Shader, name: &str) -> Result<wgpu::Buffer> {
    let source = if shader.fs.contains(&format!("struct {name} {{")) {
        shader.fs
    } else {
        shader.vs
    };
    Ok(r.device.create_buffer(&wgpu::BufferDescriptor {
        label: Some(label),
        size: pack(source, name, &[])?.len() as u64,
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    }))
}
fn plain_buffer(r: &Renderer, label: &str, size: u64) -> wgpu::Buffer {
    r.device.create_buffer(&wgpu::BufferDescriptor {
        label: Some(label),
        size,
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    })
}
fn texture(
    r: &Renderer,
    size: (u32, u32, u32),
    dimension: wgpu::TextureDimension,
    format: wgpu::TextureFormat,
    samples: u32,
) -> wgpu::Texture {
    r.device.create_texture(&wgpu::TextureDescriptor {
        label: Some("lightprobes_sponza target"),
        size: wgpu::Extent3d {
            width: size.0,
            height: size.1,
            depth_or_array_layers: size.2,
        },
        mip_level_count: 1,
        sample_count: samples,
        dimension,
        format,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT
            | if samples == 1 {
                wgpu::TextureUsages::TEXTURE_BINDING
                    | wgpu::TextureUsages::COPY_SRC
                    | wgpu::TextureUsages::COPY_DST
            } else {
                wgpu::TextureUsages::empty()
            },
        view_formats: &[],
    })
}
fn view(t: &wgpu::Texture) -> wgpu::TextureView {
    t.create_view(&Default::default())
}
/// The vertex layouts of a build: uv, normal, tangent ( Std and Mask ) and
/// position; the sky's and the shadows' own.
#[derive(Clone, Copy, PartialEq)]
enum Layout {
    Lit(Kind),
    Pulled,
    Sky,
    Shadow,
    Quad,
    Helper,
}
fn vertex_layouts(layout: Layout) -> Vec<(u64, wgpu::VertexStepMode, Vec<wgpu::VertexAttribute>)> {
    let a = |format, location| wgpu::VertexAttribute {
        format,
        offset: 0,
        shader_location: location,
    };
    let v = wgpu::VertexStepMode::Vertex;
    use wgpu::VertexFormat::{Float32x2, Float32x3, Float32x4};
    match layout {
        Layout::Lit(Kind::Base) => vec![
            (8, v, vec![a(Float32x2, 0)]),
            (12, v, vec![a(Float32x3, 1)]),
            (12, v, vec![a(Float32x3, 2)]),
        ],
        Layout::Lit(_) => vec![
            (8, v, vec![a(Float32x2, 0)]),
            (12, v, vec![a(Float32x3, 1)]),
            (16, v, vec![a(Float32x4, 2)]),
            (12, v, vec![a(Float32x3, 3)]),
        ],
        Layout::Pulled => vec![],
        Layout::Sky | Layout::Quad => vec![(12, v, vec![a(Float32x3, 0)])],
        Layout::Shadow => vec![
            (8, v, vec![a(Float32x2, 0)]),
            (12, v, vec![a(Float32x3, 1)]),
        ],
        Layout::Helper => {
            let i = wgpu::VertexStepMode::Instance;
            vec![
                (12, v, vec![a(Float32x3, 0)]),
                (12, v, vec![a(Float32x3, 1)]),
                (12, i, vec![a(Float32x3, 2)]),
                (
                    64,
                    i,
                    (3..7)
                        .map(|k| wgpu::VertexAttribute {
                            format: Float32x4,
                            offset: 16 * (k - 3) as u64,
                            shader_location: k,
                        })
                        .collect(),
                ),
            ]
        }
    }
}
/// A render pipeline of captured WGSL.
#[allow(clippy::too_many_arguments)]
fn pipeline(
    r: &Renderer,
    label: &str,
    (vs, fs): (&str, &str),
    layout: Layout,
    color: Option<wgpu::TextureFormat>,
    samples: u32,
    cull: Option<wgpu::Face>,
    cw: bool,
    depth: Option<bool>,
) -> wgpu::RenderPipeline {
    let module = |source: &str| {
        r.device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some(label),
            source: wgpu::ShaderSource::Wgsl(source.into()),
        })
    };
    let layouts = vertex_layouts(layout);
    let buffers: Vec<wgpu::VertexBufferLayout> = layouts
        .iter()
        .map(|(stride, step_mode, attributes)| wgpu::VertexBufferLayout {
            array_stride: *stride,
            step_mode: *step_mode,
            attributes,
        })
        .collect();
    let targets = [color.map(|format| wgpu::ColorTargetState {
        format,
        blend: None,
        write_mask: wgpu::ColorWrites::ALL,
    })];
    r.device
        .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some(label),
            layout: None,
            vertex: wgpu::VertexState {
                module: &module(vs),
                entry_point: Some("main"),
                compilation_options: Default::default(),
                buffers: &buffers,
            },
            fragment: Some(wgpu::FragmentState {
                module: &module(fs),
                entry_point: Some("main"),
                compilation_options: Default::default(),
                targets: &targets,
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                front_face: if cw {
                    wgpu::FrontFace::Cw
                } else {
                    wgpu::FrontFace::Ccw
                },
                cull_mode: cull,
                ..Default::default()
            },
            depth_stencil: depth.map(|write| wgpu::DepthStencilState {
                format: DEPTH,
                depth_write_enabled: write,
                depth_compare: wgpu::CompareFunction::LessEqual,
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
}
/// A lit build's pipeline: FrontSide culls back faces, the double-sided
/// alpha-tested materials none.
fn lit_pipeline(
    r: &Renderer,
    shader: &Shader,
    kind: Kind,
    pulled: bool,
    format: wgpu::TextureFormat,
    samples: u32,
) -> wgpu::RenderPipeline {
    pipeline(
        r,
        "lightprobes_sponza lit",
        (shader.vs, shader.fs),
        if pulled {
            Layout::Pulled
        } else {
            Layout::Lit(kind)
        },
        Some(format),
        samples,
        if kind == Kind::Mask {
            None
        } else {
            Some(wgpu::Face::Back)
        },
        false,
        Some(true),
    )
}

/// The resources a captured build's groups bind besides its structs.
struct Resources<'a> {
    render: &'a wgpu::Buffer,
    object: &'a wgpu::Buffer,
    maps: &'a [Option<crate::texture_gpu::GpuTexture>; 3],
    dfg: &'a wgpu::TextureView,
    linear: &'a wgpu::Sampler,
    compare: &'a wgpu::Sampler,
    shadow: Option<&'a wgpu::TextureView>,
    grid: Option<&'a wgpu::TextureView>,
    capture: Option<&'a Capture>,
    camera_index: Option<&'a wgpu::Buffer>,
    pulled: Option<&'a BatchDraw>,
}
/// The groups of a build's pipeline over the resources.
fn groups(
    r: &Renderer,
    p: &wgpu::RenderPipeline,
    shader: &Shader,
    res: &Resources,
) -> Result<Vec<wgpu::BindGroup>> {
    let count = shader.bindings.iter().map(|b| b.0).max().unwrap_or(0) + 1;
    let mut out = vec![];
    for g in 0..count {
        let mut entries: Vec<(u32, wgpu::BindingResource)> = vec![];
        for &(group, binding, kind) in shader.bindings {
            if group != g {
                continue;
            }
            let tex = |t: Tex| -> Result<&wgpu::TextureView> {
                match t {
                    Tex::Dfg => Ok(res.dfg),
                    _ => res.maps[match t {
                        Tex::Map => 0,
                        Tex::MetalRough => 1,
                        _ => 2,
                    }]
                    .as_ref()
                    .map(|m| &m.view)
                    .ok_or(Error::Invalid("lightprobes_sponza map")),
                }
            };
            let sampler = |t: Tex| -> Result<&wgpu::Sampler> {
                match t {
                    Tex::Dfg => Ok(res.linear),
                    _ => res.maps[match t {
                        Tex::Map => 0,
                        Tex::MetalRough => 1,
                        _ => 2,
                    }]
                    .as_ref()
                    .map(|m| &m.sampler)
                    .ok_or(Error::Invalid("lightprobes_sponza map")),
                }
            };
            let missing = || Error::Invalid("lightprobes_sponza binding");
            let resource = match kind {
                B::Render => res.render.as_entire_binding(),
                B::Object => res.object.as_entire_binding(),
                B::Sampler(t) => wgpu::BindingResource::Sampler(sampler(t)?),
                B::Texture(t) => wgpu::BindingResource::TextureView(tex(t)?),
                B::ShadowSampler => wgpu::BindingResource::Sampler(res.compare),
                B::Shadow => wgpu::BindingResource::TextureView(res.shadow.ok_or_else(missing)?),
                B::GridSampler => wgpu::BindingResource::Sampler(res.linear),
                B::Grid => wgpu::BindingResource::TextureView(res.grid.ok_or_else(missing)?),
                B::Views => res.capture.ok_or_else(missing)?.views.as_entire_binding(),
                B::Projections => res
                    .capture
                    .ok_or_else(missing)?
                    .projections
                    .as_entire_binding(),
                B::Positions => res
                    .capture
                    .ok_or_else(missing)?
                    .positions
                    .as_entire_binding(),
                B::CameraIndex => res.camera_index.ok_or_else(missing)?.as_entire_binding(),
                B::Vertices => res.pulled.ok_or_else(missing)?.vertices.as_entire_binding(),
                B::Triangles => res
                    .pulled
                    .ok_or_else(missing)?
                    .triangles
                    .as_entire_binding(),
                B::Queue => res.pulled.ok_or_else(missing)?.queue.as_entire_binding(),
                B::World => res.pulled.ok_or_else(missing)?.world.as_entire_binding(),
            };
            entries.push((binding, resource));
        }
        out.push(super::pmrem_cube_uv::bind(
            r,
            p.get_bind_group_layout(g),
            &entries,
        ));
    }
    Ok(out)
}

/// A Sponza material: color, metalness, roughness, normal scale, alpha test
/// and its maps ( base color, metal-rough, normal ).
struct Mat {
    color: [f64; 3],
    metalness: f64,
    roughness: f64,
    normal_scale: [f64; 2],
    alpha_test: f64,
    maps: [Option<crate::texture_gpu::GpuTexture>; 3],
}
/// One of Sponza's meshes, in the scene's order.
struct Mesh {
    /// The render item's id ( the sky is 0 ).
    id: u64,
    material: usize,
    kind: Kind,
    model: Matrix4,
    /// GLTFLoader's bounding sphere and box ( the accessors' ).
    sphere: (Vector3, f64),
    uv: wgpu::Buffer,
    normal: wgpu::Buffer,
    tangent: Option<wgpu::Buffer>,
    position: wgpu::Buffer,
    index: wgpu::Buffer,
    count: u32,
    /// The attributes createMeshlets reads.
    positions: Vec<f32>,
    normals: Vec<f32>,
    uvs: Vec<f32>,
    tangents: Option<Vec<f32>>,
    indices: Vec<u32>,
    /// The object structs: the main view's four builds, the capture's two
    /// and the shadow's.
    main_objects: Vec<wgpu::Buffer>,
    capture_objects: Vec<wgpu::Buffer>,
    shadow_object: wgpu::Buffer,
}
impl Mesh {
    fn draw(&self, pass: &mut wgpu::RenderPass) {
        pass.set_vertex_buffer(0, self.uv.slice(..));
        pass.set_vertex_buffer(1, self.normal.slice(..));
        match &self.tangent {
            Some(t) => {
                pass.set_vertex_buffer(2, t.slice(..));
                pass.set_vertex_buffer(3, self.position.slice(..));
            }
            None => pass.set_vertex_buffer(2, self.position.slice(..)),
        }
        pass.set_index_buffer(self.index.slice(..), wgpu::IndexFormat::Uint32);
        pass.draw_indexed(0..self.count, 0, 0..1);
    }
    fn draw_shadow(&self, pass: &mut wgpu::RenderPass) {
        pass.set_vertex_buffer(0, self.uv.slice(..));
        pass.set_vertex_buffer(1, self.position.slice(..));
        pass.set_index_buffer(self.index.slice(..), wgpu::IndexFormat::Uint32);
        pass.draw_indexed(0..self.count, 0, 0..1);
    }
    /// Frustum.intersectsObject: the bounding sphere in world space.
    fn culled(&self, planes: &[[f64; 4]; 6]) -> bool {
        let center = self.model.transform_point3(self.sphere.0);
        let scale = self.model.to_scale_rotation_translation().0.max_element();
        culled(planes, center, self.sphere.1 * scale)
    }
    /// The render list's depth: the sphere's center in clip space.
    fn z(&self, proj_screen: Matrix4) -> f64 {
        let c = self.model.transform_point3(self.sphere.0);
        (proj_screen * c.extend(1.)).z
    }
}
fn culled(planes: &[[f64; 4]; 6], center: Vector3, radius: f64) -> bool {
    planes
        .iter()
        .any(|p| p[0] * center.x + p[1] * center.y + p[2] * center.z + p[3] < -radius)
}

/// The page's params, in the GUI's order.
struct Params {
    meshlet: bool,
    enabled: bool,
    light_azimuth: f64,
    light_elevation: f64,
    light_intensity: f64,
    shadows: bool,
    count: [u32; 3],
    bounces: u32,
    show_probes: bool,
    probe_size: f64,
    bounds: Vector3,
    size: Vector3,
}
/// The probes' capture: the face atlas and its depth, the 48 sub-cameras'
/// views, projections and positions, a cameraIndex struct per sub-camera,
/// the batch rows of the SH projection.
struct Capture {
    faces: wgpu::TextureView,
    depth: wgpu::TextureView,
    views: wgpu::Buffer,
    projections: wgpu::Buffer,
    positions: wgpu::Buffer,
    indices: Vec<wgpu::Buffer>,
}
/// A LightProbeGrid: its live atlas, the bounce grid's snapshot, the batch
/// rows, the SH and repack structs ( one per Z slice ), and the helper's
/// instance matrices and UVWs.
struct Grid {
    res: [u32; 3],
    atlas: wgpu::Texture,
    atlas_view: wgpu::TextureView,
    bounce: wgpu::Texture,
    bounce_view: wgpu::TextureView,
    batch: wgpu::TextureView,
    sh_group: wgpu::BindGroup,
    repack: Vec<Vec<wgpu::BindGroup>>,
    matrices: wgpu::Buffer,
    uvw: wgpu::Buffer,
}
/// A shadow view's render struct ( projection and view ) and its groups for
/// the direct, alpha-tested and pulled depth pipelines.
struct ShadowView {
    render: wgpu::Buffer,
    groups: [wgpu::BindGroup; 3],
}
/// The targets of the canvas size.
struct Targets {
    width: u32,
    height: u32,
    format: wgpu::TextureFormat,
    samples: u32,
    msaa: Option<(wgpu::TextureView, wgpu::TextureView)>,
    color: wgpu::TextureView,
    depth: wgpu::TextureView,
    /// Per build ( variant × kind ) the main pass's pipelines: direct and
    /// pulled.
    direct: Vec<wgpu::RenderPipeline>,
    pulled: Vec<wgpu::RenderPipeline>,
    sky: wgpu::RenderPipeline,
    helper: wgpu::RenderPipeline,
    output: (wgpu::RenderPipeline, Vec<wgpu::BindGroup>),
    screen: RenderTarget,
}
/// Per batch ( material, layout, variant ): its buffers' generation, object
/// struct and groups.
type BatchGroups =
    HashMap<(usize, String, usize), ((u64, u64), wgpu::Buffer, Vec<wgpu::BindGroup>)>;
/// One draw of a render list, sorted as three sorts opaque items: group
/// order, render order, depth, id.
enum Item {
    Mesh(usize),
    Batch(usize),
    Sky,
    Helper,
}

pub(super) struct Demo {
    controls: FirstPerson,
    params: Params,
    pending: Vec<usize>,
    time: f64,
    last: f64,
    /// The timer's elapsed time ( the sky's clouds ).
    elapsed: f64,
    meshes: Vec<Mesh>,
    materials: Vec<Mat>,
    /// The engine's materials, which hold the maps' images.
    _sources: Vec<Arc<EngineMaterial>>,
    probe_far: f64,
    caster_sphere: (Vector3, f64),
    sun_position: Vector3,
    bake_index: u32,
    bake_pass: u32,
    helper_created: bool,
    grid: Option<Grid>,
    capture: Capture,
    meshlet: MeshletPipeline,
    linear: wgpu::Sampler,
    compare: wgpu::Sampler,
    /// The SunLight's cascade atlas and the bake light's map ( color, depth ).
    shadow_color: wgpu::TextureView,
    shadow_depth: wgpu::TextureView,
    bake_color: wgpu::TextureView,
    bake_depth: wgpu::TextureView,
    shadow_pipelines: [wgpu::RenderPipeline; 2],
    cascades: [ShadowView; 2],
    bake_shadow: ShadowView,
    /// The main pass's render structs per build ( variant × kind × pulled ),
    /// and per mesh and variant its groups ( created again when the grid
    /// changes ).
    main_renders: Vec<wgpu::Buffer>,
    main_groups: Vec<Vec<Vec<wgpu::BindGroup>>>,
    /// Per batch ( material, layout, variant ): its generation, object struct
    /// and groups.
    batch_groups: BatchGroups,
    /// The capture's pipelines per kind and shadows, render structs and
    /// groups per mesh.
    capture_pipelines: Vec<wgpu::RenderPipeline>,
    capture_renders: Vec<wgpu::Buffer>,
    capture_groups: Vec<Vec<Vec<wgpu::BindGroup>>>,
    capture_index_groups: Vec<Vec<wgpu::BindGroup>>,
    sky_capture: Vec<(
        wgpu::RenderPipeline,
        wgpu::Buffer,
        wgpu::Buffer,
        Vec<Vec<wgpu::BindGroup>>,
    )>,
    shadow_groups: Vec<wgpu::BindGroup>,
    /// The sky: box positions, index, the main build's structs.
    sky_positions: wgpu::Buffer,
    sky_index: wgpu::Buffer,
    sky_count: u32,
    sky_render: wgpu::Buffer,
    sky_object: wgpu::Buffer,
    sky_groups: Vec<wgpu::BindGroup>,
    helper_groups: Vec<wgpu::BindGroup>,
    sh_pipeline: wgpu::RenderPipeline,
    sh_object: wgpu::Buffer,
    repack_pipelines: Vec<wgpu::RenderPipeline>,
    repack_objects: Vec<wgpu::Buffer>,
    helper_mesh: (wgpu::Buffer, wgpu::Buffer, wgpu::Buffer, u32),
    helper_render: wgpu::Buffer,
    helper_object: wgpu::Buffer,
    output_render: wgpu::Buffer,
    output_object: wgpu::Buffer,
    output_quad: wgpu::Buffer,
    targets: Option<Targets>,
    /// The grid generation the groups bind.
    grid_generation: u64,
    groups_generation: u64,
}

/// getSponzaModelURL(): the Sponza entry of the sample model index.
pub(super) async fn sponza_url() -> Result<String> {
    let index: serde_json::Value = serde_json::from_slice(&fetch(MODEL_INDEX_URL).await?)
        .map_err(|e| Error::Asset(e.to_string()))?;
    let sponza = index
        .as_array()
        .and_then(|models| models.iter().find(|m| m["name"] == "Sponza"))
        .ok_or(Error::Asset(
            "Sponza entry was not found in the glTF sample model index.".into(),
        ))?;
    let variants = &sponza["variants"];
    let variant = ["glTF-Binary", "glTF", "glTF-Embedded"]
        .iter()
        .find_map(|k| variants[k].as_str())
        .or_else(|| variants.as_object()?.values().find_map(|v| v.as_str()))
        .ok_or(Error::Asset(
            "Sponza has no supported glTF variant in the model index.".into(),
        ))?;
    let folder = if variant.ends_with(".glb") {
        "glTF-Binary"
    } else {
        "glTF"
    };
    let name = sponza["name"].as_str().unwrap_or("Sponza");
    Ok(format!("{SAMPLE_ASSETS_BASE_URL}{name}/{folder}/{variant}"))
}

/// Object3D.lookAt for a camera from `p` along `direction`.
fn look(p: Vector3, (direction, up): ([f64; 3], [f64; 3])) -> Matrix4 {
    Matrix4::look_at_rh(
        p,
        p + Vector3::from_array(direction),
        Vector3::from_array(up),
    )
}
/// LightShadow._updateMatrix: the shadow matrix of a projection and view,
/// inside a viewport of the frame extents.
fn shadow_matrix(
    projection: Matrix4,
    view: Matrix4,
    viewport: Option<([f64; 4], [f64; 2])>,
) -> Matrix4 {
    let (sx, sy, ox, oy) = match viewport {
        Some((v, e)) => (v[2] / e[0], v[3] / e[1], v[0] / e[0], v[1] / e[1]),
        None => (1., 1., 0., 0.),
    };
    let bias = Matrix4::from_cols_array(&[
        0.5 * sx,
        0.,
        0.,
        0.,
        0.,
        0.5 * sy,
        0.,
        0.,
        0.,
        0.,
        1.,
        0.,
        0.5 * sx + ox,
        0.5 * sy + oy,
        0.,
        1.,
    ]);
    bias * projection * view
}
/// An orthographic camera's WebGPU projection.
fn orthographic(
    left: f64,
    right: f64,
    top: f64,
    bottom: f64,
    near: f64,
    far: f64,
) -> Result<Matrix4> {
    OrthographicCamera {
        left,
        right,
        top,
        bottom,
        near,
        far,
        ..Default::default()
    }
    .projection_matrix()
}

/// SunLightShadow.updateMatrices: the two cascades' projections, views,
/// shadow matrices ( in the atlas ) and ( begin, end, fade ) depths.
#[allow(clippy::type_complexity)]
fn sun_cascades(
    camera_projection: Matrix4,
    camera_world: Matrix4,
    near: f64,
    far: f64,
    light_position: Vector3,
) -> Result<[(Matrix4, Matrix4, Matrix4, [f64; 4]); 2]> {
    let (radius, shadow_near, shadow_far) = (1f64, 0.5f64, 50f64);
    let map = SHADOW as f64;
    let inset = ((radius.ceil() + 1.) / map).min(0.25);
    let resolution = map * (1. - 2. * inset);
    let camera_far = (near + 1e-6).max(shadow_far.min(far));
    let mut splits = [near, 0., camera_far];
    let amount = 0.5;
    let uniform = near + (camera_far - near) * amount;
    let logarithmic = if near > 0. {
        near * (camera_far / near).powf(amount)
    } else {
        uniform
    };
    splits[1] = (uniform + logarithmic) * 0.5;
    let direction = (-light_position).normalize();
    let up = if Vector3::Y.dot(direction).abs() > 0.99 {
        Vector3::Z
    } else {
        Vector3::Y
    };
    // Matrix4.lookAt( 0, direction, up ): the rotation whose -Z looks along
    // the direction.
    let z = -direction;
    let x = up.cross(z).normalize();
    let y = z.cross(x);
    let orientation = Matrix4::from_cols(x.extend(0.), y.extend(0.), z.extend(0.), Vector4::W);
    let view_to_light = orientation.transpose() * camera_world;
    let inverse = camera_projection.inverse();
    let mut near_corners = [Vector3::ZERO; 4];
    let mut far_corners = [Vector3::ZERO; 4];
    let mut global_max_z = f64::NEG_INFINITY;
    for i in 0..4 {
        let x = if i == 0 || i == 1 { 1. } else { -1. };
        let y = if i == 0 || i == 3 { 1. } else { -1. };
        let n = inverse.project_point3(Vector3::new(x, y, 0.));
        let f = n * (camera_far / near);
        near_corners[i] = view_to_light.transform_point3(n);
        far_corners[i] = view_to_light.transform_point3(f);
        global_max_z = global_max_z.max(near_corners[i].z).max(far_corners[i].z);
    }
    global_max_z += camera_far;
    let mut data = [[0f64; 4]; 2];
    let mut out = vec![];
    for i in 0..2 {
        let cascade_near = if i == 0 { splits[0] } else { data[i - 1][2] };
        let cascade_far = splits[i + 1];
        let fade = cascade_far - 0.1 * (cascade_far - splits[i]);
        data[i] = [
            if i == 0 { -1e10 } else { cascade_near },
            cascade_far,
            fade,
            0.,
        ];
        let near_alpha = (cascade_near - near) / (camera_far - near);
        let far_alpha = (cascade_far - near) / (camera_far - near);
        let mut corners = [Vector3::ZERO; 8];
        let mut center = Vector3::ZERO;
        for j in 0..4 {
            corners[j * 2] = near_corners[j].lerp(far_corners[j], near_alpha);
            corners[j * 2 + 1] = near_corners[j].lerp(far_corners[j], far_alpha);
            center += corners[j * 2] + corners[j * 2 + 1];
        }
        center *= 1. / 8.;
        let mut radius_sq = 0f64;
        let mut min_z = f64::INFINITY;
        for c in &corners {
            radius_sq = radius_sq.max(c.distance_squared(center));
            min_z = min_z.min(c.z);
        }
        let mut r = radius_sq.sqrt();
        if resolution > 1. {
            r /= 1. - 1. / resolution;
            let texel = 2. * r / resolution;
            // Math.round: halves round up.
            center.x = (center.x / texel + 0.5).floor() * texel;
            center.y = (center.y / texel + 0.5).floor() * texel;
        }
        center.z = global_max_z + shadow_near;
        let position = orientation.transform_point3(center);
        let mut world = orientation;
        world.w_axis = position.extend(1.);
        let projection = orthographic(
            -r,
            r,
            r,
            -r,
            shadow_near,
            global_max_z - min_z + 2. * shadow_near,
        )?;
        let view = world.inverse();
        let viewport = [i as f64 + inset, inset, 1. - 2. * inset, 1. - 2. * inset];
        out.push((
            projection,
            view,
            shadow_matrix(projection, view, Some((viewport, [2., 1.]))),
            data[i],
        ));
    }
    let [a, b]: [_; 2] = out.try_into().map_err(|_| Error::Invalid("sun cascades"))?;
    Ok([a, b])
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
            far: 1000.,
            aspect,
            ..Default::default()
        }));
        let n = s.get_mut(c)?;
        n.position = Vector3::new(-10.25, 4.99, 0.40);
        n.quaternion = Euler {
            angles: Vector3::new(1.6505, -1.5008, 1.6507),
            order: EulerOrder::XYZ,
        }
        .quaternion();
        // FirstPersonControls( camera ): the orientation from the camera's look.
        let direction = s.get(c)?.quaternion * -Vector3::Z;
        let phi = (direction.y / direction.length()).clamp(-1., 1.).acos();
        let mut controls = FirstPerson::default();
        controls.speed = 2.;
        controls.lat = 90. - phi.to_degrees();
        controls.lon = direction.x.atan2(direction.z).to_degrees();
        // The meshlet pipeline's clusterizer and simplifier, ready as the
        // page awaits them before its first frame.
        let meshopt = Meshopt::three(MESHOPT_SIMPLIFIER, MESHOPT_CLUSTERIZER).await?;
        let meshlet = MeshletPipeline::new(r, meshopt)?;
        // The model.
        let url = sponza_url().await?;
        let base = url.rsplit_once('/').ok_or(Error::Invalid("Sponza URL"))?.0;
        let (asset, buffers, images) = load_asset_bytes(&fetch(&url).await?, base).await?;
        let mut sponza = Scene::new();
        let nodes =
            crate::gltf::import_decoded(&asset, &buffers, &images)?.instantiate(&mut sponza)?;
        sponza.update()?;
        let init = |label, data: &[u8], usage| {
            r.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some(label),
                    contents: data,
                    usage,
                })
        };
        let floats = |g: &BufferGeometry, name: &str, size: usize| -> Result<Vec<f32>> {
            let a = g
                .attributes
                .get(name)
                .ok_or(Error::Invalid("Sponza attribute"))?;
            (0..a.count())
                .flat_map(|i| (0..size).map(move |k| (i, k)))
                .map(|(i, k)| a.get_component(i, k).map(|v| v as f32))
                .collect()
        };
        let linear = r.device.create_sampler(&wgpu::SamplerDescriptor {
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        let compare = r.device.create_sampler(&wgpu::SamplerDescriptor {
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            compare: Some(wgpu::CompareFunction::LessEqual),
            ..Default::default()
        });
        let mut cache = crate::texture_gpu::TextureCache::default();
        let mut sources: Vec<Arc<EngineMaterial>> = vec![];
        let mut keys = vec![];
        let mut materials = vec![];
        let mut meshes = vec![];
        let mut world_box = Box3 {
            min: Vector3::splat(f64::INFINITY),
            max: Vector3::splat(f64::NEG_INFINITY),
        };
        for h in nodes {
            let n = sponza.get(h)?;
            let NodeKind::Mesh(mesh) = &n.kind else {
                continue;
            };
            let Some(source) = mesh.materials.first() else {
                continue;
            };
            let EngineMaterial::Standard(m) = source.as_ref() else {
                continue;
            };
            // The glTF material: its maps and factors ( the import gives each
            // mesh its own copy ).
            let ptr = |t: Option<&Arc<crate::material::Texture>>| {
                t.map_or(0, |t| Arc::as_ptr(t) as usize)
            };
            let key = (
                [
                    ptr(m.properties.map.as_ref()),
                    ptr(m.metallic_roughness_map.as_ref()),
                    ptr(m.normal_map.as_ref()),
                ],
                format!(
                    "{:?} {} {} {} {:?}",
                    m.properties.color.0,
                    m.metalness,
                    m.roughness,
                    m.properties.alpha_test,
                    m.properties.side
                ),
            );
            let material = match keys.iter().position(|k| *k == key) {
                Some(i) => i,
                None => {
                    let maps = [
                        m.properties.map.as_ref(),
                        m.metallic_roughness_map.as_ref(),
                        m.normal_map.as_ref(),
                    ]
                    .map(|t| t.map(|t| cache.get(&r.device, &r.queue, t)).transpose());
                    let [a, b, c2] = maps;
                    sources.push(source.clone());
                    keys.push(key);
                    materials.push(Mat {
                        color: m.properties.color.0.to_array(),
                        metalness: m.metalness,
                        roughness: m.roughness,
                        normal_scale: [m.normal_scale.x, m.normal_scale.y],
                        alpha_test: m.properties.alpha_test,
                        maps: [a?, b?, c2?],
                    });
                    materials.len() - 1
                }
            };
            let mat = &materials[material];
            let kind = if m.properties.side == crate::material::Side::Double
                || m.properties.alpha_test > 0.
            {
                Kind::Mask
            } else if mat.maps[2].is_none() {
                Kind::Base
            } else {
                Kind::Std
            };
            let g = &mesh.geometry;
            let model = n.matrix_world;
            let positions = floats(g, "position", 3)?;
            let points: Vec<Vector3> = positions
                .chunks(3)
                .map(|p| Vector3::new(p[0] as f64, p[1] as f64, p[2] as f64))
                .collect();
            let local = Box3::from_points(points.iter().copied());
            world_box.union(local.transformed(model));
            let center = (local.min + local.max) * 0.5;
            let sphere = g
                .bounding_sphere
                .map_or((center, local.min.distance(local.max) / 2.), |s| {
                    (s.center, s.radius)
                });
            let indices: Vec<u32> = g
                .index
                .clone()
                .unwrap_or_else(|| (0..points.len() as u32).collect());
            let normals = floats(g, "normal", 3)?;
            let uvs = floats(g, "uv", 2)?;
            let tangents = if g.attributes.contains_key("tangent") {
                Some(floats(g, "tangent", 4)?)
            } else {
                None
            };
            let vertex = wgpu::BufferUsages::VERTEX;
            let main_objects = (0..4)
                .map(|v| {
                    struct_buffer(
                        r,
                        "lightprobes_sponza object",
                        main_shader(kind, false, v >= 2, v % 2 == 1),
                        "objectStruct",
                    )
                })
                .collect::<Result<_>>()?;
            let capture_objects = [false, true]
                .iter()
                .map(|&sh| {
                    struct_buffer(
                        r,
                        "lightprobes_sponza capture object",
                        capture_shader(kind, sh),
                        "objectStruct",
                    )
                })
                .collect::<Result<_>>()?;
            let shadow_shader = if kind == Kind::Mask {
                &shaders::SHADOW_MASK
            } else {
                &shaders::SHADOW
            };
            meshes.push(Mesh {
                id: meshes.len() as u64 + 1,
                material,
                kind,
                model,
                sphere,
                uv: init("sponza uv", bytemuck::cast_slice(&uvs), vertex),
                normal: init("sponza normal", bytemuck::cast_slice(&normals), vertex),
                tangent: tangents
                    .as_ref()
                    .map(|t| init("sponza tangent", bytemuck::cast_slice(t), vertex)),
                position: init("sponza position", bytemuck::cast_slice(&positions), vertex),
                index: init(
                    "sponza index",
                    bytemuck::cast_slice(&indices),
                    wgpu::BufferUsages::INDEX,
                ),
                count: indices.len() as u32,
                positions,
                normals,
                uvs,
                tangents,
                indices,
                main_objects,
                capture_objects,
                shadow_object: struct_buffer(
                    r,
                    "lightprobes_sponza shadow object",
                    shadow_shader,
                    "objectStruct",
                )?,
            });
        }
        // _box.setFromObject( model ): probeFar from the model's size; the
        // bake light's casters' sphere.
        let size = world_box.max - world_box.min;
        let probe_far = size.x.max(size.y).max(size.z) * 2.;
        let caster_sphere = (
            (world_box.min + world_box.max) * 0.5,
            (size.length() / 2.).max(1.),
        );
        // The shadow maps: the SunLight's two cascades side by side, the
        // bake light's 2048².
        let shadow_color = view(&texture(
            r,
            (2 * SHADOW, SHADOW, 1),
            wgpu::TextureDimension::D2,
            BYTE,
            1,
        ));
        let shadow_depth = view(&texture(
            r,
            (2 * SHADOW, SHADOW, 1),
            wgpu::TextureDimension::D2,
            DEPTH,
            1,
        ));
        let bake_color = view(&texture(
            r,
            (SHADOW, SHADOW, 1),
            wgpu::TextureDimension::D2,
            BYTE,
            1,
        ));
        let bake_depth = view(&texture(
            r,
            (SHADOW, SHADOW, 1),
            wgpu::TextureDimension::D2,
            DEPTH,
            1,
        ));
        // The shadow pipelines: FrontSide casters draw their back faces
        // ( clockwise ), the double-sided ones both.
        let shadow_pipelines = [
            pipeline(
                r,
                "lightprobes_sponza shadow",
                (shaders::SHADOW.vs, shaders::SHADOW.fs),
                Layout::Shadow,
                Some(BYTE),
                1,
                Some(wgpu::Face::Back),
                true,
                Some(true),
            ),
            pipeline(
                r,
                "lightprobes_sponza shadow mask",
                (shaders::SHADOW_MASK.vs, shaders::SHADOW_MASK.fs),
                Layout::Shadow,
                Some(BYTE),
                1,
                None,
                false,
                Some(true),
            ),
        ];
        let shadow_view = |label| -> ShadowView {
            let render = plain_buffer(r, label, 128);
            let bind0 = |p: &wgpu::RenderPipeline| {
                super::pmrem_cube_uv::bind(
                    r,
                    p.get_bind_group_layout(0),
                    &[(0, render.as_entire_binding())],
                )
            };
            ShadowView {
                groups: [
                    bind0(&shadow_pipelines[0]),
                    bind0(&shadow_pipelines[1]),
                    bind0(&meshlet.kernels().shadow_depth),
                ],
                render,
            }
        };
        let cascades = [
            shadow_view("sun cascade camera"),
            shadow_view("sun cascade camera"),
        ];
        let bake_shadow = shadow_view("bake light camera");
        // The capture: ArrayCamera's 48 sub-cameras into the face atlas.
        let matrices = |label, size| plain_buffer(r, label, size);
        let capture = Capture {
            faces: view(&texture(
                r,
                (6 * CUBE, BATCH * CUBE, 1),
                wgpu::TextureDimension::D2,
                HALF,
                1,
            )),
            depth: view(&texture(
                r,
                (6 * CUBE, BATCH * CUBE, 1),
                wgpu::TextureDimension::D2,
                DEPTH,
                1,
            )),
            views: matrices("capture views", (BATCH * 6 * 64) as u64),
            projections: matrices("capture projections", (BATCH * 6 * 64) as u64),
            positions: matrices("capture positions", (BATCH * 6 * 16) as u64),
            indices: (0..BATCH * 6)
                .map(|i| {
                    init(
                        "capture camera index",
                        bytemuck::cast_slice(&[i, 0, 0, 0]),
                        wgpu::BufferUsages::UNIFORM,
                    )
                })
                .collect(),
        };
        let capture_pipelines: Vec<wgpu::RenderPipeline> = [false, true]
            .iter()
            .flat_map(|&sh| {
                [Kind::Std, Kind::Base, Kind::Mask]
                    .map(|kind| lit_pipeline(r, capture_shader(kind, sh), kind, false, HALF, 1))
            })
            .collect();
        let capture_renders = [false, true]
            .iter()
            .flat_map(|&sh| {
                [Kind::Std, Kind::Base, Kind::Mask].map(|kind| {
                    struct_buffer(
                        r,
                        "capture render",
                        capture_shader(kind, sh),
                        "renderStruct",
                    )
                })
            })
            .collect::<Result<Vec<_>>>()?;
        // SkyMesh: the unit box scaled 450000.
        let sky_geometry = BoxGeometry::build(1., 1., 1.)?;
        let sky_index = sky_geometry.index.clone().unwrap_or_default();
        let sky_capture = [&shaders::SKY_CAPTURE, &shaders::SKY_CAPTURE_SH]
            .iter()
            .map(|shader| -> Result<_> {
                let p = pipeline(
                    r,
                    "lightprobes_sponza sky capture",
                    (shader.vs, shader.fs),
                    Layout::Sky,
                    Some(HALF),
                    1,
                    Some(wgpu::Face::Back),
                    true,
                    Some(false),
                );
                let render = struct_buffer(r, "sky capture render", shader, "renderStruct")?;
                let object = struct_buffer(r, "sky capture object", shader, "objectStruct")?;
                Ok((p, render, object, vec![]))
            })
            .collect::<Result<Vec<_>>>()?;
        let sh_pipeline = pipeline(
            r,
            "lightprobes_sponza SH",
            (PROBE_VS, PROBE_SH_FS),
            Layout::Pulled,
            Some(FLOAT),
            1,
            Some(wgpu::Face::Back),
            false,
            None,
        );
        let repack_pipelines = REPACK_FS
            .iter()
            .map(|fs| {
                pipeline(
                    r,
                    "lightprobes_sponza repack",
                    (PROBE_VS, fs),
                    Layout::Pulled,
                    Some(HALF),
                    1,
                    Some(wgpu::Face::Back),
                    false,
                    None,
                )
            })
            .collect();
        let helper = SphereGeometry::build(0.2, 16, 16)?;
        let read3 = |g: &BufferGeometry, name| floats(g, name, 3);
        let helper_index = helper.index.clone().ok_or(Error::Invalid("helper index"))?;
        let helper_mesh = (
            init(
                "helper positions",
                bytemuck::cast_slice(&read3(&helper, "position")?),
                wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            ),
            init(
                "helper normals",
                bytemuck::cast_slice(&read3(&helper, "normal")?),
                wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            ),
            init(
                "helper index",
                bytemuck::cast_slice(&helper_index),
                wgpu::BufferUsages::INDEX,
            ),
            helper_index.len() as u32,
        );
        let sky_shader = &shaders::SKY;
        let mut demo = Self {
            controls,
            params: Params {
                meshlet: true,
                enabled: true,
                light_azimuth: -75.,
                light_elevation: 60.,
                light_intensity: 100.,
                shadows: true,
                count: [10, 7, 7],
                bounces: 1,
                show_probes: false,
                probe_size: 0.2,
                bounds: Vector3::new(-0.5, 6., -0.3),
                size: Vector3::new(23.3, 12.8, 10.5),
            },
            pending: vec![],
            time: 0.,
            last: 0.,
            elapsed: 0.,
            meshes,
            materials,
            _sources: sources,
            probe_far,
            caster_sphere,
            sun_position: Vector3::Y,
            bake_index: 0,
            bake_pass: 0,
            helper_created: false,
            grid: None,
            capture,
            meshlet,
            linear,
            compare,
            shadow_color,
            shadow_depth,
            bake_color,
            bake_depth,
            shadow_pipelines,
            cascades,
            bake_shadow,
            main_renders: vec![],
            main_groups: vec![],
            batch_groups: HashMap::new(),
            capture_pipelines,
            capture_renders,
            capture_groups: vec![],
            capture_index_groups: vec![],
            sky_capture,
            shadow_groups: vec![],
            sky_positions: init(
                "sky positions",
                bytemuck::cast_slice(&floats(&sky_geometry, "position", 3)?),
                wgpu::BufferUsages::VERTEX,
            ),
            sky_index: init(
                "sky index",
                bytemuck::cast_slice(&sky_index),
                wgpu::BufferUsages::INDEX,
            ),
            sky_count: sky_index.len() as u32,
            sky_render: struct_buffer(r, "sky render", sky_shader, "renderStruct")?,
            sky_object: struct_buffer(r, "sky object", sky_shader, "objectStruct")?,
            sky_groups: vec![],
            helper_groups: vec![],
            sh_pipeline,
            sh_object: super::retro::uniform(r, "SH object", PROBE_SH_FS, "objectStruct")?,
            repack_pipelines,
            repack_objects: vec![],
            helper_mesh,
            helper_render: plain_buffer(r, "helper camera", 128),
            helper_object: super::retro::uniform(r, "helper object", HELPER_FS, "objectStruct")?,
            output_render: super::retro::uniform(r, "output render", OUTPUT_FS, "renderStruct")?,
            output_object: super::retro::uniform(r, "output object", OUTPUT_VS, "objectStruct")?,
            output_quad: init(
                "output quad",
                bytemuck::cast_slice(&[-1f32, 3., 0., -1., -1., 0., 3., -1., 0.]),
                wgpu::BufferUsages::VERTEX,
            ),
            targets: None,
            grid_generation: 0,
            groups_generation: u64::MAX,
        };
        demo.main_renders = (0..4)
            .flat_map(|v| {
                [Kind::Std, Kind::Base, Kind::Mask]
                    .into_iter()
                    .flat_map(move |k| [false, true].map(move |p| (v, k, p)))
            })
            .map(|(v, k, p)| {
                let shader = main_shader(k, p && k != Kind::Mask, v >= 2, v % 2 == 1);
                struct_buffer(r, "lightprobes_sponza render", shader, "renderStruct")
            })
            .collect::<Result<_>>()?;
        demo.update_light_position();
        demo.create_probes(r)?;
        demo.write_objects(r)?;
        Ok(demo)
    }
}

/// The identity of a 3×3 uniform ( the maps' UV transforms ).
fn identity3() -> Vec<f64> {
    m3(Matrix4::IDENTITY)
}

impl Demo {
    /// A mesh's material and transform values.
    fn object_values(&self, m: &Mesh) -> Values {
        let mat = &self.materials[m.material];
        vec![
            ("color", mat.color.to_vec()),
            ("opacity", vec![1.]),
            ("alpha_test", vec![mat.alpha_test]),
            ("metalness", vec![mat.metalness]),
            ("roughness", vec![mat.roughness]),
            ("emissive", vec![0.; 3]),
            ("emissive_intensity", vec![1.]),
            ("normal_scale", mat.normal_scale.to_vec()),
            ("map_uv", identity3()),
            ("metalness_uv", identity3()),
            ("roughness_uv", identity3()),
            ("normal_uv", identity3()),
            ("model", m4(m.model)),
            ("normal_matrix", m3(m.model.inverse().transpose())),
        ]
    }
    /// The grid's bounds and resolution ( the live grid and the bounce grid
    /// share them ).
    fn grid_values(&self) -> Values {
        let p = &self.params;
        let (max, min) = (p.bounds + p.size / 2., p.bounds - p.size / 2.);
        vec![
            ("grid_max", max.to_array().to_vec()),
            ("grid_min", min.to_array().to_vec()),
            ("grid_res", p.count.map(f64::from).to_vec()),
        ]
    }
    /// The objects' structs: the main builds, the captures and the shadow.
    fn write_objects(&self, r: &Renderer) -> Result<()> {
        for m in &self.meshes {
            let mut values = self.object_values(m);
            values.extend(self.grid_values());
            for (v, buffer) in m.main_objects.iter().enumerate() {
                write(
                    r,
                    buffer,
                    main_shader(m.kind, false, v >= 2, v % 2 == 1),
                    "objectStruct",
                    &values,
                )?;
            }
            for (sh, buffer) in m.capture_objects.iter().enumerate() {
                write(
                    r,
                    buffer,
                    capture_shader(m.kind, sh == 1),
                    "objectStruct",
                    &values,
                )?;
            }
            let shadow = if m.kind == Kind::Mask {
                &shaders::SHADOW_MASK
            } else {
                &shaders::SHADOW
            };
            write(r, &m.shadow_object, shadow, "objectStruct", &values)?;
        }
        Ok(())
    }
    /// updateLightPosition(): the sun ( and the sky's sun ) on the sphere.
    fn update_light_position(&mut self) {
        let elevation = self.params.light_elevation.to_radians();
        let azimuth = self.params.light_azimuth.to_radians();
        let phi = PI / 2. - elevation;
        self.sun_position = Vector3::new(
            phi.sin() * azimuth.sin(),
            phi.cos(),
            phi.sin() * azimuth.cos(),
        );
    }
    /// The SunLight's color times its intensity.
    fn light_color(&self) -> Vec<f64> {
        Color::from_hex(0xfff2dc)
            .0
            .to_array()
            .map(|v| v * self.params.light_intensity)
            .to_vec()
    }
    /// replaceSunLights: the bake light around the casters' sphere, its
    /// projection, view and position.
    fn bake_light(&self) -> Result<(Matrix4, Matrix4, Vector3)> {
        let (center, radius) = self.caster_sphere;
        let position = center + self.sun_position.normalize() * radius * 2.;
        let projection =
            orthographic(-radius, radius, radius, -radius, radius * 0.5, radius * 3.5)?;
        let view = Matrix4::look_at_rh(position, center, Vector3::Y);
        Ok((projection, view, position))
    }
    /// createProbes(): a new grid of the page's size and counts, then a
    /// rebake.
    fn create_probes(&mut self, r: &Renderer) -> Result<()> {
        let [nx, ny, nz] = self.params.count;
        let total = nx * ny * nz;
        let atlas_size = (nx, ny, 7 * (nz + 2));
        let atlas = texture(r, atlas_size, wgpu::TextureDimension::D3, HALF, 1);
        let bounce = texture(r, atlas_size, wgpu::TextureDimension::D3, HALF, 1);
        let batch = view(&texture(
            r,
            (COLUMNS, total, 1),
            wgpu::TextureDimension::D2,
            FLOAT,
            1,
        ));
        let sh_group = super::pmrem_cube_uv::bind(
            r,
            self.sh_pipeline.get_bind_group_layout(0),
            &[
                (0, self.sh_object.as_entire_binding()),
                (1, wgpu::BindingResource::TextureView(&self.capture.faces)),
            ],
        );
        // The repack structs, one per Z slice.
        while self.repack_objects.len() < nz as usize {
            self.repack_objects.push(super::retro::uniform(
                r,
                "repack object",
                REPACK_FS[0],
                "objectStruct",
            )?);
        }
        for (iz, object) in self.repack_objects.iter().enumerate() {
            r.queue.write_buffer(
                object,
                0,
                &pack(
                    REPACK_FS[0],
                    "objectStruct",
                    &[
                        ("nodeUniform0", &[1.]),
                        ("nodeUniform1", &[iz as f64]),
                        ("nodeUniform2", &[nx as f64, ny as f64, nz as f64]),
                        ("nodeUniform4", &identity3()),
                    ],
                )?,
            );
        }
        let repack = self.repack_objects[..nz as usize]
            .iter()
            .map(|object| {
                self.repack_pipelines
                    .iter()
                    .map(|p| {
                        super::pmrem_cube_uv::bind(
                            r,
                            p.get_bind_group_layout(0),
                            &[
                                (0, object.as_entire_binding()),
                                (1, wgpu::BindingResource::TextureView(&batch)),
                            ],
                        )
                    })
                    .collect()
            })
            .collect();
        // LightProbeGridHelper.update(): instance matrices and texel-center
        // UVWs.
        let (mut matrices, mut uvw) = (Vec::<f32>::new(), Vec::<f32>::new());
        for iz in 0..nz {
            for iy in 0..ny {
                for ix in 0..nx {
                    uvw.extend(
                        [(ix, nx), (iy, ny), (iz, nz)]
                            .map(|(i, n)| ((i as f64 + 0.5) / n as f64) as f32),
                    );
                    matrices.extend(
                        Matrix4::from_translation(self.probe_position([ix, iy, iz]))
                            .to_cols_array()
                            .map(|v| v as f32),
                    );
                }
            }
        }
        let init = |label, data: &[u8]| {
            r.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some(label),
                    contents: data,
                    usage: wgpu::BufferUsages::VERTEX,
                })
        };
        self.grid = Some(Grid {
            res: self.params.count,
            atlas_view: view(&atlas),
            atlas,
            bounce_view: view(&bounce),
            bounce,
            batch,
            sh_group,
            repack,
            matrices: init("helper matrices", bytemuck::cast_slice(&matrices)),
            uvw: init("helper uvw", bytemuck::cast_slice(&uvw)),
        });
        self.grid_generation += 1;
        self.helper_created = false;
        self.reset_probe_bake();
        self.write_objects(r)?;
        self.build_capture_groups(r)?;
        Ok(())
    }
    fn reset_probe_bake(&mut self) {
        self.bake_index = 0;
        self.bake_pass = 0;
    }
    /// getProbePosition: each probe at the center of its cell.
    fn probe_position(&self, i: [u32; 3]) -> Vector3 {
        let p = &self.params;
        let (c, s) = (p.bounds.to_array(), p.size.to_array());
        let axis = |k: usize| c[k] - s[k] / 2. + (i[k] as f64 + 0.5) * s[k] / p.count[k] as f64;
        Vector3::new(axis(0), axis(1), axis(2))
    }
    /// The capture builds' groups ( per build and mesh, with the sub-camera's
    /// index bound per draw ), the sky's per sub-camera, and the shadows'
    /// object groups.
    fn build_capture_groups(&mut self, r: &Renderer) -> Result<()> {
        let grid = self
            .grid
            .as_ref()
            .ok_or(Error::Invalid("lightprobes_sponza grid"))?;
        let mut capture_groups = vec![];
        for sh in [false, true] {
            let mut per_mesh = vec![];
            for m in &self.meshes {
                let shader = capture_shader(m.kind, sh);
                let k = usize::from(sh) * 3 + m.kind.index();
                let p = &self.capture_pipelines[k];
                let res = Resources {
                    render: &self.capture_renders[k],
                    object: &m.capture_objects[usize::from(sh)],
                    maps: &self.materials[m.material].maps,
                    dfg: &r.dfg,
                    linear: &self.linear,
                    compare: &self.compare,
                    shadow: Some(&self.bake_depth),
                    grid: Some(&grid.bounce_view),
                    capture: Some(&self.capture),
                    camera_index: Some(&self.capture.indices[0]),
                    pulled: None,
                };
                per_mesh.push(groups(r, p, shader, &res)?);
            }
            capture_groups.push(per_mesh);
        }
        self.capture_groups = capture_groups;
        // The cameraIndex groups per capture build.
        self.capture_index_groups = self
            .capture_pipelines
            .iter()
            .map(|p| {
                self.capture
                    .indices
                    .iter()
                    .map(|b| {
                        super::pmrem_cube_uv::bind(
                            r,
                            p.get_bind_group_layout(1),
                            &[(0, b.as_entire_binding())],
                        )
                    })
                    .collect()
            })
            .collect();
        for (i, (p, render, object, gs)) in self.sky_capture.iter_mut().enumerate() {
            let shader = if i == 0 {
                &shaders::SKY_CAPTURE
            } else {
                &shaders::SKY_CAPTURE_SH
            };
            let maps = [None, None, None];
            *gs = self
                .capture
                .indices
                .iter()
                .map(|index| {
                    groups(
                        r,
                        p,
                        shader,
                        &Resources {
                            render,
                            object,
                            maps: &maps,
                            dfg: &r.dfg,
                            linear: &self.linear,
                            compare: &self.compare,
                            shadow: None,
                            grid: None,
                            capture: Some(&self.capture),
                            camera_index: Some(index),
                            pulled: None,
                        },
                    )
                })
                .collect::<Result<_>>()?;
        }
        if self.shadow_groups.is_empty() {
            self.shadow_groups = self
                .meshes
                .iter()
                .map(|m| {
                    let (k, shader) = if m.kind == Kind::Mask {
                        (1, &shaders::SHADOW_MASK)
                    } else {
                        (0, &shaders::SHADOW)
                    };
                    let p = &self.shadow_pipelines[k];
                    let res = Resources {
                        render: &self.cascades[0].render,
                        object: &m.shadow_object,
                        maps: &self.materials[m.material].maps,
                        dfg: &r.dfg,
                        linear: &self.linear,
                        compare: &self.compare,
                        shadow: None,
                        grid: None,
                        capture: None,
                        camera_index: None,
                        pulled: None,
                    };
                    Ok(groups(r, p, shader, &res)?.swap_remove(1))
                })
                .collect::<Result<_>>()?;
        }
        Ok(())
    }

    /// updateProbes(): 16 probes per frame, each pass finished before the
    /// next bounce; the helper is created after the first bake.
    fn update_probes(&mut self, r: &Renderer) -> Result<()> {
        if self.grid.is_none() || self.bake_pass > self.params.bounces {
            return Ok(());
        }
        let total: u32 = self.params.count.iter().product();
        let count = PROBES_PER_FRAME.min(total - self.bake_index);
        self.bake(r, self.bake_index, count, self.bake_pass)?;
        self.helper_created = true;
        self.bake_index += count;
        if self.bake_index == total {
            self.bake_index = 0;
            self.bake_pass += 1;
        }
        Ok(())
    }

    /// LightProbeGrid.bake( renderer, scene, { start, count, pass } ).
    fn bake(&mut self, r: &Renderer, start: u32, count: u32, pass: u32) -> Result<()> {
        let grid = self
            .grid
            .as_ref()
            .ok_or(Error::Invalid("lightprobes_sponza grid"))?;
        let [nx, ny, nz] = grid.res;
        let end = start + count;
        let shadows = self.params.shadows;
        let mut encoder = r.device.create_command_encoder(&Default::default());
        // _updateBounceGrid: the previous pass's snapshot, before the first
        // range of an indirect pass overwrites the live atlas.
        if pass > 0 && start == 0 {
            encoder.copy_texture_to_texture(
                grid.atlas.as_image_copy(),
                grid.bounce.as_image_copy(),
                wgpu::Extent3d {
                    width: nx,
                    height: ny,
                    depth_or_array_layers: 7 * (nz + 2),
                },
            );
        }
        let bounce_intensity = if pass == 0 { 0. } else { 1. };
        // replaceSunLights: the bake light's shadow, rendered once for the
        // bake ( the first capture's shadow update ).
        let (light_projection, light_view, light_position) = self.bake_light()?;
        if shadows {
            let v = View {
                position: light_position,
                projection: light_projection,
                view: light_view,
                perspective: false,
                zoom: 1.,
                width: SHADOW,
                height: SHADOW,
                shadow: true,
            };
            self.render_shadow(r, &mut encoder, BAKE, &v, None)?;
        }
        // The capture builds' render structs: the light, its shadow and the
        // bounce grid.
        let light_values: Values = if shadows {
            vec![
                ("light_position", light_position.to_array().to_vec()),
                ("light_target", self.caster_sphere.0.to_array().to_vec()),
                ("light_color", self.light_color()),
                (
                    "shadow_matrix0",
                    m4(shadow_matrix(light_projection, light_view, None)),
                ),
                ("normal_bias", vec![0.]),
                ("bias0", vec![0.]),
                ("radius", vec![1.]),
                ("map_size", vec![SHADOW as f64; 2]),
                ("shadow_intensity", vec![1.]),
            ]
        } else {
            vec![
                ("light_direction", self.sun_position.to_array().to_vec()),
                ("light_color", self.light_color()),
            ]
        };
        let mut render_values = light_values;
        render_values.extend([
            ("grid_priority", vec![0.]),
            ("grid_intensity", vec![bounce_intensity]),
        ]);
        let sh = usize::from(shadows);
        for kind in [Kind::Std, Kind::Base, Kind::Mask] {
            write(
                r,
                &self.capture_renders[sh * 3 + kind.index()],
                capture_shader(kind, shadows),
                "renderStruct",
                &render_values,
            )?;
        }
        let sky_shader = if shadows {
            &shaders::SKY_CAPTURE_SH
        } else {
            &shaders::SKY_CAPTURE
        };
        let sky_values = self.sky_values();
        write(
            r,
            &self.sky_capture[sh].1,
            sky_shader,
            "renderStruct",
            &sky_values,
        )?;
        write(
            r,
            &self.sky_capture[sh].2,
            sky_shader,
            "objectStruct",
            &sky_values,
        )?;
        r.queue.submit([encoder.finish()]);
        // _captureProbes.
        let projection = Matrix4::perspective_rh(PI / 2., 1., CUBE_NEAR, self.probe_far);
        let grid = self
            .grid
            .as_ref()
            .ok_or(Error::Invalid("lightprobes_sponza grid"))?;
        let probes_per_layer = nx * nz;
        for batch_start in (start..end).step_by(BATCH as usize) {
            let batch_count = BATCH.min(end - batch_start);
            let mut views = vec![Matrix4::IDENTITY; (BATCH * 6) as usize];
            let mut positions = vec![Vector3::ZERO; (BATCH * 6) as usize];
            for slot in 0..batch_count {
                let probe = batch_start + slot;
                let (ix, iy, iz) = (probe % nx, probe / probes_per_layer, (probe / nx) % nz);
                let p = self.probe_position([ix, iy, iz]);
                for (face, f) in CAPTURE_FACES.iter().enumerate() {
                    views[(slot * 6) as usize + face] = look(p, *f);
                    positions[(slot * 6) as usize + face] = p;
                }
            }
            let flat = |m: &[Matrix4]| -> Vec<f32> {
                m.iter()
                    .flat_map(|m| m.to_cols_array().map(|v| v as f32))
                    .collect()
            };
            r.queue
                .write_buffer(&self.capture.views, 0, bytemuck::cast_slice(&flat(&views)));
            r.queue.write_buffer(
                &self.capture.projections,
                0,
                bytemuck::cast_slice(&flat(&vec![projection; (BATCH * 6) as usize])),
            );
            let position_data: Vec<f32> = positions
                .iter()
                .flat_map(|p| [p.x as f32, p.y as f32, p.z as f32, 0.])
                .collect();
            r.queue.write_buffer(
                &self.capture.positions,
                0,
                bytemuck::cast_slice(&position_data),
            );
            r.queue.write_buffer(
                &self.sh_object,
                0,
                &pack(
                    PROBE_SH_FS,
                    "objectStruct",
                    &[
                        ("nodeUniform0", &[1.]),
                        ("nodeUniform1", &[batch_start as f64]),
                        ("nodeUniform3", &identity3()),
                    ],
                )?,
            );
            let mut encoder = r.device.create_command_encoder(&Default::default());
            {
                let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("lightprobes_sponza capture"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &self.capture.faces,
                        depth_slice: None,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                    depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                        view: &self.capture.depth,
                        depth_ops: Some(wgpu::Operations {
                            load: wgpu::LoadOp::Clear(1.),
                            store: wgpu::StoreOp::Discard,
                        }),
                        stencil_ops: None,
                    }),
                    ..Default::default()
                });
                for camera in 0..(batch_count * 6) as usize {
                    let (slot, face) = (camera / 6, camera % 6);
                    pass.set_viewport(
                        (face as u32 * CUBE) as f32,
                        (slot as u32 * CUBE) as f32,
                        CUBE as f32,
                        CUBE as f32,
                        0.,
                        1.,
                    );
                    let planes = meshlet::frustum_planes(projection * views[camera]);
                    for (i, m) in self.meshes.iter().enumerate() {
                        if m.culled(&planes) {
                            continue;
                        }
                        let k = sh * 3 + m.kind.index();
                        let g = &self.capture_groups[sh][i];
                        pass.set_pipeline(&self.capture_pipelines[k]);
                        pass.set_bind_group(0, &g[0], &[]);
                        pass.set_bind_group(1, &self.capture_index_groups[k][camera], &[]);
                        pass.set_bind_group(2, &g[2], &[]);
                        m.draw(&mut pass);
                    }
                    // The sky.
                    let (p, _, _, gs) = &self.sky_capture[sh];
                    pass.set_pipeline(p);
                    for (i, g) in gs[camera].iter().enumerate() {
                        pass.set_bind_group(i as u32, g, &[]);
                    }
                    pass.set_vertex_buffer(0, self.sky_positions.slice(..));
                    pass.set_index_buffer(self.sky_index.slice(..), wgpu::IndexFormat::Uint32);
                    pass.draw_indexed(0..self.sky_count, 0, 0..1);
                }
            }
            {
                let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("lightprobes_sponza SH"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &grid.batch,
                        depth_slice: None,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Load,
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                    ..Default::default()
                });
                pass.set_viewport(
                    0.,
                    batch_start as f32,
                    COLUMNS as f32,
                    batch_count as f32,
                    0.,
                    1.,
                );
                pass.set_pipeline(&self.sh_pipeline);
                pass.set_bind_group(0, &grid.sh_group, &[]);
                pass.draw(0..3, 0..1);
            }
            r.queue.submit([encoder.finish()]);
        }
        // _repackProbes: the range's rows of each Z slice, into the slice
        // and, at the ends, the padding.
        let mut encoder = r.device.create_command_encoder(&Default::default());
        let (start_y, end_y) = (start / probes_per_layer, end / probes_per_layer);
        for iz in 0..nz {
            let slice_start = start_y * nx
                + ((start % probes_per_layer) as i64 - (iz * nx) as i64).clamp(0, nx as i64) as u32;
            let slice_end = end_y * nx
                + ((end % probes_per_layer) as i64 - (iz * nx) as i64).clamp(0, nx as i64) as u32;
            let mut probe = slice_start;
            while probe < slice_end {
                let (ix, iy) = (probe % nx, probe / nx);
                let width = (nx - ix).min(slice_end - probe);
                let height = if width == nx {
                    (ny - iy).min((slice_end - probe) / nx)
                } else {
                    1
                };
                for (t, p) in self.repack_pipelines.iter().enumerate() {
                    let base = t as u32 * (nz + 2);
                    let mut slices = vec![base + 1 + iz];
                    if iz == 0 {
                        slices.push(base);
                    }
                    if iz == nz - 1 {
                        slices.push(base + 1 + nz);
                    }
                    for slice in slices {
                        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                            label: Some("lightprobes_sponza repack"),
                            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                                view: &grid.atlas_view,
                                depth_slice: Some(slice),
                                resolve_target: None,
                                ops: wgpu::Operations {
                                    load: wgpu::LoadOp::Load,
                                    store: wgpu::StoreOp::Store,
                                },
                            })],
                            ..Default::default()
                        });
                        pass.set_viewport(
                            ix as f32,
                            iy as f32,
                            width as f32,
                            height as f32,
                            0.,
                            1.,
                        );
                        pass.set_pipeline(p);
                        pass.set_bind_group(0, &grid.repack[iz as usize][t], &[]);
                        pass.draw(0..3, 0..1);
                    }
                }
                probe += width * height;
            }
        }
        r.queue.submit([encoder.finish()]);
        Ok(())
    }

    /// SkyMesh's values: the sun, the page's turbidity, rayleigh and Mie
    /// coefficients, static clouds, the timer.
    fn sky_values(&self) -> Values {
        vec![
            ("model", m4(Matrix4::from_scale(Vector3::splat(450000.)))),
            ("sun_position", self.sun_position.to_array().to_vec()),
            ("rayleigh", vec![2.]),
            ("turbidity", vec![10.]),
            ("mie_coefficient", vec![0.005]),
            ("mie_directional_g", vec![0.8]),
            ("show_sun_disc", vec![0.]),
            ("cloud_coverage", vec![0.4]),
            ("cloud_elevation", vec![0.5]),
            ("cloud_scale", vec![0.0002]),
            ("cloud_speed", vec![0.]),
            ("cloud_density", vec![0.4]),
            ("opacity", vec![1.]),
            ("time", vec![self.elapsed]),
        ]
    }

    /// A shadow map's render: its projection through the meshlet pipeline
    /// ( or the regular one ), then its pass. The cascades draw into their
    /// viewport of the atlas, the bake light into its map.
    fn render_shadow(
        &mut self,
        r: &Renderer,
        encoder: &mut wgpu::CommandEncoder,
        key: u32,
        v: &View,
        viewport: Option<[f32; 4]>,
    ) -> Result<()> {
        let proj_screen = v.projection * v.view;
        let (items, batches) = self.project(r, key, v, true)?;
        let shadow_view = match key {
            BAKE => &self.bake_shadow,
            k => &self.cascades[(k - 1) as usize],
        };
        let data: Vec<f32> = [v.projection, v.view]
            .iter()
            .flat_map(|m| m.to_cols_array().map(|x| x as f32))
            .collect();
        r.queue
            .write_buffer(&shadow_view.render, 0, bytemuck::cast_slice(&data));
        let (color, depth) = if key == BAKE {
            (&self.bake_color, &self.bake_depth)
        } else {
            (&self.shadow_color, &self.shadow_depth)
        };
        let clear = key == BAKE;
        let load = |clear| {
            if clear {
                wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT)
            } else {
                wgpu::LoadOp::Load
            }
        };
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("lightprobes_sponza shadow"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: color,
                depth_slice: None,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: load(clear),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: depth,
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
        });
        if let Some([x, y, w, h]) = viewport {
            pass.set_viewport(x, y, w, h, 0., 1.);
        }
        for item in self.sorted(items, &batches, proj_screen, vec![]) {
            match item {
                Item::Mesh(i) => {
                    let m = &self.meshes[i];
                    let k = usize::from(m.kind == Kind::Mask);
                    pass.set_pipeline(&self.shadow_pipelines[k]);
                    pass.set_bind_group(0, &shadow_view.groups[k], &[]);
                    pass.set_bind_group(1, &self.shadow_groups[i], &[]);
                    m.draw_shadow(&mut pass);
                }
                Item::Batch(b) => {
                    pass.set_pipeline(&self.meshlet.kernels().shadow_depth);
                    pass.set_bind_group(0, &shadow_view.groups[2], &[]);
                    self.meshlet
                        .encode_depth(&mut pass, key, std::slice::from_ref(&batches[b]));
                }
                _ => {}
            }
        }
        Ok(())
    }

    /// The projection of a render list: with the meshlet pipeline the meshes
    /// it draws are collected ( processed one geometry per projection, the
    /// others culled on the CPU and drawn directly ) and culled on the GPU;
    /// the regular pipeline culls every mesh on the CPU. The direct meshes
    /// and the batches.
    fn project(
        &mut self,
        r: &Renderer,
        key: u32,
        v: &View,
        shadow: bool,
    ) -> Result<(Vec<usize>, Vec<BatchDraw>)> {
        let proj_screen = v.projection * v.view;
        let planes = meshlet::frustum_planes(proj_screen);
        let mut direct = vec![];
        if !self.params.meshlet {
            for (i, m) in self.meshes.iter().enumerate() {
                if !m.culled(&planes) {
                    direct.push(i);
                }
            }
            return Ok((direct, vec![]));
        }
        self.meshlet.begin();
        let mut objects = vec![];
        for (i, m) in self.meshes.iter().enumerate() {
            // Double-sided alpha-tested materials stay regular.
            if m.kind == Kind::Mask {
                if !m.culled(&planes) {
                    direct.push(i);
                }
                continue;
            }
            let object = MeshletObject {
                geometry: i,
                material: m.material,
                receive_shadow: !shadow,
                matrix: m.model.to_cols_array(),
                z: m.z(proj_screen),
            };
            if self.meshlet.has_clusters(i) {
                objects.push(object);
                continue;
            }
            if m.culled(&planes) {
                continue;
            }
            let source = || {
                let mut extras = vec![];
                if let Some(t) = &m.tangents {
                    extras.push(("tangent", &t[..], 4));
                }
                Ok(GeometrySource {
                    positions: &m.positions,
                    normals: &m.normals,
                    uvs: Some(&m.uvs),
                    extras,
                    index: Some(&m.indices),
                    sphere: (m.sphere.0.to_array(), m.sphere.1),
                })
            };
            if self.meshlet.get_clusters(i, source)? {
                objects.push(object);
            } else {
                direct.push(i);
            }
        }
        let projection = self.meshlet.draw_batches(r, key, v, objects)?;
        for o in projection.regular {
            if !self.meshes[o.geometry].culled(&planes) {
                direct.push(o.geometry);
            }
        }
        Ok((direct, projection.batches))
    }

    /// The render list's opaque order: depth, then id.
    fn sorted(
        &self,
        direct: Vec<usize>,
        batches: &[BatchDraw],
        proj_screen: Matrix4,
        extras: Vec<(f64, u64, Item)>,
    ) -> Vec<Item> {
        let mut items: Vec<(f64, u64, Item)> = extras;
        items.extend(direct.into_iter().map(|i| {
            (
                self.meshes[i].z(proj_screen),
                self.meshes[i].id,
                Item::Mesh(i),
            )
        }));
        for (b, d) in batches.iter().enumerate() {
            items.push((d.z, d.order, Item::Batch(b)));
        }
        items.sort_by(|a, b| a.0.total_cmp(&b.0).then(a.1.cmp(&b.1)));
        items.into_iter().map(|(_, _, i)| i).collect()
    }
}

impl Demo {
    /// The targets, the main pass's pipelines and their groups for the
    /// canvas size, format and samples.
    fn resize(&mut self, r: &Renderer, out: &RenderTarget) -> Result<()> {
        let size = (out.width, out.height);
        let samples = if out.options.samples > 1 { 4 } else { 1 };
        let mut direct = vec![];
        let mut pulled = vec![];
        for v in 0..4 {
            let (gi, sh) = (v >= 2, v % 2 == 1);
            for kind in [Kind::Std, Kind::Base, Kind::Mask] {
                direct.push(lit_pipeline(
                    r,
                    main_shader(kind, false, gi, sh),
                    kind,
                    false,
                    HALF,
                    samples,
                ));
            }
            for kind in [Kind::Std, Kind::Base] {
                pulled.push(lit_pipeline(
                    r,
                    main_shader(kind, true, gi, sh),
                    kind,
                    true,
                    HALF,
                    samples,
                ));
            }
        }
        let sky = pipeline(
            r,
            "lightprobes_sponza sky",
            (shaders::SKY.vs, shaders::SKY.fs),
            Layout::Sky,
            Some(HALF),
            samples,
            Some(wgpu::Face::Back),
            true,
            Some(false),
        );
        let helper = pipeline(
            r,
            "lightprobes_sponza helper",
            (HELPER_VS, HELPER_FS),
            Layout::Helper,
            Some(HALF),
            samples,
            Some(wgpu::Face::Back),
            false,
            Some(true),
        );
        let color = view(&texture(
            r,
            (size.0, size.1, 1),
            wgpu::TextureDimension::D2,
            HALF,
            1,
        ));
        let output_pipeline = pipeline(
            r,
            "lightprobes_sponza output",
            (OUTPUT_VS, OUTPUT_FS),
            Layout::Quad,
            Some(out.options.format),
            1,
            Some(wgpu::Face::Back),
            false,
            None,
        );
        let output_groups = vec![
            super::pmrem_cube_uv::bind(
                r,
                output_pipeline.get_bind_group_layout(0),
                &[(0, self.output_render.as_entire_binding())],
            ),
            super::pmrem_cube_uv::bind(
                r,
                output_pipeline.get_bind_group_layout(1),
                &[
                    (0, wgpu::BindingResource::Sampler(&self.linear)),
                    (1, wgpu::BindingResource::TextureView(&color)),
                    (2, self.output_object.as_entire_binding()),
                ],
            ),
        ];
        self.targets = Some(Targets {
            width: size.0,
            height: size.1,
            format: out.options.format,
            samples,
            msaa: (samples > 1).then(|| {
                (
                    view(&texture(
                        r,
                        (size.0, size.1, 1),
                        wgpu::TextureDimension::D2,
                        HALF,
                        samples,
                    )),
                    view(&texture(
                        r,
                        (size.0, size.1, 1),
                        wgpu::TextureDimension::D2,
                        DEPTH,
                        samples,
                    )),
                )
            }),
            depth: view(&texture(
                r,
                (size.0, size.1, 1),
                wgpu::TextureDimension::D2,
                DEPTH,
                1,
            )),
            color,
            direct,
            pulled,
            sky,
            helper,
            output: (output_pipeline, output_groups),
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
        self.batch_groups.clear();
        self.groups_generation = u64::MAX;
        Ok(())
    }

    /// The main builds' groups per variant and mesh, the sky's and the
    /// helper's, created again with the targets and the grid.
    fn build_main_groups(&mut self, r: &Renderer) -> Result<()> {
        let t = self
            .targets
            .as_ref()
            .ok_or(Error::Invalid("lightprobes_sponza targets"))?;
        let grid = self
            .grid
            .as_ref()
            .ok_or(Error::Invalid("lightprobes_sponza grid"))?;
        let mut all = vec![];
        for v in 0..4 {
            let (gi, sh) = (v >= 2, v % 2 == 1);
            let mut per_mesh = vec![];
            for m in &self.meshes {
                let shader = main_shader(m.kind, false, gi, sh);
                let p = &t.direct[v * 3 + m.kind.index()];
                let res = Resources {
                    render: &self.main_renders[(v * 3 + m.kind.index()) * 2],
                    object: &m.main_objects[v],
                    maps: &self.materials[m.material].maps,
                    dfg: &r.dfg,
                    linear: &self.linear,
                    compare: &self.compare,
                    shadow: Some(&self.shadow_depth),
                    grid: Some(&grid.atlas_view),
                    capture: None,
                    camera_index: None,
                    pulled: None,
                };
                per_mesh.push(groups(r, p, shader, &res)?);
            }
            all.push(per_mesh);
        }
        self.main_groups = all;
        let maps = [None, None, None];
        self.sky_groups = groups(
            r,
            &t.sky,
            &shaders::SKY,
            &Resources {
                render: &self.sky_render,
                object: &self.sky_object,
                maps: &maps,
                dfg: &r.dfg,
                linear: &self.linear,
                compare: &self.compare,
                shadow: None,
                grid: None,
                capture: None,
                camera_index: None,
                pulled: None,
            },
        )?;
        self.helper_groups = vec![
            super::pmrem_cube_uv::bind(
                r,
                t.helper.get_bind_group_layout(0),
                &[(0, self.helper_render.as_entire_binding())],
            ),
            super::pmrem_cube_uv::bind(
                r,
                t.helper.get_bind_group_layout(1),
                &[
                    (0, wgpu::BindingResource::Sampler(&self.linear)),
                    (1, wgpu::BindingResource::TextureView(&grid.atlas_view)),
                    (2, self.helper_object.as_entire_binding()),
                ],
            ),
        ];
        self.batch_groups.clear();
        self.groups_generation = self.grid_generation;
        Ok(())
    }

    /// The pulled builds' object structs and groups of the drawn batches,
    /// created again when the culling's or the pool's buffers change.
    fn prepare_batches(&mut self, r: &Renderer, batches: &[BatchDraw], v: usize) -> Result<()> {
        let t = self
            .targets
            .as_ref()
            .ok_or(Error::Invalid("lightprobes_sponza targets"))?;
        let grid = self
            .grid
            .as_ref()
            .ok_or(Error::Invalid("lightprobes_sponza grid"))?;
        let (gi, sh) = (v >= 2, v % 2 == 1);
        for b in batches {
            let kind = if b.layout.contains("tangent") {
                Kind::Std
            } else {
                Kind::Base
            };
            let key = (b.material, b.layout.clone(), v);
            if self
                .batch_groups
                .get(&key)
                .is_some_and(|(g, _, _)| *g == b.generation)
            {
                continue;
            }
            let shader = main_shader(kind, true, gi, sh);
            let object = match self.batch_groups.remove(&key) {
                Some((_, object, _)) => object,
                None => {
                    struct_buffer(r, "lightprobes_sponza batch object", shader, "objectStruct")?
                }
            };
            let mat = &self.materials[b.material];
            let mut values: Values = vec![
                ("color", mat.color.to_vec()),
                ("opacity", vec![1.]),
                ("metalness", vec![mat.metalness]),
                ("roughness", vec![mat.roughness]),
                ("emissive", vec![0.; 3]),
                ("emissive_intensity", vec![1.]),
                ("normal_scale", mat.normal_scale.to_vec()),
                ("map_uv", identity3()),
                ("metalness_uv", identity3()),
                ("roughness_uv", identity3()),
                ("normal_uv", identity3()),
                ("data_offset", vec![b.data_offset as f64]),
                ("queue_offset", vec![b.queue_offset as f64]),
            ];
            values.extend(self.grid_values());
            write(r, &object, shader, "objectStruct", &values)?;
            let p = &t.pulled[v * 2 + kind.index()];
            let res = Resources {
                render: &self.main_renders[(v * 3 + kind.index()) * 2 + 1],
                object: &object,
                maps: &mat.maps,
                dfg: &r.dfg,
                linear: &self.linear,
                compare: &self.compare,
                shadow: Some(&self.shadow_depth),
                grid: Some(&grid.atlas_view),
                capture: None,
                camera_index: None,
                pulled: Some(b),
            };
            let gs = groups(r, p, shader, &res)?;
            self.batch_groups.insert(key, (b.generation, object, gs));
        }
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
        // updateProbes(), then the frame.
        self.update_probes(r)?;
        if self.groups_generation != self.grid_generation {
            self.build_main_groups(r)?;
        }
        s.update()?;
        let (camera, world) = s.camera(c)?;
        let (camera, world) = (camera.clone(), world);
        let projection = camera.projection_matrix()?;
        let view = world.inverse();
        let position = world.w_axis.truncate();
        let proj_screen = projection * view;
        let (gi, shadows) = (self.params.enabled, self.params.shadows);
        let v = variant(gi, shadows);
        let (width, height) = {
            let t = self
                .targets
                .as_ref()
                .ok_or(Error::Invalid("lightprobes_sponza targets"))?;
            (t.width, t.height)
        };
        let mut encoder = r.device.create_command_encoder(&Default::default());
        // The camera's projection: its culling runs before the shadows.
        let main_view = View {
            position,
            projection,
            view,
            perspective: true,
            zoom: 1.,
            width,
            height,
            shadow: false,
        };
        let (direct, batches) = self.project(r, MAIN, &main_view, false)?;
        // The SunLight's cascades, when the shadows are on: the atlas is
        // cleared, then each cascade renders into its viewport.
        let (near, far) = match &camera {
            Camera::Perspective(p) => (p.near, p.far),
            Camera::Orthographic(o) => (o.near, o.far),
        };
        let cascades = sun_cascades(projection, world, near, far, self.sun_position)?;
        if shadows {
            {
                let _clear = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("lightprobes_sponza shadow clear"),
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
            }
            let inset = 2. / SHADOW as f64;
            for (i, (p, cascade_view, _, _)) in cascades.iter().enumerate() {
                let sv = View {
                    position: cascade_view.inverse().w_axis.truncate(),
                    projection: *p,
                    view: *cascade_view,
                    perspective: false,
                    zoom: 1.,
                    width: 2 * SHADOW,
                    height: SHADOW,
                    shadow: true,
                };
                let x = ((i as f64 + inset) * SHADOW as f64) as f32;
                let w = ((1. - 2. * inset) * SHADOW as f64) as f32;
                let y = (SHADOW as f64 * inset) as f32;
                self.render_shadow(r, &mut encoder, CASCADES[i], &sv, Some([x, y, w, w]))?;
            }
        }
        // The main builds' render structs.
        let mut values: Values = vec![
            ("cameraProjectionMatrix", m4(projection)),
            ("cameraViewMatrix", m4(view)),
            ("light_direction", self.sun_position.to_array().to_vec()),
            ("light_color", self.light_color()),
            ("normal_bias", vec![0.]),
            ("bias0", vec![0.]),
            ("bias1", vec![0.]),
            ("radius", vec![1.]),
            ("map_size", vec![(2 * SHADOW) as f64, SHADOW as f64]),
            ("shadow_intensity", vec![1.]),
            ("grid_priority", vec![0.]),
            ("grid_intensity", vec![1.]),
        ];
        for (i, (_, _, matrix, data)) in cascades.iter().enumerate() {
            values.push((["shadow_matrix0", "shadow_matrix1"][i], m4(*matrix)));
            values.push((["cascade0", "cascade1"][i], data.to_vec()));
        }
        for kind in [Kind::Std, Kind::Base, Kind::Mask] {
            for pulled in [false, true] {
                let shader = main_shader(kind, pulled && kind != Kind::Mask, gi, shadows);
                write(
                    r,
                    &self.main_renders[(v * 3 + kind.index()) * 2 + usize::from(pulled)],
                    shader,
                    "renderStruct",
                    &values,
                )?;
            }
        }
        let mut sky_values = self.sky_values();
        sky_values.extend([
            ("cameraProjectionMatrix", m4(projection)),
            ("cameraViewMatrix", m4(view)),
            ("cameraPosition", position.to_array().to_vec()),
        ]);
        write(
            r,
            &self.sky_render,
            &shaders::SKY,
            "renderStruct",
            &sky_values,
        )?;
        write(
            r,
            &self.sky_object,
            &shaders::SKY,
            "objectStruct",
            &sky_values,
        )?;
        let camera_data: Vec<f32> = [projection, view]
            .iter()
            .flat_map(|m| m.to_cols_array().map(|x| x as f32))
            .collect();
        r.queue
            .write_buffer(&self.helper_render, 0, bytemuck::cast_slice(&camera_data));
        let res = self.params.count.map(f64::from);
        r.queue.write_buffer(
            &self.helper_object,
            0,
            &pack(
                HELPER_FS,
                "objectStruct",
                &[
                    ("nodeUniform5", &res),
                    ("nodeUniform7", &identity3()),
                    ("nodeUniform10", &m4(Matrix4::IDENTITY)),
                ],
            )?,
        );
        let t = self
            .targets
            .as_ref()
            .ok_or(Error::Invalid("lightprobes_sponza targets"))?;
        r.queue.write_buffer(
            &self.output_render,
            0,
            &pack(
                OUTPUT_FS,
                "renderStruct",
                &[
                    (
                        "cameraProjectionMatrix",
                        &[
                            1., 0., 0., 0., 0., 1., 0., 0., 0., 0., -1., 0., 0., 0., 0., 1.,
                        ],
                    ),
                    ("cameraViewMatrix", &m4(Matrix4::IDENTITY)),
                    ("nodeUniform1", &[t.width as f64, t.height as f64]),
                    ("nodeUniform2", &[1.]),
                ],
            )?,
        );
        r.queue.write_buffer(
            &self.output_object,
            0,
            &pack(
                OUTPUT_VS,
                "objectStruct",
                &[("nodeUniform5", &m4(Matrix4::IDENTITY))],
            )?,
        );
        self.prepare_batches(r, &batches, v)?;
        // The main pass, in the render list's order.
        // The sky ( created first ) and the helper ( after the meshes ) sort
        // by their geometries' centers, the origin.
        let origin_z = (proj_screen * Vector4::W).z;
        let mut extras = vec![(origin_z, 0, Item::Sky)];
        if self.params.show_probes && self.helper_created {
            extras.push((origin_z, 1 << 32, Item::Helper));
        }
        let items = self.sorted(direct, &batches, proj_screen, extras);
        let t = self
            .targets
            .as_ref()
            .ok_or(Error::Invalid("lightprobes_sponza targets"))?;
        let grid = self
            .grid
            .as_ref()
            .ok_or(Error::Invalid("lightprobes_sponza grid"))?;
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("lightprobes_sponza scene"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: t.msaa.as_ref().map_or(&t.color, |m| &m.0),
                    depth_slice: None,
                    resolve_target: t.msaa.as_ref().map(|_| &t.color),
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: t.msaa.as_ref().map_or(&t.depth, |m| &m.1),
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.),
                        store: wgpu::StoreOp::Discard,
                    }),
                    stencil_ops: None,
                }),
                ..Default::default()
            });
            for item in &items {
                match *item {
                    Item::Mesh(i) => {
                        let m = &self.meshes[i];
                        pass.set_pipeline(&t.direct[v * 3 + m.kind.index()]);
                        for (g, group) in self.main_groups[v][i].iter().enumerate() {
                            pass.set_bind_group(g as u32, group, &[]);
                        }
                        m.draw(&mut pass);
                    }
                    Item::Batch(b) => {
                        let d = &batches[b];
                        let kind = if d.layout.contains("tangent") {
                            Kind::Std
                        } else {
                            Kind::Base
                        };
                        let Some((_, _, gs)) =
                            self.batch_groups.get(&(d.material, d.layout.clone(), v))
                        else {
                            continue;
                        };
                        pass.set_pipeline(&t.pulled[v * 2 + kind.index()]);
                        for (g, group) in gs.iter().enumerate() {
                            pass.set_bind_group(g as u32, group, &[]);
                        }
                        pass.draw_indirect(&d.args, d.args_offset);
                    }
                    Item::Sky => {
                        pass.set_pipeline(&t.sky);
                        for (g, group) in self.sky_groups.iter().enumerate() {
                            pass.set_bind_group(g as u32, group, &[]);
                        }
                        pass.set_vertex_buffer(0, self.sky_positions.slice(..));
                        pass.set_index_buffer(self.sky_index.slice(..), wgpu::IndexFormat::Uint32);
                        pass.draw_indexed(0..self.sky_count, 0, 0..1);
                    }
                    Item::Helper => {
                        let (positions, normals, index, count) = &self.helper_mesh;
                        pass.set_pipeline(&t.helper);
                        for (g, group) in self.helper_groups.iter().enumerate() {
                            pass.set_bind_group(g as u32, group, &[]);
                        }
                        pass.set_vertex_buffer(0, positions.slice(..));
                        pass.set_vertex_buffer(1, normals.slice(..));
                        pass.set_vertex_buffer(2, grid.uvw.slice(..));
                        pass.set_vertex_buffer(3, grid.matrices.slice(..));
                        pass.set_index_buffer(index.slice(..), wgpu::IndexFormat::Uint32);
                        pass.draw_indexed(0..*count, 0, 0..grid.res.iter().product());
                    }
                }
            }
        }
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("lightprobes_sponza output"),
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
            pass.set_pipeline(&t.output.0);
            for (g, group) in t.output.1.iter().enumerate() {
                pass.set_bind_group(g as u32, group, &[]);
            }
            pass.set_vertex_buffer(0, self.output_quad.slice(..));
            pass.draw(0..3, 0..1);
        }
        r.queue.submit([encoder.finish()]);
        Ok(true)
    }
    pub fn output(&self) -> Option<&RenderTarget> {
        self.targets.as_ref().map(|t| &t.screen)
    }
    pub fn output_mut(&mut self) -> Option<&mut RenderTarget> {
        self.targets.as_mut().map(|t| &mut t.screen)
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, dt: f64, animate: bool) -> Result<()> {
        if animate {
            self.time += dt;
        }
        Ok(())
    }
    /// animate(): the GUI's changes, the controls with the timer's delta.
    pub fn prepare(&mut self, s: &mut Scene, c: Object3D, r: &Renderer) -> Result<()> {
        let delta = (self.time - self.last).max(0.);
        self.last = self.time;
        self.elapsed = self.time;
        for index in std::mem::take(&mut self.pending) {
            match index {
                // Pipeline, GI, the helper's visibility.
                0 | 1 | 10 => {}
                2 | 3 => {
                    self.update_light_position();
                    self.reset_probe_bake();
                }
                4 | 5 | 9 => self.reset_probe_bake(),
                11 => {
                    // A new LightProbeGridHelper of the probe size.
                    let helper = SphereGeometry::build(self.params.probe_size, 16, 16)?;
                    let read = |name: &str| -> Result<Vec<f32>> {
                        let a = helper
                            .attributes
                            .get(name)
                            .ok_or(Error::Invalid("helper attribute"))?;
                        (0..a.count())
                            .flat_map(|i| (0..3).map(move |k| (i, k)))
                            .map(|(i, k)| a.get_component(i, k).map(|v| v as f32))
                            .collect()
                    };
                    r.queue.write_buffer(
                        &self.helper_mesh.0,
                        0,
                        bytemuck::cast_slice(&read("position")?),
                    );
                    r.queue.write_buffer(
                        &self.helper_mesh.1,
                        0,
                        bytemuck::cast_slice(&read("normal")?),
                    );
                }
                _ => self.create_probes(r)?,
            }
        }
        self.controls.update(s, c, delta)
    }
    pub fn draw(&mut self, kind: u32, x: f64, y: f64) {
        self.controls.pointer(kind, x, y);
    }
    pub fn key(&mut self, code: u32, down: bool) {
        self.controls.key(code, down);
    }
    /// The GUI: Pipeline ( 0 Meshlet, 1 Default ), GI, light azimuth,
    /// elevation and intensity, shadows, the probe counts, the bounces, and
    /// the helper's visibility and size.
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        let v = f64::from(value);
        match index {
            0 => self.params.meshlet = v < 0.5,
            1 => self.params.enabled = v > 0.5,
            2 => self.params.light_azimuth = v,
            3 => self.params.light_elevation = v,
            4 => self.params.light_intensity = v,
            5 => self.params.shadows = v > 0.5,
            6..=8 => self.params.count[index - 6] = v as u32,
            9 => self.params.bounces = v as u32,
            10 => self.params.show_probes = v > 0.5,
            11 => self.params.probe_size = v,
            _ => return Err(Error::Invalid("webgpu_lightprobes_sponza parameter")),
        }
        self.pending.push(index);
        Ok(())
    }
    pub fn seek(&mut self, t: f64) {
        self.time = t;
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
        self.prepare(scene, camera, renderer)
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
        Some(Ok(()))
    }
}

/// The program of three.js examples/webgpu_lightprobes_sponza.html: its renderer, its scene and its
/// frame loop.
#[wasm_bindgen::prelude::wasm_bindgen]
pub async fn webgpu_lightprobes_sponza(
    canvas: web_sys::HtmlCanvasElement,
    animate: bool,
) -> std::result::Result<super::BrowserApp, wasm_bindgen::JsValue> {
    super::program(
        canvas,
        animate,
        super::PageOptions {
            samples: 4,
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
                1000.,
                Box::new(Demo::create(scene, camera, 481, renderer).await?),
            ))
        },
    )
    .await
}
