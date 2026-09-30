//! webgl_points_dynamic: the male02 and female02 OBJ vertices as points, nine
//! bodies with eight clones each, crumbling to the floor and rising back as
//! the page's per-vertex random walk moves them. As in the original, the walk
//! runs on the CPU over the Float32 positions (one Math.random draw per
//! coordinate, in the page's order) and a moved body's positions are written
//! into its resident vertex buffer (positions.needsUpdate); its eight clones
//! draw that one buffer. Each point is a square of gl_PointSize pixels
//! (attenuated, at least one pixel) expanded by the vertex shader from the
//! resident position, as WebGL rasterizes points; the clones are culled by
//! their geometry's first bounding sphere, as three.js computes it once. The
//! composer's RenderPass, BloomPass, FilmPass, FocusShader and OutputPass are
//! full-screen draws on half-float GPU targets (see composer_passes).
use super::composer_passes::{Draw, Kit};
use super::controls_attributes::webgl_perspective;
use super::gltf_viewer::fetch;
use super::terrain_loaders::parse_obj;
use crate::{
    Error, Result, camera::*, geometry::*, math::*, render_target::*, renderer::*, scene::*,
};

// The composer targets, by index.
const READ: usize = 0;
const WRITE_A: usize = 1;
const WRITE_B: usize = 2;
const BLOOM_X: usize = 3;
const BLOOM_Y: usize = 4;
/// One uniform slot per draw, at the dynamic offset alignment.
const SLOT: usize = 256;
/// createMesh()'s clone offsets; the last is the colored original.
const CLONES: [[f64; 3]; 8] = [
    [6000., 0., -4000.],
    [5000., 0., 0.],
    [1000., 0., 5000.],
    [1000., 0., -5000.],
    [4000., 0., 2000.],
    [-4000., 0., 1000.],
    [-5000., 0., -5000.],
    [0., 0., 0.],
];
/// PointsMaterial as WebGLRenderer draws it: gl_PointSize = size × scale /
/// −mvPosition.z (at least one pixel), the color with FogExp2 over the
/// interpolated fog depth. d.params: size, scale (half the target height),
/// the target size.
const POINTS: &str = "struct D{model_view:mat4x4<f32>,projection:mat4x4<f32>,color:vec4<f32>,params:vec4<f32>,fog:vec4<f32>}
@group(0) @binding(0) var<uniform> d:D;
struct V{@builtin(position) p:vec4<f32>,@location(0) fog_depth:f32}
@vertex fn vs(@builtin(vertex_index) i:u32,@location(0) position:vec3<f32>)->V{
 let mv=d.model_view*vec4(position,1.0);var clip=d.projection*mv;
 let size=max(d.params.x*(d.params.y/-mv.z),1.0);
 let corner=array(vec2(-1.0,-1.0),vec2(1.0,-1.0),vec2(-1.0,1.0),vec2(-1.0,1.0),vec2(1.0,-1.0),vec2(1.0,1.0))[i];
 // GL's clip depth (-w..w) into WebGPU's (0..w).
 clip=vec4(clip.xy+corner*size/d.params.zw*clip.w,(clip.z+clip.w)*0.5,clip.w);
 return V(clip,-mv.z);}
@fragment fn fs(v:V)->@location(0) vec4<f32>{
 let factor=1.0-exp(-d.fog.w*d.fog.w*v.fog_depth*v.fog_depth);
 return vec4(mix(d.color.rgb,d.fog.rgb,factor),1.0);}";
