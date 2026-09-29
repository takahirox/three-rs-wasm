//! webgpu_reversed_depth_buffer: five pairs of nearly coplanar red and green
//! planes, receding to z = −3200 and swinging about a turning axis, drawn by
//! three renderers side by side — a normal Depth24Plus z-buffer, a
//! logarithmic one (fragment depth from the view z) and a reversed Depth32Float
//! one (reversed projection, cleared to 0, GreaterEqual) — each its own
//! pass( scene, camera ) viewport of 0.33 × the window width.
use crate::{Error, Result, math::*, render_target::*, renderer::*, scene::*};

/// MeshBasicMaterial( { vertexColors } ): positionView = ( cameraViewMatrix ×
/// modelWorldMatrix × position ).xyz, clip = cameraProjectionMatrix ×
/// positionView. The logarithmic variant writes viewZToLogarithmicDepth.
const SHADER: &str = "struct U{view:mat4x4<f32>,model:mat4x4<f32>,projection:mat4x4<f32>,clip:vec4<f32>}
@group(0) @binding(0) var<uniform> u:U;
struct O{@builtin(position) clip:vec4<f32>,@location(0) color:vec3<f32>,@location(1) view:vec3<f32>}
@vertex fn vs(@location(0) position:vec3<f32>,@location(1) color:vec3<f32>)->O{let mv=u.view*u.model;let view=(mv*vec4(position,1.0)).xyz;return O(u.projection*vec4(view,1.0),color,view);}
@fragment fn fs(i:O)->@location(0) vec4<f32>{return max(vec4(i.color,1.0),vec4(0.0));}
struct L{@location(0) color:vec4<f32>,@builtin(frag_depth) depth:f32}
@fragment fn fs_log(i:O)->L{let near=max(u.clip.x,1e-6);return L(max(vec4(i.color,1.0),vec4(0.0)),log2(-i.view.z/near)/log2(u.clip.y/near));}";
const COUNT: usize = 5;
const SLOT: u64 = 256;
const NEAR: f64 = 5.;
const FAR: f64 = 9999.;

