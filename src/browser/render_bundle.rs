//! webgpu_performance_renderbundle: 4,000 meshes of fifteen primitive
//! geometries, each with its own random MeshToonNodeMaterial color and
//! DoubleSide, and a ten-instance InstancedMesh, in a BundleGroup under a
//! directional light, with autoRotating OrbitControls. As three records the
//! BundleGroup, the port records every draw once into a WebGPU render bundle
//! and replays it each frame. The `dynamic` switch turns the objects and
//! toggles the group's `static` flag: a static group's per-object data is
//! not refreshed, a non-static one's is uploaded each frame before the
//! replay. Without `render bundle` the draws are encoded directly.
use super::controls_attributes::{Controls, camera_state};
use crate::{
    Error, Result, camera::*, geometry::*, math::*, render_target::*, renderer::*, scene::*,
};
use std::f64::consts::{PI, TAU};

const COUNT: usize = 4000;
const INSTANCES: usize = 10;
/// MeshToonNodeMaterial under one DirectionalLight: getGradientIrradiance
/// without a gradient map, × the light color, × BRDF_Lambert. Objects are
/// ( model, color, flags.x = double sided ); the instanced cone reads its
/// instance matrices after the objects.
const SHADER: &str = "struct U{view:mat4x4<f32>,projection:mat4x4<f32>,light:vec4<f32>}
struct Object{model:mat4x4<f32>,color:vec4<f32>,flags:vec4<f32>}
@group(0) @binding(0) var<uniform> u:U;
@group(0) @binding(1) var<storage,read> objects:array<Object>;
@group(0) @binding(2) var<storage,read> instances:array<mat4x4<f32>>;
struct O{@builtin(position) clip:vec4<f32>,@location(0) normal:vec3<f32>,@location(1) @interpolate(flat) id:u32}
@vertex fn vs(@builtin(instance_index) index:u32,@location(0) position:vec3<f32>,@location(1) normal:vec3<f32>)->O{
 var id=index;var p=position;var n=normal;
 if index>=4000u {id=4000u;let m=instances[index-4000u];let b=mat3x3(m[0].xyz,m[1].xyz,m[2].xyz);
  p=(m*vec4(position,1.0)).xyz;n=b*(normal/vec3(dot(b[0],b[0]),dot(b[1],b[1]),dot(b[2],b[2])));}
 let mv=u.view*objects[id].model;let nm=mat3x3(mv[0].xyz,mv[1].xyz,mv[2].xyz);
 return O(u.projection*mv*vec4(p,1.0),normalize(nm*n),id);}
@fragment fn fs(i:O,@builtin(front_facing) front:bool)->@location(0) vec4<f32>{
 let o=objects[i.id];var n=normalize(i.normal);if o.flags.x>0.5 && !front {n=-n;}
 let coord=vec2(dot(n,u.light.xyz)*0.5+0.5,0.0);let fw=fwidth(coord)*0.5;
 let gradient=mix(vec3(0.7),vec3(1.0),smoothstep(0.7-fw.x,0.7+fw.x,coord.x));
 return vec4(gradient*u.light.w*o.color.rgb*(1.0/3.141592653589793),1.0);}";

