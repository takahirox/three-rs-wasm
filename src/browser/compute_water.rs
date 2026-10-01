//! webgpu_compute_water: a 128 × 128 height field in ping-pong storage
//! buffers, seeded from 15 octaves of SimplexNoise, stepped by the page's
//! compute kernel every 7 − speed frames with the pointer's raycast ripple,
//! and 100 instanced ducks that ride and drift on it (a second kernel). The
//! water mesh reads its heights and normals from the current buffer in the
//! vertex stage, a transparent double-sided standard material over the
//! blouberg_sunrise sky (a blurred PMREM background) with the pool border,
//! under ACES tone mapping with 4× MSAA. Every stage runs the WGSL three.js
//! r186 generates for the page (in `compute_water/`, without the unused
//! subgroup built-in; the PMREM, background and output modules are the
//! retro, compute_cloth and cubemap_dynamic ones, which are byte-identical).
use super::controls_attributes::{Controls, camera_state, viewport_css};
use super::deferred::{Draw, sampled_pipeline, set};
use super::gltf_viewer::load_asset;
use super::lights_projector::{m3, m4, pack};
use super::pmrem_cube_uv::{Pmrem, bind, raw_pipeline, rgbe_pmrem};
use super::retro::{mipmapped, target, uniform};
use super::shadowmap_opacity::Mipmaps;
use super::ssao::{Random, Simplex};
use crate::{
    Error, Result, camera::*, geometry::*, math::*, render_target::*, renderer::*, scene::*,
};
use std::f64::consts::PI;
use wgpu::util::DeviceExt;

