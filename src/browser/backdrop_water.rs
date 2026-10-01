//! webgpu_backdrop_water: the skinned, dancing Michelle on an ice pillar in a
//! field of floating ice spheres, under a water surface whose backdrop node
//! refracts and tints the scene behind it. The scene pass draws the gradient
//! background, the hundred ice spheres ( triplanar water texture ), Michelle
//! ( GPU skinning with the mixer's bone matrices, casting the sun's shadow )
//! and the pillar with animated Voronoi caustics; the framebuffer's color and
//! depth are then copied for the transparent water's backdrop ( Voronoi
//! ripples, depth-based refraction ). A depth-driven Gaussian blur, tinted
//! and vignetted under water, makes the output. The orbit auto-rotates.
//! Every stage runs the WGSL three.js r186 generates for the page (in
//! `backdrop_water/`; the output vertex module is the volume_caustics one,
//! byte-identical); the animation is sampled on the CPU as AnimationMixer does.
use super::controls_attributes::{Controls, camera_state};
use super::deferred::{Draw, sampled_pipeline, set};
use super::gltf_viewer::{decode_texture_image, fetch, load_asset};
use super::lights_projector::{m3, m4, pack};
use super::pmrem_cube_uv::bind;
use super::retro::{mipmapped, uniform};
use super::shadowmap_opacity::Mipmaps;
use super::three_mixer::ThreeMixer;
use crate::{
    Error, Result, camera::*, geometry::*, math::*, render_target::*, renderer::*, scene::*,
};
use std::f64::consts::PI;
use wgpu::util::DeviceExt;