/// One geometry's range in the shared buffers: indexed ( first index,
/// count ) or not ( first vertex, count ).
#[derive(Clone, Copy)]
struct Range {
    indexed: bool,
    first: u32,
    count: u32,
}
struct Gpu {
    format: wgpu::TextureFormat,
    samples: u32,
    double: wgpu::RenderPipeline,
    front: wgpu::RenderPipeline,
}
pub(super) struct Demo {
    seed: u32,
    controls: Controls,
    /// render bundle, webgpu, dynamic.
    params: [f64; 3],
    ranges: Vec<Range>,
    /// Per mesh: position, rotation (Euler XYZ), scale and rotation speed.
    meshes: Vec<(Vector3, Vector3, f64, Vector3)>,
    colors: Vec<[f32; 4]>,
    /// The instanced cone's Float32 matrices and rotation speeds.
    instance_matrices: Vec<[f32; 16]>,
    instance_speeds: Vec<Vector3>,
    dirty: bool,
    /// BundleGroup.static: its objects' data is not refreshed.
    static_group: bool,
    /// Frames until a resized group's objects are refreshed: the original's
    /// re-recorded group shows its objects' current data from the second
    /// frame after the resize.
    refresh: u8,
    pending: bool,
    vertices: wgpu::Buffer,
    indices: wgpu::Buffer,
    uniforms: wgpu::Buffer,
    objects: wgpu::Buffer,
    instances: wgpu::Buffer,
    bind: wgpu::BindGroup,
    layout: wgpu::BindGroupLayout,
    module: wgpu::ShaderModule,
    gpu: Option<Gpu>,
    target: Option<(RenderTarget, wgpu::TextureView)>,
    bundle: Option<wgpu::RenderBundle>,
}
fn geometries() -> Result<Vec<BufferGeometry>> {
    let t = (1. + 5f64.sqrt()) / 2.;
    let r = 1. / t;
    let dodecahedron = [
        [-1., -1., -1.],
        [-1., -1., 1.],
        [-1., 1., -1.],
        [-1., 1., 1.],
        [1., -1., -1.],
        [1., -1., 1.],
        [1., 1., -1.],
        [1., 1., 1.],
        [0., -r, -t],
        [0., -r, t],
        [0., r, -t],
        [0., r, t],
        [-r, -t, 0.],
        [-r, t, 0.],
        [r, -t, 0.],
        [r, t, 0.],
        [-t, 0., -r],
        [t, 0., -r],
        [-t, 0., r],
        [t, 0., r],
    ]
    .map(Vector3::from_array);
    let faces = [
        3, 11, 7, 3, 7, 15, 3, 15, 13, 7, 19, 17, 7, 17, 6, 7, 6, 15, 17, 4, 8, 17, 8, 10, 17, 10,
        6, 8, 0, 16, 8, 16, 2, 8, 2, 10, 0, 12, 1, 0, 1, 18, 0, 18, 16, 6, 10, 2, 6, 2, 13, 6, 13,
        15, 2, 16, 18, 2, 18, 3, 2, 3, 13, 18, 1, 9, 18, 9, 11, 18, 11, 3, 4, 14, 12, 4, 12, 0, 4,
        0, 8, 11, 9, 5, 11, 5, 19, 11, 19, 7, 19, 5, 14, 19, 14, 4, 19, 4, 17, 1, 12, 14, 1, 14, 5,
        1, 5, 9,
    ];
    // PolyhedronGeometry( [ 0, 0, 0 ], [ 0, 0, 0 ] ): one degenerate triangle
    // at the origin.
    let degenerate = {
        let mut g = BufferGeometry::default();
        g.set_attribute(
            "position",
            Attribute::F32(crate::attribute::BufferAttribute::new(
                vec![0.; 9],
                3,
                false,
            )?),
        );
        g.set_attribute(
            "normal",
            Attribute::F32(crate::attribute::BufferAttribute::new(
                vec![0.; 9],
                3,
                false,
            )?),
        );
        g
    };
    Ok(vec![
        CylinderGeometry::build(0., 1., 2., 3, 1, false, 0., TAU)?,
        BoxGeometry::build(2., 2., 2.)?,
        PlaneGeometry::build(2., 2., 1, 1)?,
        CapsuleGeometry::build(1., 1., 4, 8, 1)?,
        CircleGeometry::build(1., 3, 0., TAU)?,
        CylinderGeometry::build(1., 1., 2., 3, 1, false, 0., TAU)?,
        PolyhedronGeometry::build(&dodecahedron, &faces, 1., 0)?,
        IcosahedronGeometry::build(1., 0)?,
        OctahedronGeometry::build(1., 0)?,
        degenerate,
        RingGeometry::build(1., 1.5, 3, 1, 0., TAU)?,
        SphereGeometry::build(1., 3, 2)?,
        TetrahedronGeometry::build(1., 0)?,
        TorusGeometry::build(1., 0.5, 3, 3, TAU, 0., TAU)?,
        TorusKnotGeometry::build(1., 0.5, 20, 3, 1, 1)?,
    ])
}
/// Euler.setFromQuaternion( q, 'XYZ' ): the angles child.rotation holds
/// after matrix.decompose().
fn euler_xyz(q: Quaternion) -> Vector3 {
    let m = Matrix4::from_quat(q);
    let (m11, m12, m13) = (m.x_axis.x, m.y_axis.x, m.z_axis.x);
    let (m22, m23) = (m.y_axis.y, m.z_axis.y);
    let (m32, m33) = (m.y_axis.z, m.z_axis.z);
    let y = m13.clamp(-1., 1.).asin();
    if m13.abs() < 0.9999999 {
        Vector3::new((-m23).atan2(m33), y, (-m12).atan2(m11))
    } else {
        Vector3::new(m32.atan2(m22), y, 0.)
    }
}
fn euler_matrix(p: Vector3, r: Vector3, s: f64) -> Matrix4 {
    let q = Euler {
        angles: r,
        order: EulerOrder::XYZ,
    }
    .quaternion();
    Matrix4::from_scale_rotation_translation(Vector3::splat(s), q, p)
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, _id: u32, r: &Renderer) -> Result<Self> {
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
        n.position = Vector3::new(0., 0., 50.);
        n.quaternion = Quaternion::IDENTITY;
        s.background = Color::from_hex(0xc1c1c1);
        let mut controls = Controls::new(None, (0., f64::INFINITY), PI, true);
        controls.auto_rotate = Some(1.);
        let (mut vertices, mut indices, mut ranges) = (vec![], vec![], vec![]);
        for g in geometries()? {
            let base = (vertices.len() / 6) as u32;
            let positions = g.positions()?;
            let normal = g
                .attributes
                .get("normal")
                .ok_or(Error::Invalid("bundle normals"))?;
            for (i, p) in positions.iter().enumerate() {
                let n = normal.vector3(i)?;
                vertices.extend([p.x, p.y, p.z, n.x, n.y, n.z].map(|v| v as f32));
            }
            ranges.push(match &g.index {
                Some(index) => {
                    let first = indices.len() as u32;
                    indices.extend(index.iter().map(|&i| i + base));
                    Range {
                        indexed: true,
                        first,
                        count: index.len() as u32,
                    }
                }
                None => Range {
                    indexed: false,
                    first: base,
                    count: positions.len() as u32,
                },
            });
        }
        use wgpu::BufferUsages as B;
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
            "bundle vertices",
            bytemuck::cast_slice(&vertices),
            B::VERTEX,
        );
        let index_buffer = init("bundle indices", bytemuck::cast_slice(&indices), B::INDEX);
        let make = |label, size: usize, usage| {
            r.device.create_buffer(&wgpu::BufferDescriptor {
                label: Some(label),
                size: size as u64,
                usage: usage | B::COPY_DST,
                mapped_at_creation: false,
            })
        };
        let uniforms = make("bundle uniforms", 144, B::UNIFORM);
        let objects = make("bundle objects", (COUNT + 1) * 96, B::STORAGE);
        let instances = make("bundle instances", INSTANCES * 64, B::STORAGE);
        let entry = |binding, ty| wgpu::BindGroupLayoutEntry {
            binding,
            visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
            ty: wgpu::BindingType::Buffer {
                ty,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        };
        let storage = wgpu::BufferBindingType::Storage { read_only: true };
        let layout = r
            .device
            .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: None,
                entries: &[
                    entry(0, wgpu::BufferBindingType::Uniform),
                    entry(1, storage),
                    entry(2, storage),
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
                    resource: objects.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: instances.as_entire_binding(),
                },
            ],
        });
        let module = r.device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("render bundle"),
            source: wgpu::ShaderSource::Wgsl(SHADER.into()),
        });
        let mut demo = Self {
            seed: 186,
            controls,
            params: [1., 1., 0.],
            ranges,
            meshes: vec![],
            colors: vec![],
            instance_matrices: vec![],
            instance_speeds: vec![],
            dirty: true,
            static_group: true,
            refresh: 0,
            pending: false,
            vertices: vertex_buffer,
            indices: index_buffer,
            uniforms,
            objects,
            instances,
            bind,
            layout,
            module,
            gpu: None,
            target: None,
            bundle: None,
        };
        demo.init_mesh();
        demo.controls.update(s, c)?;
        // The first animate() of the original's loop.
        demo.step(s, c)?;
        Ok(demo)
    }
    fn random(&mut self) -> f64 {
        self.seed = self.seed.wrapping_mul(1664525).wrapping_add(1013904223);
        self.seed as f64 / 4294967296.
    }
    /// randomizeMatrix(): position, Euler rotation and uniform scale.
    fn random_transform(&mut self) -> (Vector3, Vector3, f64) {
        let p = Vector3::new(
            self.random() * 80. - 40.,
            self.random() * 80. - 40.,
            self.random() * 80. - 40.,
        );
        let r = Vector3::new(
            self.random() * TAU,
            self.random() * TAU,
            self.random() * TAU,
        );
        (p, r, 0.35 + self.random() * 0.5)
    }
    fn random_speed(&mut self) -> Vector3 {
        Vector3::new(
            self.random() * 0.05,
            self.random() * 0.05,
            self.random() * 0.05,
        )
    }
    /// initMesh(): per mesh its color, then its matrix and rotation speed;
    /// then the instanced cone's matrices and speeds.
    fn init_mesh(&mut self) {
        for _ in 0..COUNT {
            let hex = (self.random() * 0xffffff as f64).floor() as u32;
            let c = Color::from_hex(hex).0;
            self.colors.push([c.x as f32, c.y as f32, c.z as f32, 1.]);
            let (p, r, s) = self.random_transform();
            let speed = self.random_speed();
            let q = Euler {
                angles: r,
                order: EulerOrder::XYZ,
            }
            .quaternion();
            self.meshes.push((p, euler_xyz(q), s, speed));
        }
        for _ in 0..INSTANCES {
            let (p, r, s) = self.random_transform();
            self.instance_matrices
                .push(euler_matrix(p, r, s).to_cols_array().map(|v| v as f32));
            let speed = self.random_speed();
            self.instance_speeds.push(speed);
        }
        // The InstancedMesh's MeshToonNodeMaterial is white.
        self.colors.push([1.; 4]);
    }
    /// animate(): animateMeshes() when dynamic, then controls.update().
    fn step(&mut self, s: &mut Scene, c: Object3D) -> Result<()> {
        if self.params[2] > 0.5 {
            for m in &mut self.meshes {
                m.1 += m.3;
            }
            for (matrix, speed) in self.instance_matrices.iter_mut().zip(&self.instance_speeds) {
                let rotation = Matrix4::from_quat(
                    Euler {
                        angles: *speed,
                        order: EulerOrder::XYZ,
                    }
                    .quaternion(),
                );
                let m = Matrix4::from_cols_array(&matrix.map(f64::from)) * rotation;
                *matrix = m.to_cols_array().map(|v| v as f32);
            }
        }
        if self.refresh > 0 {
            self.refresh -= 1;
            if self.refresh == 0 {
                self.dirty = true;
            }
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
        if std::mem::take(&mut self.pending) {
            self.step(s, c)?;
        }
        Ok(())
    }
    fn object_data(&self) -> Vec<f32> {
        let mut data = Vec::with_capacity((COUNT + 1) * 24);
        for (i, color) in self.colors.iter().enumerate() {
            let (model, double) = match self.meshes.get(i) {
                Some(&(p, r, s, _)) => (euler_matrix(p, r, s), 1.),
                None => (Matrix4::IDENTITY, 0.),
            };
            data.extend(model.to_cols_array().map(|v| v as f32));
            data.extend(color);
            data.extend([double, 0., 0., 0.]);
        }
        data
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
            t.width != out.width || t.height != out.height || t.options.samples != samples
        }) {
            let target =
                RenderTarget::with_options(&r.device, out.width, out.height, out.options.clone())?;
            let depth = r
                .device
                .create_texture(&wgpu::TextureDescriptor {
                    label: Some("bundle depth"),
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
            // onWindowResize(): group.needsUpdate = true — the bundle is
            // recorded again and every object refreshed.
            self.bundle = None;
            self.refresh = 2;
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
            let make = |cull: Option<wgpu::Face>| {
                r.device
                    .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                        label: Some("render bundle toon"),
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
                            targets: &[Some(format.into())],
                        }),
                        primitive: wgpu::PrimitiveState {
                            cull_mode: cull,
                            ..Default::default()
                        },
                        depth_stencil: Some(wgpu::DepthStencilState {
                            format: wgpu::TextureFormat::Depth24Plus,
                            depth_write_enabled: true,
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
                double: make(None),
                front: make(Some(wgpu::Face::Back)),
            });
            self.bundle = None;
        }
        if std::mem::take(&mut self.dirty) || !self.static_group {
            r.queue
                .write_buffer(&self.objects, 0, bytemuck::cast_slice(&self.object_data()));
            r.queue.write_buffer(
                &self.instances,
                0,
                bytemuck::cast_slice(&self.instance_matrices),
            );
        }
        s.update()?;
        let (camera, world) = s.camera(c)?;
        let view = world.inverse();
        // The DirectionalLight at ( 0, 1, 0 ) toward the origin, in view space,
        // and its intensity.
        let light = view.transform_vector3(Vector3::Y).normalize();
        let mut data = vec![];
        for m in [view, camera.projection_matrix()?] {
            data.extend(m.to_cols_array().map(|v| v as f32));
        }
        data.extend([light.x as f32, light.y as f32, light.z as f32, 3.4]);
        r.queue
            .write_buffer(&self.uniforms, 0, bytemuck::cast_slice(&data));
        let gpu = self.gpu.as_ref().ok_or(Error::Invalid("bundle pipeline"))?;
        let use_bundle = self.params[0] > 0.5;
        // BundleGroup: recorded once; its version never changes.
        if use_bundle && self.bundle.is_none() {
            let mut encoder =
                r.device
                    .create_render_bundle_encoder(&wgpu::RenderBundleEncoderDescriptor {
                        label: Some("bundle group"),
                        color_formats: &[Some(format)],
                        depth_stencil: Some(wgpu::RenderBundleDepthStencil {
                            format: wgpu::TextureFormat::Depth24Plus,
                            depth_read_only: false,
                            stencil_read_only: true,
                        }),
                        sample_count: samples,
                        multiview: None,
                    });
            self.record(&mut encoder, gpu);
            self.bundle = Some(encoder.finish(&wgpu::RenderBundleDescriptor {
                label: Some("bundle group"),
            }));
        }
        let (target, depth) = self
            .target
            .as_ref()
            .ok_or(Error::Invalid("bundle target"))?;
        let background = s.background.0;
        let (view, resolve) = if samples > 1 {
            (
                target
                    .multisampled_views
                    .first()
                    .ok_or(Error::Invalid("bundle MSAA view"))?,
                Some(&target.view),
            )
        } else {
            (&target.view, None)
        };
        let mut encoder = r.device.create_command_encoder(&Default::default());
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("render bundle"),
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
            match (&self.bundle, use_bundle) {
                (Some(bundle), true) => pass.execute_bundles([bundle]),
                _ => self.record(&mut pass, gpu),
            }
        }
        r.queue.submit([encoder.finish()]);
        Ok(true)
    }
    /// The group's draws: each mesh with its own object index, then the
    /// instanced cone.
    fn record<'a>(&'a self, pass: &mut impl wgpu::util::RenderEncoder<'a>, gpu: &'a Gpu) {
        pass.set_bind_group(0, Some(&self.bind), &[]);
        pass.set_vertex_buffer(0, self.vertices.slice(..));
        pass.set_index_buffer(self.indices.slice(..), wgpu::IndexFormat::Uint32);
        pass.set_pipeline(&gpu.double);
        for i in 0..COUNT {
            let range = self.ranges[i % self.ranges.len()];
            let id = i as u32;
            if range.indexed {
                pass.draw_indexed(range.first..range.first + range.count, 0, id..id + 1);
            } else {
                pass.draw(range.first..range.first + range.count, id..id + 1);
            }
        }
        pass.set_pipeline(&gpu.front);
        let cone = self.ranges[0];
        let first = COUNT as u32;
        pass.draw_indexed(
            cone.first..cone.first + cone.count,
            0,
            first..first + INSTANCES as u32,
        );
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
    /// render bundle, webgpu and dynamic.
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        *self
            .params
            .get_mut(index)
            .ok_or(Error::Invalid("render bundle parameter"))? = value as f64;
        // The dynamic handler: group.static = ! group.static.
        if index == 2 {
            self.static_group = !self.static_group;
        }
        Ok(())
    }
    /// A still frame: one animate() call.
    pub fn seek(&mut self, _t: f64) {
        self.pending = true;
    }
}