/// One render pass: its depth view, depth clear, color load and the
/// ( view, pipeline ) draws.
type Pass<'a> = (
    &'a wgpu::TextureView,
    f32,
    wgpu::LoadOp<wgpu::Color>,
    &'a [(usize, &'a wgpu::RenderPipeline)],
);
struct Pipelines {
    format: wgpu::TextureFormat,
    normal: wgpu::RenderPipeline,
    logarithmic: wgpu::RenderPipeline,
    reversed: wgpu::RenderPipeline,
}
pub(super) struct Demo {
    vertices: wgpu::Buffer,
    uniforms: wgpu::Buffer,
    layout: wgpu::BindGroupLayout,
    bind: wgpu::BindGroup,
    module: wgpu::ShaderModule,
    pipelines: Option<Pipelines>,
    target: Option<(RenderTarget, wgpu::TextureView, wgpu::TextureView)>,
    /// Each mesh's position and scale.
    placements: Vec<(Vector3, f64)>,
    time: f64,
}
/// makePerspective( … ) of fov 72 at the given aspect in WebGPU clip space,
/// or reversed ( c = near / ( far − near ), d = far × near / ( far − near ) ).
fn projection(aspect: f64, reversed: bool) -> Matrix4 {
    let top = NEAR * (72f64.to_radians() * 0.5).tan();
    let height = 2. * top;
    let width = aspect * height;
    let left = -0.5 * width;
    let (right, bottom) = (left + width, top - height);
    let x = 2. * NEAR / (right - left);
    let y = 2. * NEAR / (top - bottom);
    let a = (right + left) / (right - left);
    let b = (top + bottom) / (top - bottom);
    let (c, d) = if reversed {
        (NEAR / (FAR - NEAR), FAR * NEAR / (FAR - NEAR))
    } else {
        (-FAR / (FAR - NEAR), -FAR * NEAR / (FAR - NEAR))
    };
    Matrix4::from_cols_array(&[x, 0., 0., 0., 0., y, 0., 0., a, b, c, -1., 0., 0., d, 0.])
}
fn pipeline(
    r: &Renderer,
    module: &wgpu::ShaderModule,
    layout: &wgpu::PipelineLayout,
    format: wgpu::TextureFormat,
    fragment: &str,
    depth: wgpu::TextureFormat,
    compare: wgpu::CompareFunction,
) -> wgpu::RenderPipeline {
    r.device
        .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("reversed depth planes"),
            layout: Some(layout),
            vertex: wgpu::VertexState {
                module,
                entry_point: Some("vs"),
                compilation_options: Default::default(),
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: 24,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3],
                }],
            },
            fragment: Some(wgpu::FragmentState {
                module,
                entry_point: Some(fragment),
                compilation_options: Default::default(),
                targets: &[Some(format.into())],
            }),
            primitive: wgpu::PrimitiveState {
                cull_mode: Some(wgpu::Face::Back),
                ..Default::default()
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: depth,
                depth_write_enabled: true,
                depth_compare: compare,
                stencil: Default::default(),
                bias: Default::default(),
            }),
            multisample: Default::default(),
            multiview: None,
            cache: None,
        })
}
fn depth(r: &Renderer, width: u32, height: u32, format: wgpu::TextureFormat) -> wgpu::TextureView {
    r.device
        .create_texture(&wgpu::TextureDescriptor {
            label: Some("reversed depth buffer"),
            size: wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        })
        .create_view(&Default::default())
}
impl Demo {
    pub async fn create(_s: &mut Scene, _c: Object3D, _id: u32, r: &Renderer) -> Result<Self> {
        let (d, o) = (0.0001f32, 0.5f32);
        let mut data = vec![];
        for (dz, dx, color) in [(d, -o, [1f32, 0., 0.]), (-d, o, [0., 1., 0.])] {
            for (x, y) in [
                (-1., -1.),
                (1., -1.),
                (-1., 1.),
                (1., -1.),
                (1., 1.),
                (-1., 1.),
            ] {
                data.extend([x + dx, y, dz]);
                data.extend(color);
            }
        }
        use wgpu::util::DeviceExt;
        let vertices = r
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("reversed depth planes"),
                contents: bytemuck::cast_slice(&data),
                usage: wgpu::BufferUsages::VERTEX,
            });
        let uniforms = r.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("reversed depth uniforms"),
            size: SLOT * 3 * COUNT as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let layout = r
            .device
            .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: None,
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: true,
                        min_binding_size: wgpu::BufferSize::new(208),
                    },
                    count: None,
                }],
            });
        let bind = r.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                    buffer: &uniforms,
                    offset: 0,
                    size: wgpu::BufferSize::new(208),
                }),
            }],
        });
        let module = r.device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("reversed depth"),
            source: wgpu::ShaderSource::Wgsl(SHADER.into()),
        });
        // xCount 1, yCount 5: z = −800 i, scale 1 + 50 i.
        let placements = (0..COUNT)
            .map(|i| {
                let z = -800. * i as f64;
                let y = i as f64;
                (
                    Vector3::new(0., (4. - 0.2 * z) * (y - 2.5 + 1.), z),
                    1. + 50. * i as f64,
                )
            })
            .collect();
        Ok(Self {
            vertices,
            uniforms,
            layout,
            bind,
            module,
            pipelines: None,
            target: None,
            placements,
            time: 0.,
        })
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, dt: f64, animate: bool) -> Result<()> {
        if animate {
            self.time += dt;
        }
        Ok(())
    }
    pub fn prepare(&mut self, _s: &mut Scene, _c: Object3D, _r: &Renderer) -> Result<()> {
        Ok(())
    }
    pub fn render(
        &mut self,
        r: &Renderer,
        _s: &mut Scene,
        _c: Object3D,
        out: &RenderTarget,
    ) -> Result<bool> {
        if self
            .target
            .as_ref()
            .is_none_or(|(t, _, _)| t.width != out.width || t.height != out.height)
        {
            let target =
                RenderTarget::with_options(&r.device, out.width, out.height, out.options.clone())?;
            let normal = depth(r, out.width, out.height, wgpu::TextureFormat::Depth24Plus);
            let reversed = depth(r, out.width, out.height, wgpu::TextureFormat::Depth32Float);
            self.target = Some((target, normal, reversed));
        }
        let (target, normal_depth, reversed_depth) =
            self.target.as_ref().ok_or(Error::Invalid("depth target"))?;
        let format = target.options.format;
        if self.pipelines.as_ref().is_none_or(|p| p.format != format) {
            let layout = r
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: None,
                    bind_group_layouts: &[&self.layout],
                    push_constant_ranges: &[],
                });
            let make = |fragment, depth, compare| {
                pipeline(r, &self.module, &layout, format, fragment, depth, compare)
            };
            use wgpu::{CompareFunction as C, TextureFormat as F};
            self.pipelines = Some(Pipelines {
                format,
                normal: make("fs", F::Depth24Plus, C::LessEqual),
                logarithmic: make("fs_log", F::Depth24Plus, C::LessEqual),
                reversed: make("fs", F::Depth32Float, C::GreaterEqual),
            });
        }
        let pipelines = self.pipelines.as_ref().ok_or(Error::Invalid("pipelines"))?;
        let dpr = web_sys::window()
            .ok_or(Error::Invalid("window"))?
            .device_pixel_ratio();
        let css_width = out.width as f64 / dpr;
        let css_height = out.height as f64 / dpr;
        // setSize( 0.33 × innerWidth, innerHeight ) at the pixel ratio.
        let width = (0.33 * css_width * dpr).floor() as f32;
        let height = (css_height * dpr).floor() as f32;
        let aspect = 0.33 * css_width / css_height;
        let camera = Matrix4::from_translation(Vector3::new(0., 0., 12.)).inverse();
        let axis = Vector3::new(self.time.sin(), self.time.cos(), 0.);
        let rotation = Quaternion::from_axis_angle(axis, 30f64.to_radians());
        let mut data = vec![0f32; (SLOT as usize / 4) * 3 * COUNT];
        for (v, reversed) in [false, false, true].into_iter().enumerate() {
            let projection = projection(aspect, reversed);
            for (i, (position, scale)) in self.placements.iter().enumerate() {
                let model = Matrix4::from_scale_rotation_translation(
                    Vector3::splat(*scale),
                    rotation,
                    *position,
                );
                let slot = &mut data[(v * COUNT + i) * SLOT as usize / 4..];
                for (k, m) in [camera, model, projection].iter().enumerate() {
                    for (j, e) in m.to_cols_array().iter().enumerate() {
                        slot[k * 16 + j] = *e as f32;
                    }
                }
                slot[48] = NEAR as f32;
                slot[49] = FAR as f32;
            }
        }
        r.queue
            .write_buffer(&self.uniforms, 0, bytemuck::cast_slice(&data));
        let mut encoder = r.device.create_command_encoder(&Default::default());
        let passes: [Pass; 2] = [
            (
                normal_depth,
                1.,
                wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                &[(0, &pipelines.normal), (1, &pipelines.logarithmic)],
            ),
            (
                reversed_depth,
                0.,
                wgpu::LoadOp::Load,
                &[(2, &pipelines.reversed)],
            ),
        ];
        for (depth_view, clear, load, draws) in passes {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("reversed depth views"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &target.view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load,
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: depth_view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(clear),
                        store: wgpu::StoreOp::Discard,
                    }),
                    stencil_ops: None,
                }),
                ..Default::default()
            });
            pass.set_vertex_buffer(0, self.vertices.slice(..));
            for &(v, pipeline) in draws {
                let x = (v as f64 * 0.33 * css_width * dpr).round() as f32;
                pass.set_viewport(x, 0., width.min(out.width as f32 - x), height, 0., 1.);
                pass.set_pipeline(pipeline);
                // The opaque list sorts front to back; reversed depth negates z.
                let order: Vec<usize> = if v == 2 {
                    (0..COUNT).rev().collect()
                } else {
                    (0..COUNT).collect()
                };
                for i in order {
                    pass.set_bind_group(0, &self.bind, &[((v * COUNT + i) as u64 * SLOT) as u32]);
                    pass.draw(0..12, 0..1);
                }
            }
        }
        r.queue.submit([encoder.finish()]);
        Ok(true)
    }
    pub fn output(&self) -> Option<&RenderTarget> {
        self.target.as_ref().map(|(t, _, _)| t)
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
        Err(Error::Invalid("reversed depth parameter"))
    }
    pub fn seek(&mut self, t: f64) {
        self.time = t;
    }
}
