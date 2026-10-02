//! webgpu_water: the Draco pool.glb ( its walls and its transmissive,
//! emissive clutter ) under moonless_golf's UltraHDR sky, four floor planes
//! and Water2Mesh's water: the flowing normal maps mix the refraction ( the
//! framebuffer copied before the water ) and the reflection ( a
//! ReflectorNode render of the scene from the mirrored camera ) by the
//! Fresnel term. The scene pass writes the beauty and emissive MRT; the
//! emissive feeds BloomNode, the output adds it, tone maps ( ACES at 0.5 )
//! and encodes sRGB, and FXAA resolves the edges. Every stage runs the WGSL
//! three.js r186 generates for the page (in `water/`; the PMREM, cube, bloom
//! blur and quad modules are the deferred, volume_caustics and ssr ones,
//! byte-identical).
use super::controls_attributes::{Controls, camera_state};
use super::deferred::{Draw, culled_pipeline, render_cube, set, ultra_hdr_environment};
use super::gltf_viewer::{decode_texture_image, fetch, load_asset};
use super::lights_projector::{m3, m4, pack};
use super::pmrem_cube_uv::{Pmrem, bind};
use super::retro::{mipmapped, mipmapped_raw, uniform};
use super::shadowmap_opacity::{Mipmaps, mip_count};
use crate::{
    Error, Result, camera::*, geometry::*, math::*, render_target::*, renderer::*, scene::*,
};
use std::f64::consts::PI;
use wgpu::util::DeviceExt;

