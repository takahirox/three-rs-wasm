//! webgpu_hdr: an HDR brush ( intensity 4, radial falloff by hardness and
//! radius ) drawn additively over a white background in an orthographic
//! pixel-space pass, through AfterImageNode ( the previous result faded by
//! the decay where above 0.1, kept where brighter ) to the extended-sRGB
//! output. The brush follows the pointer. Every stage runs the WGSL
//! three.js r186 generates for the page (in `hdr/`). The original presents
//! a half-float, extended-range canvas: on a standard display its colors
//! clamp to the displayable range, as the port's 8-bit output does.
use super::controls_attributes::viewport_css;
use super::deferred::{color, sampled_pipeline};
use super::lights_projector::{m4, pack};
use super::pmrem_cube_uv::bind;
use super::retro::{target, uniform};
use crate::{Error, Result, geometry::*, math::*, render_target::*, renderer::*, scene::*};
use wgpu::util::DeviceExt;

const HALF: wgpu::TextureFormat = wgpu::TextureFormat::Rgba16Float;
const DEPTH: wgpu::TextureFormat = wgpu::TextureFormat::Depth24Plus;
macro_rules! wgsl {
    ($name:literal) => {
        include_str!(concat!("hdr/", $name, ".wgsl"))
    };
}
const UV: [wgpu::VertexAttribute; 1] = wgpu::vertex_attr_array![0 => Float32x2];
const POSITION: [wgpu::VertexAttribute; 1] = wgpu::vertex_attr_array![1 => Float32x3];
struct Targets {
    width: u32,
    height: u32,
    format: wgpu::TextureFormat,
    samples: u32,
    msaa: Option<(wgpu::TextureView, wgpu::TextureView)>,
    brush: wgpu::TextureView,
    depth: wgpu::TextureView,
    /// AfterImageNode's two targets: each frame composes into one from the other.
    images: [wgpu::TextureView; 2],
    brush_draw: (wgpu::RenderPipeline, [wgpu::BindGroup; 2]),
    /// Composing into image i ( from the other ).
    afterimage: (wgpu::RenderPipeline, [wgpu::BindGroup; 2]),
    /// Presenting image i.
    output: (wgpu::RenderPipeline, [wgpu::BindGroup; 2]),
    screen: RenderTarget,
}
pub(super) struct Demo {
    /// intensity, hardness, radius, afterImageDecay.
    params: [f64; 4],
    /// The brush position in CSS pixels from the bottom left.
    brush: Vector2,
    /// The CSS size of the canvas.
    css: (f64, f64),
    /// A requested frame steps the after image.
    pending: bool,
    /// The image holding the latest result.
    current: usize,
    plane: [wgpu::Buffer; 3],
    quad_uv: wgpu::Buffer,
    brush_render: wgpu::Buffer,
    brush_object: wgpu::Buffer,
    afterimage_object: wgpu::Buffer,
    sampler: wgpu::Sampler,
    targets: Option<Targets>,
}
impl Demo {
    pub async fn create(_s: &mut Scene, _c: Object3D, _id: u32, r: &Renderer) -> Result<Self> {
        let g = PlaneGeometry::build(1., 1., 1, 1)?;
        let read = |name: &str, n: usize| -> Result<Vec<f32>> {
            let a = g
                .attributes
                .get(name)
                .ok_or(Error::Invalid("hdr attribute"))?;
            (0..a.count())
                .flat_map(|i| (0..n).map(move |k| (i, k)))
                .map(|(i, k)| a.get_component(i, k).map(|v| v as f32))
                .collect()
        };
        let index = g.index.clone().ok_or(Error::Invalid("hdr index"))?;
        let init = |label, data: &[u8], usage| {
            r.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some(label),
                    contents: data,
                    usage,
                })
        };
        let vertex = wgpu::BufferUsages::VERTEX;
        Ok(Self {
            params: [4., 0.4, 0.5, 0.985],
            brush: Vector2::ZERO,
            css: (1., 1.),
            pending: true,
            current: 0,
            plane: [
                init("hdr brush", bytemuck::cast_slice(&read("uv", 2)?), vertex),
                init(
                    "hdr brush",
                    bytemuck::cast_slice(&read("position", 3)?),
                    vertex,
                ),
                init(
                    "hdr brush",
                    bytemuck::cast_slice(&index),
                    wgpu::BufferUsages::INDEX,
                ),
            ],
            quad_uv: init(
                "hdr quad",
                bytemuck::cast_slice(&[0f32, -1., 0., 1., 2., 1.]),
                vertex,
            ),
            brush_render: uniform(r, "hdr brush", wgsl!("brush_vs"), "renderStruct")?,
            brush_object: uniform(r, "hdr brush", wgsl!("brush_fs"), "objectStruct")?,
            afterimage_object: uniform(
                r,
                "hdr after image",
                wgsl!("afterimage_fs"),
                "objectStruct",
            )?,
            sampler: r.device.create_sampler(&wgpu::SamplerDescriptor {
                mag_filter: wgpu::FilterMode::Linear,
                min_filter: wgpu::FilterMode::Linear,
                ..Default::default()
            }),
            targets: None,
        })
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, _dt: f64, animate: bool) -> Result<()> {
        if animate {
            self.pending = true;
        }
        Ok(())
    }
    pub fn prepare(&mut self, _s: &mut Scene, _c: Object3D, _r: &Renderer) -> Result<()> {
        Ok(())
    }
    fn resize(&mut self, r: &Renderer, out: &RenderTarget) -> Result<()> {
        let size = (out.width, out.height);
        let samples = if out.options.samples > 1 { 4 } else { 1 };
        let readable = || target(r, size, HALF, 1);
        let brush = readable();
        let images = [readable(), readable()];
        let layouts = [
            wgpu::VertexBufferLayout {
                array_stride: 8,
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes: &UV,
            },
            wgpu::VertexBufferLayout {
                array_stride: 12,
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes: &POSITION,
            },
        ];
        // AdditiveBlending: src-alpha / one for color, one / one for alpha.
        let brush_pipeline = r
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("hdr brush"),
                layout: None,
                vertex: wgpu::VertexState {
                    module: &r.device.create_shader_module(wgpu::ShaderModuleDescriptor {
                        label: Some("hdr brush"),
                        source: wgpu::ShaderSource::Wgsl(wgsl!("brush_vs").into()),
                    }),
                    entry_point: Some("main"),
                    compilation_options: Default::default(),
                    buffers: &layouts,
                },
                fragment: Some(wgpu::FragmentState {
                    module: &r.device.create_shader_module(wgpu::ShaderModuleDescriptor {
                        label: Some("hdr brush"),
                        source: wgpu::ShaderSource::Wgsl(wgsl!("brush_fs").into()),
                    }),
                    entry_point: Some("main"),
                    compilation_options: Default::default(),
                    targets: &[Some(wgpu::ColorTargetState {
                        format: HALF,
                        blend: Some(wgpu::BlendState {
                            color: wgpu::BlendComponent {
                                src_factor: wgpu::BlendFactor::SrcAlpha,
                                dst_factor: wgpu::BlendFactor::One,
                                operation: wgpu::BlendOperation::Add,
                            },
                            alpha: wgpu::BlendComponent {
                                src_factor: wgpu::BlendFactor::One,
                                dst_factor: wgpu::BlendFactor::One,
                                operation: wgpu::BlendOperation::Add,
                            },
                        }),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                }),
                primitive: wgpu::PrimitiveState {
                    cull_mode: Some(wgpu::Face::Back),
                    ..Default::default()
                },
                depth_stencil: Some(wgpu::DepthStencilState {
                    format: DEPTH,
                    depth_write_enabled: false,
                    depth_compare: wgpu::CompareFunction::Always,
                    stencil: Default::default(),
                    bias: Default::default(),
                }),
                multisample: wgpu::MultisampleState {
                    count: samples,
                    ..Default::default()
                },
                multiview: None,
                cache: None,
            });
        let brush_groups = [
            bind(
                r,
                brush_pipeline.get_bind_group_layout(0),
                &[(0, self.brush_render.as_entire_binding())],
            ),
            bind(
                r,
                brush_pipeline.get_bind_group_layout(1),
                &[(0, self.brush_object.as_entire_binding())],
            ),
        ];
        let quad = |label, vs, fs, format| {
            sampled_pipeline(
                r,
                label,
                (vs, fs),
                &layouts[..1],
                &[format],
                None,
                (false, false),
                (1, wgpu::PrimitiveTopology::TriangleList),
            )
        };
        let tex = wgpu::BindingResource::TextureView;
        let sampler = wgpu::BindingResource::Sampler;
        let afterimage = quad(
            "hdr after image",
            wgsl!("afterimage_vs"),
            wgsl!("afterimage_fs"),
            HALF,
        );
        // Composing into image i reads the other as the old image.
        let afterimage_groups = [1, 0].map(|old| {
            bind(
                r,
                afterimage.get_bind_group_layout(0),
                &[
                    (0, sampler(&self.sampler)),
                    (1, tex(&images[old])),
                    (2, sampler(&self.sampler)),
                    (3, tex(&brush)),
                    (4, self.afterimage_object.as_entire_binding()),
                ],
            )
        });
        let output = quad(
            "hdr output",
            wgsl!("output_vs"),
            wgsl!("output_fs"),
            out.options.format,
        );
        let output_groups = [0, 1].map(|i| {
            bind(
                r,
                output.get_bind_group_layout(0),
                &[(0, sampler(&self.sampler)), (1, tex(&images[i]))],
            )
        });
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
            brush,
            depth: target(r, size, DEPTH, 1),
            images,
            brush_draw: (brush_pipeline, brush_groups),
            afterimage: (afterimage, afterimage_groups),
            output: (output, output_groups),
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
        self.current = 0;
        self.pending = true;
        Ok(())
    }
    pub fn render(
        &mut self,
        r: &Renderer,
        _s: &mut Scene,
        _c: Object3D,
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
        let (css_width, css_height, _) = viewport_css();
        self.css = (css_width, css_height);
        let t = self.targets.as_ref().ok_or(Error::Invalid("hdr targets"))?;
        let mut encoder = r.device.create_command_encoder(&Default::default());
        // renderPipeline.render(): only requested frames step the after image.
        if std::mem::take(&mut self.pending) {
            let write = |buffer: &wgpu::Buffer,
                         source: &str,
                         name: &str,
                         values: &[(&str, Vec<f64>)]|
             -> Result<()> {
                let values: Vec<(&str, &[f64])> =
                    values.iter().map(|(n, v)| (*n, &v[..])).collect();
                r.queue
                    .write_buffer(buffer, 0, &pack(source, name, &values)?);
                Ok(())
            };
            // OrthographicCamera( 0, innerWidth, innerHeight, 0, 1, 2 ) at z = 1.
            let (w, h) = self.css;
            let projection = Matrix4::from_cols_array(&[
                2. / w,
                0.,
                0.,
                0.,
                0.,
                2. / h,
                0.,
                0.,
                0.,
                0.,
                -1.,
                0.,
                -1.,
                -1.,
                -1.,
                1.,
            ]);
            write(
                &self.brush_render,
                wgsl!("brush_vs"),
                "renderStruct",
                &[
                    ("cameraProjectionMatrix", m4(projection)),
                    (
                        "cameraViewMatrix",
                        m4(Matrix4::from_translation(Vector3::new(0., 0., -1.))),
                    ),
                ],
            )?;
            let [intensity, hardness, radius, decay] = self.params;
            let model = Matrix4::from_translation(Vector3::new(self.brush.x, self.brush.y, 0.))
                * Matrix4::from_scale(Vector3::new(300., 300., 1.));
            write(
                &self.brush_object,
                wgsl!("brush_fs"),
                "objectStruct",
                &[
                    ("intensity", vec![intensity]),
                    ("radius", vec![radius]),
                    ("hardness", vec![hardness]),
                    ("nodeUniform5", m4(model)),
                ],
            )?;
            write(
                &self.afterimage_object,
                wgsl!("afterimage_fs"),
                "objectStruct",
                &[("afterImageDecay", vec![decay])],
            )?;
            {
                let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("hdr brush"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: t.msaa.as_ref().map_or(&t.brush, |m| &m.0),
                        depth_slice: None,
                        resolve_target: t.msaa.as_ref().map(|_| &t.brush),
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color::WHITE),
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
                pass.set_pipeline(&t.brush_draw.0);
                pass.set_bind_group(0, &t.brush_draw.1[0], &[]);
                pass.set_bind_group(1, &t.brush_draw.1[1], &[]);
                pass.set_vertex_buffer(0, self.plane[0].slice(..));
                pass.set_vertex_buffer(1, self.plane[1].slice(..));
                pass.set_index_buffer(self.plane[2].slice(..), wgpu::IndexFormat::Uint32);
                pass.draw_indexed(0..6, 0, 0..1);
            }
            let next = 1 - self.current;
            {
                let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("hdr after image"),
                    color_attachments: &[color(&t.images[next], wgpu::Color::BLACK)],
                    ..Default::default()
                });
                pass.set_pipeline(&t.afterimage.0);
                pass.set_bind_group(0, &t.afterimage.1[next], &[]);
                pass.set_vertex_buffer(0, self.quad_uv.slice(..));
                pass.draw(0..3, 0..1);
            }
            self.current = next;
        }
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("hdr output"),
                color_attachments: &[color(&t.screen.view, wgpu::Color::BLACK)],
                ..Default::default()
            });
            pass.set_pipeline(&t.output.0);
            pass.set_bind_group(0, &t.output.1[self.current], &[]);
            pass.set_vertex_buffer(0, self.quad_uv.slice(..));
            pass.draw(0..3, 0..1);
        }
        r.queue.submit([encoder.finish()]);
        Ok(true)
    }
    pub fn output(&self) -> Option<&RenderTarget> {
        self.targets.as_ref().map(|t| &t.screen)
    }
    /// pointermove: the brush centre in CSS pixels, y up from the bottom.
    pub fn gpu_pointer(&mut self, x: f64, y: f64) -> Result<bool> {
        let (width, height, _) = viewport_css();
        self.brush = Vector2::new((x + 1.) / 2. * width, (y + 1.) / 2. * height);
        Ok(false)
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
    /// intensity, hardness, radius and afterImageDecay.
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        *self
            .params
            .get_mut(index)
            .ok_or(Error::Invalid("hdr parameter"))? = value as f64;
        Ok(())
    }
    pub fn seek(&mut self, _t: f64) {
        self.pending = true;
    }
}