const HALF: wgpu::TextureFormat = wgpu::TextureFormat::Rgba16Float;
const DEPTH: wgpu::TextureFormat = wgpu::TextureFormat::Depth24Plus;
const WIDTH: usize = 128;
const BOUNDS: f64 = 6.;
const DUCKS: u32 = 100;
macro_rules! wgsl {
    ($name:literal) => {
        include_str!(concat!("compute_water/", $name, ".wgsl"))
    };
}
const BACKGROUND_VS: &str = include_str!("compute_cloth/background_vs.wgsl");
const BACKGROUND_FS: &str = include_str!("compute_cloth/background_fs.wgsl");
const OUTPUT_VS: &str = include_str!("cubemap_dynamic/output_vs.wgsl");
const OUTPUT_FS: &str = include_str!("cubemap_dynamic/output_fs.wgsl");
/// An indexed mesh's vertex buffers, triangle and wireframe indices and counts.
struct Mesh {
    buffers: Vec<wgpu::Buffer>,
    index: (wgpu::Buffer, u32),
    wireframe: (wgpu::Buffer, u32),
}
impl Mesh {
    fn new(r: &Renderer, label: &str, attributes: &[&[f32]], index: &[u32]) -> Self {
        let init = |data: &[u8], usage| {
            r.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some(label),
                    contents: data,
                    usage,
                })
        };
        // Geometries.getWireframeAttribute: each triangle's three edges.
        let edges: Vec<u32> = index
            .chunks(3)
            .flat_map(|t| [t[0], t[1], t[1], t[2], t[2], t[0]])
            .collect();
        Self {
            buffers: attributes
                .iter()
                .map(|a| init(bytemuck::cast_slice(a), wgpu::BufferUsages::VERTEX))
                .collect(),
            index: (
                init(bytemuck::cast_slice(index), wgpu::BufferUsages::INDEX),
                index.len() as u32,
            ),
            wireframe: (
                init(bytemuck::cast_slice(&edges), wgpu::BufferUsages::INDEX),
                edges.len() as u32,
            ),
        }
    }
    fn draw(&self, pass: &mut wgpu::RenderPass, wireframe: bool, instances: u32) {
        for (slot, buffer) in self.buffers.iter().enumerate() {
            pass.set_vertex_buffer(slot as u32, buffer.slice(..));
        }
        let (index, count) = if wireframe {
            &self.wireframe
        } else {
            &self.index
        };
        pass.set_index_buffer(index.slice(..), wgpu::IndexFormat::Uint32);
        pass.draw_indexed(0..*count, 0, 0..instances);
    }
}
fn attributes(g: &BufferGeometry, name: &str, n: usize) -> Result<Vec<f32>> {
    let a = g
        .attributes
        .get(name)
        .ok_or(Error::Invalid("water attribute"))?;
    (0..a.count())
        .flat_map(|i| (0..n).map(move |k| (i, k)))
        .map(|(i, k)| a.get_component(i, k).map(|v| v as f32))
        .collect()
}
/// Per material: triangle and wireframe pipelines with their groups.
struct Lit {
    triangles: Draw,
    lines: Draw,
}
struct Targets {
    width: u32,
    height: u32,
    format: wgpu::TextureFormat,
    samples: u32,
    msaa: Option<(wgpu::TextureView, wgpu::TextureView)>,
    color: wgpu::TextureView,
    depth: wgpu::TextureView,
    background: Draw,
    duck: Lit,
    border: Lit,
    /// The water's back then front faces.
    water: [Lit; 2],
    output: Draw,
    screen: RenderTarget,
}
pub(super) struct Demo {
    controls: Controls,
    /// mouseSize, mouseDeep, viscosity, speed, ducksEnabled, wireframe.
    params: [f64; 6],
    /// Requested frames not yet stepped.
    pending: usize,
    frame: u32,
    ping_pong: u32,
    read_from_a: f64,
    /// The page's pointer state: down, first click, origin update, the
    /// pointer in NDC, OrbitControls.enabled, mousePos and mouseSpeed.
    mouse_down: bool,
    first_click: bool,
    update_origin: bool,
    pointer: Vector2,
    orbit_enabled: bool,
    mouse_pos: [f64; 2],
    mouse_speed: [f64; 2],
    queue: Vec<(u32, f64, f64)>,
    heights: [wgpu::Buffer; 3],
    ducks: wgpu::Buffer,
    instances: wgpu::Buffer,
    /// Height A→B and B→A, then the ducks, with their groups.
    kernels: [(wgpu::ComputePipeline, wgpu::BindGroup); 3],
    height_object: wgpu::Buffer,
    duck_object: wgpu::Buffer,
    background: Mesh,
    duck: Mesh,
    border: Mesh,
    water: Mesh,
    duck_map: wgpu::TextureView,
    pmrem: Pmrem,
    renders: [wgpu::Buffer; 4],
    objects: [wgpu::Buffer; 4],
    output_render: wgpu::Buffer,
    output_object: wgpu::Buffer,
    output_quad: wgpu::Buffer,
    clamp: wgpu::Sampler,
    repeat: wgpu::Sampler,
    targets: Option<Targets>,
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, _id: u32, r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 75.,
            near: 1.,
            far: 3000.,
            aspect,
            ..Default::default()
        }));
        s.get_mut(c)?.position = Vector3::new(0., 2., 4.);
        let mut controls = Controls::new(None, (0., f64::INFINITY), PI, true);
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
        // The fixture's seeded Math.random: SimplexNoise's permutation, then
        // the ducks' starting positions.
        let mut random = Random(186);
        let simplex = Simplex::from_random(|| random.next());
        let noise = |x: f64, y: f64| {
            let (mut mult_r, mut mult, mut r) = (0.1, 0.025, 0.);
            for i in 0..15 {
                r += mult_r * simplex.noise(x * mult, y * mult);
                mult_r *= 0.53 + 0.025 * i as f64;
                mult *= 1.25;
            }
            r
        };
        let mut heights = vec![0f32; WIDTH * WIDTH];
        for j in 0..WIDTH {
            for i in 0..WIDTH {
                heights[j * WIDTH + i] = noise(i as f64, j as f64) as f32;
            }
        }
        let storage = wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST;
        let heights =
            [0; 3].map(|_| init("water heights", bytemuck::cast_slice(&heights), storage));
        let mut duck_data = vec![0f32; DUCKS as usize * 8];
        for d in duck_data.chunks_mut(8) {
            d[0] = ((random.next() - 0.5) * BOUNDS * 0.7) as f32;
            d[2] = ((random.next() - 0.5) * BOUNDS * 0.7) as f32;
        }
        let ducks = init("ducks", bytemuck::cast_slice(&duck_data), storage);
        let identity: Vec<f32> = (0..DUCKS)
            .flat_map(|_| Matrix4::IDENTITY.to_cols_array().map(|v| v as f32))
            .collect();
        let instances = init(
            "duck instances",
            bytemuck::cast_slice(&identity),
            wgpu::BufferUsages::UNIFORM,
        );
        let height_object = uniform(r, "water height object", wgsl!("height_ab"), "objectStruct")?;
        let duck_object = uniform(r, "water duck object", wgsl!("ducks"), "objectStruct")?;
        let compute = |label, source: &str| {
            r.device
                .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                    label: Some(label),
                    layout: None,
                    module: &r.device.create_shader_module(wgpu::ShaderModuleDescriptor {
                        label: Some(label),
                        source: wgpu::ShaderSource::Wgsl(source.into()),
                    }),
                    entry_point: Some("main"),
                    compilation_options: Default::default(),
                    cache: None,
                })
        };
        let [a, b, prev] = &heights;
        let kernel = |label, source, entries: &[(u32, &wgpu::Buffer)]| {
            let p = compute(label, source);
            let entries: Vec<(u32, wgpu::BindingResource)> = entries
                .iter()
                .map(|(i, b)| (*i, b.as_entire_binding()))
                .collect();
            let group = bind(r, p.get_bind_group_layout(0), &entries);
            (p, group)
        };
        let kernels = [
            kernel(
                "Update Height A→B",
                wgsl!("height_ab"),
                &[(0, a), (1, prev), (2, &height_object), (3, b)],
            ),
            kernel(
                "Update Height B→A",
                wgsl!("height_ba"),
                &[(0, b), (1, prev), (2, &height_object), (3, a)],
            ),
            kernel(
                "Update Ducks",
                wgsl!("ducks"),
                &[(0, &ducks), (1, &duck_object), (2, a), (3, b)],
            ),
        ];
        // The duck: Draco glb geometry and its sRGB base color map.
        let (asset, buffers, images) = load_asset("/web/gallery/assets/gltf/duck.glb").await?;
        let primitive = asset
            .meshes()
            .next()
            .and_then(|m| m.primitives().next())
            .ok_or(Error::Invalid("duck mesh"))?;
        let reader = primitive.reader(|b| buffers.get(b.index()).map(Vec::as_slice));
        let positions: Vec<f32> = reader
            .read_positions()
            .ok_or(Error::Invalid("duck positions"))?
            .flatten()
            .collect();
        let normals: Vec<f32> = reader
            .read_normals()
            .ok_or(Error::Invalid("duck normals"))?
            .flatten()
            .collect();
        let uvs: Vec<f32> = reader
            .read_tex_coords(0)
            .ok_or(Error::Invalid("duck uv"))?
            .into_f32()
            .flatten()
            .collect();
        let index: Vec<u32> = reader
            .read_indices()
            .ok_or(Error::Invalid("duck index"))?
            .into_u32()
            .collect();
        let duck = Mesh::new(r, "duck", &[&positions, &normals, &uvs], &index);
        let mut mipmaps = Mipmaps::new(r);
        let duck_map = mipmapped(
            r,
            &mut mipmaps,
            images.first().ok_or(Error::Invalid("duck map"))?,
            wgpu::TextureFormat::Rgba8UnormSrgb,
        );
        // Water, border ( turned in the geometry ) and the background sphere.
        let plane = PlaneGeometry::build(BOUNDS, BOUNDS, WIDTH as u32 - 1, WIDTH as u32 - 1)?;
        let water = Mesh::new(
            r,
            "water",
            &[
                &attributes(&plane, "position", 3)?,
                &attributes(&plane, "normal", 3)?,
            ],
            &plane.index.clone().ok_or(Error::Invalid("water index"))?,
        );
        let torus = TorusGeometry::build(4.2, 0.1, 12, 4, 2. * PI, 0., 2. * PI)?;
        let turn = Matrix4::from_rotation_y(PI * 0.25) * Matrix4::from_rotation_x(PI * 0.5);
        let rotate = |v: &[f32], point: bool| -> Vec<f32> {
            v.chunks(3)
                .flat_map(|p| {
                    let p = Vector3::new(p[0] as f64, p[1] as f64, p[2] as f64);
                    let q = if point {
                        turn.transform_point3(p)
                    } else {
                        turn.transform_vector3(p).normalize()
                    };
                    [q.x as f32, q.y as f32, q.z as f32]
                })
                .collect()
        };
        let border = Mesh::new(
            r,
            "pool border",
            &[
                &rotate(&attributes(&torus, "normal", 3)?, false),
                &rotate(&attributes(&torus, "position", 3)?, true),
            ],
            &torus.index.clone().ok_or(Error::Invalid("border index"))?,
        );
        let ball = SphereGeometry::build(1., 32, 32)?;
        let background = Mesh::new(
            r,
            "water background",
            &[
                &attributes(&ball, "normal", 3)?,
                &attributes(&ball, "position", 3)?,
            ],
            &ball
                .index
                .clone()
                .ok_or(Error::Invalid("background index"))?,
        );
        let clamp = r.device.create_sampler(&wgpu::SamplerDescriptor {
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
        let (pmrem, _) =
            rgbe_pmrem(r, &clamp, "/web/environments/blouberg_sunrise_2_1k.hdr").await?;
        let renders = [
            uniform(r, "water background render", BACKGROUND_FS, "renderStruct")?,
            uniform(r, "duck render", wgsl!("duck_fs"), "renderStruct")?,
            uniform(r, "border render", wgsl!("border_fs"), "renderStruct")?,
            uniform(r, "water render", wgsl!("water_fs"), "renderStruct")?,
        ];
        let objects = [
            uniform(r, "water background object", BACKGROUND_FS, "objectStruct")?,
            uniform(r, "duck object", wgsl!("duck_fs"), "objectStruct")?,
            uniform(r, "border object", wgsl!("border_fs"), "objectStruct")?,
            uniform(r, "water object", wgsl!("water_fs"), "objectStruct")?,
        ];
        Ok(Self {
            controls,
            params: [0.12, 0.5, 0.96, 5., 1., 0.],
            // The page renders its first frame on loading, before any capture.
            pending: 1,
            frame: 0,
            ping_pong: 0,
            read_from_a: 1.,
            mouse_down: false,
            first_click: true,
            update_origin: false,
            pointer: Vector2::ZERO,
            orbit_enabled: true,
            mouse_pos: [0.; 2],
            mouse_speed: [0.; 2],
            queue: vec![],
            heights,
            ducks,
            instances,
            kernels,
            height_object,
            duck_object,
            background,
            duck,
            border,
            water,
            duck_map,
            pmrem,
            renders,
            objects,
            output_render: uniform(r, "water output render", OUTPUT_FS, "renderStruct")?,
            output_object: uniform(r, "water output object", OUTPUT_VS, "objectStruct")?,
            output_quad: init(
                "water quad",
                bytemuck::cast_slice(&[-1f32, 3., 0., -1., -1., 0., 3., -1., 0.]),
                wgpu::BufferUsages::VERTEX,
            ),
            clamp,
            repeat,
            targets: None,
        })
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, _dt: f64, animate: bool) -> Result<()> {
        if animate {
            self.pending += 1;
        }
        Ok(())
    }
    pub fn prepare(&mut self, _s: &mut Scene, _c: Object3D, _r: &Renderer) -> Result<()> {
        Ok(())
    }
    /// raycast(): the pointer against the 6 × 6 ray plane at y = 0, moving
    /// the ripple's position and speed, and holding the orbit while the
    /// pointer drags on the water.
    fn raycast(&mut self, s: &mut Scene, c: Object3D) -> Result<()> {
        if self.mouse_down && (self.first_click || !self.orbit_enabled) {
            s.update()?;
            let (camera, world) = s.camera(c)?;
            // Raycaster.setFromCamera: from the camera through the pointer at z = 0.5.
            let unproject = world * camera.projection_matrix()?.inverse();
            let origin = world.transform_point3(Vector3::ZERO);
            let direction =
                (unproject.project_point3(Vector3::new(self.pointer.x, self.pointer.y, 0.5))
                    - origin)
                    .normalize();
            // The plane faces up ( FrontSide ): only rays coming down hit it.
            let hit = (direction.y < 0.)
                .then(|| origin + direction * (-origin.y / direction.y))
                .filter(|p| p.x.abs() <= BOUNDS / 2. && p.z.abs() <= BOUNDS / 2.);
            if let Some(point) = hit {
                if self.update_origin {
                    self.mouse_pos = [point.x, point.z];
                    self.update_origin = false;
                }
                self.mouse_speed = [point.x - self.mouse_pos[0], point.z - self.mouse_pos[1]];
                self.mouse_pos = [point.x, point.z];
                if self.first_click {
                    self.orbit_enabled = false;
                }
            } else {
                self.update_origin = true;
                self.mouse_speed = [0.; 2];
            }
            self.first_click = false;
        } else {
            self.update_origin = true;
            self.mouse_speed = [0.; 2];
        }
        Ok(())
    }
    /// One render(): the raycast, and every 7 − speed frames a height step
    /// ( alternating buffers ) and, with the ducks enabled, the duck step.
    fn step(&mut self, r: &Renderer, s: &mut Scene, c: Object3D) -> Result<()> {
        self.raycast(s, c)?;
        self.frame += 1;
        let [mouse_size, mouse_deep, viscosity, speed, ducks_enabled, _] = self.params;
        if self.frame as f64 >= 7. - speed {
            let write =
                |buffer: &wgpu::Buffer, source: &str, values: &[(&str, &[f64])]| -> Result<()> {
                    r.queue
                        .write_buffer(buffer, 0, &pack(source, "objectStruct", values)?);
                    Ok(())
                };
            write(
                &self.height_object,
                wgsl!("height_ab"),
                &[
                    ("viscosity", &[viscosity]),
                    ("mousePos", &self.mouse_pos),
                    ("mouseSize", &[mouse_size]),
                    ("mouseDeep", &[mouse_deep]),
                    ("mouseSpeed", &self.mouse_speed),
                    ("nodeUniform8", &[(WIDTH * WIDTH) as f64]),
                ],
            )?;
            let kernel = self.ping_pong as usize;
            self.read_from_a = if self.ping_pong == 0 { 0. } else { 1. };
            self.ping_pong = 1 - self.ping_pong;
            write(
                &self.duck_object,
                wgsl!("ducks"),
                &[
                    ("nodeUniform1", &[self.read_from_a]),
                    ("nodeUniform4", &[DUCKS as f64]),
                ],
            )?;
            let mut encoder = r.device.create_command_encoder(&Default::default());
            {
                let mut pass = encoder.begin_compute_pass(&Default::default());
                let (p, g) = &self.kernels[kernel];
                pass.set_pipeline(p);
                pass.set_bind_group(0, g, &[]);
                pass.dispatch_workgroups(8, 8, 1);
            }
            if ducks_enabled > 0.5 {
                let mut pass = encoder.begin_compute_pass(&Default::default());
                let (p, g) = &self.kernels[2];
                pass.set_pipeline(p);
                pass.set_bind_group(0, g, &[]);
                pass.dispatch_workgroups(DUCKS.div_ceil(64), 1, 1);
            }
            r.queue.submit([encoder.finish()]);
            self.frame = 0;
        }
        Ok(())
    }
    fn resize(&mut self, r: &Renderer, out: &RenderTarget) -> Result<()> {
        let size = (out.width, out.height);
        let samples = if out.options.samples > 1 { 4 } else { 1 };
        let tex = wgpu::BindingResource::TextureView;
        let sampler = wgpu::BindingResource::Sampler;
        let float3 = |location| {
            [wgpu::VertexAttribute {
                format: wgpu::VertexFormat::Float32x3,
                offset: 0,
                shader_location: location,
            }]
        };
        let attrs = [
            float3(0),
            float3(1),
            [wgpu::VertexAttribute {
                format: wgpu::VertexFormat::Float32x2,
                offset: 0,
                shader_location: 2,
            }],
        ];
        let layout = |i: usize, stride| wgpu::VertexBufferLayout {
            array_stride: stride,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &attrs[i],
        };
        let two = [layout(0, 12), layout(1, 12)];
        let three = [layout(0, 12), layout(1, 12), layout(2, 8)];
        let depth = Some((wgpu::CompareFunction::LessEqual, true));
        let pipeline =
            |label, shaders, buffers: &[wgpu::VertexBufferLayout], cw, blended, topology| {
                sampled_pipeline(
                    r,
                    label,
                    shaders,
                    buffers,
                    &[HALF],
                    depth,
                    (cw, blended),
                    (samples, topology),
                )
            };
        let groups = |p: &wgpu::RenderPipeline,
                      render: &wgpu::Buffer,
                      object: &[(u32, wgpu::BindingResource)]| {
            vec![
                bind(
                    r,
                    p.get_bind_group_layout(0),
                    &[(0, render.as_entire_binding())],
                ),
                bind(r, p.get_bind_group_layout(1), object),
            ]
        };
        let lit = |label,
                   shaders,
                   buffers: &[wgpu::VertexBufferLayout],
                   cw,
                   blended,
                   render,
                   object: &[(u32, wgpu::BindingResource)]| {
            let draw = |topology| {
                let p = pipeline(label, shaders, buffers, cw, blended, topology);
                let g = groups(&p, render, object);
                (p, g)
            };
            Lit {
                triangles: draw(wgpu::PrimitiveTopology::TriangleList),
                lines: draw(wgpu::PrimitiveTopology::LineList),
            }
        };
        let background = {
            let p = sampled_pipeline(
                r,
                "water background",
                (BACKGROUND_VS, BACKGROUND_FS),
                &two,
                &[HALF],
                Some((wgpu::CompareFunction::Always, false)),
                (true, false),
                (samples, wgpu::PrimitiveTopology::TriangleList),
            );
            let g = groups(
                &p,
                &self.renders[0],
                &[
                    (0, self.objects[0].as_entire_binding()),
                    (1, sampler(&self.clamp)),
                    (2, tex(&self.pmrem.view)),
                ],
            );
            (p, g)
        };
        let duck = lit(
            "duck",
            (wgsl!("duck_vs"), wgsl!("duck_fs")),
            &three,
            false,
            false,
            &self.renders[1],
            &[
                (0, self.objects[1].as_entire_binding()),
                (1, sampler(&self.repeat)),
                (2, tex(&self.duck_map)),
                (3, sampler(&self.clamp)),
                (4, tex(&r.dfg)),
                (5, sampler(&self.clamp)),
                (6, tex(&self.pmrem.view)),
                (7, self.instances.as_entire_binding()),
                (8, self.ducks.as_entire_binding()),
            ],
        );
        let border = lit(
            "pool border",
            (wgsl!("border_vs"), wgsl!("border_fs")),
            &two,
            false,
            false,
            &self.renders[2],
            &[
                (0, self.objects[2].as_entire_binding()),
                (1, sampler(&self.clamp)),
                (2, tex(&r.dfg)),
                (3, sampler(&self.clamp)),
                (4, tex(&self.pmrem.view)),
            ],
        );
        let water_object = [
            (0, self.objects[3].as_entire_binding()),
            (1, sampler(&self.clamp)),
            (2, tex(&r.dfg)),
            (3, sampler(&self.clamp)),
            (4, tex(&self.pmrem.view)),
            (5, self.heights[0].as_entire_binding()),
            (6, self.heights[1].as_entire_binding()),
        ];
        // DoubleSide transparency: the back faces ( clockwise ), then the front.
        let water = [true, false].map(|cw| {
            lit(
                "water",
                (wgsl!("water_vs"), wgsl!("water_fs")),
                &two,
                cw,
                true,
                &self.renders[3],
                &water_object,
            )
        });
        let color = target(r, size, HALF, 1);
        let quad = [float3(0)];
        let output_pipeline = raw_pipeline(
            r,
            "water output",
            OUTPUT_VS,
            OUTPUT_FS,
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
        let output = (
            output_pipeline.clone(),
            groups(
                &output_pipeline,
                &self.output_render,
                &[
                    (0, sampler(&self.clamp)),
                    (1, tex(&color)),
                    (2, self.output_object.as_entire_binding()),
                ],
            ),
        );
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
            color,
            background,
            duck,
            border,
            water,
            output,
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
        self.drain();
        for _ in 0..std::mem::take(&mut self.pending) {
            self.step(r, s, c)?;
        }
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
            .ok_or(Error::Invalid("water targets"))?;
        s.update()?;
        let (camera, world) = s.camera(c)?;
        let (projection, view) = (camera.projection_matrix()?, world.inverse());
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
        let camera_values = |light: [&'static str; 3]| {
            vec![
                ("cameraProjectionMatrix", m4(projection)),
                ("cameraViewMatrix", m4(view)),
                ("cameraWorldMatrix", m4(world)),
                // The sun: white at intensity 4, from ( -1, 2.6, 1.4 ) toward the origin.
                (light[0], vec![4.; 3]),
                (light[1], vec![-1., 2.6, 1.4]),
                (light[2], vec![0.; 3]),
            ]
        };
        let mut background = camera_values(["", "", ""]);
        background.extend([
            ("nodeUniform0", vec![0.3]),
            ("nodeUniform9", vec![1.]),
            ("nodeUniform3", m4(Matrix4::IDENTITY)),
        ]);
        write(&self.renders[0], BACKGROUND_FS, "renderStruct", &background)?;
        write(
            &self.renders[1],
            wgsl!("duck_fs"),
            "renderStruct",
            &camera_values(["nodeUniform17", "nodeUniform15", "nodeUniform16"]),
        )?;
        write(
            &self.renders[2],
            wgsl!("border_fs"),
            "renderStruct",
            &camera_values(["nodeUniform13", "nodeUniform11", "nodeUniform12"]),
        )?;
        write(
            &self.renders[3],
            wgsl!("water_fs"),
            "renderStruct",
            &camera_values(["nodeUniform16", "nodeUniform14", "nodeUniform15"]),
        )?;
        let identity3 = m3(Matrix4::IDENTITY);
        let texel =
            |a: &'static str, b: &'static str| [(a, vec![1. / 768.]), (b, vec![1. / 1024.])];
        let mut values = vec![
            ("nodeUniform1", vec![8.]),
            ("nodeUniform2", m4(Matrix4::IDENTITY)),
            ("nodeUniform5", identity3.clone()),
            ("nodeUniform10", vec![1.]),
            ("nodeUniform12", m4(Matrix4::IDENTITY)),
        ];
        values.extend(texel("nodeUniform6", "nodeUniform7"));
        write(&self.objects[0], BACKGROUND_FS, "objectStruct", &values)?;
        let mut values = vec![
            ("nodeUniform2", vec![1.; 3]),
            ("nodeUniform4", identity3.clone()),
            ("nodeUniform5", vec![1.]),
            ("nodeUniform6", vec![0.]),
            ("nodeUniform7", vec![0.02]),
            ("nodeUniform9", identity3.clone()),
            ("nodeUniform10", vec![0.; 3]),
            ("nodeUniform11", vec![1.]),
            ("nodeUniform13", m4(Matrix4::IDENTITY)),
            ("nodeUniform18", vec![8.]),
            ("nodeUniform19", m4(Matrix4::IDENTITY)),
            ("nodeUniform24", vec![1.25]),
        ];
        values.extend(texel("nodeUniform21", "nodeUniform22"));
        write(&self.objects[1], wgsl!("duck_fs"), "objectStruct", &values)?;
        let mut values = vec![
            (
                "nodeUniform0",
                Color::from_hex(0x908877).0.to_array().to_vec(),
            ),
            ("nodeUniform1", vec![1.]),
            ("nodeUniform2", vec![0.]),
            ("nodeUniform3", vec![0.2]),
            ("nodeUniform5", identity3.clone()),
            ("nodeUniform6", vec![0.; 3]),
            ("nodeUniform7", vec![1.]),
            ("nodeUniform9", m4(Matrix4::IDENTITY)),
            ("nodeUniform14", vec![8.]),
            ("nodeUniform15", m4(Matrix4::IDENTITY)),
            ("nodeUniform20", vec![1.25]),
        ];
        values.extend(texel("nodeUniform17", "nodeUniform18"));
        write(
            &self.objects[2],
            wgsl!("border_fs"),
            "objectStruct",
            &values,
        )?;
        let water_model = Matrix4::from_rotation_x(-PI * 0.5);
        let mut values = vec![
            ("nodeUniform0", vec![self.read_from_a]),
            (
                "nodeUniform3",
                Color::from_hex(0x9bd2ec).0.to_array().to_vec(),
            ),
            ("nodeUniform4", vec![0.8]),
            ("nodeUniform5", vec![0.9]),
            ("nodeUniform6", vec![0.]),
            ("nodeUniform8", m3(water_model.inverse().transpose())),
            ("nodeUniform9", vec![0.; 3]),
            ("nodeUniform10", vec![1.]),
            ("nodeUniform12", m4(water_model)),
            ("nodeUniform17", vec![8.]),
            ("nodeUniform18", m4(Matrix4::IDENTITY)),
            ("nodeUniform23", vec![1.25]),
        ];
        values.extend(texel("nodeUniform20", "nodeUniform21"));
        write(&self.objects[3], wgsl!("water_fs"), "objectStruct", &values)?;
        write(
            &self.output_render,
            OUTPUT_FS,
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
                ("nodeUniform2", vec![0.5]),
            ],
        )?;
        write(
            &self.output_object,
            OUTPUT_VS,
            "objectStruct",
            &[("nodeUniform5", m4(Matrix4::IDENTITY))],
        )?;
        let wireframe = self.params[5] > 0.5;
        let lit = |l: &Lit| {
            if wireframe {
                l.lines.clone()
            } else {
                l.triangles.clone()
            }
        };
        let mut encoder = r.device.create_command_encoder(&Default::default());
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("water scene"),
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
            self.background.draw(&mut pass, false, 1);
            if self.params[4] > 0.5 {
                set(&mut pass, &lit(&t.duck));
                self.duck.draw(&mut pass, wireframe, DUCKS);
            }
            set(&mut pass, &lit(&t.border));
            self.border.draw(&mut pass, wireframe, 1);
            for side in &t.water {
                set(&mut pass, &lit(side));
                self.water.draw(&mut pass, wireframe, 1);
            }
        }
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("water output"),
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
    /// Queued pointer events, in CSS pixels.
    pub fn draw(&mut self, kind: u32, x: f64, y: f64) {
        self.queue.push((kind, x, y));
    }
    /// The container's pointer listeners: down, move ( NDC ) and up.
    fn drain(&mut self) {
        for (kind, x, y) in std::mem::take(&mut self.queue) {
            let (w, h, _) = viewport_css();
            match kind {
                10..=12 => {
                    self.mouse_down = true;
                    self.first_click = true;
                    self.update_origin = true;
                }
                0 => self.pointer = Vector2::new(x / w * 2. - 1., -(y / h) * 2. + 1.),
                20..=22 => {
                    self.mouse_down = false;
                    self.first_click = false;
                    self.update_origin = false;
                    self.orbit_enabled = true;
                }
                _ => {}
            }
        }
    }
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
        self.drain();
        if !self.orbit_enabled {
            return Ok(());
        }
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
    /// Mouse Size, Mouse Deep, viscosity, speed, ducksEnabled and wireframe.
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        *self
            .params
            .get_mut(index)
            .ok_or(Error::Invalid("compute water parameter"))? = value as f64;
        Ok(())
    }
    pub fn seek(&mut self, _t: f64) {
        self.pending += 1;
    }
}