const HALF: wgpu::TextureFormat = wgpu::TextureFormat::Rgba16Float;
const BYTE: wgpu::TextureFormat = wgpu::TextureFormat::Rgba8Unorm;
const SRGB: wgpu::TextureFormat = wgpu::TextureFormat::Rgba8UnormSrgb;
const DEPTH: wgpu::TextureFormat = wgpu::TextureFormat::Depth24Plus;
const SHADOW: u32 = 512;
macro_rules! wgsl {
    ($name:literal) => {
        include_str!(concat!("backdrop_water/", $name, ".wgsl"))
    };
}
const OUTPUT_VS: &str = include_str!("volume_caustics/composite_vs.wgsl");
/// The ice spheres' first Object3D id: their y follows sin( elapsed + id ).
const FIRST_ICE_ID: f64 = 19.;
/// The fixture's seeded Math.random ( a 32-bit LCG from 186 ).
struct FixtureRandom(u32);
impl FixtureRandom {
    fn next(&mut self) -> f64 {
        self.0 = self.0.wrapping_mul(1664525).wrapping_add(1013904223);
        self.0 as f64 / 4294967296.
    }
}
fn linear(hex: u32, intensity: f64) -> Vec<f64> {
    [16, 8, 0]
        .map(|shift| {
            let c = ((hex >> shift) & 255) as f64 / 255.;
            let l = if c < 0.04045 {
                c * 0.0773993808
            } else {
                (c * 0.9478672986 + 0.0521327014).powf(2.4)
            };
            l * intensity
        })
        .to_vec()
}
/// Euler XYZ → Quaternion → Matrix4.compose, in three.js's order.
fn compose(position: [f64; 3], rotation: [f64; 3], scale: f64) -> Matrix4 {
    let [rx, ry, rz] = rotation.map(|a| a / 2.);
    let (c1, c2, c3) = (rx.cos(), ry.cos(), rz.cos());
    let (s1, s2, s3) = (rx.sin(), ry.sin(), rz.sin());
    let x = s1 * c2 * c3 + c1 * s2 * s3;
    let y = c1 * s2 * c3 - s1 * c2 * s3;
    let z = c1 * c2 * s3 + s1 * s2 * c3;
    let w = c1 * c2 * c3 - s1 * s2 * s3;
    let (x2, y2, z2) = (x + x, y + y, z + z);
    let (xx, xy, xz) = (x * x2, x * y2, x * z2);
    let (yy, yz, zz) = (y * y2, y * z2, z * z2);
    let (wx, wy, wz) = (w * x2, w * y2, w * z2);
    Matrix4::from_cols_array(&[
        (1. - (yy + zz)) * scale,
        (xy + wz) * scale,
        (xz - wy) * scale,
        0.,
        (xy - wz) * scale,
        (1. - (xx + zz)) * scale,
        (yz + wx) * scale,
        0.,
        (xz + wy) * scale,
        (yz - wx) * scale,
        (1. - (xx + yy)) * scale,
        0.,
        position[0],
        position[1],
        position[2],
        1.,
    ])
}
fn texture(
    r: &Renderer,
    size: (u32, u32),
    format: wgpu::TextureFormat,
    usage: wgpu::TextureUsages,
) -> wgpu::Texture {
    r.device.create_texture(&wgpu::TextureDescriptor {
        label: Some("backdrop water target"),
        size: wgpu::Extent3d {
            width: size.0,
            height: size.1,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | usage,
        view_formats: &[],
    })
}
fn view(t: &wgpu::Texture) -> wgpu::TextureView {
    t.create_view(&Default::default())
}
fn color_attachment(
    view: &wgpu::TextureView,
    load: wgpu::LoadOp<wgpu::Color>,
) -> Option<wgpu::RenderPassColorAttachment<'_>> {
    Some(wgpu::RenderPassColorAttachment {
        view,
        depth_slice: None,
        resolve_target: None,
        ops: wgpu::Operations {
            load,
            store: wgpu::StoreOp::Store,
        },
    })
}
fn depth_attachment(
    view: &wgpu::TextureView,
    load: wgpu::LoadOp<f32>,
) -> Option<wgpu::RenderPassDepthStencilAttachment<'_>> {
    Some(wgpu::RenderPassDepthStencilAttachment {
        view,
        depth_ops: Some(wgpu::Operations {
            load,
            store: wgpu::StoreOp::Store,
        }),
        stencil_ops: None,
    })
}
/// An indexed or plain mesh: its position and normal buffers ( in the
/// material's attribute order ), optional index and count.
struct Mesh {
    buffers: Vec<wgpu::Buffer>,
    index: Option<(wgpu::Buffer, wgpu::IndexFormat)>,
    count: u32,
}
impl Mesh {
    fn draw(&self, pass: &mut wgpu::RenderPass) {
        for (slot, b) in self.buffers.iter().enumerate() {
            pass.set_vertex_buffer(slot as u32, b.slice(..));
        }
        match &self.index {
            Some((index, format)) => {
                pass.set_index_buffer(index.slice(..), *format);
                pass.draw_indexed(0..self.count, 0, 0..1);
            }
            None => pass.draw(0..self.count, 0..1),
        }
    }
}
/// One floating ice sphere: its Euler rotation, which turns about y, and
/// its object struct.
struct Ice {
    position: [f64; 2],
    rotation: [f64; 3],
    object: wgpu::Buffer,
}
struct Targets {
    width: u32,
    height: u32,
    format: wgpu::TextureFormat,
    scene: (wgpu::Texture, wgpu::TextureView),
    depth: (wgpu::Texture, wgpu::TextureView),
    backdrop: (wgpu::Texture, wgpu::TextureView),
    backdrop_depth: (wgpu::Texture, wgpu::TextureView),
    screen: RenderTarget,
    water: Draw,
    blur: [Draw; 2],
    blurred: [wgpu::TextureView; 2],
    output: Draw,
}
pub(super) struct Demo {
    controls: Controls,
    /// The floor position.
    floor_y: f64,
    /// The page's Timer: elapsed seconds and the last update.
    time: f64,
    last: f64,
    pending: bool,
    mixer: ThreeMixer,
    model: Object3D,
    skinned: Object3D,
    bones: Vec<(Object3D, Matrix4)>,
    background: Mesh,
    sphere: Mesh,
    michelle: Mesh,
    pillar: Mesh,
    water: Mesh,
    ice: Vec<Ice>,
    shadow: (wgpu::TextureView, wgpu::TextureView),
    background_draw: Draw,
    ice_draws: Vec<Draw>,
    shadow_draw: Draw,
    michelle_draw: Draw,
    pillar_draw: Draw,
    background_render: wgpu::Buffer,
    background_object: wgpu::Buffer,
    ice_render: wgpu::Buffer,
    shadow_render: wgpu::Buffer,
    shadow_object: wgpu::Buffer,
    michelle_render: wgpu::Buffer,
    michelle_object: wgpu::Buffer,
    bone_buffer: wgpu::Buffer,
    pillar_render: wgpu::Buffer,
    pillar_object: wgpu::Buffer,
    water_render: wgpu::Buffer,
    water_object: wgpu::Buffer,
    blur_renders: [wgpu::Buffer; 2],
    blur_objects: [wgpu::Buffer; 2],
    output_render: wgpu::Buffer,
    output_object: wgpu::Buffer,
    quad_uv: wgpu::Buffer,
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
            fov: 50.,
            near: 0.25,
            far: 30.,
            aspect,
            ..Default::default()
        }));
        s.get_mut(c)?.position = Vector3::new(3., 2., 4.);
        let mut controls = Controls::new(None, (1., 10.), PI * 0.9, true);
        controls.auto_rotate = Some(1.);
        controls.set_target(Vector3::new(0., 0.2, 0.));
        // init()'s controls.update() already turns by one auto-rotation step.
        controls.frame_update(s, c)?;
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
        let attribute = |g: &BufferGeometry, name: &str| -> Result<wgpu::Buffer> {
            let a = g
                .attributes
                .get(name)
                .ok_or(Error::Invalid("backdrop water attribute"))?;
            let data: Vec<f32> = (0..a.count())
                .flat_map(|i| (0..3).map(move |k| (i, k)))
                .map(|(i, k)| a.get_component(i, k).map(|v| v as f32))
                .collect::<Result<_>>()?;
            Ok(init("backdrop water", bytemuck::cast_slice(&data), vertex))
        };
        let mesh = |g: &BufferGeometry, names: [&str; 2]| -> Result<Mesh> {
            let index = g.index.clone();
            Ok(Mesh {
                buffers: vec![attribute(g, names[0])?, attribute(g, names[1])?],
                count: match &index {
                    Some(i) => i.len() as u32,
                    None => g
                        .attributes
                        .get("position")
                        .ok_or(Error::Invalid("backdrop water position"))?
                        .count() as u32,
                },
                index: index.map(|i| {
                    (
                        init(
                            "backdrop water index",
                            bytemuck::cast_slice(&i),
                            index_usage,
                        ),
                        wgpu::IndexFormat::Uint32,
                    )
                }),
            })
        };
        let background = mesh(&SphereGeometry::build(1., 32, 32)?, ["normal", "position"])?;
        let sphere = mesh(&IcosahedronGeometry::build(1., 3)?, ["position", "normal"])?;
        let pillar = mesh(
            &CylinderGeometry::build(1.1, 1.1, 10., 32, 1, false, 0., 2. * PI)?,
            ["position", "normal"],
        )?;
        let water = mesh(
            &BoxGeometry::build(50., 0.001, 50.)?,
            ["position", "normal"],
        )?;
        // The hundred ice spheres: rotation from the page's Math.random, in
        // a 10-column grid scaled by 3.5 ( z steps by i / 10, unfloored ).
        let mut random = FixtureRandom(186);
        let ice: Vec<Ice> = (0..100)
            .map(|i| -> Result<Ice> {
                let rotation = [random.next(), random.next(), random.next()];
                Ok(Ice {
                    position: [(i % 10) as f64 * 3.5 - 15.75, i as f64 / 10. * 3.5 - 17.5],
                    rotation,
                    object: uniform(r, "backdrop water ice", wgsl!("ice_fs"), "objectStruct")?,
                })
            })
            .collect::<Result<_>>()?;
        // Michelle: the glTF scene, its skinned mesh's attributes ( skin
        // weights normalized as GLTFLoader does ) and the four maps.
        let (asset, buffers, images) =
            load_asset("/web/gallery/assets/tsl-viewport/models/gltf/Michelle.glb").await?;
        let primitive = asset
            .meshes()
            .next()
            .and_then(|m| m.primitives().next())
            .ok_or(Error::Invalid("Michelle mesh"))?;
        let reader = primitive.reader(|b| buffers.get(b.index()).map(Vec::as_slice));
        let positions: Vec<f32> = reader
            .read_positions()
            .ok_or(Error::Invalid("Michelle positions"))?
            .flatten()
            .collect();
        let joints: Vec<u32> = reader
            .read_joints(0)
            .ok_or(Error::Invalid("Michelle joints"))?
            .into_u16()
            .flat_map(|j| j.map(u32::from))
            .collect();
        let weights: Vec<f32> = reader
            .read_weights(0)
            .ok_or(Error::Invalid("Michelle weights"))?
            .into_f32()
            .flat_map(|w| {
                let w = w.map(f64::from);
                let sum = w.iter().map(|v| v.abs()).sum::<f64>();
                let scale = 1. / sum;
                if scale.is_finite() {
                    w.map(|v| (v * scale) as f32)
                } else {
                    [1., 0., 0., 0.]
                }
            })
            .collect();
        let normals: Vec<f32> = reader
            .read_normals()
            .ok_or(Error::Invalid("Michelle normals"))?
            .flatten()
            .collect();
        let uv: Vec<f32> = reader
            .read_tex_coords(0)
            .ok_or(Error::Invalid("Michelle uv"))?
            .into_f32()
            .flatten()
            .collect();
        let index: Vec<u32> = reader
            .read_indices()
            .ok_or(Error::Invalid("Michelle index"))?
            .into_u32()
            .collect();
        let michelle = Mesh {
            buffers: vec![
                init("Michelle", bytemuck::cast_slice(&positions), vertex),
                init("Michelle", bytemuck::cast_slice(&joints), vertex),
                init("Michelle", bytemuck::cast_slice(&weights), vertex),
                init("Michelle", bytemuck::cast_slice(&normals), vertex),
                init("Michelle", bytemuck::cast_slice(&uv), vertex),
            ],
            index: Some((
                init("Michelle index", bytemuck::cast_slice(&index), index_usage),
                wgpu::IndexFormat::Uint32,
            )),
            count: index.len() as u32,
        };
        let skin = asset
            .skins()
            .next()
            .ok_or(Error::Invalid("Michelle skin"))?;
        let inverses: Vec<Matrix4> = skin
            .reader(|b| buffers.get(b.index()).map(Vec::as_slice))
            .read_inverse_bind_matrices()
            .ok_or(Error::Invalid("Michelle inverse bind matrices"))?
            .map(|m| Matrix4::from_cols_array(&std::array::from_fn(|i| m[i / 4][i % 4] as f64)))
            .collect();
        let material = primitive.material();
        let pbr = material.pbr_metallic_roughness();
        let source = |t: Option<gltf::Texture>| -> Result<usize> {
            t.map(|t| t.source().index())
                .ok_or(Error::Invalid("Michelle map"))
        };
        let base = source(pbr.base_color_texture().map(|t| t.texture()))?;
        let orm = source(pbr.metallic_roughness_texture().map(|t| t.texture()))?;
        let normal_map = source(material.normal_texture().map(|t| t.texture()))?;
        let specular = material
            .specular()
            .and_then(|s| s.specular_color_texture())
            .map(|t| t.texture().source().index())
            .ok_or(Error::Invalid("Michelle specular map"))?;
        let mut mipmaps = Mipmaps::new(r);
        let image = |i: usize| images.get(i).ok_or(Error::Invalid("Michelle image"));
        let maps = [
            mipmapped(r, &mut mipmaps, image(base)?, SRGB),
            mipmapped(r, &mut mipmaps, image(orm)?, BYTE),
            mipmapped(r, &mut mipmaps, image(specular)?, SRGB),
            mipmapped(r, &mut mipmaps, image(normal_map)?, BYTE),
        ];
        let instance =
            crate::gltf::import_animated_decoded(&asset, &buffers, &images)?.instantiate(s)?;
        let model = s.insert(NodeKind::Group);
        for &h in &instance.roots {
            s.add(model, h)?;
        }
        let skinned = *instance
            .meshes
            .first()
            .ok_or(Error::Invalid("Michelle skinned mesh"))?;
        let bones = skin
            .joints()
            .map(|j| instance.nodes[j.index()])
            .zip(inverses)
            .collect();
        let mut mixer = ThreeMixer::new();
        let action = mixer.clip_action(s, instance.clips[0].clone(), false)?;
        mixer.play(s, action)?;
        let mut water_image =
            decode_texture_image(&fetch("/web/gallery/assets/water.jpg").await?).await?;
        water_image.srgb = false;
        // TextureLoader's texture is flipped ( flipY ).
        let row = water_image.width as usize * 4;
        let water_image = crate::material::Texture {
            rgba: water_image
                .rgba
                .chunks(row)
                .rev()
                .flatten()
                .copied()
                .collect(),
            ..water_image
        };
        let water_map = mipmapped(r, &mut mipmaps, &water_image, BYTE);
        let repeat = r.device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::Repeat,
            address_mode_v: wgpu::AddressMode::Repeat,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        let clamp = r.device.create_sampler(&wgpu::SamplerDescriptor {
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
        let shadow_color = view(&texture(
            r,
            (SHADOW, SHADOW),
            BYTE,
            wgpu::TextureUsages::RENDER_ATTACHMENT,
        ));
        let shadow_depth = view(&texture(
            r,
            (SHADOW, SHADOW),
            DEPTH,
            wgpu::TextureUsages::RENDER_ATTACHMENT,
        ));
        let tex = wgpu::BindingResource::TextureView;
        let sampler = wgpu::BindingResource::Sampler;
        let two = [
            wgpu::vertex_attr_array![0 => Float32x3],
            wgpu::vertex_attr_array![1 => Float32x3],
        ];
        let two_layouts = [0, 1].map(|i| wgpu::VertexBufferLayout {
            array_stride: 12,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &two[i],
        });
        let skin_attrs = [
            wgpu::vertex_attr_array![0 => Float32x3],
            wgpu::vertex_attr_array![1 => Uint32x4],
            wgpu::vertex_attr_array![2 => Float32x4],
            wgpu::vertex_attr_array![3 => Float32x3],
            wgpu::vertex_attr_array![4 => Float32x2],
        ];
        let skin_layouts = [(0, 12), (1, 16), (2, 16), (3, 12), (4, 8)].map(|(i, stride)| {
            wgpu::VertexBufferLayout {
                array_stride: stride,
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes: &skin_attrs[i],
            }
        });
        let triangles = (1, wgpu::PrimitiveTopology::TriangleList);
        let draw = |p: &wgpu::RenderPipeline,
                    render: &wgpu::Buffer,
                    object: &[(u32, wgpu::BindingResource)]|
         -> Draw {
            (
                p.clone(),
                vec![
                    bind(
                        r,
                        p.get_bind_group_layout(0),
                        &[(0, render.as_entire_binding())],
                    ),
                    bind(r, p.get_bind_group_layout(1), object),
                ],
            )
        };
        let background_render = uniform(
            r,
            "backdrop water background",
            wgsl!("background_fs"),
            "renderStruct",
        )?;
        let background_object = uniform(
            r,
            "backdrop water background",
            wgsl!("background_fs"),
            "objectStruct",
        )?;
        // The background mesh: the sphere's inside, no depth write.
        let background_pipeline = sampled_pipeline(
            r,
            "backdrop water background",
            (wgsl!("background_vs"), wgsl!("background_fs")),
            &two_layouts,
            &[HALF],
            Some((wgpu::CompareFunction::Always, false)),
            (true, false),
            triangles,
        );
        let background_draw = draw(
            &background_pipeline,
            &background_render,
            &[(0, background_object.as_entire_binding())],
        );
        let lit = |label, shaders, layouts: &[wgpu::VertexBufferLayout], format| {
            sampled_pipeline(
                r,
                label,
                shaders,
                layouts,
                &[format],
                Some((wgpu::CompareFunction::LessEqual, true)),
                (false, false),
                triangles,
            )
        };
        let ice_render = uniform(r, "backdrop water ice", wgsl!("ice_fs"), "renderStruct")?;
        let ice_pipeline = lit(
            "backdrop water ice",
            (wgsl!("ice_vs"), wgsl!("ice_fs")),
            &two_layouts,
            HALF,
        );
        let ice_draws = ice
            .iter()
            .map(|i| {
                draw(
                    &ice_pipeline,
                    &ice_render,
                    &[
                        (0, sampler(&repeat)),
                        (1, tex(&water_map)),
                        (2, i.object.as_entire_binding()),
                        (3, sampler(&clamp)),
                        (4, tex(&r.dfg)),
                    ],
                )
            })
            .collect();
        let bone_buffer = r.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Michelle bones"),
            size: 64 * 65,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        // Michelle is double-sided: no culling in either pass.
        let no_cull = |label, shaders: (&str, &str), format| {
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
                        buffers: &skin_layouts,
                    },
                    fragment: Some(wgpu::FragmentState {
                        module: &module(shaders.1),
                        entry_point: Some("main"),
                        compilation_options: Default::default(),
                        targets: &[Some(wgpu::ColorTargetState::from(format))],
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
        };
        let shadow_render = uniform(
            r,
            "backdrop water shadow",
            wgsl!("shadow_vs"),
            "renderStruct",
        )?;
        let shadow_object = uniform(
            r,
            "backdrop water shadow",
            wgsl!("shadow_vs"),
            "objectStruct",
        )?;
        let shadow_pipeline = no_cull(
            "backdrop water shadow",
            (wgsl!("shadow_vs"), wgsl!("shadow_fs")),
            BYTE,
        );
        let shadow_draw = draw(
            &shadow_pipeline,
            &shadow_render,
            &[
                (0, sampler(&repeat)),
                (1, tex(&maps[0])),
                (2, shadow_object.as_entire_binding()),
                (3, bone_buffer.as_entire_binding()),
            ],
        );
        let michelle_render = uniform(
            r,
            "backdrop water Michelle",
            wgsl!("michelle_fs"),
            "renderStruct",
        )?;
        let michelle_object = uniform(
            r,
            "backdrop water Michelle",
            wgsl!("michelle_fs"),
            "objectStruct",
        )?;
        let michelle_pipeline = no_cull(
            "backdrop water Michelle",
            (wgsl!("michelle_vs"), wgsl!("michelle_fs")),
            HALF,
        );
        let michelle_draw = draw(
            &michelle_pipeline,
            &michelle_render,
            &[
                (0, michelle_object.as_entire_binding()),
                (1, sampler(&repeat)),
                (2, tex(&maps[0])),
                (3, sampler(&repeat)),
                (4, tex(&maps[1])),
                (5, sampler(&repeat)),
                (6, tex(&maps[2])),
                (7, sampler(&clamp)),
                (8, tex(&r.dfg)),
                (9, sampler(&repeat)),
                (10, tex(&maps[3])),
                (11, sampler(&compare)),
                (12, tex(&shadow_depth)),
                (13, bone_buffer.as_entire_binding()),
            ],
        );
        let pillar_render = uniform(
            r,
            "backdrop water pillar",
            wgsl!("floor_fs"),
            "renderStruct",
        )?;
        let pillar_object = uniform(
            r,
            "backdrop water pillar",
            wgsl!("floor_fs"),
            "objectStruct",
        )?;
        let pillar_pipeline = lit(
            "backdrop water pillar",
            (wgsl!("floor_vs"), wgsl!("floor_fs")),
            &two_layouts,
            HALF,
        );
        let pillar_draw = draw(
            &pillar_pipeline,
            &pillar_render,
            &[
                (0, sampler(&repeat)),
                (1, tex(&water_map)),
                (2, pillar_object.as_entire_binding()),
                (3, sampler(&clamp)),
                (4, tex(&r.dfg)),
                (5, sampler(&compare)),
                (6, tex(&shadow_depth)),
            ],
        );
        Ok(Self {
            controls,
            floor_y: 0.2,
            time: 0.,
            last: 0.,
            pending: true,
            mixer,
            model,
            skinned,
            bones,
            background,
            sphere,
            michelle,
            pillar,
            water,
            ice,
            shadow: (shadow_color, shadow_depth),
            background_draw,
            ice_draws,
            shadow_draw,
            michelle_draw,
            pillar_draw,
            background_render,
            background_object,
            ice_render,
            shadow_render,
            shadow_object,
            michelle_render,
            michelle_object,
            bone_buffer,
            pillar_render,
            pillar_object,
            water_render: uniform(r, "backdrop water", wgsl!("water_fs"), "renderStruct")?,
            water_object: uniform(r, "backdrop water", wgsl!("water_fs"), "objectStruct")?,
            blur_renders: [
                uniform(r, "backdrop water blur", wgsl!("blur_h_fs"), "renderStruct")?,
                uniform(r, "backdrop water blur", wgsl!("blur_v_fs"), "renderStruct")?,
            ],
            blur_objects: [
                uniform(r, "backdrop water blur", wgsl!("blur_h_fs"), "objectStruct")?,
                uniform(r, "backdrop water blur", wgsl!("blur_v_fs"), "objectStruct")?,
            ],
            output_render: uniform(
                r,
                "backdrop water output",
                wgsl!("output_fs"),
                "renderStruct",
            )?,
            output_object: uniform(
                r,
                "backdrop water output",
                wgsl!("output_fs"),
                "objectStruct",
            )?,
            quad_uv: init(
                "backdrop water quad",
                bytemuck::cast_slice(&[0f32, -1., 0., 1., 2., 1.]),
                vertex,
            ),
            clamp,
            targets: None,
        })
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
    fn resize(&mut self, r: &Renderer, out: &RenderTarget) -> Result<()> {
        let (width, height) = (out.width, out.height);
        let size = (width, height);
        let attach = wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC;
        let make = |format, usage| {
            let t = texture(r, size, format, usage);
            let v = view(&t);
            (t, v)
        };
        let scene = make(HALF, attach);
        let depth = make(DEPTH, attach);
        let backdrop = make(HALF, wgpu::TextureUsages::COPY_DST);
        let backdrop_depth = make(DEPTH, wgpu::TextureUsages::COPY_DST);
        let blurred = [
            make(HALF, wgpu::TextureUsages::RENDER_ATTACHMENT).1,
            make(HALF, wgpu::TextureUsages::RENDER_ATTACHMENT).1,
        ];
        let tex = wgpu::BindingResource::TextureView;
        let sampler = wgpu::BindingResource::Sampler;
        let two = [
            wgpu::vertex_attr_array![0 => Float32x3],
            wgpu::vertex_attr_array![1 => Float32x3],
        ];
        let two_layouts = [0, 1].map(|i| wgpu::VertexBufferLayout {
            array_stride: 12,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &two[i],
        });
        let water_pipeline = sampled_pipeline(
            r,
            "backdrop water",
            (wgsl!("water_vs"), wgsl!("water_fs")),
            &two_layouts,
            &[HALF],
            Some((wgpu::CompareFunction::LessEqual, true)),
            (false, true),
            (1, wgpu::PrimitiveTopology::TriangleList),
        );
        let water = (
            water_pipeline.clone(),
            vec![
                bind(
                    r,
                    water_pipeline.get_bind_group_layout(0),
                    &[(0, self.water_render.as_entire_binding())],
                ),
                bind(
                    r,
                    water_pipeline.get_bind_group_layout(1),
                    &[
                        (0, self.water_object.as_entire_binding()),
                        (1, tex(&backdrop.1)),
                        (2, tex(&backdrop_depth.1)),
                        (3, tex(&backdrop_depth.1)),
                    ],
                ),
            ],
        );
        let quad_attrs = [wgpu::vertex_attr_array![0 => Float32x2]];
        let quad = [wgpu::VertexBufferLayout {
            array_stride: 8,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &quad_attrs[0],
        }];
        let screen = |label, shaders, format| {
            sampled_pipeline(
                r,
                label,
                shaders,
                &quad,
                &[format],
                None,
                (false, false),
                (1, wgpu::PrimitiveTopology::TriangleList),
            )
        };
        let blur = [
            (wgsl!("blur_h_fs"), &scene.1, 0),
            (wgsl!("blur_v_fs"), &blurred[0], 1),
        ]
        .map(|(fs, source, i)| {
            let p = screen("backdrop water blur", (wgsl!("blur_vs"), fs), HALF);
            (
                p.clone(),
                vec![
                    bind(
                        r,
                        p.get_bind_group_layout(0),
                        &[(0, self.blur_renders[i].as_entire_binding())],
                    ),
                    bind(
                        r,
                        p.get_bind_group_layout(1),
                        &[
                            (0, sampler(&self.clamp)),
                            (1, tex(source)),
                            (2, self.blur_objects[i].as_entire_binding()),
                            (3, tex(&depth.1)),
                        ],
                    ),
                ],
            )
        });
        let format = out.options.format;
        let output_pipeline = screen(
            "backdrop water output",
            (OUTPUT_VS, wgsl!("output_fs")),
            format,
        );
        let output = (
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
                        (0, self.output_object.as_entire_binding()),
                        (1, sampler(&self.clamp)),
                        (2, tex(&blurred[1])),
                    ],
                ),
            ],
        );
        let screen_target = RenderTarget::with_options(
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
            depth,
            backdrop,
            backdrop_depth,
            screen: screen_target,
            water,
            blur,
            blurred,
            output,
        });
        Ok(())
    }
    fn present(&self, encoder: &mut wgpu::CommandEncoder, t: &Targets) {
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("backdrop water output"),
            color_attachments: &[color_attachment(
                &t.screen.view,
                wgpu::LoadOp::Clear(wgpu::Color::BLACK),
            )],
            ..Default::default()
        });
        set(&mut pass, &t.output);
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
        let t = self
            .targets
            .as_ref()
            .ok_or(Error::Invalid("backdrop water targets"))?;
        if !std::mem::take(&mut self.pending) {
            let mut encoder = r.device.create_command_encoder(&Default::default());
            self.present(&mut encoder, t);
            r.queue.submit([encoder.finish()]);
            return Ok(true);
        }
        // animate(): the timer, the auto-rotating controls, the floor, the
        // mixer and the model, and the ice spheres.
        let elapsed = self.time;
        let delta = (elapsed - self.last).max(0.);
        self.last = elapsed;
        self.controls.frame_update(s, c)?;
        let floor_y = self.floor_y - 5.;
        self.mixer.update(s, delta)?;
        s.get_mut(self.model)?.position.y = self.floor_y;
        for ice in &mut self.ice {
            ice.rotation[1] += delta * 0.3;
        }
        s.update()?;
        let (camera, world) = s.camera(c)?;
        let (projection, view_matrix) = (camera.projection_matrix()?, world.inverse());
        let camera_position = world.w_axis.truncate();
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
        ];
        let sun = linear(0xffe499, 5.);
        let (water_sky, water_ground) = (linear(0x333366, 5.), linear(0x74ccf4, 5.));
        let sky = linear(0x74ccf4, 1.);
        let up = vec![0., 1., 0.];
        let fog = linear(0x0487e2, 1.);
        // The lights as the shaders name them: sun color, the water and sky
        // hemisphere lights ( sky, direction, ground ), the sun position and
        // target, and the fog color, near and far.
        let lights = |names: [&'static str; 13]| -> Vec<(&'static str, Vec<f64>)> {
            vec![
                (names[0], sun.clone()),
                (names[1], water_sky.clone()),
                (names[2], up.clone()),
                (names[3], water_ground.clone()),
                (names[4], sky.clone()),
                (names[5], up.clone()),
                (names[6], vec![0.; 3]),
                (names[7], vec![0.5, 3., 0.5]),
                (names[8], vec![0.; 3]),
                (names[9], fog.clone()),
                (names[10], vec![7.]),
                (names[11], vec![25.]),
                (names[12], vec![]),
            ]
            .into_iter()
            .filter(|(n, _)| !n.is_empty())
            .collect()
        };
        // The sun's shadow camera ( orthographic ±1.5, near 0.5, far 15 ).
        let light = Vector3::new(0.5, 3., 0.5);
        let shadow_view = Matrix4::look_at_rh(light, Vector3::ZERO, Vector3::Y);
        let (near, far) = (0.5, 15.);
        let shadow_projection = Matrix4::from_cols_array(&[
            1. / 1.5,
            0.,
            0.,
            0.,
            0.,
            1. / 1.5,
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
        let size = vec![t.width as f64, t.height as f64];
        let tsl_time = vec![elapsed];
        write(
            &self.background_render,
            wgsl!("background_fs"),
            "renderStruct",
            &[
                ("nodeUniform3", vec![1.]),
                camera_values[0].clone(),
                camera_values[1].clone(),
            ],
        )?;
        write(
            &self.background_object,
            wgsl!("background_fs"),
            "objectStruct",
            &[
                ("nodeUniform1", identity3.clone()),
                ("nodeUniform4", vec![1.]),
                ("nodeUniform6", m4(Matrix4::IDENTITY)),
            ],
        )?;
        let mut ice_values = camera_values.to_vec();
        ice_values.extend(lights([
            "nodeUniform13",
            "nodeUniform15",
            "nodeUniform16",
            "nodeUniform14",
            "nodeUniform18",
            "nodeUniform19",
            "nodeUniform17",
            "nodeUniform11",
            "nodeUniform12",
            "nodeUniform20",
            "nodeUniform21",
            "nodeUniform22",
            "",
        ]));
        write(
            &self.ice_render,
            wgsl!("ice_fs"),
            "renderStruct",
            &ice_values,
        )?;
        // object.position.y = sin( elapsed + object.id ) × 0.3 inside the group
        // at ( −15.75, −1, −17.5 ).
        let mut opaque: Vec<(f64, usize)> = vec![];
        let view_projection = projection * view_matrix;
        let frustum = Frustum::from_projection(view_projection);
        for (i, ice) in self.ice.iter().enumerate() {
            let y = (elapsed + FIRST_ICE_ID + i as f64).sin() * 0.3 - 1.;
            let model = compose([ice.position[0], y, ice.position[1]], ice.rotation, 1.);
            write(
                &ice.object,
                wgsl!("ice_fs"),
                "objectStruct",
                &[
                    ("nodeUniform1", vec![1.]),
                    ("nodeUniform2", vec![0.]),
                    ("nodeUniform3", vec![1.]),
                    ("nodeUniform5", m3(model.inverse().transpose())),
                    ("nodeUniform6", vec![0.; 3]),
                    ("nodeUniform7", vec![1.]),
                    ("nodeUniform9", m4(model)),
                ],
            )?;
            let center = model.w_axis.truncate();
            if frustum.intersects_sphere(Sphere { center, radius: 1. }) {
                opaque.push(((view_projection * center.extend(1.)).z, i));
            }
        }
        // Michelle: the bind matrix is the identity; bindMatrixInverse is
        // the inverse of the skinned mesh's world matrix.
        let mesh_world = s.get(self.skinned)?.matrix_world;
        let bones: Vec<f32> = self
            .bones
            .iter()
            .map(|(h, inverse)| Ok(s.get(*h)?.matrix_world * *inverse))
            .collect::<Result<Vec<Matrix4>>>()?
            .iter()
            .flat_map(|m| m.to_cols_array().map(|v| v as f32))
            .collect();
        r.queue
            .write_buffer(&self.bone_buffer, 0, bytemuck::cast_slice(&bones));
        let bind_inverse = mesh_world.inverse();
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
            wgsl!("shadow_vs"),
            "objectStruct",
            &[
                ("nodeUniform0", m4(bind_inverse)),
                ("nodeUniform2", m4(Matrix4::IDENTITY)),
                ("nodeUniform4", identity3.clone()),
                ("nodeUniform5", vec![1.]),
                ("nodeUniform8", m4(mesh_world)),
            ],
        )?;
        let shadow_values = |names: [&'static str; 6]| -> Vec<(&'static str, Vec<f64>)> {
            vec![
                (names[0], m4(shadow_matrix)),
                (names[1], vec![0.]),
                (names[2], vec![-0.001]),
                (names[3], vec![1.]),
                (names[4], vec![SHADOW as f64; 2]),
                (names[5], vec![1.]),
            ]
        };
        let mut michelle_values = camera_values.to_vec();
        michelle_values.extend(lights([
            "nodeUniform29",
            "nodeUniform38",
            "nodeUniform39",
            "nodeUniform37",
            "nodeUniform41",
            "nodeUniform42",
            "nodeUniform40",
            "nodeUniform27",
            "nodeUniform28",
            "nodeUniform43",
            "nodeUniform44",
            "nodeUniform45",
            "",
        ]));
        michelle_values.extend(shadow_values([
            "nodeUniform30",
            "nodeUniform31",
            "nodeUniform32",
            "nodeUniform34",
            "nodeUniform35",
            "nodeUniform36",
        ]));
        write(
            &self.michelle_render,
            wgsl!("michelle_fs"),
            "renderStruct",
            &michelle_values,
        )?;
        write(
            &self.michelle_object,
            wgsl!("michelle_fs"),
            "objectStruct",
            &[
                ("nodeUniform0", m4(bind_inverse)),
                ("nodeUniform2", m4(Matrix4::IDENTITY)),
                ("nodeUniform3", vec![1.; 3]),
                ("nodeUniform5", identity3.clone()),
                ("nodeUniform6", vec![1.]),
                ("nodeUniform7", vec![0.5]),
                ("nodeUniform9", identity3.clone()),
                ("nodeUniform10", vec![1.]),
                ("nodeUniform11", identity3.clone()),
                ("nodeUniform13", m3(mesh_world.inverse().transpose())),
                ("nodeUniform14", vec![1.45]),
                ("nodeUniform15", vec![1.; 3]),
                ("nodeUniform17", identity3.clone()),
                ("nodeUniform18", vec![1.]),
                ("nodeUniform19", vec![0.; 3]),
                ("nodeUniform20", vec![1.]),
                ("nodeUniform22", m4(mesh_world)),
                ("nodeUniform24", identity3.clone()),
                ("nodeUniform25", vec![1., -1.]),
            ],
        )?;
        let pillar_model = Matrix4::from_translation(Vector3::new(0., floor_y, 0.));
        let mut pillar_values = vec![("nodeUniform2", tsl_time.clone())];
        pillar_values.extend(camera_values.to_vec());
        pillar_values.extend(lights([
            "nodeUniform14",
            "nodeUniform23",
            "nodeUniform24",
            "nodeUniform22",
            "nodeUniform26",
            "nodeUniform27",
            "nodeUniform25",
            "nodeUniform12",
            "nodeUniform13",
            "nodeUniform28",
            "nodeUniform29",
            "nodeUniform30",
            "",
        ]));
        pillar_values.extend(shadow_values([
            "nodeUniform15",
            "nodeUniform16",
            "nodeUniform17",
            "nodeUniform21",
            "nodeUniform20",
            "nodeUniform19",
        ]));
        write(
            &self.pillar_render,
            wgsl!("floor_fs"),
            "renderStruct",
            &pillar_values,
        )?;
        write(
            &self.pillar_object,
            wgsl!("floor_fs"),
            "objectStruct",
            &[
                ("nodeUniform1", m4(pillar_model)),
                ("nodeUniform4", identity3.clone()),
                ("nodeUniform6", vec![1.]),
                ("nodeUniform7", vec![0.]),
                ("nodeUniform8", vec![1.]),
                ("nodeUniform9", vec![0.; 3]),
                ("nodeUniform10", vec![1.]),
            ],
        )?;
        let mut water_values = vec![
            ("cameraNear", vec![0.25]),
            ("cameraFar", vec![30.]),
            ("nodeUniform1", tsl_time),
        ];
        water_values.extend(camera_values.to_vec());
        water_values.extend(lights([
            "",
            "nodeUniform4",
            "nodeUniform8",
            "nodeUniform3",
            "nodeUniform10",
            "nodeUniform11",
            "nodeUniform9",
            "",
            "",
            "nodeUniform18",
            "nodeUniform19",
            "nodeUniform20",
            "",
        ]));
        water_values.push(("nodeUniform13", size.clone()));
        write(
            &self.water_render,
            wgsl!("water_fs"),
            "renderStruct",
            &water_values,
        )?;
        write(
            &self.water_object,
            wgsl!("water_fs"),
            "objectStruct",
            &[
                ("nodeUniform0", m4(Matrix4::IDENTITY)),
                ("nodeUniform2", vec![1.]),
                ("nodeUniform6", identity3.clone()),
            ],
        )?;
        let camera_xyz = camera_position.to_array().to_vec();
        for (i, fs) in [wgsl!("blur_h_fs"), wgsl!("blur_v_fs")]
            .into_iter()
            .enumerate()
        {
            write(
                &self.blur_renders[i],
                fs,
                "renderStruct",
                &[("nodeUniform2", size.clone())],
            )?;
            write(
                &self.blur_objects[i],
                fs,
                "objectStruct",
                &[
                    ("nodeUniform1", camera_xyz.clone()),
                    ("nodeUniform3", vec![0.25]),
                    ("nodeUniform4", vec![30.]),
                    (
                        "nodeUniform6",
                        vec![1. / t.width as f64, 1. / t.height as f64],
                    ),
                ],
            )?;
        }
        write(
            &self.output_render,
            wgsl!("output_fs"),
            "renderStruct",
            &[("nodeUniform1", size)],
        )?;
        write(
            &self.output_object,
            wgsl!("output_fs"),
            "objectStruct",
            &[("nodeUniform0", camera_xyz)],
        )?;
        let mut encoder = r.device.create_command_encoder(&Default::default());
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("backdrop water shadow"),
                color_attachments: &[color_attachment(
                    &self.shadow.0,
                    wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                )],
                depth_stencil_attachment: depth_attachment(&self.shadow.1, wgpu::LoadOp::Clear(1.)),
                ..Default::default()
            });
            set(&mut pass, &self.shadow_draw);
            self.michelle.draw(&mut pass);
        }
        // Opaque objects front to back by clip depth: the ice spheres, the
        // skinned model ( by its origin ) and the pillar.
        const MICHELLE: usize = usize::MAX;
        const PILLAR: usize = usize::MAX - 1;
        let model_origin = s.get(self.model)?.matrix_world.w_axis;
        opaque.push(((view_projection * model_origin).z, MICHELLE));
        if frustum.intersects_sphere(Sphere {
            center: Vector3::new(0., floor_y, 0.),
            radius: (1.1f64 * 1.1 + 25.).sqrt(),
        }) {
            opaque.push((
                (view_projection * Vector4::new(0., floor_y, 0., 1.)).z,
                PILLAR,
            ));
        }
        opaque.sort_by(|a, b| a.0.total_cmp(&b.0));
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("backdrop water scene"),
                color_attachments: &[color_attachment(
                    &t.scene.1,
                    wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                )],
                depth_stencil_attachment: depth_attachment(&t.depth.1, wgpu::LoadOp::Clear(1.)),
                ..Default::default()
            });
            set(&mut pass, &self.background_draw);
            self.background.draw(&mut pass);
            for (_, i) in opaque {
                match i {
                    MICHELLE => {
                        set(&mut pass, &self.michelle_draw);
                        self.michelle.draw(&mut pass);
                    }
                    PILLAR => {
                        set(&mut pass, &self.pillar_draw);
                        self.pillar.draw(&mut pass);
                    }
                    i => {
                        set(&mut pass, &self.ice_draws[i]);
                        self.sphere.draw(&mut pass);
                    }
                }
            }
        }
        // viewportSharedTexture and viewportDepthTexture: the framebuffer's
        // color and depth before the transparent water.
        let full = wgpu::Extent3d {
            width: t.width,
            height: t.height,
            depth_or_array_layers: 1,
        };
        encoder.copy_texture_to_texture(
            t.scene.0.as_image_copy(),
            t.backdrop.0.as_image_copy(),
            full,
        );
        encoder.copy_texture_to_texture(
            t.depth.0.as_image_copy(),
            t.backdrop_depth.0.as_image_copy(),
            full,
        );
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("backdrop water water"),
                color_attachments: &[color_attachment(&t.scene.1, wgpu::LoadOp::Load)],
                depth_stencil_attachment: depth_attachment(&t.depth.1, wgpu::LoadOp::Load),
                ..Default::default()
            });
            if frustum.intersects_sphere(Sphere {
                center: Vector3::ZERO,
                radius: (25f64 * 25. * 2. + 0.0005 * 0.0005).sqrt(),
            }) {
                set(&mut pass, &t.water);
                self.water.draw(&mut pass);
            }
        }
        for (i, draw) in t.blur.iter().enumerate() {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("backdrop water blur"),
                color_attachments: &[color_attachment(
                    &t.blurred[i],
                    wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                )],
                ..Default::default()
            });
            set(&mut pass, draw);
            pass.set_vertex_buffer(0, self.quad_uv.slice(..));
            pass.draw(0..3, 0..1);
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
            // The wheel handler updates at once, with no pointer down: the
            // auto-rotation turns a step with the dolly.
            self.controls.dolly(wheel, &camera, Vector2::ZERO);
            self.controls.frame_update(s, c)?;
        } else if pan {
            self.controls.pan(&camera, dx, dy, height);
        } else {
            self.controls.rotate(dx, dy, height);
        }
        Ok(())
    }
    /// The floor position.
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        if index != 0 {
            return Err(Error::Invalid("backdrop water parameter"));
        }
        self.floor_y = value as f64;
        Ok(())
    }
    pub fn seek(&mut self, t: f64) {
        self.time = t;
        self.pending = true;
    }
}
