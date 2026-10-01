//! webgpu_cubemap_dynamic: a mirror sphere reflecting a 256² CubeCamera
//! render of the pisaHDR sky, an orbiting textured box and an orbiting
//! textured torus knot. Each frame the CubeCamera renders its six faces with
//! the sphere hidden, PMREMNode regenerates the sphere's environment from
//! the cube (the cubeUV pass and ten GGX levels), and the scene renders with
//! 4× MSAA before the ACES output pass. The box and the torus light from the
//! sky's PMREM, generated once. Every stage runs the WGSL three.js r186
//! generates for the page (in `cubemap_dynamic/`).
use super::controls_attributes::{Controls, camera_state};
use super::gltf_viewer::{decode_texture_image, fetch};
use super::lights_projector::{m3, m4, pack};
use super::pmrem_cube_uv::{GGX_8, Pmrem, bind, flat_camera, raw_pipeline, source_pipeline};
use super::retro::{mipmapped, target, uniform};
use super::shadowmap_opacity::Mipmaps;
use super::trackball_sprites::parse_rgbe;
use crate::{
    Error, Result, camera::*, geometry::*, math::*, render_target::*, renderer::*, scene::*,
};
use std::f64::consts::PI;
use wgpu::util::DeviceExt;

const HALF: wgpu::TextureFormat = wgpu::TextureFormat::Rgba16Float;
const DEPTH: wgpu::TextureFormat = wgpu::TextureFormat::Depth24Plus;
const CUBE: u32 = 256;
macro_rules! wgsl {
    ($name:literal) => {
        include_str!(concat!("cubemap_dynamic/", $name, ".wgsl"))
    };
}
/// CubeCamera( 1, 1000 )'s WebGPU projection and its six face views, in layer order.
fn face_cameras() -> (Matrix4, [Matrix4; 6]) {
    let m = 1000. / (1. - 1000.);
    let projection = Matrix4::from_cols_array(&[
        -1., 0., 0., 0., 0., -1., 0., 0., 0., 0., m, -1., 0., 0., m, 0.,
    ]);
    let views = [
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
    ]
    .map(|v| Matrix4::from_cols_array(&v));
    (projection, views)
}
/// Vertex buffers, an optional index and the bounding sphere in local space.
struct Mesh {
    buffers: Vec<wgpu::Buffer>,
    index: Option<wgpu::Buffer>,
    count: u32,
    sphere: Sphere,
}
impl Mesh {
    fn draw(&self, pass: &mut wgpu::RenderPass) {
        for (slot, buffer) in self.buffers.iter().enumerate() {
            pass.set_vertex_buffer(slot as u32, buffer.slice(..));
        }
        if let Some(index) = &self.index {
            pass.set_index_buffer(index.slice(..), wgpu::IndexFormat::Uint32);
            pass.draw_indexed(0..self.count, 0, 0..1);
        } else {
            pass.draw(0..self.count, 0..1);
        }
    }
}
/// The attributes `names` of `geometry` ( stride 2 for uv, else 3 ) as vertex buffers.
fn mesh(r: &Renderer, geometry: &BufferGeometry, names: &[&str]) -> Result<Mesh> {
    let read = |name: &str| -> Result<Vec<f32>> {
        let a = geometry
            .attributes
            .get(name)
            .ok_or(Error::Invalid("cubemap attribute"))?;
        let n = if name == "uv" { 2 } else { 3 };
        (0..a.count())
            .flat_map(|i| (0..n).map(move |k| (i, k)))
            .map(|(i, k)| a.get_component(i, k).map(|v| v as f32))
            .collect()
    };
    let init = |data: &[u8], usage| {
        r.device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("cubemap mesh"),
                contents: data,
                usage,
            })
    };
    let positions = read("position")?;
    let points: Vec<Vector3> = positions
        .chunks(3)
        .map(|p| Vector3::new(p[0] as f64, p[1] as f64, p[2] as f64))
        .collect();
    // computeBoundingSphere: the box center and the farthest point.
    let (min, max) = points.iter().fold(
        (
            Vector3::splat(f64::INFINITY),
            Vector3::splat(f64::NEG_INFINITY),
        ),
        |(a, b), p| (a.min(*p), b.max(*p)),
    );
    let center = (min + max) * 0.5;
    let radius = points.iter().map(|p| p.distance(center)).fold(0., f64::max);
    let buffers = names
        .iter()
        .map(|name| {
            Ok(init(
                bytemuck::cast_slice(&read(name)?),
                wgpu::BufferUsages::VERTEX,
            ))
        })
        .collect::<Result<_>>()?;
    let (index, count) = match &geometry.index {
        Some(index) => (
            Some(init(bytemuck::cast_slice(index), wgpu::BufferUsages::INDEX)),
            index.len() as u32,
        ),
        None => (None, points.len() as u32),
    };
    Ok(Mesh {
        buffers,
        index,
        count,
        sphere: Sphere { center, radius },
    })
}
type Draw = (wgpu::RenderPipeline, Vec<wgpu::BindGroup>);
/// One pass's pipelines: the background, the box and torus, and the sphere
/// ( None in the CubeCamera pass, which hides it ).
struct Pipelines {
    background: wgpu::RenderPipeline,
    standard: wgpu::RenderPipeline,
    mirror: Option<wgpu::RenderPipeline>,
}
struct Targets {
    width: u32,
    height: u32,
    format: wgpu::TextureFormat,
    samples: u32,
    msaa: Option<(wgpu::TextureView, wgpu::TextureView)>,
    color: wgpu::TextureView,
    depth: wgpu::TextureView,
    /// The main pass: background, box, torus and sphere draws.
    background: Draw,
    objects: [Draw; 2],
    mirror: Draw,
    output: Draw,
    screen: RenderTarget,
}
pub(super) struct Demo {
    controls: Controls,
    /// The example clock and the last animated time ( per-frame increments
    /// as 60 fps steps ).
    time: f64,
    last: f64,
    pending: Option<f64>,
    /// cube and torus rotation.x and rotation.y.
    rotation: (f64, f64),
    /// roughness, metalness, exposure, environmentIntensity, envMapIntensity.
    params: [f64; 5],
    background: Mesh,
    /// The box and the torus knot.
    objects: [Mesh; 2],
    sphere: Mesh,
    output_quad: wgpu::Buffer,
    /// The CubeCamera's target, its faces and depth.
    cube_faces: Vec<wgpu::TextureView>,
    cube_depth: wgpu::TextureView,
    /// The face passes' draws: per face, the background and the two objects.
    face_draws: Vec<(Draw, [Draw; 2])>,
    /// The sky's PMREM and the dynamic one, with its source pass.
    dynamic: Pmrem,
    dynamic_source: Draw,
    sky_pmrem: Pmrem,
    sky: wgpu::TextureView,
    map: wgpu::TextureView,
    linear: wgpu::Sampler,
    clamp: wgpu::Sampler,
    /// The face cameras' and the main camera's render structs.
    background_renders: Vec<wgpu::Buffer>,
    standard_renders: Vec<wgpu::Buffer>,
    mirror_render: wgpu::Buffer,
    background_object: wgpu::Buffer,
    object_buffers: [wgpu::Buffer; 2],
    mirror_object: wgpu::Buffer,
    output_render: wgpu::Buffer,
    output_object: wgpu::Buffer,
    targets: Option<Targets>,
}
fn layouts(slots: &[(u32, wgpu::VertexFormat, u64)]) -> Vec<[wgpu::VertexAttribute; 1]> {
    slots
        .iter()
        .map(|&(location, format, _)| {
            [wgpu::VertexAttribute {
                format,
                offset: 0,
                shader_location: location,
            }]
        })
        .collect()
}
const X3: wgpu::VertexFormat = wgpu::VertexFormat::Float32x3;
const X2: wgpu::VertexFormat = wgpu::VertexFormat::Float32x2;
/// normal, position.
const BACKGROUND_SLOTS: [(u32, wgpu::VertexFormat, u64); 2] = [(0, X3, 12), (1, X3, 12)];
/// uv, normal, position.
const STANDARD_SLOTS: [(u32, wgpu::VertexFormat, u64); 3] = [(0, X2, 8), (1, X3, 12), (2, X3, 12)];
fn pipelines(r: &Renderer, samples: u32, mirror: bool) -> Pipelines {
    let build = |label, vs, fs, slots: &[(u32, wgpu::VertexFormat, u64)], depth, cw| {
        let attributes = layouts(slots);
        let buffers: Vec<wgpu::VertexBufferLayout> = slots
            .iter()
            .zip(&attributes)
            .map(|(&(_, _, stride), attributes)| wgpu::VertexBufferLayout {
                array_stride: stride,
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes,
            })
            .collect();
        raw_pipeline(r, label, vs, fs, &buffers, HALF, samples, Some(depth), cw)
    };
    let less = (wgpu::CompareFunction::LessEqual, true);
    Pipelines {
        background: build(
            "cubemap background",
            wgsl!("background_vs"),
            wgsl!("background_fs"),
            &BACKGROUND_SLOTS,
            (wgpu::CompareFunction::Always, false),
            true,
        ),
        standard: build(
            "cubemap standard",
            wgsl!("standard_vs"),
            wgsl!("standard_fs"),
            &STANDARD_SLOTS,
            less,
            false,
        ),
        mirror: mirror.then(|| {
            build(
                "cubemap mirror",
                wgsl!("mirror_vs"),
                wgsl!("mirror_fs"),
                &BACKGROUND_SLOTS,
                less,
                false,
            )
        }),
    }
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, _id: u32, r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 60.,
            near: 1.,
            far: 1000.,
            aspect,
            ..Default::default()
        }));
        let n = s.get_mut(c)?;
        n.position = Vector3::new(0., 0., 75.);
        n.quaternion = Quaternion::IDENTITY;
        let mut controls = Controls::new(None, (0., f64::INFINITY), PI, true);
        controls.auto_rotate = Some(2.);
        controls.update(s, c)?;
        // HDRCubeTextureLoader: six half-float faces, no mipmaps, not flipped.
        let sky_texture = r.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("pisaHDR"),
            size: wgpu::Extent3d {
                width: CUBE,
                height: CUBE,
                depth_or_array_layers: 6,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: HALF,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        for (layer, face) in ["px", "nx", "py", "ny", "pz", "nz"].iter().enumerate() {
            let (width, height, texels) = parse_rgbe(
                &fetch(&format!(
                    "/web/gallery/assets/tsl-environment/textures/cube/pisaHDR/{face}.hdr"
                ))
                .await?,
            )?;
            if (width, height) != (CUBE, CUBE) {
                return Err(Error::Invalid("pisaHDR face size"));
            }
            r.queue.write_texture(
                wgpu::TexelCopyTextureInfo {
                    texture: &sky_texture,
                    mip_level: 0,
                    origin: wgpu::Origin3d {
                        x: 0,
                        y: 0,
                        z: layer as u32,
                    },
                    aspect: wgpu::TextureAspect::All,
                },
                bytemuck::cast_slice(&texels),
                wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(width * 8),
                    rows_per_image: Some(height),
                },
                wgpu::Extent3d {
                    width,
                    height,
                    depth_or_array_layers: 1,
                },
            );
        }
        let cube_view = |t: &wgpu::Texture| {
            t.create_view(&wgpu::TextureViewDescriptor {
                dimension: Some(wgpu::TextureViewDimension::Cube),
                ..Default::default()
            })
        };
        let sky = cube_view(&sky_texture);
        // uv_grid_opengl.jpg: flipped, mipmapped, no color space.
        let image = decode_texture_image(
            &fetch("/web/gallery/assets/environment-materials/textures/uv_grid_opengl.jpg").await?,
        )
        .await?;
        let row = image.width as usize * 4;
        let flipped = crate::material::Texture {
            rgba: image.rgba.chunks(row).rev().flatten().copied().collect(),
            ..image
        };
        let mut mipmaps = Mipmaps::new(r);
        let map = mipmapped(r, &mut mipmaps, &flipped, wgpu::TextureFormat::Rgba8Unorm);
        let sampler = |mipmap| {
            r.device.create_sampler(&wgpu::SamplerDescriptor {
                mag_filter: wgpu::FilterMode::Linear,
                min_filter: wgpu::FilterMode::Linear,
                mipmap_filter: mipmap,
                ..Default::default()
            })
        };
        let linear = sampler(wgpu::FilterMode::Linear);
        let clamp = sampler(wgpu::FilterMode::Nearest);
        let background = mesh(
            r,
            &SphereGeometry::build(1., 32, 32)?,
            &["normal", "position"],
        )?;
        let standard = ["uv", "normal", "position"];
        let objects = [
            mesh(r, &BoxGeometry::build(15., 15., 15.)?, &standard)?,
            mesh(
                r,
                &TorusKnotGeometry::build(8., 3., 128, 16, 2, 3)?,
                &standard,
            )?,
        ];
        let sphere = mesh(
            r,
            &IcosahedronGeometry::build(15., 8)?,
            &["normal", "position"],
        )?;
        // The CubeCamera's 256² half-float target ( one level ) and its depth.
        let cube_texture = r.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("cubemap target"),
            size: wgpu::Extent3d {
                width: CUBE,
                height: CUBE,
                depth_or_array_layers: 6,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: HALF,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });
        let cube_faces = (0..6)
            .map(|layer| {
                cube_texture.create_view(&wgpu::TextureViewDescriptor {
                    dimension: Some(wgpu::TextureViewDimension::D2),
                    base_array_layer: layer,
                    array_layer_count: Some(1),
                    ..Default::default()
                })
            })
            .collect();
        let cube = cube_view(&cube_texture);
        let cube_depth = target(r, (CUBE, CUBE), DEPTH, 1);
        let init = |label, data: &[u8], usage| {
            r.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some(label),
                    contents: data,
                    usage,
                })
        };
        // PMREMGenerator.fromCubemap for the sky ( once ) and the CubeCamera's target ( each frame ).
        let source = source_pipeline(
            r,
            "cubemap PMREM",
            wgsl!("pmrem_cube_vs"),
            wgsl!("pmrem_cube_fs"),
        );
        let source_object = init(
            "cubemap PMREM",
            &pack(
                wgsl!("pmrem_cube_fs"),
                "objectStruct",
                &[
                    ("nodeUniform1", &m4(Matrix4::IDENTITY)),
                    ("nodeUniform4", &m4(Matrix4::IDENTITY)),
                ],
            )?,
            wgpu::BufferUsages::UNIFORM,
        );
        let source_camera = flat_camera(r, wgsl!("pmrem_cube_vs"))?;
        let source_groups = |texture: &wgpu::TextureView| {
            vec![
                bind(
                    r,
                    source.get_bind_group_layout(0),
                    &[(0, source_camera.as_entire_binding())],
                ),
                bind(
                    r,
                    source.get_bind_group_layout(1),
                    &[
                        (0, wgpu::BindingResource::Sampler(&linear)),
                        (1, wgpu::BindingResource::TextureView(texture)),
                        (2, source_object.as_entire_binding()),
                    ],
                ),
            ]
        };
        let sky_pmrem = Pmrem::new(r, &clamp, 8, GGX_8)?;
        let groups = source_groups(&sky);
        let mut encoder = r.device.create_command_encoder(&Default::default());
        sky_pmrem.encode(&mut encoder, &source, [&groups[0], &groups[1]]);
        r.queue.submit([encoder.finish()]);
        let dynamic = Pmrem::new(r, &clamp, 8, GGX_8)?;
        let dynamic_source = (source.clone(), source_groups(&cube));
        let render = |label, source| uniform(r, label, source, "renderStruct");
        let background_renders = (0..7)
            .map(|_| render("cubemap background render", wgsl!("background_fs")))
            .collect::<Result<Vec<_>>>()?;
        let standard_renders = (0..7)
            .map(|_| render("cubemap standard render", wgsl!("standard_fs")))
            .collect::<Result<Vec<_>>>()?;
        let object = |label, source| uniform(r, label, source, "objectStruct");
        let mut demo = Self {
            controls,
            time: 0.,
            last: 0.,
            pending: None,
            rotation: (0., 0.),
            params: [0.05, 1., 1., 1., 1.],
            background,
            objects,
            sphere,
            output_quad: init(
                "cubemap quad",
                bytemuck::cast_slice(&[-1f32, 3., 0., -1., -1., 0., 3., -1., 0.]),
                wgpu::BufferUsages::VERTEX,
            ),
            cube_faces,
            cube_depth,
            face_draws: vec![],
            dynamic,
            dynamic_source,
            sky_pmrem,
            sky,
            map,
            linear,
            clamp,
            background_renders,
            standard_renders,
            mirror_render: render("cubemap mirror render", wgsl!("mirror_fs"))?,
            background_object: object("cubemap background object", wgsl!("background_fs"))?,
            object_buffers: [
                object("cubemap box object", wgsl!("standard_fs"))?,
                object("cubemap torus object", wgsl!("standard_fs"))?,
            ],
            mirror_object: object("cubemap mirror object", wgsl!("mirror_fs"))?,
            output_render: render("cubemap output render", wgsl!("output_fs"))?,
            output_object: object("cubemap output object", wgsl!("output_vs"))?,
            targets: None,
        };
        let faces = pipelines(r, 1, false);
        demo.face_draws = (0..6)
            .map(|face| {
                (
                    demo.background_draw(r, &faces.background, face),
                    [0, 1].map(|i| demo.standard_draw(r, &faces.standard, face, i)),
                )
            })
            .collect();
        Ok(demo)
    }
    fn background_draw(&self, r: &Renderer, p: &wgpu::RenderPipeline, camera: usize) -> Draw {
        let groups = vec![
            bind(
                r,
                p.get_bind_group_layout(0),
                &[(0, self.background_renders[camera].as_entire_binding())],
            ),
            bind(
                r,
                p.get_bind_group_layout(1),
                &[
                    (0, wgpu::BindingResource::Sampler(&self.linear)),
                    (1, wgpu::BindingResource::TextureView(&self.sky)),
                    (2, self.background_object.as_entire_binding()),
                ],
            ),
        ];
        (p.clone(), groups)
    }
    fn standard_draw(
        &self,
        r: &Renderer,
        p: &wgpu::RenderPipeline,
        camera: usize,
        object: usize,
    ) -> Draw {
        let groups = vec![
            bind(
                r,
                p.get_bind_group_layout(0),
                &[(0, self.standard_renders[camera].as_entire_binding())],
            ),
            bind(
                r,
                p.get_bind_group_layout(1),
                &[
                    (0, self.object_buffers[object].as_entire_binding()),
                    (1, wgpu::BindingResource::Sampler(&self.linear)),
                    (2, wgpu::BindingResource::TextureView(&self.map)),
                    (3, wgpu::BindingResource::Sampler(&self.clamp)),
                    (4, wgpu::BindingResource::TextureView(&r.dfg)),
                    (5, wgpu::BindingResource::Sampler(&self.clamp)),
                    (6, wgpu::BindingResource::TextureView(&self.sky_pmrem.view)),
                ],
            ),
        ];
        (p.clone(), groups)
    }
    /// animate( msTime ): the per-frame turns as 60 fps steps of the elapsed time.
    fn step(&mut self, s: &mut Scene, c: Object3D, dt: f64) -> Result<()> {
        let steps = dt * 60.;
        self.rotation.0 += 0.02 * steps;
        self.rotation.1 += 0.03 * steps;
        self.controls.frame_update_dt(s, c, dt)
    }
    pub fn update(&mut self, s: &mut Scene, c: Object3D, dt: f64, animate: bool) -> Result<()> {
        if animate {
            self.time += dt;
            self.step(s, c, dt)?;
        }
        Ok(())
    }
    pub fn prepare(&mut self, s: &mut Scene, c: Object3D, _r: &Renderer) -> Result<()> {
        if let Some(t) = self.pending.take() {
            let dt = t - self.last;
            self.last = t;
            self.time = t;
            self.step(s, c, dt)?;
        }
        Ok(())
    }
    fn resize(&mut self, r: &Renderer, out: &RenderTarget) -> Result<()> {
        let size = (out.width, out.height);
        let samples = if out.options.samples > 1 { 4 } else { 1 };
        let main = pipelines(r, samples, true);
        let mirror = main
            .mirror
            .as_ref()
            .ok_or(Error::Invalid("mirror pipeline"))?;
        let mirror_groups = vec![
            bind(
                r,
                mirror.get_bind_group_layout(0),
                &[(0, self.mirror_render.as_entire_binding())],
            ),
            bind(
                r,
                mirror.get_bind_group_layout(1),
                &[
                    (0, self.mirror_object.as_entire_binding()),
                    (1, wgpu::BindingResource::Sampler(&self.clamp)),
                    (2, wgpu::BindingResource::TextureView(&r.dfg)),
                    (3, wgpu::BindingResource::Sampler(&self.clamp)),
                    (4, wgpu::BindingResource::TextureView(&self.dynamic.view)),
                ],
            ),
        ];
        let color = target(r, size, HALF, 1);
        let quad = [[wgpu::VertexAttribute {
            format: X3,
            offset: 0,
            shader_location: 0,
        }]];
        let output = raw_pipeline(
            r,
            "cubemap output",
            wgsl!("output_vs"),
            wgsl!("output_fs"),
            &[wgpu::VertexBufferLayout {
                array_stride: 12,
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes: &quad[0],
            }],
            out.options.format,
            1,
            None,
            false,
        );
        let output_groups = vec![
            bind(
                r,
                output.get_bind_group_layout(0),
                &[(0, self.output_render.as_entire_binding())],
            ),
            bind(
                r,
                output.get_bind_group_layout(1),
                &[
                    (0, wgpu::BindingResource::Sampler(&self.clamp)),
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
                    target(r, size, HALF, samples),
                    target(r, size, DEPTH, samples),
                )
            }),
            depth: target(r, size, DEPTH, 1),
            background: self.background_draw(r, &main.background, 6),
            objects: [0, 1].map(|i| self.standard_draw(r, &main.standard, 6, i)),
            mirror: (mirror.clone(), mirror_groups),
            output: (output, output_groups),
            color,
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
        let t = self
            .targets
            .as_ref()
            .ok_or(Error::Invalid("cubemap targets"))?;
        s.update()?;
        let (camera, world) = s.camera(c)?;
        let (projection, view) = (camera.projection_matrix()?, world.inverse());
        let [roughness, metalness, exposure, environment, env_map] = self.params;
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
        // The box and the torus orbit with the clock and turn by the steps.
        let time = self.time;
        let rotation = Euler {
            angles: Vector3::new(self.rotation.0, self.rotation.1, 0.),
            order: EulerOrder::XYZ,
        }
        .quaternion();
        let models = [time, time + 10.].map(|t| {
            Matrix4::from_rotation_translation(
                rotation,
                Vector3::new(t.cos() * 30., t.sin() * 30., t.sin() * 30.),
            )
        });
        let normal = m3(Matrix4::from_quat(rotation));
        let identity3 = m3(Matrix4::IDENTITY);
        for (i, model) in models.iter().enumerate() {
            write(
                &self.object_buffers[i],
                wgsl!("standard_fs"),
                "objectStruct",
                &[
                    ("nodeUniform0", vec![1.; 3]),
                    ("nodeUniform2", identity3.clone()),
                    ("nodeUniform3", vec![1.]),
                    ("nodeUniform4", vec![0.]),
                    ("nodeUniform5", vec![0.1]),
                    ("nodeUniform7", normal.clone()),
                    ("nodeUniform8", vec![0.; 3]),
                    ("nodeUniform9", vec![1.]),
                    ("nodeUniform11", m4(*model)),
                    ("nodeUniform12", vec![8.]),
                    ("nodeUniform13", m4(Matrix4::IDENTITY)),
                    ("nodeUniform15", vec![1. / 768.]),
                    ("nodeUniform16", vec![1. / 1024.]),
                    // The box lights from scene.environment, the torus from its envMap.
                    (
                        "nodeUniform18",
                        vec![if i == 0 { environment } else { env_map }],
                    ),
                ],
            )?;
        }
        write(
            &self.mirror_object,
            wgsl!("mirror_fs"),
            "objectStruct",
            &[
                ("nodeUniform0", vec![1.; 3]),
                ("nodeUniform1", vec![1.]),
                ("nodeUniform2", vec![metalness]),
                ("nodeUniform3", vec![roughness]),
                ("nodeUniform5", identity3.clone()),
                ("nodeUniform6", vec![0.; 3]),
                ("nodeUniform7", vec![1.]),
                ("nodeUniform9", m4(Matrix4::IDENTITY)),
                ("nodeUniform10", vec![8.]),
                ("nodeUniform11", m4(Matrix4::IDENTITY)),
                ("nodeUniform13", vec![1. / 768.]),
                ("nodeUniform14", vec![1. / 1024.]),
                ("nodeUniform16", vec![1.]),
            ],
        )?;
        write(
            &self.background_object,
            wgsl!("background_fs"),
            "objectStruct",
            &[
                ("nodeUniform1", m4(Matrix4::IDENTITY)),
                ("nodeUniform4", identity3),
                ("nodeUniform7", vec![1.]),
                ("nodeUniform9", m4(Matrix4::IDENTITY)),
            ],
        )?;
        // The six face cameras, then the main camera.
        let (face_projection, face_views) = face_cameras();
        let cameras: Vec<(Matrix4, Matrix4)> = face_views
            .iter()
            .map(|v| (face_projection, *v))
            .chain([(projection, view)])
            .collect();
        for (i, (projection, view)) in cameras.iter().enumerate() {
            let camera = vec![
                ("cameraProjectionMatrix", m4(*projection)),
                ("cameraViewMatrix", m4(*view)),
            ];
            let mut values = camera.clone();
            values.extend([
                ("nodeUniform5", vec![0.]),
                ("nodeUniform6", vec![1.]),
                ("nodeUniform2", m4(Matrix4::IDENTITY)),
            ]);
            write(
                &self.background_renders[i],
                wgsl!("background_fs"),
                "renderStruct",
                &values,
            )?;
            let mut values = camera;
            values.push(("cameraWorldMatrix", m4(view.inverse())));
            write(
                &self.standard_renders[i],
                wgsl!("standard_fs"),
                "renderStruct",
                &values,
            )?;
            if i == 6 {
                write(
                    &self.mirror_render,
                    wgsl!("mirror_fs"),
                    "renderStruct",
                    &values,
                )?;
            }
        }
        write(
            &self.output_render,
            wgsl!("output_fs"),
            "renderStruct",
            &[
                (
                    "cameraProjectionMatrix",
                    vec![
                        1., 0., 0., 0., 0., 1., 0., 0., 0., 0., -1., 0., 0., 0., 0., 1.,
                    ],
                ),
                ("cameraViewMatrix", m4(Matrix4::IDENTITY)),
                ("nodeUniform1", vec![t.width as f64, t.height as f64]),
                ("nodeUniform2", vec![exposure]),
            ],
        )?;
        write(
            &self.output_object,
            wgsl!("output_vs"),
            "objectStruct",
            &[("nodeUniform5", m4(Matrix4::IDENTITY))],
        )?;
        // Opaque objects front to back by their projected depth, culled per camera.
        let order = |projection: Matrix4, view: Matrix4, objects: &[(usize, Matrix4, &Mesh)]| {
            let frustum = Frustum::from_projection(projection * view);
            let mut visible: Vec<(f64, usize)> = objects
                .iter()
                .filter(|(_, model, m)| {
                    frustum.intersects_sphere(Sphere {
                        center: model.transform_point3(m.sphere.center),
                        radius: m.sphere.radius
                            * model.to_scale_rotation_translation().0.max_element(),
                    })
                })
                .map(|(i, model, _)| {
                    (
                        (projection * view)
                            .project_point3(model.w_axis.truncate())
                            .z,
                        *i,
                    )
                })
                .collect();
            visible.sort_by(|a, b| a.0.total_cmp(&b.0));
            visible.into_iter().map(|(_, i)| i).collect::<Vec<_>>()
        };
        let set = |pass: &mut wgpu::RenderPass, (pipeline, groups): &Draw| {
            pass.set_pipeline(pipeline);
            for (i, g) in groups.iter().enumerate() {
                pass.set_bind_group(i as u32, g, &[]);
            }
        };
        let mut encoder = r.device.create_command_encoder(&Default::default());
        // cubeCamera.update() with the sphere's material hidden.
        let face_objects = [
            (0, models[0], &self.objects[0]),
            (1, models[1], &self.objects[1]),
        ];
        for (face, (background, objects)) in self.face_draws.iter().enumerate() {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("cubemap face"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &self.cube_faces[face],
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.cube_depth,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.),
                        store: wgpu::StoreOp::Discard,
                    }),
                    stencil_ops: None,
                }),
                ..Default::default()
            });
            set(&mut pass, background);
            self.background.draw(&mut pass);
            for i in order(face_projection, face_views[face], &face_objects) {
                set(&mut pass, &objects[i]);
                self.objects[i].draw(&mut pass);
            }
        }
        // PMREMNode: the sphere's environment from the new cube.
        let (source, groups) = &self.dynamic_source;
        self.dynamic
            .encode(&mut encoder, source, [&groups[0], &groups[1]]);
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("cubemap scene"),
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
            set(&mut pass, &t.background);
            self.background.draw(&mut pass);
            let objects = [
                (0, models[0], &self.objects[0]),
                (1, models[1], &self.objects[1]),
                (2, Matrix4::IDENTITY, &self.sphere),
            ];
            for i in order(projection, view, &objects) {
                if i == 2 {
                    set(&mut pass, &t.mirror);
                    self.sphere.draw(&mut pass);
                } else {
                    set(&mut pass, &t.objects[i]);
                    self.objects[i].draw(&mut pass);
                }
            }
        }
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("cubemap output"),
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
            pass.set_vertex_buffer(0, self.output_quad.slice(..));
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
        Ok(())
    }
    /// roughness, metalness, exposure, environmentIntensity and envMapIntensity.
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        *self
            .params
            .get_mut(index)
            .ok_or(Error::Invalid("cubemap parameter"))? = value as f64;
        Ok(())
    }
    pub fn seek(&mut self, t: f64) {
        self.pending = Some(t);
    }
}
