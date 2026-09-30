//! webgl_test_memory2: 100 spheres sharing one geometry whose ShaderMaterials
//! are replaced every frame by new ones, each fragment shader compiled with
//! its own random color baked in, then disposed after the render. As the
//! original compiles 100 programs per frame, the port creates 100 shader
//! modules and pipelines per frame and drops them after the frame; the
//! sphere geometry and the spheres' matrices stay resident.
use crate::{
    Error, Result, camera::*, geometry::*, math::*, render_target::*, renderer::*, scene::*,
};

const COUNT: usize = 100;
/// The page's vertex shader and its fragment shader with XXX replaced:
/// every fourth row and column of WebGL's bottom-up gl_FragCoord gets the
/// color, the rest white.
fn source(color: [f64; 3]) -> String {
    format!(
        "struct U{{mvp:array<mat4x4<f32>,{COUNT}>,height:vec4<f32>}}
@group(0) @binding(0) var<uniform> u:U;
@vertex fn vs(@builtin(instance_index) i:u32,@location(0) position:vec3<f32>)->@builtin(position) vec4<f32>{{return u.mvp[i]*vec4(position,1.0);}}
fn glsl_mod(x:f32,y:f32)->f32{{return x-y*floor(x/y);}}
@fragment fn fs(@builtin(position) p:vec4<f32>)->@location(0) vec4<f32>{{
 let y=u.height.x-p.y;
 if glsl_mod(p.x,4.0001)<1.0 || glsl_mod(y,4.0001)<1.0 {{return vec4(vec3<f32>({:?},{:?},{:?}),1.0);}}
 return vec4(1.0);}}",
        color[0] as f32, color[1] as f32, color[2] as f32
    )
}
pub(super) struct Demo {
    seed: u32,
    positions: Vec<Vector3>,
    /// This frame's colors, one compiled program each.
    colors: Vec<[f64; 3]>,
    pending: bool,
    vertices: wgpu::Buffer,
    indices: wgpu::Buffer,
    index_count: u32,
    uniforms: wgpu::Buffer,
    bind: wgpu::BindGroup,
    layout: wgpu::PipelineLayout,
    target: Option<(RenderTarget, wgpu::TextureView)>,
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, _id: u32, r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 40.,
            near: 1.,
            far: 10000.,
            aspect,
            ..Default::default()
        }));
        let n = s.get_mut(c)?;
        n.position = Vector3::new(0., 0., 2000.);
        n.quaternion = Quaternion::IDENTITY;
        s.background = Color::WHITE;
        let g = SphereGeometry::build(15., 64, 32)?;
        let vertices: Vec<f32> = g
            .positions()?
            .iter()
            .flat_map(|p| [p.x as f32, p.y as f32, p.z as f32])
            .collect();
        let indices = g.index.clone().ok_or(Error::Invalid("sphere index"))?;
        use wgpu::util::DeviceExt;
        let vertex_buffer = r
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("memory spheres"),
                contents: bytemuck::cast_slice(&vertices),
                usage: wgpu::BufferUsages::VERTEX,
            });
        let index_buffer = r
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("memory sphere index"),
                contents: bytemuck::cast_slice(&indices),
                usage: wgpu::BufferUsages::INDEX,
            });
        let uniforms = r.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("memory matrices"),
            size: (COUNT * 64 + 16) as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let group = r
            .device
            .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: None,
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });
        let bind = r.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &group,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniforms.as_entire_binding(),
            }],
        });
        let layout = r
            .device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[&group],
                push_constant_ranges: &[],
            });
        let mut demo = Self {
            seed: 186,
            positions: vec![],
            colors: vec![],
            pending: false,
            vertices: vertex_buffer,
            indices: index_buffer,
            index_count: indices.len() as u32,
            uniforms,
            bind,
            layout,
            target: None,
        };
        // init(): each mesh's first material, then its position.
        for _ in 0..COUNT {
            demo.color();
            let p = Vector3::new(
                (0.5 - demo.random()) * 1000.,
                (0.5 - demo.random()) * 1000.,
                (0.5 - demo.random()) * 1000.,
            );
            demo.positions.push(p);
        }
        demo.frame();
        Ok(demo)
    }
    fn random(&mut self) -> f64 {
        self.seed = self.seed.wrapping_mul(1664525).wrapping_add(1013904223);
        self.seed as f64 / 4294967296.
    }
    /// generateFragmentShader(): Math.random() for each of r, g and b.
    fn color(&mut self) -> [f64; 3] {
        [self.random(), self.random(), self.random()]
    }
    /// render(): a new material, and program, for every mesh.
    fn frame(&mut self) {
        self.colors = (0..COUNT).map(|_| self.color()).collect();
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, _dt: f64, animate: bool) -> Result<()> {
        if animate {
            self.frame();
        }
        Ok(())
    }
    pub fn prepare(&mut self, _s: &mut Scene, _c: Object3D, _r: &Renderer) -> Result<()> {
        if std::mem::take(&mut self.pending) {
            self.frame();
        }
        Ok(())
    }
    pub fn render(
        &mut self,
        r: &Renderer,
        s: &mut Scene,
        c: Object3D,
        out: &RenderTarget,
    ) -> Result<bool> {
        if self
            .target
            .as_ref()
            .is_none_or(|(t, _)| t.width != out.width || t.height != out.height)
        {
            let target =
                RenderTarget::with_options(&r.device, out.width, out.height, out.options.clone())?;
            let depth = r
                .device
                .create_texture(&wgpu::TextureDescriptor {
                    label: Some("memory depth"),
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
            self.target = Some((target, depth));
        }
        let (target, depth) = self
            .target
            .as_ref()
            .ok_or(Error::Invalid("memory target"))?;
        s.update()?;
        let (camera, world) = s.camera(c)?;
        let view_projection = camera.projection_matrix()? * world.inverse();
        let mut data = vec![];
        for p in &self.positions {
            data.extend(
                (view_projection * Matrix4::from_translation(*p))
                    .to_cols_array()
                    .map(|v| v as f32),
            );
        }
        data.extend([out.height as f32, 0., 0., 0.]);
        r.queue
            .write_buffer(&self.uniforms, 0, bytemuck::cast_slice(&data));
        let format = target.options.format;
        // The frame's programs: compiled for this render, disposed after it.
        let pipelines: Vec<wgpu::RenderPipeline> = self
            .colors
            .iter()
            .map(|&color| {
                let module = r.device.create_shader_module(wgpu::ShaderModuleDescriptor {
                    label: Some("memory material"),
                    source: wgpu::ShaderSource::Wgsl(source(color).into()),
                });
                r.device
                    .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                        label: Some("memory material"),
                        layout: Some(&self.layout),
                        vertex: wgpu::VertexState {
                            module: &module,
                            entry_point: Some("vs"),
                            compilation_options: Default::default(),
                            buffers: &[wgpu::VertexBufferLayout {
                                array_stride: 12,
                                step_mode: wgpu::VertexStepMode::Vertex,
                                attributes: &wgpu::vertex_attr_array![0 => Float32x3],
                            }],
                        },
                        fragment: Some(wgpu::FragmentState {
                            module: &module,
                            entry_point: Some("fs"),
                            compilation_options: Default::default(),
                            targets: &[Some(format.into())],
                        }),
                        primitive: wgpu::PrimitiveState {
                            cull_mode: Some(wgpu::Face::Back),
                            ..Default::default()
                        },
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
                    })
            })
            .collect();
        let mut encoder = r.device.create_command_encoder(&Default::default());
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("memory spheres"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &target.view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::WHITE),
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
            pass.set_bind_group(0, &self.bind, &[]);
            pass.set_vertex_buffer(0, self.vertices.slice(..));
            pass.set_index_buffer(self.indices.slice(..), wgpu::IndexFormat::Uint32);
            for (i, pipeline) in pipelines.iter().enumerate() {
                pass.set_pipeline(pipeline);
                pass.draw_indexed(0..self.index_count, 0, i as u32..i as u32 + 1);
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
        Err(Error::Invalid("test memory parameter"))
    }
    /// A still frame: one render() call of the interval.
    pub fn seek(&mut self, _t: f64) {
        self.pending = true;
    }
}