const HALF: wgpu::TextureFormat = wgpu::TextureFormat::Rgba16Float;
const BYTE: wgpu::TextureFormat = wgpu::TextureFormat::Rgba8Unorm;
const SRGB: wgpu::TextureFormat = wgpu::TextureFormat::Rgba8UnormSrgb;
const DEPTH: wgpu::TextureFormat = wgpu::TextureFormat::Depth24Plus;
/// BloomNode's mip levels.
const LEVELS: usize = 5;
/// WaterNode's flow-map cycle and its half.
const CYCLE: f64 = 0.15;
const HALF_CYCLE: f64 = CYCLE * 0.5;
const FLOW_SPEED: f64 = 0.03;
/// The water's plane: Water2Mesh at y = 0.2, facing up.
const WATER_Y: f64 = 0.2;
/// The floors: position and scale of PlaneGeometry( 1, 1 ) rotated flat.
const FLOORS: [([f64; 3], [f64; 3]); 4] = [
    ([20., 0., 0.], [15., 1., 80.]),
    ([-20., 0., 0.], [15., 1., 80.]),
    ([0., 0., 30.], [30., 1., 20.]),
    ([0., 0., -30.], [30., 1., 20.]),
];
macro_rules! wgsl {
    ($name:literal) => {
        include_str!(concat!("water/", $name, ".wgsl"))
    };
}
const HIGH_VS: &str = include_str!("ssr/blur_vs.wgsl");
const BLUR_VS: &str = include_str!("volume_caustics/blur_vs.wgsl");
const QUAD_VS: &str = include_str!("volume_caustics/composite_vs.wgsl");
const BLUR_FS: [&str; LEVELS] = [
    include_str!("volume_caustics/blur0_fs.wgsl"),
    include_str!("volume_caustics/blur1_fs.wgsl"),
    include_str!("volume_caustics/blur2_fs.wgsl"),
    include_str!("volume_caustics/blur3_fs.wgsl"),
    include_str!("volume_caustics/blur4_fs.wgsl"),
];
/// A mesh's vertex buffers ( in the pipeline's slot order ), index and
/// local bounding sphere.
struct Mesh {
    buffers: Vec<wgpu::Buffer>,
    index: wgpu::Buffer,
    format: wgpu::IndexFormat,
    count: u32,
    sphere: Sphere,
}
impl Mesh {
    fn draw(&self, pass: &mut wgpu::RenderPass) {
        for (i, b) in self.buffers.iter().enumerate() {
            pass.set_vertex_buffer(i as u32, b.slice(..));
        }
        pass.set_index_buffer(self.index.slice(..), self.format);
        pass.draw_indexed(0..self.count, 0, 0..1);
    }
    fn visible(&self, frustum: &Frustum, model: Matrix4) -> bool {
        let (scale, _, _) = model.to_scale_rotation_translation();
        frustum.intersects_sphere(Sphere {
            center: model.transform_point3(self.sphere.center),
            radius: self.sphere.radius * scale.abs().max_element(),
        })
    }
}
fn bounds(positions: &[f32]) -> Sphere {
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
}
fn texture(
    r: &Renderer,
    size: (u32, u32),
    format: wgpu::TextureFormat,
    mips: u32,
    usage: wgpu::TextureUsages,
) -> wgpu::Texture {
    r.device.create_texture(&wgpu::TextureDescriptor {
        label: Some("water texture"),
        size: wgpu::Extent3d {
            width: size.0,
            height: size.1,
            depth_or_array_layers: 1,
        },
        mip_level_count: mips,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format,
        usage,
        view_formats: &[],
    })
}
fn view(t: &wgpu::Texture) -> wgpu::TextureView {
    t.create_view(&Default::default())
}
/// The bloom chain's targets and draws for one size.
struct Bloom {
    bright: wgpu::TextureView,
    levels: Vec<(wgpu::TextureView, wgpu::TextureView, (u32, u32))>,
    high: (wgpu::RenderPipeline, wgpu::BindGroup),
    blurs: Vec<[(wgpu::RenderPipeline, wgpu::BindGroup, wgpu::Buffer); 2]>,
    composite: (wgpu::RenderPipeline, wgpu::BindGroup),
}
/// The scene draws of one pass ( the main MRT pass or the reflector's ).
struct SceneDraws {
    background: Draw,
    /// One per floor.
    floor: Vec<Draw>,
    pool: Draw,
    /// Back and front faces ( the reflector draws the front faces only ).
    clutter: Vec<Draw>,
}
struct Targets {
    width: u32,
    height: u32,
    format: wgpu::TextureFormat,
    beauty: wgpu::Texture,
    beauty_view: wgpu::TextureView,
    emissive: wgpu::TextureView,
    depth: wgpu::Texture,
    depth_view: wgpu::TextureView,
    transmission: wgpu::Texture,
    transmission_levels: Vec<(wgpu::TextureView, wgpu::BindGroup)>,
    shared: wgpu::Texture,
    shared_depth: wgpu::Texture,
    reflection: wgpu::Texture,
    reflection_view: wgpu::TextureView,
    reflection_depth: wgpu::TextureView,
    reflection_transmission: wgpu::Texture,
    reflection_levels: Vec<(wgpu::TextureView, wgpu::BindGroup)>,
    main: SceneDraws,
    mirrored: SceneDraws,
    water: Draw,
    bloom: Bloom,
    ldr: wgpu::TextureView,
    output: Draw,
    fxaa: Draw,
    screen: RenderTarget,
}
/// The uniform buffers of one scene pass.
struct Uniforms {
    background_render: wgpu::Buffer,
    background_object: wgpu::Buffer,
    floor_render: wgpu::Buffer,
    floor_objects: Vec<wgpu::Buffer>,
    pool_render: wgpu::Buffer,
    pool_object: wgpu::Buffer,
    clutter_render: wgpu::Buffer,
    clutter_object: wgpu::Buffer,
}
pub(super) struct Demo {
    controls: Controls,
    pending: bool,
    time: f64,
    last: f64,
    /// WaterNode's flowConfig: the two offsets and the half cycle.
    flow: [f64; 3],
    /// color ( hex ), scale and the flow direction.
    color: u32,
    scale: f64,
    direction: [f64; 2],
    background: Mesh,
    floor: Mesh,
    walls: Mesh,
    clutter: Mesh,
    water: Mesh,
    cube: wgpu::TextureView,
    pmrem: Pmrem,
    walls_maps: [wgpu::TextureView; 3],
    clutter_maps: [wgpu::TextureView; 5],
    normal_maps: [wgpu::TextureView; 2],
    main: Uniforms,
    mirrored: Uniforms,
    water_render: wgpu::Buffer,
    water_object: wgpu::Buffer,
    high_object: wgpu::Buffer,
    composite_object: wgpu::Buffer,
    tints: wgpu::Buffer,
    output_render: wgpu::Buffer,
    fxaa_steps: wgpu::Buffer,
    fxaa_object: wgpu::Buffer,
    quad_uv: wgpu::Buffer,
    repeat: wgpu::Sampler,
    linear: wgpu::Sampler,
    trilinear: wgpu::Sampler,
    transmission_sampler: wgpu::Sampler,
    mipmaps: Mipmaps,
    targets: Option<Targets>,
}
fn floor_model((position, scale): ([f64; 3], [f64; 3])) -> Matrix4 {
    Matrix4::from_translation(Vector3::from_array(position))
        * Matrix4::from_scale(Vector3::from_array(scale))
}
fn water_model() -> Matrix4 {
    Matrix4::from_translation(Vector3::new(0., WATER_Y, -2.)) * Matrix4::from_rotation_x(-PI / 2.)
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, _id: u32, r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 45.,
            near: 0.1,
            far: 200.,
            aspect,
            ..Default::default()
        }));
        s.get_mut(c)?.position = Vector3::new(-20., 6., -30.);
        let mut controls = Controls::new(Some(0.05), (0., f64::INFINITY), PI, true);
        controls.set_target(Vector3::new(0., 0., -5.));
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
        let geometry = |g: &BufferGeometry, names: &[&str]| -> Result<Mesh> {
            let read = |name: &str| -> Result<Vec<f32>> {
                let a = g
                    .attributes
                    .get(name)
                    .ok_or(Error::Invalid("water attribute"))?;
                let size = if name == "uv" { 2 } else { 3 };
                (0..a.count())
                    .flat_map(|i| (0..size).map(move |k| (i, k)))
                    .map(|(i, k)| a.get_component(i, k).map(|v| v as f32))
                    .collect()
            };
            let index = g.index.clone().ok_or(Error::Invalid("water index"))?;
            Ok(Mesh {
                buffers: names
                    .iter()
                    .map(|n| {
                        read(n).map(|d| init("water geometry", bytemuck::cast_slice(&d), vertex))
                    })
                    .collect::<Result<_>>()?,
                index: init(
                    "water index",
                    bytemuck::cast_slice(&index),
                    wgpu::BufferUsages::INDEX,
                ),
                format: wgpu::IndexFormat::Uint32,
                count: index.len() as u32,
                sphere: bounds(&read("position")?),
            })
        };
        let background = geometry(&SphereGeometry::build(1., 32, 32)?, &["normal", "position"])?;
        let mut floor_geometry = PlaneGeometry::build(1., 1., 1, 1)?;
        floor_geometry.apply_matrix4(Matrix4::from_rotation_x(-PI / 2.))?;
        let floor = geometry(&floor_geometry, &["normal", "position"])?;
        let water = geometry(&PlaneGeometry::build(30., 40., 1, 1)?, &["uv", "position"])?;
        // The pool: two Draco primitives ( clutter, walls ) with uv, normal and position.
        let (asset, buffers, images) = load_asset("/web/gallery/assets/gltf/pool.glb").await?;
        let mut meshes = vec![];
        for node in asset.nodes() {
            let primitive = node
                .mesh()
                .and_then(|m| m.primitives().next())
                .ok_or(Error::Invalid("pool mesh"))?;
            let reader = primitive.reader(|b| buffers.get(b.index()).map(Vec::as_slice));
            let uv: Vec<f32> = reader
                .read_tex_coords(0)
                .ok_or(Error::Invalid("pool uv"))?
                .into_f32()
                .flatten()
                .collect();
            let normals: Vec<f32> = reader
                .read_normals()
                .ok_or(Error::Invalid("pool normals"))?
                .flatten()
                .collect();
            let positions: Vec<f32> = reader
                .read_positions()
                .ok_or(Error::Invalid("pool positions"))?
                .flatten()
                .collect();
            let index: Vec<u32> = reader
                .read_indices()
                .ok_or(Error::Invalid("pool index"))?
                .into_u32()
                .collect();
            meshes.push(Mesh {
                sphere: bounds(&positions),
                buffers: [&uv[..], &normals, &positions]
                    .iter()
                    .map(|a| init("pool", bytemuck::cast_slice(a), vertex))
                    .collect(),
                index: init(
                    "pool index",
                    bytemuck::cast_slice(&index),
                    wgpu::BufferUsages::INDEX,
                ),
                format: wgpu::IndexFormat::Uint32,
                count: index.len() as u32,
            });
        }
        let walls = meshes.pop().ok_or(Error::Invalid("pool walls"))?;
        let clutter = meshes.pop().ok_or(Error::Invalid("pool clutter"))?;
        let mut mipmaps = Mipmaps::new(r);
        let image = |i: usize| images.get(i).ok_or(Error::Invalid("pool image"));
        // By their EXT_texture_webp sources: the walls' map, metalness /
        // roughness and normal map are images 7, 6 and 5; the clutter's map,
        // metalness / roughness, normal, emissive and transmission maps are
        // images 4, 3, 2, 0 and 1.
        let walls_maps = [
            mipmapped(r, &mut mipmaps, image(7)?, SRGB),
            mipmapped(r, &mut mipmaps, image(6)?, BYTE),
            mipmapped(r, &mut mipmaps, image(5)?, BYTE),
        ];
        let clutter_maps = [
            mipmapped(r, &mut mipmaps, image(4)?, SRGB),
            mipmapped(r, &mut mipmaps, image(3)?, BYTE),
            mipmapped(r, &mut mipmaps, image(2)?, BYTE),
            mipmapped(r, &mut mipmaps, image(0)?, SRGB),
            mipmapped(r, &mut mipmaps, image(1)?, BYTE),
        ];
        // The water normal maps: TextureLoader's flipY, mipmapped.
        let mut normal_maps = vec![];
        for url in [
            "/web/gallery/assets/water/Water_1_M_Normal.jpg",
            "/web/gallery/assets/water/Water_2_M_Normal.jpg",
        ] {
            let image = decode_texture_image(&fetch(url).await?).await?;
            let row = image.width as usize * 4;
            let flipped: Vec<u8> = image.rgba.chunks(row).rev().flatten().copied().collect();
            normal_maps.push(mipmapped_raw(
                r,
                &mut mipmaps,
                &flipped,
                (image.width, image.height),
                4,
                BYTE,
            ));
        }
        let normal_maps: [wgpu::TextureView; 2] = normal_maps
            .try_into()
            .map_err(|_| Error::Invalid("water normal maps"))?;
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
        let trilinear = sampler(ClampToEdge, Linear, Linear);
        let transmission_sampler = sampler(ClampToEdge, Nearest, Linear);
        // scene.background and scene.environment: the equirect as a cube
        // ( CubeMapNode ) and its PMREM.
        let (equirect, height, pmrem, _) = ultra_hdr_environment(
            r,
            &linear,
            &sampler(ClampToEdge, Linear, Nearest),
            "/web/environments/moonless_golf_2k.hdr.jpg",
        )
        .await?;
        let cube = render_cube(r, &sampler(ClampToEdge, Linear, Nearest), &equirect, height)?;
        let uniforms = |prefix: &str| -> Result<Uniforms> {
            let (bg, floor, pool, clutter) = if prefix.is_empty() {
                (
                    wgsl!("background_fs"),
                    wgsl!("floor_fs"),
                    wgsl!("pool_fs"),
                    wgsl!("clutter_fs"),
                )
            } else {
                (
                    wgsl!("reflect_background_fs"),
                    wgsl!("reflect_floor_fs"),
                    wgsl!("reflect_pool_fs"),
                    wgsl!("reflect_clutter_fs"),
                )
            };
            Ok(Uniforms {
                background_render: uniform(r, "water background", bg, "renderStruct")?,
                background_object: uniform(r, "water background", bg, "objectStruct")?,
                floor_render: uniform(r, "water floor", floor, "renderStruct")?,
                floor_objects: (0..FLOORS.len())
                    .map(|_| uniform(r, "water floor", floor, "objectStruct"))
                    .collect::<Result<_>>()?,
                pool_render: uniform(r, "water pool", pool, "renderStruct")?,
                pool_object: uniform(r, "water pool", pool, "objectStruct")?,
                clutter_render: uniform(r, "water clutter", clutter, "renderStruct")?,
                clutter_object: uniform(r, "water clutter", clutter, "objectStruct")?,
            })
        };
        let tints: Vec<f32> = (0..LEVELS).flat_map(|_| [1f32, 1., 1., 0.]).collect();
        let steps: [f32; 24] = [
            1., 0., 0., 0., 1.5, 0., 0., 0., 2., 0., 0., 0., 2., 0., 0., 0., 2., 0., 0., 0., 4.,
            0., 0., 0.,
        ];
        let demo = Self {
            controls,
            pending: true,
            time: 0.,
            last: 0.,
            flow: [0., HALF_CYCLE, 0.],
            color: 0x99e0ff,
            scale: 2.,
            direction: [1., 1.],
            background,
            floor,
            walls,
            clutter,
            water,
            cube,
            pmrem,
            walls_maps,
            clutter_maps,
            normal_maps,
            main: uniforms("")?,
            mirrored: uniforms("reflect")?,
            water_render: uniform(r, "water", wgsl!("water_fs"), "renderStruct")?,
            water_object: uniform(r, "water", wgsl!("water_fs"), "objectStruct")?,
            high_object: uniform(r, "bloom high pass", wgsl!("high_fs"), "objectStruct")?,
            composite_object: uniform(r, "bloom composite", wgsl!("composite_fs"), "objectStruct")?,
            tints: init(
                "bloom tints",
                bytemuck::cast_slice(&tints),
                wgpu::BufferUsages::UNIFORM,
            ),
            output_render: uniform(r, "water output", wgsl!("output_fs"), "renderStruct")?,
            fxaa_steps: init(
                "fxaa steps",
                bytemuck::cast_slice(&steps),
                wgpu::BufferUsages::UNIFORM,
            ),
            fxaa_object: uniform(r, "fxaa", wgsl!("fxaa_fs"), "objectStruct")?,
            quad_uv: init(
                "quad uv",
                bytemuck::cast_slice(&[0f32, -1., 0., 1., 2., 1.]),
                vertex,
            ),
            repeat,
            linear,
            trilinear,
            transmission_sampler,
            mipmaps,
            targets: None,
        };
        demo.write_static(r)?;
        Ok(demo)
    }
    /// The object uniforms that never change: the backgrounds, the pool's
    /// materials, the bloom and the output settings.
    fn write_static(&self, r: &Renderer) -> Result<()> {
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
        let texel = [vec![1. / 1536.], vec![1. / 2048.]];
        let node = |model: Matrix4| m3(model.inverse().transpose());
        let walls_model = self.pool_models()?.1;
        let clutter_model = self.pool_models()?.0;
        for (u, prefix) in [(&self.main, false), (&self.mirrored, true)] {
            let (bg, pool, clutter) = if prefix {
                (
                    wgsl!("reflect_background_fs"),
                    wgsl!("reflect_pool_fs"),
                    wgsl!("reflect_clutter_fs"),
                )
            } else {
                (
                    wgsl!("background_fs"),
                    wgsl!("pool_fs"),
                    wgsl!("clutter_fs"),
                )
            };
            write(
                &u.background_object,
                bg,
                "objectStruct",
                &[
                    ("nodeUniform1", id4.clone()),
                    ("nodeUniform4", id3.clone()),
                    ("nodeUniform7", vec![1.]),
                    ("nodeUniform9", id4.clone()),
                ],
            )?;
            write(
                &u.pool_object,
                pool,
                "objectStruct",
                &[
                    ("nodeUniform0", vec![1.; 3]),
                    ("nodeUniform2", id3.clone()),
                    ("nodeUniform3", vec![1.]),
                    ("nodeUniform5", id3.clone()),
                    ("nodeUniform6", vec![0.]),
                    ("nodeUniform7", vec![1.]),
                    ("nodeUniform8", id3.clone()),
                    ("nodeUniform9", vec![1.]),
                    ("nodeUniform10", id3.clone()),
                    ("nodeUniform12", node(walls_model)),
                    ("nodeUniform13", vec![1.5]),
                    ("nodeUniform14", vec![1.; 3]),
                    ("nodeUniform15", vec![1.]),
                    ("nodeUniform16", vec![0.; 3]),
                    ("nodeUniform17", vec![1.]),
                    ("nodeUniform19", m4(walls_model)),
                    ("nodeUniform21", id3.clone()),
                    ("nodeUniform22", vec![1., -1.]),
                    ("nodeUniform23", vec![9.]),
                    ("nodeUniform24", id4.clone()),
                    ("nodeUniform26", texel[0].clone()),
                    ("nodeUniform27", texel[1].clone()),
                    ("nodeUniform29", vec![1.]),
                ],
            )?;
            write(
                &u.clutter_object,
                clutter,
                "objectStruct",
                &[
                    ("nodeUniform0", vec![1.; 3]),
                    ("nodeUniform2", id3.clone()),
                    ("nodeUniform3", vec![1.]),
                    ("nodeUniform5", id3.clone()),
                    ("nodeUniform6", vec![1.]),
                    ("nodeUniform7", vec![1.]),
                    ("nodeUniform8", id3.clone()),
                    ("nodeUniform9", vec![1.]),
                    ("nodeUniform10", id3.clone()),
                    ("nodeUniform12", node(clutter_model)),
                    ("nodeUniform13", vec![1.5]),
                    ("nodeUniform14", vec![1.; 3]),
                    ("nodeUniform15", vec![1.]),
                    ("nodeUniform16", vec![1.]),
                    ("nodeUniform18", id3.clone()),
                    ("nodeUniform19", vec![0.]),
                    ("nodeUniform20", vec![f64::INFINITY]),
                    ("nodeUniform21", vec![1.; 3]),
                    ("nodeUniform22", vec![1.; 3]),
                    ("nodeUniform23", vec![1.]),
                    ("nodeUniform25", id3.clone()),
                    ("nodeUniform28", m4(clutter_model)),
                    ("nodeUniform30", id3.clone()),
                    ("nodeUniform31", vec![1., -1.]),
                    ("nodeUniform33", m4(clutter_model)),
                    ("nodeUniform37", vec![9.]),
                    ("nodeUniform38", id4.clone()),
                    ("nodeUniform40", texel[0].clone()),
                    ("nodeUniform41", texel[1].clone()),
                    ("nodeUniform43", vec![1.]),
                ],
            )?;
        }
        write(
            &self.high_object,
            wgsl!("high_fs"),
            "objectStruct",
            &[("nodeUniform1", vec![0.]), ("nodeUniform2", vec![0.01])],
        )?;
        // bloom( emissivePass, 2 ): strength 2, radius 0.
        write(
            &self.composite_object,
            wgsl!("composite_fs"),
            "objectStruct",
            &[
                ("nodeUniform0", vec![0.]),
                ("nodeUniform3", id3.clone()),
                ("nodeUniform5", id3.clone()),
                ("nodeUniform7", id3.clone()),
                ("nodeUniform9", id3.clone()),
                ("nodeUniform11", id3),
                ("nodeUniform12", vec![2.]),
            ],
        )?;
        write(
            &self.output_render,
            wgsl!("output_fs"),
            "renderStruct",
            &[("nodeUniform2", vec![0.5])],
        )?;
        Ok(())
    }
    /// The clutter's and the walls' models ( the glTF nodes in order ).
    fn pool_models(&self) -> Result<(Matrix4, Matrix4)> {
        // Both glTF nodes rotate −90° about x and scale by 0.4361.
        let q = Quaternion::from_xyzw(-0.7071069478988566, 0., 0., 0.7071066144741992);
        let model = |scale: Vector3| {
            Matrix4::from_translation(Vector3::new(0., 0., 2.))
                * Matrix4::from_scale(Vector3::splat(0.1))
                * Matrix4::from_scale_rotation_translation(scale, q, Vector3::ZERO)
        };
        Ok((
            model(Vector3::new(
                0.43610429763793945,
                0.4361043790762458,
                0.4361043790762458,
            )),
            model(Vector3::new(
                0.43610429763793945,
                0.4361043790762458,
                0.4361043790762458,
            )),
        ))
    }
    fn scene_draws(
        &self,
        r: &Renderer,
        u: &Uniforms,
        mirrored: bool,
        transmission: &wgpu::TextureView,
    ) -> SceneDraws {
        let tex = wgpu::BindingResource::TextureView;
        let sampler = wgpu::BindingResource::Sampler;
        let formats: &[wgpu::TextureFormat] = if mirrored { &[HALF] } else { &[HALF, HALF] };
        let less = Some((wgpu::CompareFunction::LessEqual, true));
        let pair = [
            wgpu::vertex_attr_array![0 => Float32x3],
            wgpu::vertex_attr_array![1 => Float32x3],
        ];
        let pair_layouts = [0, 1].map(|i| wgpu::VertexBufferLayout {
            array_stride: 12,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &pair[i],
        });
        let triple = [
            wgpu::vertex_attr_array![0 => Float32x2],
            wgpu::vertex_attr_array![1 => Float32x3],
            wgpu::vertex_attr_array![2 => Float32x3],
        ];
        let triple_layouts =
            [(0, 8), (1, 12), (2, 12)].map(|(i, stride)| wgpu::VertexBufferLayout {
                array_stride: stride,
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes: &triple[i],
            });
        let pipeline = |label, shaders, layouts: &[wgpu::VertexBufferLayout], depth, cw, cull| {
            culled_pipeline(
                r,
                label,
                shaders,
                layouts,
                formats,
                depth,
                (cw, false),
                (1, wgpu::PrimitiveTopology::TriangleList),
                cull,
            )
        };
        let draw = |p: wgpu::RenderPipeline,
                    render: &wgpu::Buffer,
                    entries: &[(u32, wgpu::BindingResource)]|
         -> Draw {
            let groups = vec![
                bind(
                    r,
                    p.get_bind_group_layout(0),
                    &[(0, render.as_entire_binding())],
                ),
                bind(r, p.get_bind_group_layout(1), entries),
            ];
            (p, groups)
        };
        let (bg, floor, pool, clutter) = if mirrored {
            (
                (
                    wgsl!("reflect_background_vs"),
                    wgsl!("reflect_background_fs"),
                ),
                (wgsl!("reflect_floor_vs"), wgsl!("reflect_floor_fs")),
                (wgsl!("reflect_pool_vs"), wgsl!("reflect_pool_fs")),
                vec![(
                    wgsl!("reflect_clutter_vs"),
                    wgsl!("reflect_clutter_fs"),
                    false,
                )],
            )
        } else {
            (
                (wgsl!("background_vs"), wgsl!("background_fs")),
                (wgsl!("floor_vs"), wgsl!("floor_fs")),
                (wgsl!("pool_vs"), wgsl!("pool_fs")),
                vec![
                    (wgsl!("clutter_vs"), wgsl!("clutter_back_fs"), true),
                    (wgsl!("clutter_vs"), wgsl!("clutter_fs"), false),
                ],
            )
        };
        let back = Some(wgpu::Face::Back);
        let background = draw(
            pipeline(
                "water background",
                bg,
                &pair_layouts,
                Some((wgpu::CompareFunction::Always, false)),
                true,
                back,
            ),
            &u.background_render,
            &[
                (0, sampler(&self.trilinear)),
                (1, tex(&self.cube)),
                (2, u.background_object.as_entire_binding()),
            ],
        );
        let floor_pipeline = pipeline("water floor", floor, &pair_layouts, less, false, None);
        let floor = u
            .floor_objects
            .iter()
            .map(|object| {
                draw(
                    floor_pipeline.clone(),
                    &u.floor_render,
                    &[
                        (0, object.as_entire_binding()),
                        (1, sampler(&self.linear)),
                        (2, tex(&r.dfg)),
                        (3, sampler(&self.linear)),
                        (4, tex(&self.pmrem.view)),
                    ],
                )
            })
            .collect();
        let pool = draw(
            pipeline("water pool", pool, &triple_layouts, less, false, None),
            &u.pool_render,
            &[
                (0, u.pool_object.as_entire_binding()),
                (1, sampler(&self.repeat)),
                (2, tex(&self.walls_maps[0])),
                (3, sampler(&self.repeat)),
                (4, tex(&self.walls_maps[1])),
                (5, sampler(&self.linear)),
                (6, tex(&r.dfg)),
                (7, sampler(&self.repeat)),
                (8, tex(&self.walls_maps[2])),
                (9, sampler(&self.linear)),
                (10, tex(&self.pmrem.view)),
            ],
        );
        let clutter = clutter
            .into_iter()
            .map(|(vs, fs, cw)| {
                let m = &self.clutter_maps;
                draw(
                    pipeline("water clutter", (vs, fs), &triple_layouts, less, cw, back),
                    &u.clutter_render,
                    &[
                        (0, u.clutter_object.as_entire_binding()),
                        (1, sampler(&self.repeat)),
                        (2, tex(&m[0])),
                        (3, sampler(&self.repeat)),
                        (4, tex(&m[1])),
                        (5, sampler(&self.repeat)),
                        (6, tex(&m[2])),
                        (7, sampler(&self.repeat)),
                        (8, tex(&m[3])),
                        (9, sampler(&self.repeat)),
                        (10, tex(&m[4])),
                        (11, sampler(&self.linear)),
                        (12, tex(&r.dfg)),
                        (13, sampler(&self.transmission_sampler)),
                        (14, tex(transmission)),
                        (15, sampler(&self.linear)),
                        (16, tex(&self.pmrem.view)),
                    ],
                )
            })
            .collect();
        SceneDraws {
            background,
            floor,
            pool,
            clutter,
        }
    }
    fn resize(&mut self, r: &Renderer, out: &RenderTarget) -> Result<()> {
        let size = (out.width, out.height);
        let attach = wgpu::TextureUsages::RENDER_ATTACHMENT;
        let sampled = wgpu::TextureUsages::TEXTURE_BINDING;
        let copy_src = wgpu::TextureUsages::COPY_SRC;
        let copy_dst = wgpu::TextureUsages::COPY_DST;
        let beauty = texture(r, size, HALF, 1, attach | sampled | copy_src);
        let emissive = view(&texture(r, size, HALF, 1, attach | sampled));
        let depth = texture(r, size, DEPTH, 1, attach | copy_src);
        let mips = mip_count(size);
        let transmission = texture(r, size, HALF, mips, sampled | copy_dst | attach);
        let transmission_levels = self.mipmaps.levels(r, &transmission);
        let shared = texture(r, size, HALF, 1, sampled | copy_dst);
        let shared_depth = texture(r, size, DEPTH, 1, sampled | copy_dst);
        let reflection = texture(r, size, HALF, 1, attach | sampled | copy_src);
        let reflection_depth = view(&texture(r, size, DEPTH, 1, attach));
        let reflection_transmission = texture(r, size, HALF, mips, sampled | copy_dst | attach);
        let reflection_levels = self.mipmaps.levels(r, &reflection_transmission);
        let main = self.scene_draws(r, &self.main, false, &view(&transmission));
        let mirrored = self.scene_draws(r, &self.mirrored, true, &view(&reflection_transmission));
        let tex = wgpu::BindingResource::TextureView;
        let sampler = wgpu::BindingResource::Sampler;
        let reflection_view = view(&reflection);
        let water_attrs = [
            wgpu::vertex_attr_array![0 => Float32x2],
            wgpu::vertex_attr_array![1 => Float32x3],
        ];
        let water_layouts = [(0, 8), (1, 12)].map(|(i, stride)| wgpu::VertexBufferLayout {
            array_stride: stride,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &water_attrs[i],
        });
        let shared_view = view(&shared);
        let shared_depth_view = view(&shared_depth);
        let water_pipeline = culled_pipeline(
            r,
            "water",
            (wgsl!("water_vs"), wgsl!("water_fs")),
            &water_layouts,
            &[HALF, HALF],
            Some((wgpu::CompareFunction::LessEqual, true)),
            (false, true),
            (1, wgpu::PrimitiveTopology::TriangleList),
            Some(wgpu::Face::Back),
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
                        (1, sampler(&self.repeat)),
                        (2, tex(&self.normal_maps[0])),
                        (3, sampler(&self.repeat)),
                        (4, tex(&self.normal_maps[1])),
                        (5, tex(&shared_view)),
                        (6, tex(&shared_depth_view)),
                        (7, sampler(&self.linear)),
                        (8, tex(&reflection_view)),
                    ],
                ),
            ],
        );
        // BloomNode over the emissive MRT at half resolution.
        let uv = [wgpu::VertexBufferLayout {
            array_stride: 8,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &wgpu::vertex_attr_array![0 => Float32x2],
        }];
        let quad = |label, vs, fs, format| {
            culled_pipeline(
                r,
                label,
                (vs, fs),
                &uv,
                &[format],
                None,
                (false, false),
                (1, wgpu::PrimitiveTopology::TriangleList),
                Some(wgpu::Face::Back),
            )
        };
        let round = |(w, h): (u32, u32)| {
            (
                ((w as f64 / 2.).round() as u32).max(1),
                ((h as f64 / 2.).round() as u32).max(1),
            )
        };
        let target = |size| view(&texture(r, size, HALF, 1, attach | sampled));
        let half = round(size);
        let bright = target(half);
        let high = quad("bloom high pass", HIGH_VS, wgsl!("high_fs"), HALF);
        let high_group = bind(
            r,
            high.get_bind_group_layout(0),
            &[
                (0, sampler(&self.linear)),
                (1, tex(&emissive)),
                (2, self.high_object.as_entire_binding()),
            ],
        );
        let mut levels = vec![];
        let mut level_size = half;
        for _ in 0..LEVELS {
            levels.push((target(level_size), target(level_size), level_size));
            level_size = round(level_size);
        }
        let mut blurs = vec![];
        for (i, fs) in BLUR_FS.iter().enumerate() {
            let p = quad("bloom blur", BLUR_VS, fs, HALF);
            let input = if i == 0 { &bright } else { &levels[i - 1].1 };
            let pass = |input: &wgpu::TextureView| -> Result<_> {
                let object = uniform(r, "bloom blur", fs, "objectStruct")?;
                let group = bind(
                    r,
                    p.get_bind_group_layout(0),
                    &[
                        (0, sampler(&self.linear)),
                        (1, wgpu::BindingResource::TextureView(input)),
                        (2, object.as_entire_binding()),
                    ],
                );
                Ok((p.clone(), group, object))
            };
            blurs.push([pass(input)?, pass(&levels[i].0)?]);
        }
        let composite = quad("bloom composite", QUAD_VS, wgsl!("composite_fs"), HALF);
        let mut entries = vec![
            (0, self.composite_object.as_entire_binding()),
            (1, self.tints.as_entire_binding()),
        ];
        for (i, (_, vertical, _)) in levels.iter().enumerate() {
            entries.push((2 + i as u32 * 2, sampler(&self.linear)));
            entries.push((3 + i as u32 * 2, tex(vertical)));
        }
        let composite_group = bind(r, composite.get_bind_group_layout(0), &entries);
        // renderOutput( beauty + bloom ), then FXAA to the canvas.
        let ldr = target(size);
        let beauty_view = view(&beauty);
        let output_pipeline = quad("water output", QUAD_VS, wgsl!("output_fs"), HALF);
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
                        (0, sampler(&self.linear)),
                        (1, tex(&beauty_view)),
                        (2, sampler(&self.linear)),
                        (3, tex(&levels[0].0)),
                    ],
                ),
            ],
        );
        let fxaa_pipeline = quad(
            "fxaa",
            wgsl!("fxaa_vs"),
            wgsl!("fxaa_fs"),
            out.options.format,
        );
        let fxaa = (
            fxaa_pipeline.clone(),
            vec![bind(
                r,
                fxaa_pipeline.get_bind_group_layout(0),
                &[
                    (0, sampler(&self.linear)),
                    (1, tex(&ldr)),
                    (2, self.fxaa_steps.as_entire_binding()),
                    (3, self.fxaa_object.as_entire_binding()),
                ],
            )],
        );
        self.targets = Some(Targets {
            width: size.0,
            height: size.1,
            format: out.options.format,
            beauty_view,
            beauty,
            emissive,
            depth_view: view(&depth),
            depth,
            transmission,
            transmission_levels,
            shared,
            shared_depth,
            reflection,
            reflection_view,
            reflection_depth,
            reflection_transmission,
            reflection_levels,
            main,
            mirrored,
            water,
            bloom: Bloom {
                bright,
                levels,
                high: (high, high_group),
                blurs,
                composite: (composite, composite_group),
            },
            ldr,
            output,
            fxaa,
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
    /// WaterNode.updateFlow( delta ).
    fn update_flow(&mut self, delta: f64) {
        let f = &mut self.flow;
        f[0] += FLOW_SPEED * delta;
        f[1] = f[0] + HALF_CYCLE;
        if f[0] >= CYCLE {
            f[0] = 0.;
            f[1] = HALF_CYCLE;
        } else if f[1] >= CYCLE {
            f[1] -= CYCLE;
        }
        f[2] = HALF_CYCLE;
    }
    fn present(&self, encoder: &mut wgpu::CommandEncoder, t: &Targets) {
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("fxaa"),
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
        set(&mut pass, &t.fxaa);
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
            self.pending = true;
        }
        if !std::mem::take(&mut self.pending) {
            let t = self
                .targets
                .as_ref()
                .ok_or(Error::Invalid("water targets"))?;
            let mut encoder = r.device.create_command_encoder(&Default::default());
            self.present(&mut encoder, t);
            r.queue.submit([encoder.finish()]);
            return Ok(true);
        }
        // animate(): controls.update(), then the render, whose water node
        // steps its flow by the frame's delta.
        self.controls.frame_update(s, c)?;
        let delta = self.time - self.last;
        self.last = self.time;
        self.update_flow(delta);
        s.update()?;
        let t = self
            .targets
            .as_ref()
            .ok_or(Error::Invalid("water targets"))?;
        let (camera, world) = s.camera(c)?;
        let (projection, view) = (camera.projection_matrix()?, world.inverse());
        let camera_position = world.w_axis.truncate();
        // ReflectorNode: the virtual camera mirrored in the water plane and
        // its oblique near plane.
        let normal = Vector3::Y;
        let plane_point = Vector3::new(0., WATER_Y, -2.);
        let reflect = |v: Vector3| v - normal * (2. * v.dot(normal));
        let reflected = (plane_point - camera_position).dot(normal) <= 0.;
        let mirror_view = -reflect(plane_point - camera_position) + plane_point;
        let rotation = Matrix4::from_mat3(glam::DMat3::from_mat4(world));
        let look_at = rotation.transform_vector3(Vector3::new(0., 0., -1.)) + camera_position;
        let target = -reflect(plane_point - look_at) + plane_point;
        let up = reflect(rotation.transform_vector3(Vector3::Y));
        let virtual_world = {
            let z = (mirror_view - target).normalize();
            let mut x = up.cross(z);
            if x.length_squared() == 0. {
                x = Vector3::X;
            }
            let x = x.normalize();
            let y = z.cross(x);
            Matrix4::from_cols(
                x.extend(0.),
                y.extend(0.),
                z.extend(0.),
                mirror_view.extend(1.),
            )
        };
        let virtual_view = virtual_world.inverse();
        let mut virtual_projection = projection;
        {
            let plane_normal = virtual_view.transform_vector3(normal).normalize();
            let point = virtual_view.transform_point3(normal * normal.dot(plane_point));
            let mut clip = plane_normal.extend(-point.dot(plane_normal));
            let mut e = virtual_projection.to_cols_array();
            let q = Vector4::new(
                (clip.x.signum() + e[8]) / e[0],
                (clip.y.signum() + e[9]) / e[5],
                -1.,
                (1. + e[10]) / e[14],
            );
            clip *= 1. / clip.dot(q);
            e[2] = clip.x;
            e[6] = clip.y;
            e[10] = clip.z;
            e[14] = clip.w;
            virtual_projection = Matrix4::from_cols_array(&e);
        }
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
        let size = vec![t.width as f64, t.height as f64];
        let floor_models: Vec<Matrix4> = FLOORS.iter().map(|f| floor_model(*f)).collect();
        let floor_values = |model: Matrix4| {
            vec![
                (
                    "nodeUniform0",
                    Color::from_hex(0x444444).0.to_array().to_vec(),
                ),
                ("nodeUniform1", vec![1.]),
                ("nodeUniform2", vec![0.]),
                ("nodeUniform3", vec![1.]),
                ("nodeUniform5", m3(model.inverse().transpose())),
                ("nodeUniform6", vec![0.; 3]),
                ("nodeUniform7", vec![1.]),
                ("nodeUniform9", m4(model)),
                ("nodeUniform10", vec![9.]),
                ("nodeUniform11", m4(Matrix4::IDENTITY)),
                ("nodeUniform13", vec![1. / 1536.]),
                ("nodeUniform14", vec![1. / 2048.]),
                ("nodeUniform16", vec![1.]),
            ]
        };
        let passes = [
            (&self.main, projection, view, world, camera_position, false),
            (
                &self.mirrored,
                virtual_projection,
                virtual_view,
                virtual_world,
                mirror_view,
                true,
            ),
        ];
        for (u, p, v, w, position, mirrored) in passes {
            let (bg, floor, pool, clutter) = if mirrored {
                (
                    wgsl!("reflect_background_fs"),
                    wgsl!("reflect_floor_fs"),
                    wgsl!("reflect_pool_fs"),
                    wgsl!("reflect_clutter_fs"),
                )
            } else {
                (
                    wgsl!("background_fs"),
                    wgsl!("floor_fs"),
                    wgsl!("pool_fs"),
                    wgsl!("clutter_fs"),
                )
            };
            let camera_values = vec![
                ("cameraProjectionMatrix", m4(p)),
                ("cameraViewMatrix", m4(v)),
            ];
            let mut bg_values = vec![
                ("nodeUniform5", vec![0.]),
                ("nodeUniform6", vec![1.]),
                ("nodeUniform2", m4(Matrix4::IDENTITY)),
            ];
            bg_values.extend(camera_values.clone());
            write(&u.background_render, bg, "renderStruct", &bg_values)?;
            let mut lit = camera_values.clone();
            lit.push(("cameraWorldMatrix", m4(w)));
            write(&u.floor_render, floor, "renderStruct", &lit)?;
            write(&u.pool_render, pool, "renderStruct", &lit)?;
            let mut clutter_values = lit.clone();
            clutter_values.push(("cameraPosition", position.to_array().to_vec()));
            clutter_values.push(("nodeUniform35", size.clone()));
            write(&u.clutter_render, clutter, "renderStruct", &clutter_values)?;
            for (object, model) in u.floor_objects.iter().zip(&floor_models) {
                write(object, floor, "objectStruct", &floor_values(*model))?;
            }
        }
        let model = water_model();
        write(
            &self.water_render,
            wgsl!("water_fs"),
            "renderStruct",
            &[
                ("cameraNear", vec![0.1]),
                ("cameraFar", vec![200.]),
                ("cameraProjectionMatrix", m4(projection)),
                ("cameraViewMatrix", m4(view)),
                ("cameraPosition", camera_position.to_array().to_vec()),
                ("nodeUniform12", size.clone()),
            ],
        )?;
        let id3 = m3(Matrix4::IDENTITY);
        write(
            &self.water_object,
            wgsl!("water_fs"),
            "objectStruct",
            &[
                ("nodeUniform0", self.direction.to_vec()),
                ("nodeUniform2", id3.clone()),
                ("nodeUniform3", vec![self.scale]),
                ("nodeUniform4", self.flow.to_vec()),
                ("nodeUniform6", id3.clone()),
                (
                    "nodeUniform7",
                    Color::from_hex(self.color).0.to_array().to_vec(),
                ),
                ("nodeUniform14", m4(model)),
                ("nodeUniform17", vec![0.02]),
                ("nodeUniform18", vec![1.]),
            ],
        )?;
        for (blur, (_, _, level)) in t.bloom.blurs.iter().zip(&t.bloom.levels) {
            for ((_, _, object), direction) in blur.iter().zip([[1., 0.], [0., 1.]]) {
                write(
                    object,
                    BLUR_FS[0],
                    "objectStruct",
                    &[
                        ("nodeUniform1", id3.clone()),
                        ("nodeUniform2", id3.clone()),
                        ("nodeUniform3", direction.to_vec()),
                        (
                            "nodeUniform4",
                            vec![1. / level.0 as f64, 1. / level.1 as f64],
                        ),
                        ("nodeUniform5", id3.clone()),
                    ],
                )?;
            }
        }
        write(
            &self.fxaa_object,
            wgsl!("fxaa_fs"),
            "objectStruct",
            &[(
                "nodeUniform2",
                vec![1. / t.width as f64, 1. / t.height as f64],
            )],
        )?;
        let mut encoder = r.device.create_command_encoder(&Default::default());
        let black = wgpu::Color::BLACK;
        let depth_ops = |load| {
            Some(wgpu::RenderPassDepthStencilAttachment {
                view: &t.depth_view,
                depth_ops: Some(wgpu::Operations {
                    load,
                    store: wgpu::StoreOp::Store,
                }),
                stencil_ops: None,
            })
        };
        let attachment = |v, load| {
            Some(wgpu::RenderPassColorAttachment {
                view: v,
                depth_slice: None,
                resolve_target: None,
                ops: wgpu::Operations {
                    load,
                    store: wgpu::StoreOp::Store,
                },
            })
        };
        let scene_pass = |encoder: &mut wgpu::CommandEncoder, clear: bool| {
            let (color, depth) = if clear {
                (wgpu::LoadOp::Clear(black), wgpu::LoadOp::Clear(1.))
            } else {
                (wgpu::LoadOp::Load, wgpu::LoadOp::Load)
            };
            encoder
                .begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("water scene"),
                    color_attachments: &[
                        attachment(&t.beauty_view, color),
                        attachment(&t.emissive, color),
                    ],
                    depth_stencil_attachment: depth_ops(depth),
                    ..Default::default()
                })
                .forget_lifetime()
        };
        let (clutter_model, walls_model) = self.pool_models()?;
        // One scene pass: the background, the opaque floors and walls, then
        // the transmissive clutter's faces, each over a fresh mipmapped copy
        // of its color target.
        let scene =
            |encoder: &mut wgpu::CommandEncoder,
             d: &SceneDraws,
             frustum: &Frustum,
             open: &dyn Fn(&mut wgpu::CommandEncoder, bool) -> wgpu::RenderPass<'static>,
             source: &wgpu::Texture,
             copy: &wgpu::Texture,
             levels: &[(wgpu::TextureView, wgpu::BindGroup)]| {
                {
                    let mut pass = open(encoder, true);
                    set(&mut pass, &d.background);
                    self.background.draw(&mut pass);
                    for (m, draw) in floor_models.iter().zip(&d.floor) {
                        if self.floor.visible(frustum, *m) {
                            set(&mut pass, draw);
                            self.floor.draw(&mut pass);
                        }
                    }
                    if self.walls.visible(frustum, walls_model) {
                        set(&mut pass, &d.pool);
                        self.walls.draw(&mut pass);
                    }
                }
                if self.clutter.visible(frustum, clutter_model) {
                    for draw in &d.clutter {
                        encoder.copy_texture_to_texture(
                            source.as_image_copy(),
                            copy.as_image_copy(),
                            source.size(),
                        );
                        self.mipmaps.encode(encoder, HALF, levels);
                        let mut pass = open(encoder, false);
                        set(&mut pass, draw);
                        self.clutter.draw(&mut pass);
                    }
                }
            };
        let frustum = Frustum::from_projection(projection * view);
        scene(
            &mut encoder,
            &t.main,
            &frustum,
            &scene_pass,
            &t.beauty,
            &t.transmission,
            &t.transmission_levels,
        );
        if self.water.visible(&frustum, model) {
            // The reflector renders when the camera is above the water.
            if reflected {
                let reflection_pass = |encoder: &mut wgpu::CommandEncoder, clear: bool| {
                    let (color, depth) = if clear {
                        (wgpu::LoadOp::Clear(black), wgpu::LoadOp::Clear(1.))
                    } else {
                        (wgpu::LoadOp::Load, wgpu::LoadOp::Load)
                    };
                    encoder
                        .begin_render_pass(&wgpu::RenderPassDescriptor {
                            label: Some("water reflection"),
                            color_attachments: &[attachment(&t.reflection_view, color)],
                            depth_stencil_attachment: Some(
                                wgpu::RenderPassDepthStencilAttachment {
                                    view: &t.reflection_depth,
                                    depth_ops: Some(wgpu::Operations {
                                        load: depth,
                                        store: wgpu::StoreOp::Store,
                                    }),
                                    stencil_ops: None,
                                },
                            ),
                            ..Default::default()
                        })
                        .forget_lifetime()
                };
                scene(
                    &mut encoder,
                    &t.mirrored,
                    &Frustum::from_projection(virtual_projection * virtual_view),
                    &reflection_pass,
                    &t.reflection,
                    &t.reflection_transmission,
                    &t.reflection_levels,
                );
            }
            // viewportSharedTexture and the depth for viewportSafeUV: the
            // framebuffer before the water.
            encoder.copy_texture_to_texture(
                t.beauty.as_image_copy(),
                t.shared.as_image_copy(),
                t.beauty.size(),
            );
            encoder.copy_texture_to_texture(
                t.depth.as_image_copy(),
                t.shared_depth.as_image_copy(),
                t.depth.size(),
            );
            let mut pass = scene_pass(&mut encoder, false);
            set(&mut pass, &t.water);
            self.water.draw(&mut pass);
        }
        let quad_pass = |encoder: &mut wgpu::CommandEncoder, target: &wgpu::TextureView| {
            encoder
                .begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("water quad"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: target,
                        depth_slice: None,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(black),
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                    ..Default::default()
                })
                .forget_lifetime()
        };
        let fullscreen = |pass: &mut wgpu::RenderPass| {
            pass.set_vertex_buffer(0, self.quad_uv.slice(..));
            pass.draw(0..3, 0..1);
        };
        let b = &t.bloom;
        {
            let mut pass = quad_pass(&mut encoder, &b.bright);
            pass.set_pipeline(&b.high.0);
            pass.set_bind_group(0, &b.high.1, &[]);
            fullscreen(&mut pass);
        }
        for (blur, (horizontal, vertical, _)) in b.blurs.iter().zip(&b.levels) {
            for ((pipeline, group, _), target) in blur.iter().zip([horizontal, vertical]) {
                let mut pass = quad_pass(&mut encoder, target);
                pass.set_pipeline(pipeline);
                pass.set_bind_group(0, group, &[]);
                fullscreen(&mut pass);
            }
        }
        {
            let mut pass = quad_pass(&mut encoder, &b.levels[0].0);
            pass.set_pipeline(&b.composite.0);
            pass.set_bind_group(0, &b.composite.1, &[]);
            fullscreen(&mut pass);
        }
        {
            let mut pass = quad_pass(&mut encoder, &t.ldr);
            set(&mut pass, &t.output);
            fullscreen(&mut pass);
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
        self.pending = true;
        Ok(())
    }
    /// color ( hex ), scale, flowX and flowY: a flow change normalizes the
    /// direction, as the page does.
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        let value = value as f64;
        match index {
            0 => self.color = value as u32,
            1 => self.scale = value,
            2 | 3 => {
                self.direction[index - 2] = value;
                let l = (self.direction[0].powi(2) + self.direction[1].powi(2)).sqrt();
                if l > 0. {
                    self.direction = self.direction.map(|d| d / l);
                }
            }
            _ => return Err(Error::Invalid("water parameter")),
        }
        self.pending = true;
        Ok(())
    }
    pub fn seek(&mut self, t: f64) {
        self.time = t;
        self.pending = true;
    }
}
