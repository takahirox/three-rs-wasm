//! webgpu_lights_projector: the Lucy statue on a Lambert floor under a
//! circling ProjectorLight with a hemisphere fill, a SpotLightHelper and ACES
//! output. The projector casts PCF shadows (a 1024² depth map from its
//! perspective shadow camera) and projects either a procedural caustic
//! (animated Worley noise), the Sintel video or colors.png through its
//! shadow matrix, clipped by the box attenuation of ProjectorLightNode.
//! Every stage runs the WGSL three.js r186 generates for the page (in
//! `lights_projector/`); uniform structs are filled from the field order of
//! the generated WGSL.
use super::controls_attributes::{Controls, camera_state};
use super::gltf_viewer::{decode_texture_image, fetch};
use crate::{
    Error, Result, camera::*, geometry::*, math::*, render_target::*, renderer::*, scene::*,
};
use std::{cell::RefCell, f64::consts::PI, rc::Rc};
use wgpu::util::DeviceExt;

const ASSETS: &str = "/web/gallery/assets";
const HALF: wgpu::TextureFormat = wgpu::TextureFormat::Rgba16Float;
const DEPTH: wgpu::TextureFormat = wgpu::TextureFormat::Depth24Plus;
const SHADOW_SIZE: u32 = 1024;
macro_rules! wgsl {
    ($name:literal) => {
        include_str!(concat!("lights_projector/", $name, ".wgsl"))
    };
}
/// Packs values into a uniform struct of the generated WGSL, laid out by the
/// WGSL rules for its field order. Matrices are given column by column.
pub(super) fn pack(source: &str, name: &str, values: &[(&str, &[f64])]) -> Result<Vec<u8>> {
    let start = source
        .find(&format!("struct {name} {{"))
        .ok_or(Error::Invalid("uniform struct"))?;
    let body = &source[start..];
    let body = &body[body.find('{').unwrap_or(0) + 1..body.find('}').unwrap_or(0)];
    let mut fields = vec![];
    let mut offset: usize = 0;
    let mut align_max = 16;
    for line in body.split(',') {
        let Some((field, kind)) = line.split_once(':') else {
            continue;
        };
        let kind = kind.trim();
        let (align, size, columns) = match kind {
            "f32" | "i32" | "u32" => (4, 4, 0),
            "vec2<f32>" => (8, 8, 0),
            "vec3<f32>" => (16, 12, 0),
            "vec4<f32>" => (16, 16, 0),
            "mat3x3<f32>" => (16, 48, 3),
            "mat4x4<f32>" => (16, 64, 4),
            _ => return Err(Error::Invalid("uniform field type")),
        };
        align_max = align_max.max(align);
        offset = offset.div_ceil(align) * align;
        fields.push((field.trim(), offset, columns, kind == "i32"));
        offset += size;
    }
    let mut data = vec![0u8; offset.div_ceil(align_max) * align_max];
    for (field, offset, columns, int) in fields {
        let Some((_, v)) = values.iter().find(|(n, _)| *n == field) else {
            continue;
        };
        for (i, x) in v.iter().enumerate() {
            // mat3x3 columns are padded to 16 bytes.
            let at = if columns == 3 {
                offset + i / 3 * 16 + i % 3 * 4
            } else {
                offset + i * 4
            };
            let bytes = if int {
                (*x as i32).to_le_bytes()
            } else {
                (*x as f32).to_le_bytes()
            };
            data[at..at + 4].copy_from_slice(&bytes);
        }
    }
    Ok(data)
}
pub(super) fn m4(m: Matrix4) -> Vec<f64> {
    m.to_cols_array().to_vec()
}
pub(super) fn m3(m: Matrix4) -> Vec<f64> {
    (0..3)
        .flat_map(|c| {
            let v = m.col(c);
            [v.x, v.y, v.z]
        })
        .collect()
}
fn uniform(r: &Renderer, label: &str, size: u64) -> wgpu::Buffer {
    r.device.create_buffer(&wgpu::BufferDescriptor {
        label: Some(label),
        size,
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    })
}
fn bind(
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
fn texture(
    r: &Renderer,
    size: (u32, u32),
    format: wgpu::TextureFormat,
    samples: u32,
    sampled: bool,
) -> wgpu::TextureView {
    r.device
        .create_texture(&wgpu::TextureDescriptor {
            label: Some("projector target"),
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
                | if sampled {
                    wgpu::TextureUsages::TEXTURE_BINDING
                } else {
                    wgpu::TextureUsages::empty()
                },
            view_formats: &[],
        })
        .create_view(&Default::default())
}
fn map_texture(r: &Renderer, width: u32, height: u32) -> wgpu::Texture {
    r.device.create_texture(&wgpu::TextureDescriptor {
        label: Some("projector map"),
        size: wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8UnormSrgb,
        usage: wgpu::TextureUsages::TEXTURE_BINDING
            | wgpu::TextureUsages::COPY_DST
            | wgpu::TextureUsages::RENDER_ATTACHMENT,
        view_formats: &[],
    })
}
/// The perspective shadow camera's projection in WebGPU clip space.
fn perspective(fov: f64, aspect: f64, near: f64, far: f64) -> Matrix4 {
    let top = near * (fov.to_radians() / 2.).tan();
    let depth = far - near;
    Matrix4::from_cols_array(&[
        near / (top * aspect),
        0.,
        0.,
        0.,
        0.,
        near / top,
        0.,
        0.,
        0.,
        0.,
        -far / depth,
        -1.,
        0.,
        0.,
        -far * near / depth,
        0.,
    ])
}
/// The current video frame (VideoTexture: sRGB, flipped on upload).
async fn video_frame(video: &web_sys::HtmlVideoElement) -> Result<web_sys::ImageBitmap> {
    use wasm_bindgen::JsCast;
    let promise = web_sys::window()
        .ok_or(Error::Invalid("window"))?
        .create_image_bitmap_with_html_video_element(video)
        .map_err(|_| Error::Invalid("video frame bitmap"))?;
    wasm_bindgen_futures::JsFuture::from(promise)
        .await
        .map_err(|_| Error::Invalid("video frame bitmap"))?
        .dyn_into()
        .map_err(|_| Error::Invalid("video frame bitmap"))
}
struct Mesh {
    positions: wgpu::Buffer,
    normals: wgpu::Buffer,
    index: wgpu::Buffer,
    count: u32,
    radius: f64,
    model: Matrix4,
    object: wgpu::Buffer,
}
/// The projected video: the element, its texture once the first frame is
/// known, and the next decoded frame.
struct Video {
    element: web_sys::HtmlVideoElement,
    texture: Option<(wgpu::Texture, wgpu::TextureView)>,
    requested: Option<f64>,
    ready: Rc<RefCell<Option<web_sys::ImageBitmap>>>,
}
struct Targets {
    width: u32,
    height: u32,
    format: wgpu::TextureFormat,
    samples: u32,
    color: wgpu::TextureView,
    resolve: Option<wgpu::TextureView>,
    depth: wgpu::TextureView,
    screen: RenderTarget,
    /// Procedural Lucy and floor, the mapped Lambert, the helper and output.
    lucy: (wgpu::RenderPipeline, wgpu::BindGroup, wgpu::BindGroup),
    plane: (wgpu::RenderPipeline, wgpu::BindGroup, wgpu::BindGroup),
    map: (wgpu::RenderPipeline, wgpu::BindGroup, [wgpu::BindGroup; 2]),
    /// The map bind groups' texture: 1 video, 2 colors.png.
    map_source: usize,
    helper: (wgpu::RenderPipeline, [wgpu::BindGroup; 2]),
    output: (wgpu::RenderPipeline, [wgpu::BindGroup; 2]),
}
pub(super) struct Demo {
    controls: Controls,
    time: f64,
    /// type, color, intensity, distance, angle, penumbra, decay, focus, shadows.
    params: [f64; 9],
    lucy: Mesh,
    plane: Mesh,
    cone: (wgpu::Buffer, wgpu::Buffer),
    shadow_depth: wgpu::TextureView,
    shadow_color: wgpu::TextureView,
    depth_pipeline: wgpu::RenderPipeline,
    depth_binds: (wgpu::BindGroup, wgpu::BindGroup),
    depth_render: wgpu::Buffer,
    depth_object: wgpu::Buffer,
    renders: [wgpu::Buffer; 3],
    helper_render: wgpu::Buffer,
    output_render: wgpu::Buffer,
    output_object: wgpu::Buffer,
    quad: wgpu::Buffer,
    sampler: wgpu::Sampler,
    compare: wgpu::Sampler,
    colors: (wgpu::Texture, wgpu::TextureView),
    video: Option<Video>,
    targets: Option<Targets>,
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, _id: u32, r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 40.,
            near: 0.1,
            far: 100.,
            aspect,
            ..Default::default()
        }));
        s.get_mut(c)?.position = Vector3::new(7., 4., 1.);
        let mut controls = Controls::new(None, (2., 10.), PI / 2., true);
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
        let mesh = |g: &BufferGeometry, model: Matrix4| -> Result<Mesh> {
            let f = |name: &str| -> Result<Vec<f32>> {
                let a = g
                    .attributes
                    .get(name)
                    .ok_or(Error::Invalid("mesh attribute"))?;
                (0..a.count())
                    .flat_map(|i| (0..3).map(move |k| (i, k)))
                    .map(|(i, k)| a.get_component(i, k).map(|v| v as f32))
                    .collect()
            };
            let positions = f("position")?;
            let radius = positions
                .chunks(3)
                .map(|p| (p[0] as f64).hypot(p[1] as f64).hypot(p[2] as f64))
                .fold(0., f64::max);
            let index = g.index.clone().ok_or(Error::Invalid("mesh index"))?;
            Ok(Mesh {
                positions: init(
                    "projector positions",
                    bytemuck::cast_slice(&positions),
                    wgpu::BufferUsages::VERTEX,
                ),
                normals: init(
                    "projector normals",
                    bytemuck::cast_slice(&f("normal")?),
                    wgpu::BufferUsages::VERTEX,
                ),
                index: init(
                    "projector index",
                    bytemuck::cast_slice(&index),
                    wgpu::BufferUsages::INDEX,
                ),
                count: index.len() as u32,
                radius,
                model,
                object: uniform(r, "lambert object", 144),
            })
        };
        let (positions, index) = super::refraction_loaders::formats::parse_ply(
            &fetch(&format!("{ASSETS}/ply/binary/Lucy100k.ply")).await?,
        )?;
        let mut g = BufferGeometry::default();
        g.set_attribute(
            "position",
            super::interactive_scenes::positions(positions.iter().map(|v| v * 0.0024).collect())?,
        );
        g.set_index(Some(index));
        g.compute_vertex_normals()?;
        let lucy = mesh(
            &g,
            Matrix4::from_translation(Vector3::new(0., 0.8, 0.))
                * Matrix4::from_rotation_y(-PI / 2.),
        )?;
        let plane = mesh(
            &PlaneGeometry::build(200., 200., 1, 1)?,
            Matrix4::from_translation(Vector3::new(0., -1., 0.))
                * Matrix4::from_rotation_x(-PI / 2.),
        )?;
        // SpotLightHelper's cone: four rays and a 32-segment circle at z = 1.
        let mut cone = vec![
            0., 0., 0., 0., 0., 1., 0., 0., 0., 1., 0., 1., 0., 0., 0., -1., 0., 1., 0., 0., 0.,
            0., 1., 1., 0., 0., 0., 0., -1., 1.,
        ];
        for i in 0..32 {
            let (p1, p2) = (i as f64 / 32. * PI * 2., (i + 1) as f64 / 32. * PI * 2.);
            cone.extend([p1.cos(), p1.sin(), 1., p2.cos(), p2.sin(), 1.]);
        }
        let cone: Vec<f32> = cone.iter().map(|&v| v as f32).collect();
        let module = |label, source: &str| {
            r.device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some(label),
                source: wgpu::ShaderSource::Wgsl(source.into()),
            })
        };
        let position = wgpu::vertex_attr_array![0 => Float32x3];
        let depth_pipeline = r
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("projector shadow"),
                layout: None,
                vertex: wgpu::VertexState {
                    module: &module("shadow", include_str!("shadowmap_vsm/depth_vs.wgsl")),
                    entry_point: Some("main"),
                    compilation_options: Default::default(),
                    buffers: &[wgpu::VertexBufferLayout {
                        array_stride: 12,
                        step_mode: wgpu::VertexStepMode::Vertex,
                        attributes: &position,
                    }],
                },
                fragment: Some(wgpu::FragmentState {
                    module: &module("shadow", include_str!("shadowmap_vsm/depth_fs.wgsl")),
                    entry_point: Some("main"),
                    compilation_options: Default::default(),
                    targets: &[Some(wgpu::TextureFormat::Rgba8Unorm.into())],
                }),
                // As the page's shadow pipeline: clockwise front faces.
                primitive: wgpu::PrimitiveState {
                    front_face: wgpu::FrontFace::Cw,
                    cull_mode: Some(wgpu::Face::Back),
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
            });
        let depth_render = uniform(r, "shadow render", 128);
        let depth_object = uniform(r, "shadow object", 80);
        let depth_binds = (
            bind(
                r,
                depth_pipeline.get_bind_group_layout(0),
                &[(0, depth_render.as_entire_binding())],
            ),
            bind(
                r,
                depth_pipeline.get_bind_group_layout(1),
                &[(0, depth_object.as_entire_binding())],
            ),
        );
        // colors.png: sRGB, flipped on upload.
        let image =
            decode_texture_image(&fetch(&format!("{ASSETS}/spot-skinning/colors.png")).await?)
                .await?;
        let colors = map_texture(r, image.width, image.height);
        let row = image.width as usize * 4;
        let flipped: Vec<u8> = image.rgba.chunks(row).rev().flatten().copied().collect();
        r.queue.write_texture(
            colors.as_image_copy(),
            &flipped,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(row as u32),
                rows_per_image: Some(image.height),
            },
            colors.size(),
        );
        let colors_view = colors.create_view(&Default::default());
        let lucy_size = pack(wgsl!("lucy_fs"), "renderStruct", &[])?.len() as u64;
        let plane_size = pack(wgsl!("plane_fs"), "renderStruct", &[])?.len() as u64;
        let map_size = pack(wgsl!("map_fs"), "renderStruct", &[])?.len() as u64;
        Ok(Self {
            controls,
            time: 0.,
            params: [0., 0xffffff as f64, 100., 0., PI / 6., 1., 2., 1., 1.],
            lucy,
            plane,
            cone: (
                init(
                    "helper cone",
                    bytemuck::cast_slice(&cone),
                    wgpu::BufferUsages::VERTEX,
                ),
                uniform(r, "helper object", 80),
            ),
            shadow_depth: texture(r, (SHADOW_SIZE, SHADOW_SIZE), DEPTH, 1, true),
            shadow_color: texture(
                r,
                (SHADOW_SIZE, SHADOW_SIZE),
                wgpu::TextureFormat::Rgba8Unorm,
                1,
                false,
            ),
            depth_pipeline,
            depth_binds,
            depth_render,
            depth_object,
            renders: [
                uniform(r, "lucy render", lucy_size),
                uniform(r, "plane render", plane_size),
                uniform(r, "map render", map_size),
            ],
            helper_render: uniform(r, "helper render", 128),
            output_render: uniform(r, "output render", 144),
            output_object: uniform(r, "output object", 64),
            quad: init(
                "quad",
                bytemuck::cast_slice(&[-1f32, 3., 0., -1., -1., 0., 3., -1., 0.]),
                wgpu::BufferUsages::VERTEX,
            ),
            sampler: r.device.create_sampler(&wgpu::SamplerDescriptor {
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
            colors: (colors, colors_view),
            video: None,
            targets: None,
        })
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, dt: f64, animate: bool) -> Result<()> {
        if animate {
            self.time += dt;
        }
        Ok(())
    }
    /// The video mode uploads each new frame once.
    pub fn prepare(&mut self, _s: &mut Scene, _c: Object3D, r: &Renderer) -> Result<()> {
        let Some(video) = self.video.as_mut() else {
            return Ok(());
        };
        // Until its first frame the video texture is a black placeholder.
        if video.texture.is_none() {
            let texture = map_texture(r, 1, 1);
            r.queue.write_texture(
                texture.as_image_copy(),
                &[0, 0, 0, 255],
                wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(4),
                    rows_per_image: Some(1),
                },
                texture.size(),
            );
            let view = texture.create_view(&Default::default());
            video.texture = Some((texture, view));
            if let Some(t) = self.targets.as_mut() {
                t.map_source = 0;
            }
        }
        if let Some(bitmap) = video.ready.borrow_mut().take() {
            let (width, height) = (bitmap.width(), bitmap.height());
            if video
                .texture
                .as_ref()
                .is_none_or(|(t, _)| t.width() != width || t.height() != height)
            {
                let texture = map_texture(r, width, height);
                let view = texture.create_view(&Default::default());
                video.texture = Some((texture, view));
                // New bind groups for the new texture.
                if let Some(t) = self.targets.as_mut() {
                    t.map_source = 0;
                }
            }
            if let Some((texture, _)) = &video.texture {
                r.queue.copy_external_image_to_texture(
                    &wgpu::CopyExternalImageSourceInfo {
                        source: wgpu::ExternalImageSource::ImageBitmap(bitmap.clone()),
                        origin: wgpu::Origin2d::ZERO,
                        flip_y: true,
                    },
                    wgpu::CopyExternalImageDestInfo {
                        texture,
                        mip_level: 0,
                        origin: wgpu::Origin3d::ZERO,
                        aspect: wgpu::TextureAspect::All,
                        color_space: wgpu::PredefinedColorSpace::Srgb,
                        premultiplied_alpha: false,
                    },
                    texture.size(),
                );
            }
            bitmap.close();
        }
        let time = video.element.current_time();
        if video.element.ready_state() >= 2 && video.requested != Some(time) {
            video.requested = Some(time);
            let (element, ready) = (video.element.clone(), video.ready.clone());
            wasm_bindgen_futures::spawn_local(async move {
                if let Ok(bitmap) = video_frame(&element).await
                    && let Some(stale) = ready.borrow_mut().replace(bitmap)
                {
                    stale.close();
                }
            });
        }
        Ok(())
    }
    fn resize(&mut self, r: &Renderer, out: &RenderTarget) -> Result<()> {
        let size = (out.width, out.height);
        let samples = out.options.samples.max(1);
        let color = texture(r, size, HALF, samples, samples == 1);
        let resolve = (samples > 1).then(|| texture(r, size, HALF, 1, true));
        let depth = texture(r, size, DEPTH, samples, false);
        let module = |label, source: &str| {
            r.device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some(label),
                source: wgpu::ShaderSource::Wgsl(source.into()),
            })
        };
        let attributes = [
            wgpu::vertex_attr_array![0 => Float32x3],
            wgpu::vertex_attr_array![1 => Float32x3],
        ];
        let buffers = attributes
            .each_ref()
            .map(|attributes| wgpu::VertexBufferLayout {
                array_stride: 12,
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes,
            });
        let pipeline = |label, vs: &str, fs: &str, buffers, format, scene: bool, lines: bool| {
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
                        targets: &[Some(wgpu::ColorTargetState::from(format))],
                    }),
                    primitive: wgpu::PrimitiveState {
                        topology: if lines {
                            wgpu::PrimitiveTopology::LineList
                        } else {
                            wgpu::PrimitiveTopology::TriangleList
                        },
                        cull_mode: Some(wgpu::Face::Back),
                        ..Default::default()
                    },
                    depth_stencil: scene.then(|| wgpu::DepthStencilState {
                        format: DEPTH,
                        depth_write_enabled: true,
                        depth_compare: wgpu::CompareFunction::LessEqual,
                        stencil: Default::default(),
                        bias: Default::default(),
                    }),
                    multisample: wgpu::MultisampleState {
                        count: if scene { samples } else { 1 },
                        ..Default::default()
                    },
                    multiview: None,
                    cache: None,
                })
        };
        let shadow = wgpu::BindingResource::TextureView(&self.shadow_depth);
        let compare = wgpu::BindingResource::Sampler(&self.compare);
        let lambert = |label, vs, fs, render: &wgpu::Buffer| {
            let p = pipeline(label, vs, fs, &buffers[..], HALF, true, false);
            let render = bind(
                r,
                p.get_bind_group_layout(0),
                &[(0, render.as_entire_binding())],
            );
            (p, render)
        };
        let object = |p: &wgpu::RenderPipeline, m: &Mesh| {
            bind(
                r,
                p.get_bind_group_layout(1),
                &[
                    (0, m.object.as_entire_binding()),
                    (1, compare.clone()),
                    (2, shadow.clone()),
                ],
            )
        };
        let (lucy, lucy_render) = lambert(
            "projector lucy",
            wgsl!("lucy_vs"),
            wgsl!("lucy_fs"),
            &self.renders[0],
        );
        let lucy_object = object(&lucy, &self.lucy);
        let (plane, plane_render) = lambert(
            "projector floor",
            wgsl!("plane_vs"),
            wgsl!("plane_fs"),
            &self.renders[1],
        );
        let plane_object = object(&plane, &self.plane);
        let (map, map_render) = lambert(
            "projector map",
            wgsl!("map_vs"),
            wgsl!("map_fs"),
            &self.renders[2],
        );
        let (map_objects, map_source) = self.map_binds(r, &map);
        let helper = pipeline(
            "spot light helper",
            wgsl!("helper_vs"),
            wgsl!("helper_fs"),
            &buffers[..1],
            HALF,
            true,
            true,
        );
        let helper_binds = [
            bind(
                r,
                helper.get_bind_group_layout(0),
                &[(0, self.helper_render.as_entire_binding())],
            ),
            bind(
                r,
                helper.get_bind_group_layout(1),
                &[(0, self.cone.1.as_entire_binding())],
            ),
        ];
        let output = pipeline(
            "render output",
            wgsl!("output_vs"),
            wgsl!("output_fs"),
            &buffers[..1],
            out.options.format,
            false,
            false,
        );
        let output_binds = [
            bind(
                r,
                output.get_bind_group_layout(0),
                &[(0, self.output_render.as_entire_binding())],
            ),
            bind(
                r,
                output.get_bind_group_layout(1),
                &[
                    (0, wgpu::BindingResource::Sampler(&self.sampler)),
                    (
                        1,
                        wgpu::BindingResource::TextureView(resolve.as_ref().unwrap_or(&color)),
                    ),
                    (2, self.output_object.as_entire_binding()),
                ],
            ),
        ];
        r.queue.write_buffer(
            &self.output_render,
            0,
            &pack(
                wgsl!("output_fs"),
                "renderStruct",
                &[
                    ("cameraProjectionMatrix", &m4(Matrix4::IDENTITY)),
                    ("cameraViewMatrix", &m4(Matrix4::IDENTITY)),
                    ("nodeUniform1", &[size.0 as f64, size.1 as f64]),
                    ("nodeUniform2", &[1.]),
                ],
            )?,
        );
        r.queue.write_buffer(
            &self.output_object,
            0,
            bytemuck::cast_slice(&[
                1f32, 0., 0., 0., 0., 1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1.,
            ]),
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
            samples,
            color,
            resolve,
            depth,
            screen,
            lucy: (lucy, lucy_render, lucy_object),
            plane: (plane, plane_render, plane_object),
            map: (map, map_render, map_objects),
            map_source,
            helper: (helper, helper_binds),
            output: (output, output_binds),
        });
        Ok(())
    }
    /// The mapped materials' object bind groups for the current map.
    fn map_binds(&self, r: &Renderer, map: &wgpu::RenderPipeline) -> ([wgpu::BindGroup; 2], usize) {
        let (view, source) = match self.video.as_ref().and_then(|v| v.texture.as_ref()) {
            Some((_, view)) if self.params[0].round() == 1. => (view, 1),
            _ => (&self.colors.1, 2),
        };
        let binds = [&self.lucy, &self.plane].map(|m| {
            bind(
                r,
                map.get_bind_group_layout(1),
                &[
                    (0, m.object.as_entire_binding()),
                    (1, wgpu::BindingResource::Sampler(&self.compare)),
                    (2, wgpu::BindingResource::TextureView(&self.shadow_depth)),
                    (3, wgpu::BindingResource::Sampler(&self.sampler)),
                    (4, wgpu::BindingResource::TextureView(view)),
                ],
            )
        });
        (binds, source)
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
                || t.samples != out.options.samples.max(1)
        }) {
            self.resize(r, out)?;
        }
        let mode = self.params[0].round() as usize;
        let video_ready = self.video.as_ref().is_some_and(|v| v.texture.is_some());
        if mode != 0
            && let Some(t) = self.targets.as_ref()
        {
            let wanted = if mode == 1 && video_ready { 1 } else { 2 };
            if t.map_source != wanted {
                let (binds, source) = self.map_binds(r, &t.map.0);
                if let Some(t) = self.targets.as_mut() {
                    t.map.2 = binds;
                    t.map_source = source;
                }
            }
        }
        s.update()?;
        let (camera, world) = s.camera(c)?;
        let (projection, view) = (camera.projection_matrix()?, world.inverse());
        // animate(): the projector circles at y = 5 every 3π·3 s.
        let t = self.time / 3.;
        let light = Vector3::new(t.cos() * 2.5, 5., t.sin() * 2.5);
        let [
            kind,
            hex,
            intensity,
            distance,
            angle,
            penumbra,
            decay,
            focus,
            shadows,
        ] = self.params;
        // ProjectorLight.update: the shadow aspect is the map's.
        let aspect = match kind.round() as usize {
            1 => self
                .video
                .as_ref()
                .and_then(|v| v.texture.as_ref())
                .map_or(1., |(t, _)| t.width() as f64 / t.height() as f64),
            _ => 1.,
        };
        let far = if distance > 0. { distance } else { 10. };
        let shadow_projection = perspective((2. * angle * focus).to_degrees(), aspect, 1., far);
        let shadow_view = Matrix4::look_at_rh(light, Vector3::ZERO, Vector3::Y);
        let bias = Matrix4::from_cols_array(&[
            0.5, 0., 0., 0., 0., 0.5, 0., 0., 0., 0., 1., 0., 0.5, 0.5, 0., 1.,
        ]);
        let shadow_matrix = bias * shadow_projection * shadow_view;
        let color = Color::from_hex(hex as u32).0.to_array();
        let linear = |hex| Color::from_hex(hex).0.to_array().map(|v| v * 0.15);
        let light_view = view.transform_point3(light);
        let values: Vec<(&str, Vec<f64>)> = vec![
            ("nodeUniform23", vec![self.time]),
            ("cameraProjectionMatrix", m4(projection)),
            ("cameraViewMatrix", m4(view)),
            ("nodeUniform5", linear(0xffffff).to_vec()),
            ("nodeUniform9", vec![0., 1., 0.]),
            ("nodeUniform4", linear(0x8d8d8d).to_vec()),
            (
                "nodeUniform12",
                vec![(angle * (1. - penumbra)).cos().min(0.99999)],
            ),
            ("nodeUniform21", vec![distance]),
            ("nodeUniform22", vec![decay]),
            ("nodeUniform14", color.map(|v| v * intensity).to_vec()),
            ("nodeUniform13", light_view.to_array().to_vec()),
            ("nodeUniform10", m4(shadow_matrix)),
            ("nodeUniform15", vec![0.]),
            ("nodeUniform16", vec![0.]),
            ("nodeUniform18", vec![1.]),
            (
                "nodeUniform19",
                vec![SHADOW_SIZE as f64, SHADOW_SIZE as f64],
            ),
            // Shadows off: the shadow factor is 1, as without the shadow map.
            ("nodeUniform20", vec![if shadows > 0.5 { 1. } else { 0. }]),
        ];
        let values: Vec<(&str, &[f64])> = values.iter().map(|(n, v)| (*n, &v[..])).collect();
        for (buffer, source) in
            self.renders
                .iter()
                .zip([wgsl!("lucy_fs"), wgsl!("plane_fs"), wgsl!("map_fs")])
        {
            r.queue
                .write_buffer(buffer, 0, &pack(source, "renderStruct", &values)?);
        }
        for (m, hex) in [(&self.lucy, 0xffffff), (&self.plane, 0xbcbcbc)] {
            r.queue.write_buffer(
                &m.object,
                0,
                &pack(
                    wgsl!("lucy_fs"),
                    "objectStruct",
                    &[
                        ("nodeUniform0", &Color::from_hex(hex).0.to_array()),
                        ("nodeUniform1", &[1.]),
                        ("nodeUniform3", &[1.]),
                        ("nodeUniform7", &m3(m.model.inverse().transpose())),
                        ("nodeUniform11", &m4(m.model)),
                    ],
                )?,
            );
        }
        // SpotLightHelper.update: the cone scaled to the light's reach and
        // turned toward the target.
        let length = if distance > 0. { distance } else { 1000. };
        let width = length * angle.tan();
        let z = (Vector3::ZERO - light).normalize();
        let x = Vector3::Y.cross(z).normalize();
        let cone = Matrix4::from_translation(light)
            * Matrix4::from_mat3(glam::DMat3::from_cols(x, z.cross(x), z))
            * Matrix4::from_scale(Vector3::new(width, width, length));
        let mut camera_uniforms: Vec<f32> = projection.to_cols_array().map(|v| v as f32).to_vec();
        camera_uniforms.extend(view.to_cols_array().map(|v| v as f32));
        r.queue.write_buffer(
            &self.helper_render,
            0,
            bytemuck::cast_slice(&camera_uniforms),
        );
        let mut helper = color.map(|v| v as f32).to_vec();
        helper.push(1.);
        helper.extend(cone.to_cols_array().map(|v| v as f32));
        r.queue
            .write_buffer(&self.cone.1, 0, bytemuck::cast_slice(&helper));
        let mut depth_object = vec![1f32, 0., 0., 0.];
        depth_object.extend(self.lucy.model.to_cols_array().map(|v| v as f32));
        r.queue
            .write_buffer(&self.depth_object, 0, bytemuck::cast_slice(&depth_object));
        let mut depth_render: Vec<f32> =
            shadow_projection.to_cols_array().map(|v| v as f32).to_vec();
        depth_render.extend(shadow_view.to_cols_array().map(|v| v as f32));
        r.queue
            .write_buffer(&self.depth_render, 0, bytemuck::cast_slice(&depth_render));
        // Visible meshes, front to back; the helper cone follows.
        let screen = projection * view;
        let frustum = Frustum::from_projection(screen);
        let mut draws = vec![];
        for (i, m) in [&self.lucy, &self.plane].into_iter().enumerate() {
            let center = m.model.transform_point3(Vector3::ZERO);
            if frustum.intersects_sphere(Sphere {
                center,
                radius: m.radius,
            }) {
                draws.push((screen.project_point3(center).z, i));
            }
        }
        draws.sort_by(|a, b| {
            a.0.partial_cmp(&b.0)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then(a.1.cmp(&b.1))
        });
        let t = self
            .targets
            .as_ref()
            .ok_or(Error::Invalid("projector targets"))?;
        let mut encoder = r.device.create_command_encoder(&Default::default());
        if shadows > 0.5 {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("projector shadow"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &self.shadow_color,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                        store: wgpu::StoreOp::Discard,
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
            pass.set_pipeline(&self.depth_pipeline);
            pass.set_bind_group(0, &self.depth_binds.0, &[]);
            pass.set_bind_group(1, &self.depth_binds.1, &[]);
            pass.set_vertex_buffer(0, self.lucy.positions.slice(..));
            pass.set_index_buffer(self.lucy.index.slice(..), wgpu::IndexFormat::Uint32);
            pass.draw_indexed(0..self.lucy.count, 0, 0..1);
        }
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("projector scene"),
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
                        store: wgpu::StoreOp::Discard,
                    }),
                    stencil_ops: None,
                }),
                ..Default::default()
            });
            for &(_, i) in &draws {
                let m = if i == 0 { &self.lucy } else { &self.plane };
                let (pipeline, render, object) = match (mode, i) {
                    (0, 0) => (&t.lucy.0, &t.lucy.1, &t.lucy.2),
                    (0, _) => (&t.plane.0, &t.plane.1, &t.plane.2),
                    _ => (&t.map.0, &t.map.1, &t.map.2[i]),
                };
                pass.set_pipeline(pipeline);
                pass.set_bind_group(0, render, &[]);
                pass.set_bind_group(1, object, &[]);
                pass.set_vertex_buffer(0, m.normals.slice(..));
                pass.set_vertex_buffer(1, m.positions.slice(..));
                pass.set_index_buffer(m.index.slice(..), wgpu::IndexFormat::Uint32);
                pass.draw_indexed(0..m.count, 0, 0..1);
            }
            pass.set_pipeline(&t.helper.0);
            pass.set_bind_group(0, &t.helper.1[0], &[]);
            pass.set_bind_group(1, &t.helper.1[1], &[]);
            pass.set_vertex_buffer(0, self.cone.0.slice(..));
            pass.draw(0..74, 0..1);
        }
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("projector output"),
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
            pass.set_pipeline(&t.output.0);
            pass.set_bind_group(0, &t.output.1[0], &[]);
            pass.set_bind_group(1, &t.output.1[1], &[]);
            pass.set_vertex_buffer(0, self.quad.slice(..));
            pass.draw(0..3, 0..1);
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
    /// type, color, intensity, distance, angle, penumbra, decay, focus and
    /// shadows. Choosing a type sets the focus as the page does, and the
    /// video starts playing the first time it is chosen.
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        *self
            .params
            .get_mut(index)
            .ok_or(Error::Invalid("projector parameter"))? = value as f64;
        if index == 0 {
            let kind = value.round() as usize;
            self.params[7] = if kind == 1 { 0.46 } else { 1. };
            if kind == 1 && self.video.is_none() {
                use wasm_bindgen::JsCast;
                let element: web_sys::HtmlVideoElement = web_sys::window()
                    .and_then(|w| w.document())
                    .and_then(|d| d.get_element_by_id("video"))
                    .ok_or(Error::Invalid("video element"))?
                    .dyn_into()
                    .map_err(|_| Error::Invalid("video element"))?;
                let _ = element.play();
                self.video = Some(Video {
                    element,
                    texture: None,
                    requested: None,
                    ready: Rc::new(RefCell::new(None)),
                });
            }
        }
        Ok(())
    }
    pub fn seek(&mut self, t: f64) {
        self.time = t;
    }
}
