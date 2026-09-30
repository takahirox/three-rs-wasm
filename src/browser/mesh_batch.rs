//! webgpu_mesh_batch: a BatchedMesh of cones, boxes and spheres (512
//! instances by default) whose first `dynamic` matrices turn every frame,
//! drawn by the node material that shades the batch color by the view
//! normal. As BatchedMesh.onBeforeRender does, each frame culls the
//! instances' bounding spheres against the frustum and sorts them by depth
//! (radix sort on the scaled depth, or the default comparator), then issues
//! one indexed draw per visible instance whose instance index reads the
//! batch id; matrices, colors and ids live in GPU buffers, the matrices
//! uploaded when they change and the ids each frame, as the original's
//! textures are.
use super::controls_attributes::{Controls, camera_state};
use crate::{
    Error, Result, camera::*, geometry::*, math::*, render_target::*, renderer::*, scene::*,
};
use std::f64::consts::PI;

const MAX_COUNT: usize = 20000;
const SHADER: &str = "struct U{view:mat4x4<f32>,projection:mat4x4<f32>,normal:mat4x4<f32>,params:vec4<f32>}
@group(0) @binding(0) var<uniform> u:U;
@group(0) @binding(1) var<storage,read> matrices:array<mat4x4<f32>>;
@group(0) @binding(2) var<storage,read> colors:array<vec4<f32>>;
@group(0) @binding(3) var<storage,read> ids:array<u32>;
struct O{@builtin(position) clip:vec4<f32>,@location(0) normal:vec3<f32>,@location(1) color:vec4<f32>}
@vertex fn vs(@builtin(instance_index) draw:u32,@location(0) position:vec3<f32>,@location(1) normal:vec3<f32>)->O{
 let id=ids[draw];let bm=matrices[id];let b=mat3x3(bm[0].xyz,bm[1].xyz,bm[2].xyz);
 let local=(bm*vec4(position,1.0)).xyz;
 let n=b*(normal/vec3(dot(b[0],b[0]),dot(b[1],b[1]),dot(b[2],b[2])));
 let mv=u.view*vec4(local,1.0);
 return O(u.projection*mv,normalize((u.normal*vec4(n,0.0)).xyz),colors[id]);}
@fragment fn fs(i:O)->@location(0) vec4<f32>{
 let n=normalize(i.normal);let c=i.color;
 return vec4(c.rgb*((n*0.5+0.5).y+0.5),c.a*u.params.x);}";