/// One createMesh(): the resident positions, the walk state and its clones.
struct Body {
    buffer: wgpu::Buffer,
    positions: Vec<f32>,
    initial: Vec<f32>,
    count: u32,
    /// The geometry's bounding sphere, computed from the first positions.
    sphere: (Vector3, f64),
    vertices_down: usize,
    vertices_up: usize,
    direction: i32,
    speed: f64,
    delay: i64,
    start: i64,
    moved: bool,
}
/// clonemeshes: one Points of a body.
struct Clone {
    body: usize,
    position: Vector3,
    color: Color,
    speed: f64,
    rotation: f64,
}
struct Targets {
    read: RenderTarget,
    depth: wgpu::TextureView,
    write: [RenderTarget; 2],
    bloom: [RenderTarget; 2],
}
pub(super) struct Demo {
    seed: u32,
    parent_rotation: f64,
    clones: Vec<Clone>,
    bodies: Vec<Body>,
    grid: (wgpu::Buffer, u32, (Vector3, f64)),
    pipeline: wgpu::RenderPipeline,
    uniforms: wgpu::Buffer,
    bind: wgpu::BindGroup,
    /// The timer's last time (s) and a still frame to step to.
    last: f64,
    pending: Option<f64>,
    film: f64,
    targets: Option<Targets>,
    screen: Option<RenderTarget>,
    kit: Kit,
}
/// BufferGeometry.computeBoundingSphere(): the box center and the farthest point.
fn bounding_sphere(positions: &[f32]) -> (Vector3, f64) {
    let points = positions
        .chunks(3)
        .map(|p| Vector3::new(p[0] as f64, p[1] as f64, p[2] as f64));
    let (min, max) = points.clone().fold(
        (
            Vector3::splat(f64::INFINITY),
            Vector3::splat(f64::NEG_INFINITY),
        ),
        |(a, b), p| (a.min(p), b.max(p)),
    );
    let center = (min + max) * 0.5;
    let radius = points
        .map(|p| p.distance_squared(center))
        .fold(0., f64::max)
        .sqrt();
    (center, radius)
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, _id: u32, r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 20.,
            near: 1.,
            far: 50000.,
            aspect,
            ..Default::default()
        }));
        s.get_mut(c)?.position = Vector3::new(0., 700., 7000.);
        s.look_at(c, Vector3::ZERO)?;
        use wgpu::util::DeviceExt;
        let vertex = |data: &[f32]| {
            r.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("points positions"),
                    contents: bytemuck::cast_slice(data),
                    usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                })
        };
        // The grid: PlaneGeometry( 15000, 15000, 64, 64 ) drawn through its
        // index, one point per index (gathered once: the grid never changes).
        let plane = PlaneGeometry::build(15000., 15000., 64, 64)?;
        let plane_positions: Vec<f32> = plane
            .positions()?
            .iter()
            .flat_map(|p| [p.x as f32, p.y as f32, p.z as f32])
            .collect();
        let index = plane.index.clone().ok_or(Error::Invalid("grid index"))?;
        let indexed: Vec<f32> = index
            .iter()
            .flat_map(|&i| plane_positions[i as usize * 3..i as usize * 3 + 3].to_vec())
            .collect();
        let grid = (
            vertex(&indexed),
            index.len() as u32,
            bounding_sphere(&plane_positions),
        );
        let module = r.device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("points"),
            source: wgpu::ShaderSource::Wgsl(POINTS.into()),
        });
        let group = r
            .device
            .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("points"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: true,
                        min_binding_size: wgpu::BufferSize::new(176),
                    },
                    count: None,
                }],
            });
        let layout = r
            .device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[&group],
                push_constant_ranges: &[],
            });
        // Each point is one instance of a two-triangle square.
        let pipeline = r
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("points"),
                layout: Some(&layout),
                vertex: wgpu::VertexState {
                    module: &module,
                    entry_point: Some("vs"),
                    compilation_options: Default::default(),
                    buffers: &[wgpu::VertexBufferLayout {
                        array_stride: 12,
                        step_mode: wgpu::VertexStepMode::Instance,
                        attributes: &wgpu::vertex_attr_array![0 => Float32x3],
                    }],
                },
                fragment: Some(wgpu::FragmentState {
                    module: &module,
                    entry_point: Some("fs"),
                    compilation_options: Default::default(),
                    targets: &[Some(wgpu::TextureFormat::Rgba16Float.into())],
                }),
                primitive: Default::default(),
                depth_stencil: Some(wgpu::DepthStencilState {
                    format: wgpu::TextureFormat::Depth24Plus,
                    depth_write_enabled: true,
                    depth_compare: wgpu::CompareFunction::LessEqual,
                    stencil: Default::default(),
                    bias: Default::default(),
                }),
                multisample: Default::default(),
                multiview: None,
                cache: None,
            });
        // The grid and 72 clones: one uniform slot each.
        let uniforms = r.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("points uniforms"),
            size: (SLOT * 73) as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let bind = r.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("points"),
            layout: &group,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                    buffer: &uniforms,
                    offset: 0,
                    size: wgpu::BufferSize::new(176),
                }),
            }],
        });
        let mut demo = Self {
            seed: 186,
            parent_rotation: 0.,
            clones: vec![],
            bodies: vec![],
            grid,
            pipeline,
            uniforms,
            bind,
            last: 0.,
            pending: None,
            film: 0.,
            targets: None,
            screen: None,
            kit: Kit::new(r),
        };
        // The male model's callback, then the female's.
        let male = demo.combine("obj/male02/male02.obj").await?;
        for (x, z, color) in [
            (-500., 600., 0xff7744),
            (500., 0., 0xff5522),
            (-250., 1500., 0xff9922),
            (-250., -1500., 0xff99ff),
        ] {
            demo.body(r, &male, x, z, color);
        }
        let female = demo.combine("obj/female02/female02.obj").await?;
        for (x, z, color) in [
            (-1000., 0., 0xffdd44),
            (0., 0., 0xffffff),
            (1000., 400., 0xff4422),
            (250., 1500., 0xff9955),
            (250., 2500., 0xff77dd),
        ] {
            demo.body(r, &female, x, z, color);
        }
        // The fixture's first render at time 0.
        demo.step(0.);
        demo.upload(r);
        Ok(demo)
    }
    fn random(&mut self) -> f64 {
        self.seed = self.seed.wrapping_mul(1664525).wrapping_add(1013904223);
        self.seed as f64 / 4294967296.
    }
    /// combineBuffer( object, 'position' ): every mesh's positions in order.
    async fn combine(&self, path: &str) -> Result<Vec<f32>> {
        let text = fetch(&format!("/web/gallery/assets/{path}")).await?;
        Ok(parse_obj(&String::from_utf8_lossy(&text))
            .into_iter()
            .flat_map(|o| o.positions)
            .collect())
    }
    /// createMesh( positions, scene, 4.05, x, -350, z, color ).
    fn body(&mut self, r: &Renderer, positions: &[f32], x: f64, z: f64, color: u32) {
        use wgpu::util::DeviceExt;
        let body = self.bodies.len();
        for (i, offset) in CLONES.iter().enumerate() {
            let speed = 0.5 + self.random();
            self.clones.push(Clone {
                body,
                position: Vector3::new(x + offset[0], -350. + offset[1], z + offset[2]),
                color: Color::from_hex(if i < CLONES.len() - 1 {
                    0x252525
                } else {
                    color
                }),
                speed,
                rotation: 0.,
            });
        }
        let delay = (200. + 200. * self.random()).floor() as i64;
        let start = (100. + 200. * self.random()).floor() as i64;
        self.bodies.push(Body {
            buffer: r
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("points positions"),
                    contents: bytemuck::cast_slice(positions),
                    usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                }),
            positions: positions.to_vec(),
            initial: positions.to_vec(),
            count: (positions.len() / 3) as u32,
            sphere: bounding_sphere(positions),
            vertices_down: 0,
            vertices_up: 0,
            direction: 0,
            speed: 15.,
            delay,
            start,
            moved: false,
        });
    }
    /// render(): the rotations and every body's walk, then the composer's
    /// FilmPass time.
    fn step(&mut self, delta: f64) {
        let delta = if delta < 2. { delta } else { 2. };
        self.parent_rotation += -0.02 * delta;
        for clone in &mut self.clones {
            clone.rotation += -0.1 * delta * clone.speed;
        }
        let mut bodies = std::mem::take(&mut self.bodies);
        for data in &mut bodies {
            let count = data.positions.len() / 3;
            if data.start > 0 {
                data.start -= 1;
            } else if data.direction == 0 {
                data.direction = -1;
            }
            for i in 0..count {
                let p = &data.positions[i * 3..i * 3 + 3];
                let (px, py, pz) = (p[0] as f64, p[1] as f64, p[2] as f64);
                // Falling down.
                if data.direction < 0 {
                    if py > 0. {
                        let x = px + 1.5 * (0.50 - self.random()) * data.speed * delta;
                        let y = py + 3.0 * (0.25 - self.random()) * data.speed * delta;
                        let z = pz + 1.5 * (0.50 - self.random()) * data.speed * delta;
                        data.positions[i * 3..i * 3 + 3]
                            .copy_from_slice(&[x as f32, y as f32, z as f32]);
                        data.moved = true;
                    } else {
                        data.vertices_down += 1;
                    }
                }
                // Rising up.
                if data.direction > 0 {
                    let q = &data.initial[i * 3..i * 3 + 3];
                    let (ix, iy, iz) = (q[0] as f64, q[1] as f64, q[2] as f64);
                    let dx = (px - ix).abs();
                    let dy = (py - iy).abs();
                    let dz = (pz - iz).abs();
                    // The page sums dx twice.
                    let d = dx + dy + dx;
                    if d > 1. {
                        let x = px - (px - ix) / dx * data.speed * delta * (0.85 - self.random());
                        let y = py - (py - iy) / dy * data.speed * delta * (1. + self.random());
                        let z = pz - (pz - iz) / dz * data.speed * delta * (0.85 - self.random());
                        data.positions[i * 3..i * 3 + 3]
                            .copy_from_slice(&[x as f32, y as f32, z as f32]);
                        data.moved = true;
                    } else {
                        data.vertices_up += 1;
                    }
                }
            }
            // All vertices down.
            if data.vertices_down >= count {
                if data.delay <= 0 {
                    data.direction = 1;
                    data.speed = 5.;
                    data.vertices_down = 0;
                    data.delay = 320;
                } else {
                    data.delay -= 1;
                }
            }
            // All vertices up.
            if data.vertices_up >= count {
                if data.delay <= 0 {
                    data.direction = -1;
                    data.speed = 15.;
                    data.vertices_up = 0;
                    data.delay = 120;
                } else {
                    data.delay -= 1;
                }
            }
        }
        self.bodies = bodies;
        // composer.render( 0.01 ): FilmPass adds the delta.
        self.film += 0.01;
    }
    /// positions.needsUpdate: a moved body's positions into its resident buffer.
    fn upload(&mut self, r: &Renderer) {
        for body in &mut self.bodies {
            if std::mem::take(&mut body.moved) {
                r.queue
                    .write_buffer(&body.buffer, 0, bytemuck::cast_slice(&body.positions));
            }
        }
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, dt: f64, animate: bool) -> Result<()> {
        if animate {
            self.step(10. * dt);
        }
        Ok(())
    }
    pub fn prepare(&mut self, _s: &mut Scene, _c: Object3D, r: &Renderer) -> Result<()> {
        if let Some(t) = self.pending.take() {
            // timer.update( t * 1000 ): 10 × getDelta().
            let delta = 10. * ((t * 1000. - self.last * 1000.) / 1000.);
            self.last = t;
            self.step(delta);
        }
        self.upload(r);
        Ok(())
    }
    fn resize(&mut self, r: &Renderer, out: &RenderTarget) -> Result<()> {
        let make = || {
            RenderTarget::with_options(
                &r.device,
                out.width,
                out.height,
                RenderTargetOptions {
                    format: wgpu::TextureFormat::Rgba16Float,
                    depth_buffer: false,
                    ..Default::default()
                },
            )
        };
        let depth = r
            .device
            .create_texture(&wgpu::TextureDescriptor {
                label: Some("points depth"),
                size: wgpu::Extent3d {
                    width: out.width,
                    height: out.height,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Depth24Plus,
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                view_formats: &[],
            })
            .create_view(&Default::default());
        self.targets = Some(Targets {
            read: make()?,
            depth,
            write: [make()?, make()?],
            bloom: [make()?, make()?],
        });
        self.screen = Some(RenderTarget::with_options(
            &r.device,
            out.width,
            out.height,
            RenderTargetOptions {
                samples: 0,
                depth_buffer: false,
                ..out.options.clone()
            },
        )?);
        self.kit.reset();
        Ok(())
    }
    /// RenderPass( scene, camera ): the background clear, then the grid and
    /// every clone not culled by its bounding sphere.
    fn scene(
        &self,
        r: &Renderer,
        s: &Scene,
        c: Object3D,
        t: &Targets,
        width: u32,
        height: u32,
    ) -> Result<()> {
        let (camera, world) = s.camera(c)?;
        let Camera::Perspective(p) = camera else {
            return Err(Error::Invalid("points camera"));
        };
        let projection = webgl_perspective(p.fov, p.aspect, p.near, p.far);
        let view = world.inverse();
        let frustum = projection * view;
        let planes = {
            let m = frustum.to_cols_array_2d();
            let row = |i: usize| Vector4::new(m[0][i], m[1][i], m[2][i], m[3][i]);
            let (x, y, z, w) = (row(0), row(1), row(2), row(3));
            [w + x, w - x, w + y, w - y, w + z, w - z].map(|p| p / p.truncate().length())
        };
        let visible = |matrix: Matrix4, (center, radius): (Vector3, f64)| {
            let c = matrix.transform_point3(center);
            let scale = matrix
                .x_axis
                .truncate()
                .length()
                .max(matrix.y_axis.truncate().length())
                .max(matrix.z_axis.truncate().length());
            planes
                .iter()
                .all(|p| p.truncate().dot(c) + p.w >= -radius * scale)
        };
        let parent = Matrix4::from_rotation_y(self.parent_rotation);
        let fog = Color::from_hex(0x000104).0;
        let mut data = vec![0f32; SLOT / 4 * 73];
        let mut slot = |i: usize, model: Matrix4, color: Vector3, size: f64| {
            let d = &mut data[i * SLOT / 4..][..44];
            d[0..16].copy_from_slice(&(view * model).to_cols_array().map(|v| v as f32));
            d[16..32].copy_from_slice(&projection.to_cols_array().map(|v| v as f32));
            d[32..36].copy_from_slice(&[color.x as f32, color.y as f32, color.z as f32, 1.]);
            d[36..40].copy_from_slice(&[
                size as f32,
                height as f32 * 0.5,
                width as f32,
                height as f32,
            ]);
            d[40..44].copy_from_slice(&[fog.x as f32, fog.y as f32, fog.z as f32, 0.0000675]);
        };
        let grid_model = parent
            * Matrix4::from_translation(Vector3::new(0., -400., 0.))
            * Matrix4::from_rotation_x(-std::f64::consts::FRAC_PI_2);
        slot(0, grid_model, Color::from_hex(0xff0000).0, 10.);
        let mut draws = vec![];
        if visible(grid_model, self.grid.2) {
            draws.push((0, None));
        }
        for (i, clone) in self.clones.iter().enumerate() {
            let model = parent
                * Matrix4::from_scale_rotation_translation(
                    Vector3::splat(4.05),
                    Quaternion::from_rotation_y(clone.rotation),
                    clone.position,
                );
            slot(i + 1, model, clone.color.0, 30.);
            if visible(model, self.bodies[clone.body].sphere) {
                draws.push((i + 1, Some(clone.body)));
            }
        }
        r.queue
            .write_buffer(&self.uniforms, 0, bytemuck::cast_slice(&data));
        let background = Color::from_hex(0x000104).0;
        let mut encoder = r.device.create_command_encoder(&Default::default());
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("points scene"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &t.read.view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: background.x,
                            g: background.y,
                            b: background.z,
                            a: 1.,
                        }),
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
            pass.set_pipeline(&self.pipeline);
            for (i, body) in draws {
                pass.set_bind_group(0, &self.bind, &[(i * SLOT) as u32]);
                match body {
                    // The indexed grid: each index is one point.
                    None => {
                        pass.set_vertex_buffer(0, self.grid.0.slice(..));
                        pass.draw(0..6, 0..self.grid.1);
                    }
                    Some(b) => {
                        let body = &self.bodies[b];
                        pass.set_vertex_buffer(0, body.buffer.slice(..));
                        pass.draw(0..6, 0..body.count);
                    }
                }
            }
        }
        r.queue.submit([encoder.finish()]);
        Ok(())
    }
    /// composer.render(): RenderPass, BloomPass( 0.75 ), FilmPass(),
    /// FocusShader and OutputPass.
    pub fn render(
        &mut self,
        r: &Renderer,
        s: &mut Scene,
        c: Object3D,
        out: &RenderTarget,
    ) -> Result<bool> {
        if self
            .screen
            .as_ref()
            .is_none_or(|t| t.width != out.width || t.height != out.height)
        {
            self.resize(r, out)?;
        }
        let (Some(t), Some(screen)) = (&self.targets, &self.screen) else {
            return Err(Error::Invalid("points targets"));
        };
        // The camera's world matrix (the scene holds only the camera).
        s.update()?;
        self.scene(r, s, c, t, out.width, out.height)?;
        let full = [0., 0., out.width as f32, out.height as f32];
        let mut convolution = Draw::new(
            "fs_convolution",
            READ,
            Some(BLOOM_X),
            full,
            [0.001953125, 0., 0., 0.],
        );
        convolution.flip = true;
        // The combine adds into the RenderPass target.
        let mut combine = Draw::new("fs_combine", BLOOM_Y, Some(READ), full, [0.75, 0., 0., 0.]);
        combine.load = true;
        combine.gl = false;
        let mut film = Draw::new(
            "fs_film",
            READ,
            Some(WRITE_A),
            full,
            [self.film as f32, 0.5, 0., 0.],
        );
        film.flip = true;
        // effectFocus: screenWidth and screenHeight in device pixels.
        let focus = Draw::new(
            "fs_focus",
            WRITE_A,
            Some(WRITE_B),
            full,
            [out.width as f32, out.height as f32, 0.94, 0.00125],
        );
        let draws = [
            convolution,
            Draw::new(
                "fs_convolution",
                BLOOM_X,
                Some(BLOOM_Y),
                full,
                [0., 0.001953125, 0., 0.],
            ),
            combine,
            film,
            focus,
            Draw::new("fs_gamma", WRITE_B, None, full, [0.; 4]),
        ];
        let targets = [&t.read, &t.write[0], &t.write[1], &t.bloom[0], &t.bloom[1]];
        let views = targets.map(|t| &t.view);
        let formats = targets.map(|t| t.options.format);
        let screen_view = &screen.view;
        self.kit.run(
            r,
            &draws,
            &views,
            &formats,
            (screen_view, out.options.format),
        );
        Ok(true)
    }
    pub fn output(&self) -> Option<&RenderTarget> {
        self.screen.as_ref()
    }
    pub fn draw(&mut self, _kind: u32, _x: f64, _y: f64) {}
    pub fn key(&mut self, _code: u32, _down: bool) {}
    #[allow(clippy::too_many_arguments)]
    pub fn input(
        &mut self,
        _s: &mut Scene,
        _c: Object3D,
        _dx: f64,
        _dy: f64,
        _wheel: f64,
        _pan: bool,
        _height: f64,
    ) -> Result<()> {
        Ok(())
    }
    pub fn parameter(&mut self, _index: usize, _value: f32) -> Result<()> {
        Err(Error::Invalid("points dynamic parameter"))
    }
    /// A still frame: one render() with the timer at t.
    pub fn seek(&mut self, t: f64) {
        self.pending = Some(t);
    }
}
