//! webgpu_volume_lighting: a turning teapot over a floor, lit by a wandering
//! point light (cube shadow map) and a sweeping spot light projecting
//! colors.png (shadow map), inside a 20 × 10 × 20 fog volume. The scene pass
//! is followed by the volumetric pass: VolumeNodeMaterial ray-marches the box
//! at a quarter resolution, sampling both lights and their shadows and the
//! 128³ ImprovedNoise smoke texture, dithered by the Bayer texture and
//! stopped at the scene depth. A two-pass Gaussian blur denoises it and the
//! output adds it to the scene with NeutralToneMapping (exposure 2). Every
//! stage runs the WGSL three.js r186 generates for the page (in
//! `volume_lighting/`).
use super::controls_attributes::{Controls, camera_state};
use super::gltf_viewer::{decode_texture_image, fetch};
use super::lights_projector::{m3, m4, pack};
use super::shadowmap_opacity::{Mipmaps, mip_count};
use crate::{
    Error, Result, camera::*, geometry::*, math::*, render_target::*, renderer::*, scene::*,
};
use std::f64::consts::PI;
use wgpu::util::DeviceExt;

pub(super) const HALF: wgpu::TextureFormat = wgpu::TextureFormat::Rgba16Float;
pub(super) const DEPTH: wgpu::TextureFormat = wgpu::TextureFormat::Depth24Plus;
pub(super) const POINT_SIZE: u32 = 512;
pub(super) const SPOT_SIZE: u32 = 1024;
const NOISE: usize = 128;
macro_rules! wgsl {
    ($name:literal) => {
        include_str!(concat!("volume_lighting/", $name, ".wgsl"))
    };
}
/// PointShadowNode's WebGPU cube faces: directions and ups.
pub(super) const FACES: [([f64; 3], [f64; 3]); 6] = [
    ([1., 0., 0.], [0., -1., 0.]),
    ([-1., 0., 0.], [0., -1., 0.]),
    ([0., -1., 0.], [0., 0., -1.]),
    ([0., 1., 0.], [0., 0., 1.]),
    ([0., 0., 1.], [0., -1., 0.]),
    ([0., 0., -1.], [0., -1., 0.]),
];
pub(super) fn uniform(r: &Renderer, label: &str, size: u64) -> wgpu::Buffer {
    r.device.create_buffer(&wgpu::BufferDescriptor {
        label: Some(label),
        size,
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    })
}
pub(super) fn sized(r: &Renderer, label: &str, source: &str, name: &str) -> Result<wgpu::Buffer> {
    Ok(uniform(r, label, pack(source, name, &[])?.len() as u64))
}
pub(super) fn bind(
    r: &Renderer,
    layout: wgpu::BindGroupLayout,
    entries: &[(u32, wgpu::BindingResource)],
) -> wgpu::BindGroup {
    r.device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: None,
        layout: &layout,
        entries: &entries
            .iter()
            .map(|(binding, resource)| wgpu::BindGroupEntry {
                binding: *binding,
                resource: resource.clone(),
            })
            .collect::<Vec<_>>(),
    })
}
pub(super) fn texture(
    r: &Renderer,
    size: (u32, u32, u32),
    format: wgpu::TextureFormat,
    sampled: bool,
) -> wgpu::Texture {
    r.device.create_texture(&wgpu::TextureDescriptor {
        label: Some("volume target"),
        size: wgpu::Extent3d {
            width: size.0,
            height: size.1,
            depth_or_array_layers: size.2,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT
            | if sampled {
                wgpu::TextureUsages::TEXTURE_BINDING
            } else {
                wgpu::TextureUsages::empty()
            },
        view_formats: &[],
    })
}
pub(super) fn view(t: &wgpu::Texture) -> wgpu::TextureView {
    t.create_view(&Default::default())
}
pub(super) fn perspective(fov: f64, near: f64, far: f64) -> Matrix4 {
    let top = near * (fov.to_radians() / 2.).tan();
    Matrix4::from_cols_array(&[
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
    ])
}
/// The teapot's shadow pipeline (both faces cast, as the double-sided
/// material does).
pub(super) fn shadow_pipeline(r: &Renderer) -> wgpu::RenderPipeline {
    let module = |label, source: &str| {
        r.device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some(label),
            source: wgpu::ShaderSource::Wgsl(source.into()),
        })
    };
    let position = wgpu::vertex_attr_array![0 => Float32x3];
    r.device
        .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("volume shadow"),
            layout: None,
            vertex: wgpu::VertexState {
                module: &module("shadow", wgsl!("shadow_vs")),
                entry_point: Some("main"),
                compilation_options: Default::default(),
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: 12,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &position,
                }],
            },
            fragment: Some(wgpu::FragmentState {
                module: &module("shadow", wgsl!("shadow_fs")),
                entry_point: Some("main"),
                compilation_options: Default::default(),
                targets: &[Some(wgpu::TextureFormat::Rgba8Unorm.into())],
            }),
            // The double-sided teapot casts from both faces.
            primitive: wgpu::PrimitiveState {
                cull_mode: None,
                ..Default::default()
            },
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
/// The spot light's map: colors.png (no color space), flipped, mipmapped.
pub(super) async fn spot_map(r: &Renderer) -> Result<wgpu::TextureView> {
    let image =
        decode_texture_image(&fetch("/web/gallery/assets/spot-skinning/colors.png").await?).await?;
    let size = (image.width, image.height);
    let colors = r.device.create_texture(&wgpu::TextureDescriptor {
        label: Some("spot map"),
        size: wgpu::Extent3d {
            width: size.0,
            height: size.1,
            depth_or_array_layers: 1,
        },
        mip_level_count: mip_count(size),
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8Unorm,
        usage: wgpu::TextureUsages::TEXTURE_BINDING
            | wgpu::TextureUsages::COPY_DST
            | wgpu::TextureUsages::RENDER_ATTACHMENT,
        view_formats: &[],
    });
    let row = size.0 as usize * 4;
    let flipped: Vec<u8> = image.rgba.chunks(row).rev().flatten().copied().collect();
    r.queue.write_texture(
        colors.as_image_copy(),
        &flipped,
        wgpu::TexelCopyBufferLayout {
            offset: 0,
            bytes_per_row: Some(row as u32),
            rows_per_image: Some(size.1),
        },
        wgpu::Extent3d {
            width: size.0,
            height: size.1,
            depth_or_array_layers: 1,
        },
    );
    let mut mipmaps = Mipmaps::new(r);
    let mut encoder = r.device.create_command_encoder(&Default::default());
    let levels = mipmaps.levels(r, &colors);
    mipmaps.encode(&mut encoder, colors.format(), &levels);
    r.queue.submit([encoder.finish()]);
    Ok(view(&colors))
}
/// createTexture3D(): the page's 128³ ImprovedNoise smoke (values wrap at 256).
pub(super) fn noise_texture(r: &Renderer) -> wgpu::TextureView {
    let mut noise = vec![0u8; NOISE * NOISE * NOISE];
    let mut i = 0;
    for z in 0..NOISE {
        for y in 0..NOISE {
            for x in 0..NOISE {
                let f = |v: usize| v as f64 / NOISE as f64 * 5. * 10.;
                let n = super::trackball_sprites::noise(f(x), f(y), f(z));
                noise[i] = ((128. + 128. * n).trunc() as i64).rem_euclid(256) as u8;
                i += 1;
            }
        }
    }
    let noise = r.device.create_texture_with_data(
        &r.queue,
        &wgpu::TextureDescriptor {
            label: Some("volume noise"),
            size: wgpu::Extent3d {
                width: NOISE as u32,
                height: NOISE as u32,
                depth_or_array_layers: NOISE as u32,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D3,
            format: wgpu::TextureFormat::R8Unorm,
            usage: wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        },
        wgpu::util::TextureDataOrder::LayerMajor,
        &noise,
    );
    noise.create_view(&Default::default())
}
/// bayer16: the addon's PNG, flipped on decode, read with textureLoad.
pub(super) fn bayer_texture(r: &Renderer) -> Result<wgpu::TextureView> {
    let bayer =
        crate::material::Texture::from_image(include_bytes!("volume_lighting/bayer16.png"))?;
    let row = bayer.width as usize * 4;
    let flipped: Vec<u8> = bayer.rgba.chunks(row).rev().flatten().copied().collect();
    let bayer = r.device.create_texture_with_data(
        &r.queue,
        &wgpu::TextureDescriptor {
            label: Some("bayer16"),
            size: wgpu::Extent3d {
                width: bayer.width,
                height: bayer.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        },
        wgpu::util::TextureDataOrder::LayerMajor,
        &flipped,
    );
    Ok(bayer.create_view(&Default::default()))
}
/// The teapot's shadows: the point light's six cube faces and the spot
/// light's map.
pub(super) struct Shadows {
    pipeline: wgpu::RenderPipeline,
    point_faces: Vec<(
        wgpu::TextureView,
        wgpu::TextureView,
        wgpu::Buffer,
        wgpu::BindGroup,
    )>,
    pub(super) point_cube: wgpu::TextureView,
    pub(super) spot: (
        wgpu::TextureView,
        wgpu::TextureView,
        wgpu::Buffer,
        wgpu::BindGroup,
    ),
    object: (wgpu::Buffer, wgpu::BindGroup),
}
impl Shadows {
    pub(super) fn new(r: &Renderer) -> Self {
        let shadow_pipeline = shadow_pipeline(r);
        let shadow_render = |label| {
            let buffer = uniform(r, label, 128);
            let group = bind(
                r,
                shadow_pipeline.get_bind_group_layout(0),
                &[(0, buffer.as_entire_binding())],
            );
            (buffer, group)
        };
        let point_color = texture(
            r,
            (POINT_SIZE, POINT_SIZE, 6),
            wgpu::TextureFormat::Rgba8Unorm,
            false,
        );
        let point_depth = texture(r, (POINT_SIZE, POINT_SIZE, 6), DEPTH, true);
        let layer = |t: &wgpu::Texture, i: u32| {
            t.create_view(&wgpu::TextureViewDescriptor {
                dimension: Some(wgpu::TextureViewDimension::D2),
                base_array_layer: i,
                array_layer_count: Some(1),
                ..Default::default()
            })
        };
        let point_faces = (0..6)
            .map(|i| {
                let (buffer, group) = shadow_render("point shadow face");
                (
                    layer(&point_color, i),
                    layer(&point_depth, i),
                    buffer,
                    group,
                )
            })
            .collect();
        let point_cube = point_depth.create_view(&wgpu::TextureViewDescriptor {
            dimension: Some(wgpu::TextureViewDimension::Cube),
            ..Default::default()
        });
        let spot_render = shadow_render("spot shadow");
        let spot_shadow = (
            view(&texture(
                r,
                (SPOT_SIZE, SPOT_SIZE, 1),
                wgpu::TextureFormat::Rgba8Unorm,
                false,
            )),
            view(&texture(r, (SPOT_SIZE, SPOT_SIZE, 1), DEPTH, true)),
            spot_render.0,
            spot_render.1,
        );
        let shadow_object_buffer = uniform(r, "teapot shadow object", 80);
        let shadow_object = (
            bind(
                r,
                shadow_pipeline.get_bind_group_layout(1),
                &[(0, shadow_object_buffer.as_entire_binding())],
            ),
            shadow_object_buffer,
        );
        Self {
            pipeline: shadow_pipeline,
            point_faces,
            point_cube,
            spot: spot_shadow,
            object: (shadow_object.1, shadow_object.0),
        }
    }
    /// Renders both shadow maps, each pass culling the teapot against its
    /// camera.
    #[allow(clippy::too_many_arguments)]
    pub(super) fn encode(
        &self,
        r: &Renderer,
        encoder: &mut wgpu::CommandEncoder,
        teapot: &Mesh,
        teapot_model: Matrix4,
        point: Vector3,
        spot_projection: Matrix4,
        spot_view: Matrix4,
    ) {
        let mut object = vec![1f32, 0., 0., 0.];
        object.extend(teapot_model.to_cols_array().map(|v| v as f32));
        r.queue
            .write_buffer(&self.object.0, 0, bytemuck::cast_slice(&object));
        let point_projection = perspective(90., 0.5, 100.);
        let camera_data = |p: Matrix4, v: Matrix4| {
            let mut data: Vec<f32> = p.to_cols_array().map(|x| x as f32).to_vec();
            data.extend(v.to_cols_array().map(|x| x as f32));
            data
        };
        let teapot_visible = |frustum: Frustum| {
            frustum.intersects_sphere(Sphere {
                center: teapot_model.transform_point3(Vector3::ZERO),
                radius: teapot.radius,
            })
        };
        let shadow_pass = |encoder: &mut wgpu::CommandEncoder,
                           color: &wgpu::TextureView,
                           depth: &wgpu::TextureView,
                           clear: wgpu::Color,
                           render: &wgpu::BindGroup,
                           visible: bool| {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("volume shadow"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: color,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(clear),
                        store: wgpu::StoreOp::Discard,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: depth,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                ..Default::default()
            });
            if visible {
                pass.set_pipeline(&self.pipeline);
                pass.set_bind_group(0, render, &[]);
                pass.set_bind_group(1, &self.object.1, &[]);
                pass.set_vertex_buffer(0, teapot.positions.slice(..));
                pass.set_index_buffer(teapot.index.slice(..), wgpu::IndexFormat::Uint32);
                pass.draw_indexed(0..teapot.count, 0, 0..1);
            }
        };
        // The point light's six cube faces, each culled against its camera.
        for (i, (color, depth, buffer, group)) in self.point_faces.iter().enumerate() {
            let (direction, up) = FACES[i];
            let face_view = Matrix4::look_at_rh(
                point,
                point + Vector3::from_array(direction),
                Vector3::from_array(up),
            );
            r.queue.write_buffer(
                buffer,
                0,
                bytemuck::cast_slice(&camera_data(point_projection, face_view)),
            );
            let visible = teapot_visible(Frustum::from_projection(point_projection * face_view));
            shadow_pass(encoder, color, depth, wgpu::Color::BLACK, group, visible);
        }
        r.queue.write_buffer(
            &self.spot.2,
            0,
            bytemuck::cast_slice(&camera_data(spot_projection, spot_view)),
        );
        let visible = teapot_visible(Frustum::from_projection(spot_projection * spot_view));
        shadow_pass(
            encoder,
            &self.spot.0,
            &self.spot.1,
            wgpu::Color::TRANSPARENT,
            &self.spot.3,
            visible,
        );
    }
}
/// Geometry with normals: positions, normals, index and bounding radius.
pub(super) struct Mesh {
    pub(super) positions: wgpu::Buffer,
    pub(super) normals: wgpu::Buffer,
    pub(super) index: wgpu::Buffer,
    pub(super) count: u32,
    pub(super) radius: f64,
}
struct Targets {
    width: u32,
    height: u32,
    format: wgpu::TextureFormat,
    color: wgpu::TextureView,
    depth: wgpu::TextureView,
    screen: RenderTarget,
    teapot: (wgpu::RenderPipeline, [wgpu::BindGroup; 2]),
    floor: (wgpu::RenderPipeline, [wgpu::BindGroup; 2]),
    volume_pass: (wgpu::RenderPipeline, [wgpu::BindGroup; 2]),
    blur: [wgpu::RenderPipeline; 2],
    output: (wgpu::RenderPipeline, wgpu::BindGroup),
    /// The volumetric targets for each resolution scale used so far.
    volumes: Vec<Volume>,
}
/// The volumetric pass's target at one size, its blur targets and the bind
/// groups reading them.
struct Volume {
    size: (u32, u32),
    target: wgpu::TextureView,
    blurred: [wgpu::TextureView; 2],
    blur: [wgpu::BindGroup; 2],
    /// Output over the blurred, or the raw, volumetric lighting.
    output: [wgpu::BindGroup; 2],
}
pub(super) struct Demo {
    controls: Controls,
    time: f64,
    /// resolution, step count, denoise strength, denoise, light intensity,
    /// spot intensity, fog intensity, smoke amount.
    params: [f64; 8],
    teapot: Mesh,
    floor: Mesh,
    volume: Mesh,
    shadows: Shadows,
    teapot_render: wgpu::Buffer,
    teapot_object: wgpu::Buffer,
    floor_render: wgpu::Buffer,
    floor_object: wgpu::Buffer,
    volume_render: wgpu::Buffer,
    volume_object: wgpu::Buffer,
    blur_objects: [wgpu::Buffer; 2],
    output_render: wgpu::Buffer,
    output_object: wgpu::Buffer,
    quad_uv: wgpu::Buffer,
    colors: wgpu::TextureView,
    bayer: wgpu::TextureView,
    noise: wgpu::TextureView,
    linear: wgpu::Sampler,
    mipmapped: wgpu::Sampler,
    noise_sampler: wgpu::Sampler,
    compare: wgpu::Sampler,
    targets: Option<Targets>,
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
            far: 100.,
            aspect,
            ..Default::default()
        }));
        s.get_mut(c)?.position = Vector3::new(-8., 1., -6.);
        let mut controls = Controls::new(None, (2., 40.), PI, true);
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
        let mesh = |g: BufferGeometry| -> Result<Mesh> {
            let f = |name: &str| -> Result<Vec<f32>> {
                let a = g
                    .attributes
                    .get(name)
                    .ok_or(Error::Invalid("volume attribute"))?;
                (0..a.count())
                    .flat_map(|i| (0..3).map(move |k| (i, k)))
                    .map(|(i, k)| a.get_component(i, k).map(|v| v as f32))
                    .collect()
            };
            let positions = f("position")?;
            let index = g.index.clone().ok_or(Error::Invalid("volume index"))?;
            Ok(Mesh {
                radius: positions
                    .chunks(3)
                    .map(|p| (p[0] as f64).hypot(p[1] as f64).hypot(p[2] as f64))
                    .fold(0., f64::max),
                positions: init("volume positions", bytemuck::cast_slice(&positions), vertex),
                normals: init(
                    "volume normals",
                    bytemuck::cast_slice(&f("normal")?),
                    vertex,
                ),
                index: init(
                    "volume index",
                    bytemuck::cast_slice(&index),
                    wgpu::BufferUsages::INDEX,
                ),
                count: index.len() as u32,
            })
        };
        let teapot = mesh(super::models_modifiers::teapot(
            0.8, 18, true, true, true, true, true,
        )?)?;
        let floor = mesh(PlaneGeometry::build(100., 100., 1, 1)?)?;
        let volume = mesh(BoxGeometry::build(20., 10., 20.)?)?;
        let noise = noise_texture(r);
        let bayer = bayer_texture(r)?;
        let colors = spot_map(r).await?;
        let shadows = Shadows::new(r);
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
            params: [0.25, 12., 0.6, 1., 3., 100., 1., 2.],
            teapot,
            floor,
            volume,
            shadows,
            teapot_render: sized(r, "teapot render", wgsl!("teapot_fs"), "renderStruct")?,
            teapot_object: sized(r, "teapot object", wgsl!("teapot_fs"), "objectStruct")?,
            floor_render: sized(r, "floor render", wgsl!("floor_fs"), "renderStruct")?,
            floor_object: sized(r, "floor object", wgsl!("floor_fs"), "objectStruct")?,
            volume_render: sized(r, "volume render", wgsl!("volume_fs"), "renderStruct")?,
            volume_object: sized(r, "volume object", wgsl!("volume_fs"), "objectStruct")?,
            blur_objects: [0; 2].map(|_| uniform(r, "blur object", 16)),
            output_render: uniform(r, "output render", 16),
            output_object: uniform(r, "output object", 16),
            quad_uv: init(
                "quad uv",
                bytemuck::cast_slice(&[0f32, -1., 0., 1., 2., 1.]),
                vertex,
            ),
            colors,
            bayer,
            noise,
            linear: sampler(wgpu::AddressMode::ClampToEdge, wgpu::FilterMode::Nearest),
            mipmapped: sampler(wgpu::AddressMode::ClampToEdge, wgpu::FilterMode::Linear),
            // RepeatWrapping in s and t; r keeps the default clamp.
            noise_sampler: r.device.create_sampler(&wgpu::SamplerDescriptor {
                address_mode_u: wgpu::AddressMode::Repeat,
                address_mode_v: wgpu::AddressMode::Repeat,
                address_mode_w: wgpu::AddressMode::ClampToEdge,
                mag_filter: wgpu::FilterMode::Linear,
                min_filter: wgpu::FilterMode::Linear,
                ..Default::default()
            }),
            compare: r.device.create_sampler(&wgpu::SamplerDescriptor {
                mag_filter: wgpu::FilterMode::Linear,
                min_filter: wgpu::FilterMode::Linear,
                compare: Some(wgpu::CompareFunction::LessEqual),
                ..Default::default()
            }),
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
        let (width, height) = (out.width, out.height);
        let color = view(&texture(r, (width, height, 1), HALF, true));
        let depth = view(&texture(r, (width, height, 1), DEPTH, true));
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
        // (depth, cull, front face clockwise, blended)
        let pipeline = |label,
                        vs: &str,
                        fs: &str,
                        buffers: &[wgpu::VertexBufferLayout],
                        format,
                        state: (bool, Option<wgpu::Face>, bool, bool)| {
            let (depth, cull, cw, blended) = state;
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
                    depth_stencil: depth.then(|| wgpu::DepthStencilState {
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
        let back = Some(wgpu::Face::Back);
        let tex = wgpu::BindingResource::TextureView;
        let sampler = wgpu::BindingResource::Sampler;
        let teapot = pipeline(
            "teapot",
            wgsl!("teapot_vs"),
            wgsl!("teapot_fs"),
            &[layout(0, 12), layout(1, 12)],
            HALF,
            (true, None, false, false),
        );
        let teapot_binds = [
            bind(
                r,
                teapot.get_bind_group_layout(0),
                &[(0, self.teapot_render.as_entire_binding())],
            ),
            bind(
                r,
                teapot.get_bind_group_layout(1),
                &[
                    (0, self.teapot_object.as_entire_binding()),
                    (1, sampler(&self.linear)),
                    (2, tex(&r.dfg)),
                    (3, sampler(&self.mipmapped)),
                    (4, tex(&self.colors)),
                ],
            ),
        ];
        let floor = pipeline(
            "volume floor",
            wgsl!("floor_vs"),
            wgsl!("floor_fs"),
            &[layout(0, 12), layout(1, 12)],
            HALF,
            (true, back, false, false),
        );
        let floor_binds = [
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
                    (3, sampler(&self.compare)),
                    (4, tex(&self.shadows.point_cube)),
                    (5, sampler(&self.compare)),
                    (6, tex(&self.shadows.spot.1)),
                    (7, sampler(&self.mipmapped)),
                    (8, tex(&self.colors)),
                ],
            ),
        ];
        // VolumeNodeMaterial: the box's back faces, blended, without depth.
        let volume = pipeline(
            "volume",
            wgsl!("volume_vs"),
            wgsl!("volume_fs"),
            &[layout(0, 12), layout(1, 12)],
            HALF,
            (false, back, true, true),
        );
        let volume_binds = [
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
                    (2, tex(&self.bayer)),
                    (3, tex(&depth)),
                    (4, sampler(&self.compare)),
                    (5, tex(&self.shadows.point_cube)),
                    (6, sampler(&self.compare)),
                    (7, tex(&self.shadows.spot.1)),
                    (8, sampler(&self.mipmapped)),
                    (9, tex(&self.colors)),
                    (10, sampler(&self.noise_sampler)),
                    (11, tex(&self.noise)),
                ],
            ),
        ];
        let blur = [wgsl!("blur_h_fs"), wgsl!("blur_v_fs")].map(|fs| {
            pipeline(
                "gaussian blur",
                wgsl!("blur_vs"),
                fs,
                &[layout(2, 8)],
                HALF,
                (false, back, false, false),
            )
        });
        let output = pipeline(
            "render output",
            wgsl!("output_vs"),
            wgsl!("output_fs"),
            &[layout(2, 8)],
            out.options.format,
            (false, back, false, false),
        );
        let output_render = bind(
            r,
            output.get_bind_group_layout(0),
            &[(0, self.output_render.as_entire_binding())],
        );
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
            format: out.options.format,
            color,
            depth,
            screen,
            teapot: (teapot, teapot_binds),
            floor: (floor, floor_binds),
            volume_pass: (volume, volume_binds),
            blur,
            output: (output, output_render),
            volumes: vec![],
        });
        Ok(())
    }
    /// The volumetric target for a size, created on first use.
    fn volume_targets(&mut self, r: &Renderer, size: (u32, u32)) {
        let Some(t) = self.targets.as_mut() else {
            return;
        };
        if t.volumes.iter().any(|v| v.size == size) {
            return;
        }
        let target = view(&texture(r, (size.0, size.1, 1), HALF, true));
        let blurred = [0; 2].map(|_| view(&texture(r, (size.0, size.1, 1), HALF, true)));
        let tex = wgpu::BindingResource::TextureView;
        let sampler = wgpu::BindingResource::Sampler(&self.linear);
        let blur = [
            (&target, &self.blur_objects[0]),
            (&blurred[0], &self.blur_objects[1]),
        ]
        .iter()
        .zip(&t.blur)
        .map(|((source, object), pipeline)| {
            bind(
                r,
                pipeline.get_bind_group_layout(0),
                &[
                    (0, sampler.clone()),
                    (1, tex(source)),
                    (2, object.as_entire_binding()),
                ],
            )
        })
        .collect::<Vec<_>>();
        let output = [&blurred[1], &target].map(|volumetric| {
            bind(
                r,
                t.output.0.get_bind_group_layout(1),
                &[
                    (0, sampler.clone()),
                    (1, tex(&t.color)),
                    (2, sampler.clone()),
                    (3, tex(volumetric)),
                    (4, self.output_object.as_entire_binding()),
                ],
            )
        });
        let Ok(blur) = <[wgpu::BindGroup; 2]>::try_from(blur) else {
            return;
        };
        t.volumes.push(Volume {
            size,
            target,
            blurred,
            blur,
            output,
        });
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
        // PassNode.setResolutionScale: the volumetric target's size.
        let volume_size = self.targets.as_ref().map_or((1, 1), |t| {
            (
                ((t.width as f64 * self.params[0]) as u32).max(1),
                ((t.height as f64 * self.params[0]) as u32).max(1),
            )
        });
        self.volume_targets(r, volume_size);
        s.update()?;
        let (camera, world) = s.camera(c)?;
        let (projection, view) = (camera.projection_matrix()?, world.inverse());
        let camera_position = world.transform_point3(Vector3::ZERO);
        // animate(): the point light wanders, the spot light sweeps, the
        // teapot turns.
        let (time, scale) = (self.time, 2.4);
        let point = Vector3::new(
            (time * 0.7).sin() * scale,
            (time * 0.5).cos() * scale,
            (time * 0.3).cos() * scale,
        );
        let spot = Vector3::new((time * 0.3).cos() * scale, 5., 2.5);
        let teapot_model = Matrix4::from_rotation_y(time * 0.2);
        let floor_model = Matrix4::from_translation(Vector3::new(0., -3., 0.))
            * Matrix4::from_rotation_x(-PI / 2.);
        let volume_model = Matrix4::from_translation(Vector3::new(0., 2., 0.));
        let [
            _,
            steps,
            strength,
            denoise,
            point_intensity,
            spot_intensity,
            fog,
            smoke,
        ] = self.params;
        // The spot light's shadow camera: 2 × angle, near 1, far 15.
        let spot_projection = perspective(60., 1., 15.);
        let spot_view = Matrix4::look_at_rh(spot, Vector3::ZERO, Vector3::Y);
        let bias = Matrix4::from_cols_array(&[
            0.5, 0., 0., 0., 0., 0.5, 0., 0., 0., 0., 1., 0., 0.5, 0.5, 0., 1.,
        ]);
        let spot_matrix = bias * spot_projection * spot_view;
        let point_matrix = Matrix4::from_translation(-point);
        let point_color = Color::from_hex(0xf9bb50)
            .0
            .to_array()
            .map(|v| v * point_intensity)
            .to_vec();
        let spot_color = vec![spot_intensity; 3];
        let cone = (PI / 6.).cos();
        let point_view = view.transform_point3(point).to_array().to_vec();
        let spot_view_position = view.transform_point3(spot).to_array().to_vec();
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
        let mut teapot_values = camera_values.clone();
        teapot_values.extend([
            ("nodeUniform11", point_color.clone()),
            ("nodeUniform12", vec![100.]),
            ("nodeUniform13", vec![2.]),
            ("nodeUniform17", vec![cone]),
            ("nodeUniform18", vec![1.]),
            ("nodeUniform22", vec![0.]),
            ("nodeUniform23", vec![2.]),
            ("nodeUniform16", spot_color.clone()),
            ("nodeUniform10", point_view.clone()),
            ("nodeUniform14", spot_view_position.clone()),
            ("nodeUniform20", spot.to_array().to_vec()),
            ("nodeUniform21", vec![0.; 3]),
            ("nodeUniform15", m4(spot_matrix)),
        ]);
        write(
            &self.teapot_render,
            wgsl!("teapot_fs"),
            "renderStruct",
            &teapot_values,
        )?;
        let standard = |model: Matrix4| {
            vec![
                ("nodeUniform0", vec![1.; 3]),
                ("nodeUniform1", vec![1.]),
                ("nodeUniform2", vec![0.]),
                ("nodeUniform3", vec![1.]),
                ("nodeUniform5", m3(model.inverse().transpose())),
                ("nodeUniform7", vec![1.]),
                ("nodeUniform9", m4(model)),
            ]
        };
        write(
            &self.teapot_object,
            wgsl!("teapot_fs"),
            "objectStruct",
            &standard(teapot_model),
        )?;
        write(
            &self.floor_object,
            wgsl!("floor_fs"),
            "objectStruct",
            &standard(floor_model),
        )?;
        let mut floor_values = camera_values.clone();
        floor_values.extend([
            ("nodeUniform11", point_color.clone()),
            ("nodeUniform22", vec![100.]),
            ("nodeUniform23", vec![2.]),
            ("nodeUniform33", vec![cone]),
            ("nodeUniform34", vec![1.]),
            ("nodeUniform37", vec![0.]),
            ("nodeUniform38", vec![2.]),
            ("nodeUniform26", spot_color.clone()),
            ("nodeUniform10", point_view.clone()),
            ("nodeUniform24", spot_view_position.clone()),
            ("nodeUniform35", spot.to_array().to_vec()),
            ("nodeUniform36", vec![0.; 3]),
            ("nodeUniform25", m4(spot_matrix)),
            ("nodeUniform12", m4(point_matrix)),
            ("nodeUniform16", vec![0.5]),
            ("nodeUniform15", vec![100.]),
            ("nodeUniform14", vec![0.]),
            ("nodeUniform17", vec![0.]),
            ("nodeUniform19", vec![1.]),
            ("nodeUniform20", vec![POINT_SIZE as f64; 2]),
            ("nodeUniform21", vec![1.]),
            ("nodeUniform27", vec![0.]),
            ("nodeUniform28", vec![0.]),
            ("nodeUniform30", vec![1.]),
            ("nodeUniform31", vec![SPOT_SIZE as f64; 2]),
            ("nodeUniform32", vec![0.98]),
        ]);
        write(
            &self.floor_render,
            wgsl!("floor_fs"),
            "renderStruct",
            &floor_values,
        )?;
        let t = self
            .targets
            .as_ref()
            .ok_or(Error::Invalid("volume targets"))?;
        let mut volume_values = camera_values;
        volume_values.extend([
            ("cameraNear", vec![0.1]),
            ("cameraFar", vec![100.]),
            ("nodeUniform43", vec![time]),
            ("nodeUniform11", point_color),
            ("nodeUniform23", vec![100.]),
            ("nodeUniform25", vec![2.]),
            ("nodeUniform34", vec![cone]),
            ("nodeUniform35", vec![1.]),
            ("nodeUniform39", vec![0.]),
            ("nodeUniform40", vec![2.]),
            ("nodeUniform27", spot_color),
            ("nodeUniform24", point_view),
            ("nodeUniform36", spot_view_position),
            ("nodeUniform37", spot.to_array().to_vec()),
            ("nodeUniform38", vec![0.; 3]),
            ("nodeUniform26", m4(spot_matrix)),
            ("nodeUniform12", m4(point_matrix)),
            ("nodeUniform15", vec![0.]),
            ("nodeUniform16", vec![100.]),
            ("nodeUniform17", vec![0.5]),
            ("nodeUniform18", vec![0.]),
            ("nodeUniform20", vec![1.]),
            ("nodeUniform21", vec![POINT_SIZE as f64; 2]),
            ("nodeUniform22", vec![1.]),
            ("nodeUniform28", vec![0.]),
            ("nodeUniform29", vec![0.]),
            ("nodeUniform31", vec![1.]),
            ("nodeUniform32", vec![SPOT_SIZE as f64; 2]),
            ("nodeUniform33", vec![0.98]),
            ("cameraPosition", camera_position.to_array().to_vec()),
            (
                "nodeUniform10",
                vec![volume_size.0 as f64, volume_size.1 as f64],
            ),
        ]);
        write(
            &self.volume_render,
            wgsl!("volume_fs"),
            "renderStruct",
            &volume_values,
        )?;
        write(
            &self.volume_object,
            wgsl!("volume_fs"),
            "objectStruct",
            &[
                ("nodeUniform0", vec![1.]),
                ("nodeUniform2", m4(volume_model)),
                ("nodeUniform3", vec![self.volume.radius]),
                ("nodeUniform4", vec![steps.trunc()]),
                ("nodeUniform14", m3(volume_model.inverse().transpose())),
                ("nodeUniform44", vec![smoke]),
            ],
        )?;
        let texel = [1. / volume_size.0 as f32, 1. / volume_size.1 as f32];
        let v = t
            .volumes
            .iter()
            .find(|v| v.size == volume_size)
            .ok_or(Error::Invalid("volume target"))?;
        for object in &self.blur_objects {
            r.queue.write_buffer(
                object,
                0,
                bytemuck::cast_slice(&[strength as f32, 0., texel[0], texel[1]]),
            );
        }
        r.queue.write_buffer(
            &self.output_render,
            0,
            bytemuck::cast_slice(&[2f32, 0., 0., 0.]),
        );
        r.queue.write_buffer(
            &self.output_object,
            0,
            bytemuck::cast_slice(&[fog as f32, 0., 0., 0.]),
        );
        let teapot_visible = |frustum: Frustum| {
            frustum.intersects_sphere(Sphere {
                center: teapot_model.transform_point3(Vector3::ZERO),
                radius: self.teapot.radius,
            })
        };
        let mut encoder = r.device.create_command_encoder(&Default::default());
        let draw =
            |pass: &mut wgpu::RenderPass, m: &Mesh, normals_first: bool, with_normals: bool| {
                if with_normals {
                    let (a, b) = if normals_first {
                        (&m.normals, &m.positions)
                    } else {
                        (&m.positions, &m.normals)
                    };
                    pass.set_vertex_buffer(0, a.slice(..));
                    pass.set_vertex_buffer(1, b.slice(..));
                } else {
                    pass.set_vertex_buffer(0, m.positions.slice(..));
                }
                pass.set_index_buffer(m.index.slice(..), wgpu::IndexFormat::Uint32);
                pass.draw_indexed(0..m.count, 0, 0..1);
            };
        self.shadows.encode(
            r,
            &mut encoder,
            &self.teapot,
            teapot_model,
            point,
            spot_projection,
            spot_view,
        );
        let frustum = Frustum::from_projection(projection * view);
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("volume scene"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &t.color,
                    depth_slice: None,
                    resolve_target: None,
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
            if teapot_visible(frustum) {
                pass.set_pipeline(&t.teapot.0);
                pass.set_bind_group(0, &t.teapot.1[0], &[]);
                pass.set_bind_group(1, &t.teapot.1[1], &[]);
                draw(&mut pass, &self.teapot, true, true);
            }
            if frustum.intersects_sphere(Sphere {
                center: floor_model.transform_point3(Vector3::ZERO),
                radius: self.floor.radius,
            }) {
                pass.set_pipeline(&t.floor.0);
                pass.set_bind_group(0, &t.floor.1[0], &[]);
                pass.set_bind_group(1, &t.floor.1[1], &[]);
                draw(&mut pass, &self.floor, true, true);
            }
        }
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("volumetric lighting"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &v.target,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                ..Default::default()
            });
            if frustum.intersects_sphere(Sphere {
                center: volume_model.transform_point3(Vector3::ZERO),
                radius: self.volume.radius,
            }) {
                pass.set_pipeline(&t.volume_pass.0);
                pass.set_bind_group(0, &t.volume_pass.1[0], &[]);
                pass.set_bind_group(1, &t.volume_pass.1[1], &[]);
                draw(&mut pass, &self.volume, false, true);
            }
        }
        let quad = |encoder: &mut wgpu::CommandEncoder,
                    target: &wgpu::TextureView,
                    pipeline: &wgpu::RenderPipeline,
                    groups: &[&wgpu::BindGroup]| {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("volume post"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: target,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                ..Default::default()
            });
            pass.set_pipeline(pipeline);
            for (i, group) in groups.iter().enumerate() {
                pass.set_bind_group(i as u32, *group, &[]);
            }
            pass.set_vertex_buffer(0, self.quad_uv.slice(..));
            pass.draw(0..3, 0..1);
        };
        let denoised = denoise > 0.5;
        if denoised {
            quad(&mut encoder, &v.blurred[0], &t.blur[0], &[&v.blur[0]]);
            quad(&mut encoder, &v.blurred[1], &t.blur[1], &[&v.blur[1]]);
        }
        quad(
            &mut encoder,
            &t.screen.view,
            &t.output.0,
            &[&t.output.1, &v.output[if denoised { 0 } else { 1 }]],
        );
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
    /// resolution, step count, denoise strength, denoise, light intensity,
    /// spot intensity, fog intensity and smoke amount.
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        *self
            .params
            .get_mut(index)
            .ok_or(Error::Invalid("volume parameter"))? = value as f64;
        Ok(())
    }
    pub fn seek(&mut self, t: f64) {
        self.time = t;
    }
}
