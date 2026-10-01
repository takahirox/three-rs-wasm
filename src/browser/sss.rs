//! webgpu_postprocessing_sss: the Draco nemetona.glb statue on a fogged
//! ground under a hemisphere light and a shadow-casting directional light,
//! with SSSNode's screen-space shadows feeding the scene's directional
//! shadow and TRAA resolving the result. A pre-pass writes velocity and
//! depth; the SSS pass marches from each pixel toward the light through the
//! pre-pass depth (its ray offset following frameId); the scene pass reads
//! the SSS term in the light's shadow context; TRAA jitters, reprojects and
//! blends; the output encodes sRGB. The shadow-maps-only and SSS views and
//! the scene without temporal filtering replace the output node. Every
//! stage runs the WGSL three.js r186 generates for the page (in `sss/`; the
//! shadow pass is the memory_outline modules, TRAA the volume_traa ones and
//! the output the memory_outline, deferred and hdr ones, byte-identical).
use super::controls_attributes::{Controls, camera_state};
use super::deferred::{Draw, sampled_pipeline, set};
use super::gltf_viewer::load_asset;
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
const SHADOW: u32 = 1024;
const LIGHT: [f64; 3] = [-3., 10., -10.];
/// Color( 0xa0a0a0 ) in linear space: the background clear and the fog.
const BACKGROUND: f64 = 0.3515325994898463;
macro_rules! wgsl {
    ($name:literal) => {
        include_str!(concat!("sss/", $name, ".wgsl"))
    };
}
const SHADOW_VS: &str = include_str!("memory_outline/shadow_vs.wgsl");
const SHADOW_FS: &str = include_str!("memory_outline/shadow_fs.wgsl");
const TRAA_VS: &str = include_str!("volume_traa/traa_vs.wgsl");
const TRAA_FS: &str = include_str!("volume_traa/traa_fs.wgsl");
const OUTPUT_VS: &str = include_str!("memory_outline/composite_vs.wgsl");
const OUTPUT_FS: &str = include_str!("hdr/output_fs.wgsl");
const SSS_OUTPUT_VS: &str = include_str!("deferred/forward_output_vs.wgsl");
/// The SSS shader's rand( uv + frameId ) seed as r186 bakes it: SSSNode
/// assigns frame.frameId over its uniform, so each (re)build of the node
/// inlines the frameId of that frame ( 0 without temporal filtering ).
const SSS_SEED: &str = "vec2<f32>( 12001.0 )";
/// SSSNode's per-frame ray offsets.
const SPATIAL_OFFSETS: [f64; 4] = [0., 0.5, 0.25, 0.75];
/// The fixture restarts frameId at 12000 after loading.
const FRAME_ID_START: usize = 12000;
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
fn texture(r: &Renderer, size: (u32, u32), format: wgpu::TextureFormat) -> wgpu::Texture {
    r.device.create_texture(&wgpu::TextureDescriptor {
        label: Some("sss target"),
        size: wgpu::Extent3d {
            width: size.0,
            height: size.1,
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
const GRAY: wgpu::Color = wgpu::Color {
    r: BACKGROUND,
    g: BACKGROUND,
    b: BACKGROUND,
    a: 1.,
};
/// TRAANode's camera matrices from its last resolve.
#[derive(Clone, Copy)]
struct Resolved {
    world: Matrix4,
    projection_inverse: Matrix4,
}
/// A mesh's vertex buffers ( in the material's attribute order ), index and
/// local bounding sphere.
struct Mesh {
    buffers: Vec<wgpu::Buffer>,
    index: wgpu::Buffer,
    count: u32,
    sphere: Sphere,
    model: Matrix4,
}
impl Mesh {
    fn draw(&self, pass: &mut wgpu::RenderPass, slots: &[usize]) {
        for (slot, &i) in slots.iter().enumerate() {
            pass.set_vertex_buffer(slot as u32, self.buffers[i].slice(..));
        }
        pass.set_index_buffer(self.index.slice(..), wgpu::IndexFormat::Uint32);
        pass.draw_indexed(0..self.count, 0, 0..1);
    }
    fn visible(&self, frustum: &Frustum) -> bool {
        let scale = self.model.to_scale_rotation_translation().0.max_element();
        frustum.intersects_sphere(Sphere {
            center: self.model.transform_point3(self.sphere.center),
            radius: self.sphere.radius * scale,
        })
    }
}
struct Targets {
    width: u32,
    height: u32,
    format: wgpu::TextureFormat,
    velocity: wgpu::TextureView,
    pre_depth: (wgpu::Texture, wgpu::TextureView),
    scene: (wgpu::Texture, wgpu::TextureView),
    depth: wgpu::TextureView,
    sss: wgpu::TextureView,
    resolve: (wgpu::Texture, wgpu::TextureView),
    history: wgpu::Texture,
    history_depth: wgpu::Texture,
    screen: RenderTarget,
    /// The pre-pass model and ground, the scene model and ground with the
    /// SSS shadow context, and without it.
    pre: [Draw; 2],
    lit: [Draw; 2],
    plain: [Draw; 2],
    traa: (wgpu::RenderPipeline, [wgpu::BindGroup; 2]),
    /// The output of the resolve, the scene color and the SSS term.
    outputs: [Draw; 3],
}
pub(super) struct Demo {
    controls: Controls,
    /// output, shadow intensity, max ray distance, quality, thickness,
    /// temporal filtering.
    params: [f64; 6],
    pending: bool,
    loaded: bool,
    frame_id: usize,
    jitter: usize,
    built: bool,
    resolves: usize,
    /// The history restarted after a resize: its depth is read from the start.
    reread: bool,
    /// The SSS node is rebuilt on the next frame that uses it, and the seed
    /// and pipeline of its last build.
    rebuild: bool,
    sss_pipeline: Option<(f64, wgpu::RenderPipeline)>,
    /// The SSS draw for the current pipeline and targets.
    sss_draw: Option<Draw>,
    /// The first frame's unjittered projection and view the velocity keeps.
    motion: Option<(Matrix4, Matrix4)>,
    resolved: Option<Resolved>,
    model: Mesh,
    ground: Mesh,
    maps: [wgpu::TextureView; 3],
    shadow: (wgpu::TextureView, wgpu::TextureView),
    shadow_draw: Draw,
    shadow_render: wgpu::Buffer,
    renders: [wgpu::Buffer; 6],
    objects: [wgpu::Buffer; 6],
    shadow_object: wgpu::Buffer,
    sss_render: wgpu::Buffer,
    sss_object: wgpu::Buffer,
    traa_object: wgpu::Buffer,
    quad_uv: wgpu::Buffer,
    placeholder: wgpu::TextureView,
    linear: wgpu::Sampler,
    repeat: wgpu::Sampler,
    compare: wgpu::Sampler,
    targets: Option<Targets>,
}
/// The material modules in render-struct order: pre-pass model and ground,
/// SSS-context model and ground, plain model and ground.
const MATERIALS: [(&str, &str); 6] = [
    (wgsl!("pre_model_vs"), wgsl!("pre_model_fs")),
    (wgsl!("pre_ground_vs"), wgsl!("pre_ground_fs")),
    (wgsl!("model_vs"), wgsl!("model_fs")),
    (wgsl!("ground_vs"), wgsl!("ground_fs")),
    (wgsl!("plain_model_vs"), wgsl!("plain_model_fs")),
    (wgsl!("plain_ground_vs"), wgsl!("plain_ground_fs")),
];
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, _id: u32, r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 45.,
            near: 0.1,
            far: 100.,
            aspect,
            ..Default::default()
        }));
        s.get_mut(c)?.position = Vector3::new(1., 2.5, -3.5);
        let mut controls = Controls::new(Some(0.05), (1., 20.), PI, true);
        controls.set_target(Vector3::new(0., 2., 0.));
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
        let bounds = |positions: &[f32]| {
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
            Sphere {
                center,
                radius: points.iter().map(|p| p.distance(center)).fold(0., f64::max),
            }
        };
        // The statue: uv, normal, tangent and position, with its three WebP maps.
        let (asset, buffers, images) = load_asset("/web/gallery/assets/gltf/nemetona.glb").await?;
        let node = asset
            .nodes()
            .next()
            .ok_or(Error::Invalid("nemetona node"))?;
        let primitive = node
            .mesh()
            .and_then(|m| m.primitives().next())
            .ok_or(Error::Invalid("nemetona mesh"))?;
        let reader = primitive.reader(|b| buffers.get(b.index()).map(Vec::as_slice));
        let uv: Vec<f32> = reader
            .read_tex_coords(0)
            .ok_or(Error::Invalid("nemetona uv"))?
            .into_f32()
            .flatten()
            .collect();
        let normals: Vec<f32> = reader
            .read_normals()
            .ok_or(Error::Invalid("nemetona normals"))?
            .flatten()
            .collect();
        let tangents: Vec<f32> = reader
            .read_tangents()
            .ok_or(Error::Invalid("nemetona tangents"))?
            .flatten()
            .collect();
        let positions: Vec<f32> = reader
            .read_positions()
            .ok_or(Error::Invalid("nemetona positions"))?
            .flatten()
            .collect();
        let index: Vec<u32> = reader
            .read_indices()
            .ok_or(Error::Invalid("nemetona index"))?
            .into_u32()
            .collect();
        let (_, q, scale) = node.transform().decomposed();
        let node_model = Matrix4::from_scale_rotation_translation(
            Vector3::new(scale[0] as f64, scale[1] as f64, scale[2] as f64),
            Quaternion::from_xyzw(q[0] as f64, q[1] as f64, q[2] as f64, q[3] as f64),
            Vector3::ZERO,
        );
        let model = Mesh {
            sphere: bounds(&positions),
            buffers: [&uv[..], &normals, &tangents, &positions]
                .iter()
                .map(|a| init("nemetona", bytemuck::cast_slice(a), vertex))
                .collect(),
            index: init(
                "nemetona index",
                bytemuck::cast_slice(&index),
                wgpu::BufferUsages::INDEX,
            ),
            count: index.len() as u32,
            // rotation.y = π, scale 10, position.y = 0.45, over the glTF node.
            model: Matrix4::from_translation(Vector3::new(0., 0.45, 0.))
                * Matrix4::from_rotation_y(PI)
                * Matrix4::from_scale(Vector3::splat(10.))
                * node_model,
        };
        let mut mipmaps = Mipmaps::new(r);
        let image = |i: usize| images.get(i).ok_or(Error::Invalid("nemetona image"));
        // Textures 0, 1 and 2 are images 0 ( map ), 2 ( normal ) and 1
        // ( metalness / roughness ), by their EXT_texture_webp sources.
        let maps = [
            mipmapped(
                r,
                &mut mipmaps,
                image(0)?,
                wgpu::TextureFormat::Rgba8UnormSrgb,
            ),
            mipmapped(r, &mut mipmaps, image(1)?, BYTE),
            mipmapped(r, &mut mipmaps, image(2)?, BYTE),
        ];
        let plane = PlaneGeometry::build(100., 100., 1, 1)?;
        let read = |name: &str| -> Result<Vec<f32>> {
            let a = plane
                .attributes
                .get(name)
                .ok_or(Error::Invalid("ground attribute"))?;
            (0..a.count())
                .flat_map(|i| (0..3).map(move |k| (i, k)))
                .map(|(i, k)| a.get_component(i, k).map(|v| v as f32))
                .collect()
        };
        let plane_index = plane.index.clone().ok_or(Error::Invalid("ground index"))?;
        let ground_positions = read("position")?;
        let ground = Mesh {
            sphere: bounds(&ground_positions),
            buffers: vec![
                init(
                    "ground normals",
                    bytemuck::cast_slice(&read("normal")?),
                    vertex,
                ),
                init(
                    "ground positions",
                    bytemuck::cast_slice(&ground_positions),
                    vertex,
                ),
            ],
            index: init(
                "ground index",
                bytemuck::cast_slice(&plane_index),
                wgpu::BufferUsages::INDEX,
            ),
            count: plane_index.len() as u32,
            model: Matrix4::from_rotation_x(-PI / 2.),
        };
        let linear = r.device.create_sampler(&wgpu::SamplerDescriptor {
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        let repeat = r.device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::Repeat,
            address_mode_v: wgpu::AddressMode::Repeat,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        // The directional light's 1024² shadow ( the statue, double-sided ).
        let shadow_color = texture(r, (SHADOW, SHADOW), BYTE);
        let shadow_depth = texture(r, (SHADOW, SHADOW), DEPTH);
        let shadow_attrs = [
            wgpu::vertex_attr_array![0 => Float32x2],
            wgpu::vertex_attr_array![1 => Float32x3],
        ];
        let shadow_layouts = [(0, 8), (1, 12)].map(|(i, stride)| wgpu::VertexBufferLayout {
            array_stride: stride,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &shadow_attrs[i],
        });
        // The statue is double-sided: no culling in the shadow pass.
        let shadow_pipeline = no_cull(r, (SHADOW_VS, SHADOW_FS), &shadow_layouts, &[BYTE]);
        let shadow_render = uniform(r, "sss shadow render", SHADOW_VS, "renderStruct")?;
        let shadow_object = uniform(r, "sss shadow object", SHADOW_FS, "objectStruct")?;
        let shadow_draw = (
            shadow_pipeline.clone(),
            vec![
                bind(
                    r,
                    shadow_pipeline.get_bind_group_layout(0),
                    &[(0, shadow_render.as_entire_binding())],
                ),
                bind(
                    r,
                    shadow_pipeline.get_bind_group_layout(1),
                    &[
                        (0, wgpu::BindingResource::Sampler(&repeat)),
                        (1, wgpu::BindingResource::TextureView(&maps[0])),
                        (2, shadow_object.as_entire_binding()),
                    ],
                ),
            ],
        );
        let renders = MATERIALS.map(|(_, fs)| uniform(r, "sss render", fs, "renderStruct"));
        let objects = MATERIALS.map(|(_, fs)| uniform(r, "sss object", fs, "objectStruct"));
        let [r0, r1, r2, r3, r4, r5] = renders;
        let [o0, o1, o2, o3, o4, o5] = objects;
        // TRAANode's DepthTexture( 1, 1 ) placeholder, cleared to the far plane.
        let placeholder = view(&texture(r, (1, 1), DEPTH));
        let mut encoder = r.device.create_command_encoder(&Default::default());
        encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("sss placeholder"),
            depth_stencil_attachment: depth(&placeholder),
            ..Default::default()
        });
        r.queue.submit([encoder.finish()]);
        Ok(Self {
            controls,
            params: [0., 1., 0.2, 0.5, 0.01, 1.],
            // The page renders its first frame on loading.
            pending: true,
            loaded: false,
            frame_id: 0,
            jitter: 0,
            built: false,
            resolves: 0,
            reread: false,
            rebuild: true,
            sss_pipeline: None,
            sss_draw: None,
            motion: None,
            resolved: None,
            model,
            ground,
            maps,
            shadow: (view(&shadow_color), view(&shadow_depth)),
            shadow_draw,
            shadow_render,
            renders: [r0?, r1?, r2?, r3?, r4?, r5?],
            objects: [o0?, o1?, o2?, o3?, o4?, o5?],
            shadow_object,
            sss_render: uniform(r, "sss pass render", wgsl!("sss_fs"), "renderStruct")?,
            sss_object: uniform(r, "sss pass object", wgsl!("sss_fs"), "objectStruct")?,
            traa_object: uniform(r, "sss traa", TRAA_FS, "objectStruct")?,
            quad_uv: init(
                "sss quad",
                bytemuck::cast_slice(&[0f32, -1., 0., 1., 2., 1.]),
                vertex,
            ),
            placeholder,
            linear,
            repeat,
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
    /// The SSS pipeline with the seed of its build baked in.
    fn sss_pipeline(r: &Renderer, seed: f64) -> wgpu::RenderPipeline {
        let source = wgsl!("sss_fs").replace(SSS_SEED, &format!("vec2<f32>( {seed:.1} )"));
        let quad_attrs = [wgpu::vertex_attr_array![0 => Float32x2]];
        sampled_pipeline(
            r,
            "sss",
            (wgsl!("sss_vs"), &source),
            &[wgpu::VertexBufferLayout {
                array_stride: 8,
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes: &quad_attrs[0],
            }],
            &[wgpu::TextureFormat::R8Unorm],
            None,
            (false, false),
            (1, wgpu::PrimitiveTopology::TriangleList),
        )
    }
    fn resize(&mut self, r: &Renderer, out: &RenderTarget) -> Result<()> {
        let (width, height) = (out.width, out.height);
        let size = (width, height);
        let make = |format| {
            let t = texture(r, size, format);
            let v = view(&t);
            (t, v)
        };
        let velocity = view(&texture(r, size, HALF));
        let pre_depth = make(DEPTH);
        let scene = make(HALF);
        let depth_target = view(&texture(r, size, DEPTH));
        let sss = view(&texture(r, size, wgpu::TextureFormat::R8Unorm));
        let resolve = make(HALF);
        let history = texture(r, size, HALF);
        let history_depth = texture(r, size, DEPTH);
        let tex = wgpu::BindingResource::TextureView;
        let sampler = wgpu::BindingResource::Sampler;
        let model_attrs = [
            wgpu::vertex_attr_array![0 => Float32x2],
            wgpu::vertex_attr_array![1 => Float32x3],
            wgpu::vertex_attr_array![2 => Float32x4],
            wgpu::vertex_attr_array![3 => Float32x3],
        ];
        let model_layouts =
            [(0, 8), (1, 12), (2, 16), (3, 12)].map(|(i, stride)| wgpu::VertexBufferLayout {
                array_stride: stride,
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes: &model_attrs[i],
            });
        let ground_attrs = [
            wgpu::vertex_attr_array![0 => Float32x3],
            wgpu::vertex_attr_array![1 => Float32x3],
        ];
        let ground_layouts = [0, 1].map(|i| wgpu::VertexBufferLayout {
            array_stride: 12,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &ground_attrs[i],
        });
        let triangles = (1, wgpu::PrimitiveTopology::TriangleList);
        let lit_pipeline = |i: usize, model: bool, formats: &[wgpu::TextureFormat]| {
            if model {
                // The statue is double-sided.
                no_cull(r, MATERIALS[i], &model_layouts, formats)
            } else {
                // The ground writes no depth.
                sampled_pipeline(
                    r,
                    "sss ground",
                    MATERIALS[i],
                    &ground_layouts,
                    formats,
                    Some((wgpu::CompareFunction::LessEqual, false)),
                    (false, false),
                    triangles,
                )
            }
        };
        let groups =
            |p: &wgpu::RenderPipeline, i: usize, object: &[(u32, wgpu::BindingResource)]| {
                vec![
                    bind(
                        r,
                        p.get_bind_group_layout(0),
                        &[(0, self.renders[i].as_entire_binding())],
                    ),
                    bind(r, p.get_bind_group_layout(1), object),
                ]
            };
        let shadow_entries = |n: u32| [(n, sampler(&self.compare)), (n + 1, tex(&self.shadow.1))];
        let sss_view = &sss;
        let model_entries = |i: usize, sss: bool| {
            let mut entries = vec![
                (0, self.objects[i].as_entire_binding()),
                (1, sampler(&self.repeat)),
                (2, tex(&self.maps[0])),
                (3, sampler(&self.repeat)),
                (4, tex(&self.maps[1])),
                (5, sampler(&self.linear)),
                (6, tex(&r.dfg)),
                (7, sampler(&self.repeat)),
                (8, tex(&self.maps[2])),
            ];
            entries.extend(shadow_entries(9));
            if sss {
                entries.extend([(11, sampler(&self.linear)), (12, tex(sss_view))]);
            }
            entries
        };
        let ground_entries = |i: usize, sss: bool| {
            let mut entries = vec![(0, self.objects[i].as_entire_binding())];
            entries.extend(shadow_entries(1));
            if sss {
                entries.extend([(3, sampler(&self.linear)), (4, tex(sss_view))]);
            }
            entries
        };
        let draw = |i: usize, model: bool, sss: bool, formats: &[wgpu::TextureFormat]| {
            let p = lit_pipeline(i, model, formats);
            let entries = if model {
                model_entries(i, sss)
            } else {
                ground_entries(i, sss)
            };
            let g = groups(&p, i, &entries);
            (p, g)
        };
        let pre = [
            draw(0, true, false, &[HALF]),
            draw(1, false, false, &[HALF]),
        ];
        let lit = [draw(2, true, true, &[HALF]), draw(3, false, true, &[HALF])];
        let plain = [
            draw(4, true, false, &[HALF]),
            draw(5, false, false, &[HALF]),
        ];
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
        let traa_pipeline = screen_pass("sss traa", TRAA_VS, TRAA_FS, &[HALF]);
        let history_view = view(&history);
        let history_depth_view = view(&history_depth);
        let traa_groups = [&self.placeholder, &history_depth_view].map(|previous| {
            bind(
                r,
                traa_pipeline.get_bind_group_layout(0),
                &[
                    // Velocity is read with textureLoad: the layout drops its sampler.
                    (1, tex(&velocity)),
                    (2, sampler(&self.linear)),
                    (3, tex(&scene.1)),
                    (4, tex(&pre_depth.1)),
                    (5, self.traa_object.as_entire_binding()),
                    (6, tex(previous)),
                    (7, sampler(&self.linear)),
                    (8, tex(&history_view)),
                ],
            )
        });
        let format = out.options.format;
        let single = |vs, fs, source: &wgpu::TextureView| {
            let p = screen_pass("sss output", vs, fs, &[format]);
            let g = bind(
                r,
                p.get_bind_group_layout(0),
                &[
                    (0, wgpu::BindingResource::Sampler(&self.linear)),
                    (1, wgpu::BindingResource::TextureView(source)),
                ],
            );
            (p, vec![g])
        };
        let outputs = [
            single(OUTPUT_VS, OUTPUT_FS, &resolve.1),
            single(OUTPUT_VS, wgsl!("output_scene_fs"), &scene.1),
            single(SSS_OUTPUT_VS, wgsl!("output_sss_fs"), &sss),
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
            velocity,
            pre_depth,
            scene,
            depth: depth_target,
            sss,
            resolve,
            history,
            history_depth,
            screen,
            pre,
            lit,
            plain,
            traa: (traa_pipeline, traa_groups),
            outputs,
        });
        Ok(())
    }
    /// The output node: 0 the resolve, 1 the scene color, 2 the SSS term.
    fn output_index(&self) -> usize {
        if self.params[0].round() as usize == 2 {
            2
        } else if self.params[5] < 0.5 {
            1
        } else {
            0
        }
    }
    fn present(&self, encoder: &mut wgpu::CommandEncoder, t: &Targets) {
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("sss output"),
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
            // New history targets: TRAA restarts from the beauty buffer. After
            // the first resolves its previous depth is the history depth
            // texture, now new and zeroed, no longer the placeholder.
            self.reread = self.resolves >= 2;
            self.resolves = 0;
            self.sss_draw = None;
        }
        if !std::mem::take(&mut self.pending) {
            let t = self.targets.as_ref().ok_or(Error::Invalid("sss targets"))?;
            let mut encoder = r.device.create_command_encoder(&Default::default());
            self.present(&mut encoder, t);
            r.queue.submit([encoder.finish()]);
            return Ok(true);
        }
        let loading = !std::mem::replace(&mut self.loaded, true);
        if !loading {
            self.frame_id += 1;
        }
        let output = self.output_index();
        let mode = self.params[0].round() as usize;
        let traa_on = output == 0;
        let sss_on = mode != 1;
        let scene_on = output != 2;
        let temporal = self.params[5] > 0.5;
        // The SSS node's (re)build bakes this frame's frameId.
        if sss_on && (self.sss_pipeline.is_none() || (self.rebuild && !loading)) {
            let seed = if temporal {
                (FRAME_ID_START + self.frame_id) as f64
            } else {
                0.
            };
            self.sss_pipeline = Some((seed, Self::sss_pipeline(r, seed)));
            self.sss_draw = None;
            if !loading {
                self.rebuild = false;
            }
        } else if !sss_on && !loading {
            self.rebuild = false;
        }
        if sss_on && self.sss_draw.is_none() {
            let (t, (_, pipeline)) = (
                self.targets.as_ref().ok_or(Error::Invalid("sss targets"))?,
                self.sss_pipeline
                    .as_ref()
                    .ok_or(Error::Invalid("sss pipeline"))?,
            );
            let groups = vec![
                bind(
                    r,
                    pipeline.get_bind_group_layout(0),
                    &[(0, self.sss_render.as_entire_binding())],
                ),
                bind(
                    r,
                    pipeline.get_bind_group_layout(1),
                    &[
                        (0, wgpu::BindingResource::TextureView(&t.pre_depth.1)),
                        (1, self.sss_object.as_entire_binding()),
                    ],
                ),
            ];
            self.sss_draw = Some((pipeline.clone(), groups));
        }
        // animate(): the damped controls advance once per requested frame.
        self.controls.frame_update(s, c)?;
        let t = self.targets.as_ref().ok_or(Error::Invalid("sss targets"))?;
        s.update()?;
        let (camera, world) = s.camera(c)?;
        let (projection, view) = (camera.projection_matrix()?, world.inverse());
        let jittered = if traa_on && self.built {
            let [x, y] = halton(self.jitter);
            let mut p = projection;
            p.z_axis.x += 2. * (x - 0.5) / t.width as f64;
            p.z_axis.y -= 2. * (y - 0.5) / t.height as f64;
            p
        } else {
            projection
        };
        // The pre-pass velocity as r186 renders it here (read back from the
        // original): its previous camera view and its current projection keep
        // the first frame's unjittered values, while the previous projection
        // follows the camera. A camera move or resize leaves a velocity.
        let (first_projection, previous_view) = *self.motion.get_or_insert((projection, view));
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
        // The directional light's orthographic shadow camera ( ±4, near 0.1, far 40 ).
        let light = Vector3::from_array(LIGHT);
        let shadow_view = Matrix4::look_at_rh(light, Vector3::ZERO, Vector3::Y);
        let (near, far) = (0.1, 40.);
        let shadow_projection = Matrix4::from_cols_array(&[
            0.25,
            0.,
            0.,
            0.,
            0.,
            0.25,
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
        let bias = Matrix4::from_cols_array(&[
            0.5, 0., 0., 0., 0., 0.5, 0., 0., 0., 0., 1., 0., 0.5, 0.5, 0., 1.,
        ]);
        let shadow_matrix = bias * shadow_projection * shadow_view;
        write(
            &self.shadow_render,
            SHADOW_VS,
            "renderStruct",
            &[
                ("cameraProjectionMatrix", m4(shadow_projection)),
                ("cameraViewMatrix", m4(shadow_view)),
            ],
        )?;
        write(
            &self.shadow_object,
            SHADOW_FS,
            "objectStruct",
            &[
                ("nodeUniform1", m3(Matrix4::IDENTITY)),
                ("nodeUniform2", vec![1.]),
                ("nodeUniform5", m4(self.model.model)),
            ],
        )?;
        let [_, shadow_intensity, max_distance, quality, thickness, _] = self.params;
        let hemisphere_sky = vec![2.; 3];
        let hemisphere_ground = Color::from_hex(0x8d8d8d)
            .0
            .to_array()
            .map(|v| v * 2.)
            .to_vec();
        let fog = vec![BACKGROUND; 3];
        let size = vec![t.width as f64, t.height as f64];
        // The model's lighting names; the ground's.
        let model_lights = |previous: bool| {
            let mut values = vec![
                ("cameraProjectionMatrix", m4(jittered)),
                ("cameraViewMatrix", m4(view)),
                ("nodeUniform19", hemisphere_sky.clone()),
                ("nodeUniform21", vec![0., 20., 0.]),
                ("nodeUniform18", hemisphere_ground.clone()),
                ("nodeUniform24", vec![3.; 3]),
                ("nodeUniform22", LIGHT.to_vec()),
                ("nodeUniform23", vec![0.; 3]),
                ("nodeUniform25", m4(shadow_matrix)),
                ("nodeUniform26", vec![0.]),
                ("nodeUniform27", vec![-0.001]),
                ("nodeUniform29", vec![2.]),
                ("nodeUniform30", vec![SHADOW as f64; 2]),
                ("nodeUniform31", vec![1.]),
            ];
            if previous {
                // The pre-pass: fog at 32-34, the previous projection.
                values.extend([
                    ("nodeUniform32", fog.clone()),
                    ("nodeUniform33", vec![10.]),
                    ("nodeUniform34", vec![50.]),
                    ("nodeUniform37", m4(projection)),
                ]);
            } else {
                values.extend([
                    ("nodeUniform34", fog.clone()),
                    ("nodeUniform35", vec![10.]),
                    ("nodeUniform36", vec![50.]),
                    ("nodeUniform33", size.clone()),
                ]);
            }
            values
        };
        let plain_model_lights = {
            let mut values = model_lights(false);
            values.retain(|(n, _)| {
                *n != "nodeUniform34"
                    && *n != "nodeUniform35"
                    && *n != "nodeUniform36"
                    && *n != "nodeUniform33"
            });
            values.extend([
                ("nodeUniform32", fog.clone()),
                ("nodeUniform33", vec![10.]),
                ("nodeUniform34", vec![50.]),
            ]);
            values
        };
        let ground_lights = |pre: bool, sss: bool| {
            let mut values = vec![
                ("cameraProjectionMatrix", m4(jittered)),
                ("cameraViewMatrix", m4(view)),
                ("nodeUniform7", hemisphere_sky.clone()),
                ("nodeUniform11", vec![0., 20., 0.]),
                ("nodeUniform6", hemisphere_ground.clone()),
                ("nodeUniform14", vec![3.; 3]),
                ("nodeUniform12", LIGHT.to_vec()),
                ("nodeUniform13", vec![0.; 3]),
                ("nodeUniform16", m4(shadow_matrix)),
                ("nodeUniform17", vec![0.]),
                ("nodeUniform18", vec![-0.001]),
                ("nodeUniform20", vec![2.]),
                ("nodeUniform21", vec![SHADOW as f64; 2]),
                ("nodeUniform22", vec![1.]),
            ];
            if sss {
                values.extend([
                    ("nodeUniform25", fog.clone()),
                    ("nodeUniform26", vec![10.]),
                    ("nodeUniform27", vec![50.]),
                    ("nodeUniform24", size.clone()),
                ]);
            } else {
                values.extend([
                    ("nodeUniform23", fog.clone()),
                    ("nodeUniform24", vec![10.]),
                    ("nodeUniform25", vec![50.]),
                ]);
            }
            if pre {
                values.push(("nodeUniform28", m4(projection)));
            }
            values
        };
        write(
            &self.renders[0],
            MATERIALS[0].1,
            "renderStruct",
            &model_lights(true),
        )?;
        write(
            &self.renders[1],
            MATERIALS[1].1,
            "renderStruct",
            &ground_lights(true, false),
        )?;
        write(
            &self.renders[2],
            MATERIALS[2].1,
            "renderStruct",
            &model_lights(false),
        )?;
        write(
            &self.renders[3],
            MATERIALS[3].1,
            "renderStruct",
            &ground_lights(false, true),
        )?;
        write(
            &self.renders[4],
            MATERIALS[4].1,
            "renderStruct",
            &plain_model_lights,
        )?;
        write(
            &self.renders[5],
            MATERIALS[5].1,
            "renderStruct",
            &ground_lights(false, false),
        )?;
        let model = self.model.model;
        let model_object = |velocity: bool| {
            let identity3 = m3(Matrix4::IDENTITY);
            let mut values = vec![
                ("nodeUniform0", vec![1.; 3]),
                ("nodeUniform2", identity3.clone()),
                ("nodeUniform3", vec![1.]),
                ("nodeUniform4", vec![0.]),
                ("nodeUniform6", identity3.clone()),
                ("nodeUniform7", vec![0.7175613340082784]),
                ("nodeUniform8", identity3.clone()),
                ("nodeUniform10", m3(model.inverse().transpose())),
                ("nodeUniform11", vec![0.; 3]),
                ("nodeUniform12", vec![1.]),
                ("nodeUniform14", m4(model)),
                ("nodeUniform16", identity3),
                ("nodeUniform17", vec![1., 1.]),
            ];
            if velocity {
                values.extend([
                    ("nodeUniform35", m4(first_projection)),
                    ("nodeUniform36", m4(model)),
                    ("nodeUniform38", m4(previous_view)),
                    ("nodeUniform39", m4(model)),
                ]);
            }
            values
        };
        let ground_model = self.ground.model;
        let ground_object = |velocity: bool| {
            let mut values = vec![
                (
                    "nodeUniform0",
                    Color::from_hex(0xcbcbcb).0.to_array().to_vec(),
                ),
                ("nodeUniform1", vec![1.]),
                ("nodeUniform2", vec![30.]),
                (
                    "nodeUniform3",
                    Color::from_hex(0x111111).0.to_array().to_vec(),
                ),
                ("nodeUniform4", vec![0.; 3]),
                ("nodeUniform5", vec![1.]),
                ("nodeUniform9", m3(ground_model.inverse().transpose())),
                ("nodeUniform15", m4(ground_model)),
            ];
            if velocity {
                values.extend([
                    ("nodeUniform26", m4(first_projection)),
                    ("nodeUniform27", m4(ground_model)),
                    ("nodeUniform29", m4(previous_view)),
                    ("nodeUniform30", m4(ground_model)),
                ]);
            }
            values
        };
        write(
            &self.objects[0],
            MATERIALS[0].1,
            "objectStruct",
            &model_object(true),
        )?;
        write(
            &self.objects[1],
            MATERIALS[1].1,
            "objectStruct",
            &ground_object(true),
        )?;
        write(
            &self.objects[2],
            MATERIALS[2].1,
            "objectStruct",
            &model_object(false),
        )?;
        write(
            &self.objects[3],
            MATERIALS[3].1,
            "objectStruct",
            &ground_object(false),
        )?;
        write(
            &self.objects[4],
            MATERIALS[4].1,
            "objectStruct",
            &model_object(false),
        )?;
        write(
            &self.objects[5],
            MATERIALS[5].1,
            "objectStruct",
            &ground_object(false),
        )?;
        let offset = if temporal {
            SPATIAL_OFFSETS[(FRAME_ID_START + self.frame_id) % 4]
        } else {
            0.
        };
        write(
            &self.sss_render,
            wgsl!("sss_fs"),
            "renderStruct",
            &[
                ("nodeUniform3", LIGHT.to_vec()),
                ("nodeUniform4", vec![0.; 3]),
            ],
        )?;
        write(
            &self.sss_object,
            wgsl!("sss_fs"),
            "objectStruct",
            &[
                ("nodeUniform1", m4(jittered.inverse())),
                ("nodeUniform2", m4(view)),
                ("nodeUniform5", vec![max_distance]),
                ("nodeUniform6", m4(jittered)),
                ("nodeUniform7", size.clone()),
                ("nodeUniform8", vec![quality]),
                ("nodeUniform9", vec![offset]),
                ("nodeUniform10", vec![0.1]),
                ("nodeUniform11", vec![100.]),
                ("nodeUniform12", vec![thickness]),
                ("nodeUniform13", vec![shadow_intensity]),
            ],
        )?;
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
        {
            let frustum = Frustum::from_projection(shadow_projection * shadow_view);
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("sss shadow"),
                color_attachments: &[color(&self.shadow.0, wgpu::Color::TRANSPARENT)],
                depth_stencil_attachment: depth(&self.shadow.1),
                ..Default::default()
            });
            if self.model.visible(&frustum) {
                set(&mut pass, &self.shadow_draw);
                self.model.draw(&mut pass, &[0, 3]);
            }
        }
        let frustum = Frustum::from_projection(jittered * view);
        let (model_visible, ground_visible) =
            (self.model.visible(&frustum), self.ground.visible(&frustum));
        // The pre-pass: velocity and depth ( the ground writes no depth ).
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("sss pre-pass"),
                color_attachments: &[color(&t.velocity, GRAY)],
                depth_stencil_attachment: depth(&t.pre_depth.1),
                ..Default::default()
            });
            if model_visible {
                set(&mut pass, &t.pre[0]);
                self.model.draw(&mut pass, &[0, 1, 2, 3]);
            }
            if ground_visible {
                set(&mut pass, &t.pre[1]);
                self.ground.draw(&mut pass, &[0, 1]);
            }
        }
        let quad = |encoder: &mut wgpu::CommandEncoder,
                    target: &wgpu::TextureView,
                    clear: wgpu::Color,
                    draw: &Draw| {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("sss quad"),
                color_attachments: &[color(target, clear)],
                ..Default::default()
            });
            set(&mut pass, draw);
            pass.set_vertex_buffer(0, self.quad_uv.slice(..));
            pass.draw(0..3, 0..1);
        };
        if sss_on {
            let draw = self.sss_draw.as_ref().ok_or(Error::Invalid("sss draw"))?;
            quad(&mut encoder, &t.sss, wgpu::Color::WHITE, draw);
        }
        if scene_on {
            let draws = if sss_on { &t.lit } else { &t.plain };
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("sss scene"),
                color_attachments: &[color(&t.scene.1, GRAY)],
                depth_stencil_attachment: depth(&t.depth),
                ..Default::default()
            });
            if model_visible {
                set(&mut pass, &draws[0]);
                self.model.draw(&mut pass, &[0, 1, 2, 3]);
            }
            if ground_visible {
                set(&mut pass, &draws[1]);
                self.ground.draw(&mut pass, &[0, 1]);
            }
        }
        if traa_on {
            let full = wgpu::Extent3d {
                width: t.width,
                height: t.height,
                depth_or_array_layers: 1,
            };
            if self.resolves == 0 {
                encoder.copy_texture_to_texture(
                    t.scene.0.as_image_copy(),
                    t.history.as_image_copy(),
                    full,
                );
            }
            let (pipeline, groups) = &t.traa;
            let draw = (
                pipeline.clone(),
                vec![groups[usize::from(self.resolves >= 2 || self.reread)].clone()],
            );
            quad(&mut encoder, &t.resolve.1, wgpu::Color::BLACK, &draw);
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
    /// output, shadow intensity, max ray distance, quality, thickness and
    /// temporal filtering.
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        // The output and the temporal filtering rebuild the output node.
        if index == 0 || index == 5 {
            self.built = false;
            self.rebuild = true;
        }
        *self
            .params
            .get_mut(index)
            .ok_or(Error::Invalid("sss parameter"))? = value as f64;
        Ok(())
    }
    pub fn seek(&mut self, _t: f64) {
        self.pending = true;
    }
}
/// A lit pipeline without culling, for the double-sided statue.
fn no_cull(
    r: &Renderer,
    (vs, fs): (&str, &str),
    buffers: &[wgpu::VertexBufferLayout],
    formats: &[wgpu::TextureFormat],
) -> wgpu::RenderPipeline {
    let module = |source: &str| {
        r.device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("sss statue"),
            source: wgpu::ShaderSource::Wgsl(source.into()),
        })
    };
    let targets: Vec<Option<wgpu::ColorTargetState>> =
        formats.iter().map(|&f| Some(f.into())).collect();
    r.device
        .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("sss statue"),
            layout: None,
            vertex: wgpu::VertexState {
                module: &module(vs),
                entry_point: Some("main"),
                compilation_options: Default::default(),
                buffers,
            },
            fragment: Some(wgpu::FragmentState {
                module: &module(fs),
                entry_point: Some("main"),
                compilation_options: Default::default(),
                targets: &targets,
            }),
            primitive: Default::default(),
            depth_stencil: Some(wgpu::DepthStencilState {
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
}