/// The addGeometry ranges in the shared buffers: index start and count,
/// and the geometry's bounding sphere.
struct Range {
    start: u32,
    count: u32,
    sphere: Sphere,
}
struct Gpu {
    format: wgpu::TextureFormat,
    samples: u32,
    opaque: wgpu::RenderPipeline,
    transparent: wgpu::RenderPipeline,
}
pub(super) struct Demo {
    seed: u32,
    controls: Controls,
    /// webgpu, count, dynamic, opacity, sortObjects, perObjectFrustumCulled,
    /// useCustomSort.
    params: [f64; 7],
    ranges: Vec<Range>,
    /// Per instance: its geometry, Float32 matrix, color and rotation speed.
    geometry_ids: Vec<usize>,
    matrices: Vec<[f32; 16]>,
    colors: Vec<[f32; 4]>,
    speeds: Vec<Matrix4>,
    matrices_dirty: bool,
    colors_dirty: bool,
    pending: bool,
    rebuild: bool,
    vertices: wgpu::Buffer,
    indices: wgpu::Buffer,
    uniforms: wgpu::Buffer,
    matrix_buffer: wgpu::Buffer,
    color_buffer: wgpu::Buffer,
    id_buffer: wgpu::Buffer,
    bind: wgpu::BindGroup,
    layout: wgpu::BindGroupLayout,
    module: wgpu::ShaderModule,
    gpu: Option<Gpu>,
    target: Option<(RenderTarget, wgpu::TextureView)>,
    /// This frame's draws: ( index start, count ) per visible instance.
    draws: Vec<(u32, u32)>,
}
fn buffer(r: &Renderer, label: &str, size: u64, usage: wgpu::BufferUsages) -> wgpu::Buffer {
    r.device.create_buffer(&wgpu::BufferDescriptor {
        label: Some(label),
        size,
        usage: usage | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    })
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, _id: u32, r: &Renderer) -> Result<Self> {
        // initGeometries(): ConeGeometry( 1, 2 ), BoxGeometry( 2, 2, 2 ),
        // SphereGeometry( 1, 16, 8 ), appended as addGeometry does.
        let tau = std::f64::consts::TAU;
        let geometries = [
            CylinderGeometry::build(0., 1., 2., 32, 1, false, 0., tau)?,
            BoxGeometry::build(2., 2., 2.)?,
            SphereGeometry::build(1., 16, 8)?,
        ];
        let (mut vertices, mut indices, mut ranges) = (vec![], vec![], vec![]);
        for mut g in geometries {
            let base = (vertices.len() / 6) as u32;
            let positions = g.positions()?;
            let normals = match g.attributes.get("normal") {
                Some(a) => (0..a.count())
                    .map(|i| a.vector3(i))
                    .collect::<Result<Vec<_>>>()?,
                None => return Err(Error::Invalid("batch geometry normals")),
            };
            for (p, n) in positions.iter().zip(&normals) {
                vertices.extend([p.x, p.y, p.z, n.x, n.y, n.z].map(|v| v as f32));
            }
            let index = g
                .index
                .clone()
                .ok_or(Error::Invalid("batch geometry index"))?;
            let start = indices.len() as u32;
            indices.extend(index.iter().map(|&i| (i + base) as u16));
            let sphere = g.compute_bounding_sphere()?;
            ranges.push(Range {
                start,
                count: index.len() as u32,
                sphere,
            });
        }
        if indices.len() % 2 == 1 {
            indices.push(0);
        }
        use wgpu::util::DeviceExt;
        let init = |label, contents: &[u8], usage| {
            r.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some(label),
                    contents,
                    usage,
                })
        };
        let vertex_buffer = init(
            "batch vertices",
            bytemuck::cast_slice(&vertices),
            wgpu::BufferUsages::VERTEX,
        );
        let index_buffer = init(
            "batch indices",
            bytemuck::cast_slice(&indices),
            wgpu::BufferUsages::INDEX,
        );
        use wgpu::BufferUsages as B;
        let uniforms = buffer(r, "batch uniforms", 208, B::UNIFORM);
        let matrix_buffer = buffer(r, "batch matrices", (MAX_COUNT * 64) as u64, B::STORAGE);
        let color_buffer = buffer(r, "batch colors", (MAX_COUNT * 16) as u64, B::STORAGE);
        let id_buffer = buffer(r, "batch ids", (MAX_COUNT * 4) as u64, B::STORAGE);
        let storage = |binding| wgpu::BindGroupLayoutEntry {
            binding,
            visibility: wgpu::ShaderStages::VERTEX,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Storage { read_only: true },
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        };
        let layout = r
            .device
            .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: None,
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                    storage(1),
                    storage(2),
                    storage(3),
                ],
            });
        let bind = r.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: uniforms.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: matrix_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: color_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: id_buffer.as_entire_binding(),
                },
            ],
        });
        let module = r.device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("mesh batch"),
            source: wgpu::ShaderSource::Wgsl(SHADER.into()),
        });
        let mut demo = Self {
            seed: 186,
            controls: Controls::new(None, (0., f64::INFINITY), PI, true),
            params: [1., 512., 16., 1., 1., 1., 1.],
            ranges,
            geometry_ids: vec![],
            matrices: vec![],
            colors: vec![],
            speeds: vec![],
            matrices_dirty: true,
            colors_dirty: true,
            pending: false,
            rebuild: false,
            vertices: vertex_buffer,
            indices: index_buffer,
            uniforms,
            matrix_buffer,
            color_buffer,
            id_buffer,
            bind,
            layout,
            module,
            gpu: None,
            target: None,
            draws: vec![],
        };
        demo.init(s, c)?;
        // The first animate() of the original's loop.
        demo.step(s, c)?;
        Ok(demo)
    }
    fn random(&mut self) -> f64 {
        self.seed = self.seed.wrapping_mul(1664525).wrapping_add(1013904223);
        self.seed as f64 / 4294967296.
    }
    /// init(): the camera, the background (0xffc1c1 when forced to WebGL),
    /// autoRotating OrbitControls and initMesh().
    fn init(&mut self, s: &mut Scene, c: Object3D) -> Result<()> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 70.,
            near: 1.,
            far: 100.,
            aspect,
            ..Default::default()
        }));
        let n = s.get_mut(c)?;
        n.position = Vector3::new(0., 0., 30.);
        n.quaternion = Quaternion::IDENTITY;
        s.background = Color::from_hex(if self.params[0] > 0.5 {
            0xc1c1ff
        } else {
            0xffc1c1
        });
        self.controls = Controls::new(None, (0., f64::INFINITY), PI, true);
        self.controls.auto_rotate = Some(1.);
        self.init_mesh();
        Ok(())
    }
    /// initBatchedMesh(): each instance's randomizeMatrix, color and
    /// rotation speed.
    fn init_mesh(&mut self) {
        let count = self.params[1] as usize;
        self.geometry_ids.clear();
        self.matrices.clear();
        self.colors.clear();
        self.speeds.clear();
        for i in 0..count {
            let p = Vector3::new(
                self.random() * 40. - 20.,
                self.random() * 40. - 20.,
                self.random() * 40. - 20.,
            );
            let r = Vector3::new(
                self.random() * 2. * PI,
                self.random() * 2. * PI,
                self.random() * 2. * PI,
            );
            let q = Euler {
                angles: r,
                order: EulerOrder::XYZ,
            }
            .quaternion();
            let scale = 0.5 + self.random() * 0.5;
            let m = Matrix4::from_scale_rotation_translation(Vector3::splat(scale), q, p);
            self.matrices.push(m.to_cols_array().map(|v| v as f32));
            // new Color( Math.random() * 0xffffff ): setHex floors the value.
            let hex = (self.random() * 0xffffff as f64).floor() as u32;
            let color = Color::from_hex(hex).0;
            self.colors
                .push([color.x as f32, color.y as f32, color.z as f32, 1.]);
            let speed = Vector3::new(
                self.random() * 0.01,
                self.random() * 0.01,
                self.random() * 0.01,
            );
            self.speeds.push(Matrix4::from_quat(
                Euler {
                    angles: speed,
                    order: EulerOrder::XYZ,
                }
                .quaternion(),
            ));
            self.geometry_ids.push(i % self.ranges.len());
        }
        self.matrices_dirty = true;
        self.colors_dirty = true;
    }
    /// animate(): animateMeshes(), then controls.update().
    fn step(&mut self, s: &mut Scene, c: Object3D) -> Result<()> {
        let dynamic = (self.params[2] as usize).min(self.matrices.len());
        for i in 0..dynamic {
            let m = Matrix4::from_cols_array(&self.matrices[i].map(f64::from)) * self.speeds[i];
            self.matrices[i] = m.to_cols_array().map(|v| v as f32);
        }
        if dynamic > 0 {
            self.matrices_dirty = true;
        }
        self.controls.frame_update(s, c)
    }
    pub fn update(&mut self, s: &mut Scene, c: Object3D, _dt: f64, animate: bool) -> Result<()> {
        if animate {
            self.step(s, c)?;
        }
        Ok(())
    }
    pub fn prepare(&mut self, s: &mut Scene, c: Object3D, _r: &Renderer) -> Result<()> {
        if std::mem::take(&mut self.rebuild) {
            self.init(s, c)?;
        }
        if std::mem::take(&mut self.pending) {
            self.step(s, c)?;
        }
        Ok(())
    }
    /// onBeforeRender(): the frustum test and depth sort of the instances,
    /// into this frame's draws and ids.
    fn draw_list(&mut self, s: &Scene, c: Object3D) -> Result<Vec<u32>> {
        let (camera, world) = s.camera(c)?;
        let projection = camera.projection_matrix()?;
        let frustum = Frustum::from_projection(projection * world.inverse());
        let [_, _, _, opacity, sort, culled, custom] = self.params;
        let transparent = opacity < 1.;
        let eye = world.w_axis.truncate();
        let forward = world.transform_vector3(-Vector3::Z).normalize();
        let mut list = vec![];
        for (i, m) in self.matrices.iter().enumerate() {
            let matrix = Matrix4::from_cols_array(&m.map(f64::from));
            let range = &self.ranges[self.geometry_ids[i]];
            let scale = matrix
                .x_axis
                .truncate()
                .length_squared()
                .max(matrix.y_axis.truncate().length_squared())
                .max(matrix.z_axis.truncate().length_squared())
                .sqrt();
            let sphere = Sphere {
                center: matrix.transform_point3(range.sphere.center),
                radius: range.sphere.radius * scale,
            };
            if culled > 0.5 && !frustum.intersects_sphere(sphere) {
                continue;
            }
            list.push(((sphere.center - eye).dot(forward), i));
        }
        if sort > 0.5 {
            if custom > 0.5 {
                // radixSort on ( z × UINT32_MAX / far ) >>> 0, stable, reversed
                // for transparent materials.
                let far = match camera {
                    Camera::Perspective(p) => p.far,
                    Camera::Orthographic(o) => o.far,
                };
                let factor = 4294967295. / far;
                let key = |z: f64| {
                    let v = (z * factor).trunc();
                    (v.rem_euclid(4294967296.)) as u32
                };
                list.sort_by_key(|&(z, _)| key(z));
                if transparent {
                    list.reverse();
                }
            } else if transparent {
                list.sort_by(|a, b| b.0.total_cmp(&a.0));
            } else {
                list.sort_by(|a, b| a.0.total_cmp(&b.0));
            }
        }
        self.draws = list
            .iter()
            .map(|&(_, i)| {
                let r = &self.ranges[self.geometry_ids[i]];
                (r.start, r.count)
            })
            .collect();
        Ok(list.iter().map(|&(_, i)| i as u32).collect())
    }
    pub fn render(
        &mut self,
        r: &Renderer,
        s: &mut Scene,
        c: Object3D,
        out: &RenderTarget,
    ) -> Result<bool> {
        let samples = out.options.samples.max(1);
        if self.target.as_ref().is_none_or(|(t, _)| {
            t.width != out.width
                || t.height != out.height
                || t.options.samples != out.options.samples
        }) {
            let target =
                RenderTarget::with_options(&r.device, out.width, out.height, out.options.clone())?;
            let depth = r
                .device
                .create_texture(&wgpu::TextureDescriptor {
                    label: Some("batch depth"),
                    size: wgpu::Extent3d {
                        width: out.width,
                        height: out.height,
                        depth_or_array_layers: 1,
                    },
                    mip_level_count: 1,
                    sample_count: samples,
                    dimension: wgpu::TextureDimension::D2,
                    format: wgpu::TextureFormat::Depth24Plus,
                    usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                    view_formats: &[],
                })
                .create_view(&Default::default());
            self.target = Some((target, depth));
        }
        let format = out.options.format;
        if self
            .gpu
            .as_ref()
            .is_none_or(|g| g.format != format || g.samples != samples)
        {
            let layout = r
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: None,
                    bind_group_layouts: &[&self.layout],
                    push_constant_ranges: &[],
                });
            let make = |transparent: bool| {
                r.device
                    .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                        label: Some("mesh batch"),
                        layout: Some(&layout),
                        vertex: wgpu::VertexState {
                            module: &self.module,
                            entry_point: Some("vs"),
                            compilation_options: Default::default(),
                            buffers: &[wgpu::VertexBufferLayout {
                                array_stride: 24,
                                step_mode: wgpu::VertexStepMode::Vertex,
                                attributes: &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3],
                            }],
                        },
                        fragment: Some(wgpu::FragmentState {
                            module: &self.module,
                            entry_point: Some("fs"),
                            compilation_options: Default::default(),
                            targets: &[Some(wgpu::ColorTargetState {
                                format,
                                blend: transparent.then_some(wgpu::BlendState::ALPHA_BLENDING),
                                write_mask: wgpu::ColorWrites::ALL,
                            })],
                        }),
                        primitive: wgpu::PrimitiveState {
                            cull_mode: Some(wgpu::Face::Back),
                            ..Default::default()
                        },
                        depth_stencil: Some(wgpu::DepthStencilState {
                            format: wgpu::TextureFormat::Depth24Plus,
                            depth_write_enabled: !transparent,
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
            };
            self.gpu = Some(Gpu {
                format,
                samples,
                opaque: make(false),
                transparent: make(true),
            });
        }
        s.update()?;
        let ids = self.draw_list(s, c)?;
        let (camera, world) = s.camera(c)?;
        let view = world.inverse();
        let mut data = vec![];
        for m in [view, camera.projection_matrix()?, view] {
            data.extend(m.to_cols_array().map(|v| v as f32));
        }
        // modelNormalViewMatrix: the view's rotation, as a 3 × 3 in a mat4.
        for k in [3, 7, 11, 12, 13, 14] {
            data[32 + k] = 0.;
        }
        data[47] = 1.;
        data.extend([self.params[3] as f32, 0., 0., 0.]);
        r.queue
            .write_buffer(&self.uniforms, 0, bytemuck::cast_slice(&data));
        if std::mem::take(&mut self.matrices_dirty) {
            r.queue
                .write_buffer(&self.matrix_buffer, 0, bytemuck::cast_slice(&self.matrices));
        }
        if std::mem::take(&mut self.colors_dirty) {
            r.queue
                .write_buffer(&self.color_buffer, 0, bytemuck::cast_slice(&self.colors));
        }
        if !ids.is_empty() {
            r.queue
                .write_buffer(&self.id_buffer, 0, bytemuck::cast_slice(&ids));
        }
        let (target, depth) = self.target.as_ref().ok_or(Error::Invalid("batch target"))?;
        let gpu = self.gpu.as_ref().ok_or(Error::Invalid("batch pipeline"))?;
        let background = s.background.0;
        let (view, resolve) = if samples > 1 {
            (
                target
                    .multisampled_views
                    .first()
                    .ok_or(Error::Invalid("batch MSAA view"))?,
                Some(&target.view),
            )
        } else {
            (&target.view, None)
        };
        let mut encoder = r.device.create_command_encoder(&Default::default());
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("mesh batch"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view,
                    depth_slice: None,
                    resolve_target: resolve,
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
                    view: depth,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.),
                        store: wgpu::StoreOp::Discard,
                    }),
                    stencil_ops: None,
                }),
                ..Default::default()
            });
            pass.set_pipeline(if self.params[3] < 1. {
                &gpu.transparent
            } else {
                &gpu.opaque
            });
            pass.set_bind_group(0, &self.bind, &[]);
            pass.set_vertex_buffer(0, self.vertices.slice(..));
            pass.set_index_buffer(self.indices.slice(..), wgpu::IndexFormat::Uint16);
            for (i, &(start, count)) in self.draws.iter().enumerate() {
                pass.draw_indexed(start..start + count, 0, i as u32..i as u32 + 1);
            }
        }
        r.queue.submit([encoder.finish()]);
        Ok(true)
    }
    pub fn output(&self) -> Option<&RenderTarget> {
        self.target.as_ref().map(|(t, _)| t)
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
    /// webgpu, count, dynamic, opacity, sortObjects, perObjectFrustumCulled,
    /// useCustomSort and the randomize geometry button.
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        let value = value as f64;
        match index {
            0 => {
                self.params[0] = value;
                self.rebuild = true;
            }
            1 => {
                self.params[1] = value.round().clamp(1., MAX_COUNT as f64);
                self.init_mesh();
            }
            2..=6 => self.params[index] = value,
            7 => {
                for i in 0..self.geometry_ids.len() {
                    self.geometry_ids[i] =
                        (self.random() * self.ranges.len() as f64).floor() as usize;
                }
            }
            _ => return Err(Error::Invalid("mesh batch parameter")),
        }
        Ok(())
    }
    /// A still frame: one animate() call.
    pub fn seek(&mut self, _t: f64) {
        self.pending = true;
    }
}
