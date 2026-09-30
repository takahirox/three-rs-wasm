//! webgpu_lights_dynamic: a hundred MeshStandardMaterial shapes (fifty
//! seeded PBR materials) in rings around a metallic sphere on a floor, lit by
//! an ambient light and orbiting point lights with MeshBasicMaterial markers.
//! In dynamic mode the DynamicLighting addon batches the point lights into
//! uniform arrays of at most 16 (the rest are ignored, as in the addon) so
//! adding lights never recompiles; the materials loop over the arrays. The
//! scene pass (multisampled, front-to-back with frustum culling) is followed
//! by the sRGB output pass. Every stage runs the WGSL three.js r186 generates
//! for the page (in `lights_dynamic/`).
use super::controls_attributes::{Controls, camera_state};
use crate::{
    Error, Result, camera::*, geometry::*, math::*, render_target::*, renderer::*, scene::*,
};
use std::f64::consts::PI;
use wgpu::util::DeviceExt;

const HALF: wgpu::TextureFormat = wgpu::TextureFormat::Rgba16Float;
const DEPTH: wgpu::TextureFormat = wgpu::TextureFormat::Depth24Plus;
/// DynamicLighting's default maxPointLights.
const MAX_DYNAMIC: usize = 16;
macro_rules! wgsl {
    ($name:literal) => {
        include_str!(concat!("lights_dynamic/", $name, ".wgsl"))
    };
}
/// A std140-style writer for the generated uniform structs.
struct Layout(Vec<u8>);
impl Layout {
    fn new(size: usize) -> Self {
        Self(vec![0; size])
    }
    fn f(mut self, offset: usize, v: f64) -> Self {
        self.0[offset..offset + 4].copy_from_slice(&(v as f32).to_le_bytes());
        self
    }
    fn i(mut self, offset: usize, v: i32) -> Self {
        self.0[offset..offset + 4].copy_from_slice(&v.to_le_bytes());
        self
    }
    fn v3(self, offset: usize, v: [f64; 3]) -> Self {
        self.f(offset, v[0]).f(offset + 4, v[1]).f(offset + 8, v[2])
    }
    fn m4(mut self, offset: usize, m: Matrix4) -> Self {
        for (i, v) in m.to_cols_array().iter().enumerate() {
            self = self.f(offset + i * 4, *v);
        }
        self
    }
    /// mat3x3<f32>: three columns padded to 16 bytes.
    fn m3(mut self, offset: usize, m: Matrix4) -> Self {
        for col in 0..3 {
            let c = m.col(col);
            self = self.v3(offset + col * 16, [c.x, c.y, c.z]);
        }
        self
    }
    fn write(self, r: &Renderer, buffer: &wgpu::Buffer) {
        r.queue.write_buffer(buffer, 0, &self.0);
    }
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
/// One shared geometry: positions, normals, index, index count and the
/// bounding-sphere radius (all are centered on their origin).
struct Mesh {
    positions: wgpu::Buffer,
    normals: wgpu::Buffer,
    index: wgpu::Buffer,
    count: u32,
    radius: f64,
}
/// A MeshStandardMaterial mesh: its geometry, material and model matrix.
struct Shape {
    mesh: usize,
    color: [f64; 3],
    roughness: f64,
    metalness: f64,
    model: Matrix4,
    object: wgpu::Buffer,
}
/// addLight(): a point light orbiting the center with its marker sphere.
struct Light {
    color: [f64; 3],
    angle: f64,
    radius: f64,
    speed: f64,
    base_y: f64,
    position: Vector3,
}
/// Pipelines and bind groups for one light-array size, and the targets for
/// one drawing-buffer size.
struct Targets {
    width: u32,
    height: u32,
    format: wgpu::TextureFormat,
    samples: u32,
    max_lights: usize,
    color: wgpu::TextureView,
    resolve: Option<wgpu::TextureView>,
    depth: wgpu::TextureView,
    screen: RenderTarget,
    standard: (wgpu::RenderPipeline, wgpu::BindGroup),
    basic: (wgpu::RenderPipeline, wgpu::BindGroup),
    output: (wgpu::RenderPipeline, [wgpu::BindGroup; 2]),
    /// Object bind groups for the shapes and the light markers.
    shapes: Vec<wgpu::BindGroup>,
    markers: Vec<wgpu::BindGroup>,
}
pub(super) struct Demo {
    controls: Controls,
    time: f64,
    /// dynamic mode and auto-add lights.
    dynamic: bool,
    auto_add: Option<f64>,
    random: u32,
    /// Lights added from the GUI and a controls reset, applied next frame.
    pending_adds: usize,
    reset_controls: bool,
    meshes: Vec<Mesh>,
    shapes: Vec<Shape>,
    lights: Vec<Light>,
    /// Marker object uniforms by light slot, kept when lights are removed.
    markers: Vec<wgpu::Buffer>,
    marker: Mesh,
    render_uniforms: wgpu::Buffer,
    /// Positions and cutoff distances, colors and decays.
    light_buffers: [wgpu::Buffer; 3],
    light_capacity: usize,
    output_render: wgpu::Buffer,
    output_object: wgpu::Buffer,
    quad: wgpu::Buffer,
    sampler: wgpu::Sampler,
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
            near: 0.1,
            far: 200.,
            aspect,
            ..Default::default()
        }));
        s.get_mut(c)?.position = Vector3::new(0., 15., 30.);
        let init = |label, data: &[u8], usage| {
            r.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some(label),
                    contents: data,
                    usage,
                })
        };
        let mesh = |g: BufferGeometry| -> Result<Mesh> {
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
                    "dynamic positions",
                    bytemuck::cast_slice(&positions),
                    wgpu::BufferUsages::VERTEX,
                ),
                normals: init(
                    "dynamic normals",
                    bytemuck::cast_slice(&f("normal")?),
                    wgpu::BufferUsages::VERTEX,
                ),
                index: init(
                    "dynamic index",
                    bytemuck::cast_slice(&index),
                    wgpu::BufferUsages::INDEX,
                ),
                count: index.len() as u32,
                radius,
            })
        };
        let mut floor = PlaneGeometry::build(120., 120., 1, 1)?;
        floor.apply_matrix4(Matrix4::from_rotation_x(-PI / 2.))?;
        let meshes = vec![
            mesh(SphereGeometry::build(0.8, 128, 128)?)?,
            mesh(BoxGeometry::segmented(1.2, 1.2, 1.2, 64, 64, 64)?)?,
            mesh(TorusGeometry::build(
                0.6,
                0.25,
                128,
                128,
                PI * 2.,
                0.,
                PI * 2.,
            )?)?,
            mesh(CylinderGeometry::build(
                0.5,
                0.5,
                1.4,
                128,
                64,
                false,
                0.,
                PI * 2.,
            )?)?,
            mesh(CylinderGeometry::build(
                0.,
                0.6,
                1.4,
                128,
                64,
                false,
                0.,
                PI * 2.,
            )?)?,
            mesh(SphereGeometry::build(2., 128, 128)?)?,
            mesh(floor)?,
        ];
        let mut demo = Self {
            controls: Controls::new(Some(0.05), (0., f64::INFINITY), PI, true),
            time: 0.,
            dynamic: true,
            auto_add: None,
            random: 186,
            pending_adds: 0,
            reset_controls: false,
            meshes,
            shapes: vec![],
            lights: vec![],
            markers: vec![],
            marker: mesh(SphereGeometry::build(0.15, 8, 8)?)?,
            render_uniforms: uniform(r, "dynamic render", 144),
            light_buffers: [0; 3].map(|_| uniform(r, "dynamic lights", 16 * MAX_DYNAMIC as u64)),
            light_capacity: MAX_DYNAMIC,
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
            targets: None,
        };
        // 50 unique materials, then 100 meshes in rings reusing them.
        let mut materials = vec![];
        for i in 0..50 {
            let (s, l) = (0.6 + demo.random() * 0.4, 0.35 + demo.random() * 0.3);
            let color = Color::from_hsl(i as f64 / 50., s, l).0.to_array();
            materials.push((color, demo.random(), demo.random()));
        }
        let shape = |mesh, (color, roughness, metalness): ([f64; 3], f64, f64), model| Shape {
            mesh,
            color,
            roughness,
            metalness,
            model,
            object: uniform(r, "standard object", 160),
        };
        let mut shapes = vec![shape(
            6,
            (Color::from_hex(0x444444).0.to_array(), 0.8, 0.),
            Matrix4::IDENTITY,
        )];
        for i in 0..100 {
            let ring = (i / 10) as f64;
            let angle = (i % 10) as f64 / 10. * PI * 2. + ring * 0.3;
            let radius = 6. + ring * 4.;
            let y = 0.7 + demo.random() * 2.;
            let (rx, ry) = (demo.random() * PI, demo.random() * PI);
            shapes.push(shape(
                i % 5,
                materials[i % 50],
                Matrix4::from_translation(Vector3::new(
                    angle.cos() * radius,
                    y,
                    angle.sin() * radius,
                )) * Matrix4::from_euler(glam::EulerRot::XYZ, rx, ry, 0.),
            ));
        }
        shapes.push(shape(
            5,
            (Color::from_hex(0xffffff).0.to_array(), 0.1, 0.9),
            Matrix4::from_translation(Vector3::new(0., 2., 0.)),
        ));
        demo.shapes = shapes;
        demo.add_light(r);
        demo.add_light(r);
        demo.controls.set_target(Vector3::new(0., 2., 0.));
        demo.controls.update(s, c)?;
        Ok(demo)
    }
    /// The fixture's seeded Math.random.
    fn random(&mut self) -> f64 {
        self.random = self.random.wrapping_mul(1664525).wrapping_add(1013904223);
        self.random as f64 / 4294967296.
    }
    fn add_light(&mut self, r: &Renderer) {
        let color = Color::from_hsl(self.random(), 0.8, 0.5).0.to_array();
        let angle = self.random() * PI * 2.;
        let radius = 5. + self.random() * 20.;
        let base_y = 1. + self.random() * 6.;
        let speed = 0.2 + self.random() * 0.8;
        self.lights.push(Light {
            color,
            angle,
            radius,
            speed,
            base_y,
            position: Vector3::new(angle.cos() * radius, base_y, angle.sin() * radius),
        });
        if self.markers.len() < self.lights.len() {
            let object = uniform(r, "basic object", 80);
            if let Some(t) = self.targets.as_mut() {
                t.markers.push(bind(
                    r,
                    t.basic.0.get_bind_group_layout(1),
                    &[(0, object.as_entire_binding())],
                ));
            }
            self.markers.push(object);
        }
    }
    fn remove_light(&mut self) {
        self.lights.pop();
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, dt: f64, animate: bool) -> Result<()> {
        if animate {
            self.time += dt;
        }
        Ok(())
    }
    pub fn prepare(&mut self, s: &mut Scene, c: Object3D, r: &Renderer) -> Result<()> {
        for _ in 0..std::mem::take(&mut self.pending_adds) {
            self.add_light(r);
        }
        if std::mem::take(&mut self.reset_controls) {
            self.controls = Controls::new(Some(0.05), (0., f64::INFINITY), PI, true);
            self.controls.set_target(Vector3::new(0., 2., 0.));
            self.controls.update(s, c)?;
        }
        // setInterval( addLight, 500 ) while auto-add is on.
        while let Some(at) = self.auto_add.filter(|&at| at <= self.time) {
            self.add_light(r);
            self.auto_add = Some(at + 0.5);
        }
        self.controls.frame_update(s, c)?;
        let time = self.time;
        for light in &mut self.lights {
            let t = time * light.speed + light.angle;
            light.position = Vector3::new(
                t.cos() * light.radius,
                light.base_y + (t * 2.).sin() * 0.5,
                t.sin() * light.radius,
            );
        }
        Ok(())
    }
    /// The light arrays hold 16 lights in dynamic mode; the per-light shaders
    /// of the default lighting take every light, here as a larger array.
    fn max_lights(&self) -> usize {
        if self.dynamic {
            MAX_DYNAMIC
        } else {
            self.lights.len().max(MAX_DYNAMIC).next_power_of_two()
        }
    }
    fn resize(&mut self, r: &Renderer, out: &RenderTarget) -> Result<()> {
        let (width, height) = (out.width, out.height);
        let samples = out.options.samples.max(1);
        let max_lights = self.max_lights();
        if max_lights > self.light_capacity {
            self.light_buffers =
                [0; 3].map(|_| uniform(r, "dynamic lights", 16 * max_lights as u64));
            self.light_capacity = max_lights;
        }
        let module = |label, source: &str| {
            r.device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some(label),
                source: wgpu::ShaderSource::Wgsl(
                    source.replace("MAX_LIGHTS", &max_lights.to_string()).into(),
                ),
            })
        };
        let texture = |format, samples, sampled: bool| {
            r.device
                .create_texture(&wgpu::TextureDescriptor {
                    label: Some("dynamic target"),
                    size: wgpu::Extent3d {
                        width,
                        height,
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
        };
        let color = texture(HALF, samples, samples == 1);
        let resolve = (samples > 1).then(|| texture(HALF, 1, true));
        let depth = texture(DEPTH, samples, false);
        let attributes = [
            wgpu::vertex_attr_array![0 => Float32x3],
            wgpu::vertex_attr_array![1 => Float32x3],
        ];
        let layout = |i: usize| wgpu::VertexBufferLayout {
            array_stride: 12,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &attributes[i],
        };
        let pipeline = |label,
                        vs: &str,
                        fs: &str,
                        buffers: &[wgpu::VertexBufferLayout],
                        format: wgpu::TextureFormat,
                        scene: bool| {
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
                        targets: &[Some(format.into())],
                    }),
                    primitive: wgpu::PrimitiveState {
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
        let render_group = |p: &wgpu::RenderPipeline| {
            bind(
                r,
                p.get_bind_group_layout(0),
                &[
                    (0, self.render_uniforms.as_entire_binding()),
                    (1, self.light_buffers[0].as_entire_binding()),
                    (2, self.light_buffers[1].as_entire_binding()),
                    (3, self.light_buffers[2].as_entire_binding()),
                ],
            )
        };
        let standard = pipeline(
            "standard",
            wgsl!("standard_vs"),
            wgsl!("standard_fs"),
            &[layout(0), layout(1)],
            HALF,
            true,
        );
        let standard_render = render_group(&standard);
        let dfg = wgpu::BindingResource::TextureView(&r.dfg);
        let sampler = wgpu::BindingResource::Sampler(&self.sampler);
        let shapes = self
            .shapes
            .iter()
            .map(|shape| {
                bind(
                    r,
                    standard.get_bind_group_layout(1),
                    &[
                        (0, shape.object.as_entire_binding()),
                        (1, sampler.clone()),
                        (2, dfg.clone()),
                    ],
                )
            })
            .collect();
        let basic = pipeline(
            "basic",
            wgsl!("basic_vs"),
            wgsl!("basic_fs"),
            &[layout(0)],
            HALF,
            true,
        );
        let basic_render = render_group(&basic);
        let markers = self
            .markers
            .iter()
            .map(|object| {
                bind(
                    r,
                    basic.get_bind_group_layout(1),
                    &[(0, object.as_entire_binding())],
                )
            })
            .collect();
        let output = pipeline(
            "render output",
            wgsl!("output_vs"),
            wgsl!("output_fs"),
            &[layout(0)],
            out.options.format,
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
                    (0, sampler),
                    (
                        1,
                        wgpu::BindingResource::TextureView(resolve.as_ref().unwrap_or(&color)),
                    ),
                    (2, self.output_object.as_entire_binding()),
                ],
            ),
        ];
        Layout::new(144)
            .m4(0, Matrix4::IDENTITY)
            .m4(64, Matrix4::IDENTITY)
            .f(128, width as f64)
            .f(132, height as f64)
            .write(r, &self.output_render);
        Layout::new(64)
            .m4(0, Matrix4::IDENTITY)
            .write(r, &self.output_object);
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
            samples,
            max_lights,
            color,
            resolve,
            depth,
            screen,
            standard: (standard, standard_render),
            basic: (basic, basic_render),
            output: (output, output_binds),
            shapes,
            markers,
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
        let max_lights = self.max_lights();
        if self.targets.as_ref().is_none_or(|t| {
            t.width != out.width
                || t.height != out.height
                || t.format != out.options.format
                || t.samples != out.options.samples.max(1)
                || t.max_lights != max_lights
        }) {
            self.resize(r, out)?;
        }
        s.update()?;
        let (camera, world) = s.camera(c)?;
        let (projection, view) = (camera.projection_matrix()?, world.inverse());
        let screen = projection * view;
        let frustum = Frustum::from_projection(screen);
        // The point-light arrays, in light creation order and view space.
        let count = self.lights.len().min(max_lights);
        let mut data = [
            vec![0f32; max_lights * 4],
            vec![0f32; max_lights * 4],
            vec![0f32; max_lights * 4],
        ];
        for (i, light) in self.lights.iter().take(count).enumerate() {
            let p = view.transform_point3(light.position);
            data[0][i * 4..i * 4 + 4].copy_from_slice(&[p.x as f32, p.y as f32, p.z as f32, 0.]);
            for k in 0..3 {
                data[1][i * 4 + k] = (light.color[k] * 1000.) as f32;
            }
            data[2][i * 4] = 2.;
        }
        for (buffer, data) in self.light_buffers.iter().zip(&data) {
            r.queue.write_buffer(buffer, 0, bytemuck::cast_slice(data));
        }
        let ambient = Color::from_hex(0x404040).0.to_array().map(|v| v * 0.5);
        Layout::new(144)
            .m4(0, projection)
            .m4(64, view)
            .v3(128, ambient)
            .i(140, count as i32)
            .write(r, &self.render_uniforms);
        // Visible objects, sorted front to back (then in creation order).
        let mut draws = vec![];
        for (i, shape) in self.shapes.iter().enumerate() {
            let center = shape.model.transform_point3(Vector3::ZERO);
            let radius = self.meshes[shape.mesh].radius;
            if frustum.intersects_sphere(Sphere { center, radius }) {
                draws.push((screen.project_point3(center).z, i, false));
            }
            Layout::new(160)
                .v3(0, shape.color)
                .f(12, 1.)
                .f(16, shape.metalness)
                .f(20, shape.roughness)
                .m3(32, shape.model.inverse().transpose())
                .f(92, 1.)
                .m4(96, shape.model)
                .write(r, &shape.object);
        }
        // Each light's marker is created right after its light, following the
        // scene's meshes.
        for (i, (light, object)) in self.lights.iter().zip(&self.markers).enumerate() {
            if frustum.intersects_sphere(Sphere {
                center: light.position,
                radius: self.marker.radius,
            }) {
                draws.push((screen.project_point3(light.position).z, i, true));
            }
            Layout::new(80)
                .v3(0, light.color)
                .f(12, 1.)
                .m4(16, Matrix4::from_translation(light.position))
                .write(r, object);
        }
        draws.sort_by(|a, b| {
            a.0.partial_cmp(&b.0)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then((a.2, a.1).cmp(&(b.2, b.1)))
        });
        let t = self
            .targets
            .as_ref()
            .ok_or(Error::Invalid("dynamic targets"))?;
        let mut encoder = r.device.create_command_encoder(&Default::default());
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("dynamic scene"),
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
            let mut current = None;
            for &(_, i, marker) in &draws {
                if current != Some(marker) {
                    let (pipeline, render) = if marker { &t.basic } else { &t.standard };
                    pass.set_pipeline(pipeline);
                    pass.set_bind_group(0, render, &[]);
                    current = Some(marker);
                }
                let m = if marker {
                    pass.set_bind_group(1, &t.markers[i], &[]);
                    pass.set_vertex_buffer(0, self.marker.positions.slice(..));
                    &self.marker
                } else {
                    let m = &self.meshes[self.shapes[i].mesh];
                    pass.set_bind_group(1, &t.shapes[i], &[]);
                    pass.set_vertex_buffer(0, m.normals.slice(..));
                    pass.set_vertex_buffer(1, m.positions.slice(..));
                    m
                };
                pass.set_index_buffer(m.index.slice(..), wgpu::IndexFormat::Uint32);
                pass.draw_indexed(0..m.count, 0, 0..1);
            }
        }
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("dynamic output"),
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
    /// dynamic mode, auto-add lights, add light, remove light and remove all
    /// lights. Toggling the mode recreates the renderer: new OrbitControls
    /// around the same camera, and auto-add off.
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        match index {
            0 => {
                self.dynamic = value > 0.5;
                self.auto_add = None;
                self.reset_controls = true;
                self.targets = None;
            }
            1 => self.auto_add = (value > 0.5).then_some(self.time + 0.5),
            2 => self.pending_adds += 1,
            3 => {
                if self.pending_adds > 0 {
                    self.pending_adds -= 1;
                } else {
                    self.remove_light();
                }
            }
            4 => {
                self.pending_adds = 0;
                while !self.lights.is_empty() {
                    self.remove_light();
                }
            }
            _ => return Err(Error::Invalid("dynamic lights parameter")),
        }
        Ok(())
    }
    pub fn seek(&mut self, t: f64) {
        self.time = t;
    }
}
