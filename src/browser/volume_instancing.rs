//! webgl_volume_instancing: VOXLoader's Menger sponge as an 81³ red
//! Data3DTexture, ray marched inside 50,000 randomly placed back-facing unit
//! boxes of one InstancedMesh (100 steps, the hit position as color and its
//! depth written per fragment), under autoRotating damped OrbitControls.
//! The instance matrices and the volume are uploaded once.
use super::controls_attributes::Controls;
use super::gltf_viewer::fetch;
use crate::{Error, Result, camera::*, math::*, render_target::*, renderer::*, scene::*};
use std::f64::consts::PI;

const COUNT: u32 = 50_000;
/// The RawShaderMaterial: the vertex stage passes the clip position (w
/// divided per fragment) and modelViewMatrix × instanceMatrix; the fragment
/// stage inverts both, as the original does, with WebGL's clip conventions
/// (u.gl is the WebGL projection matrix).
const SHADER: &str = "struct U{view:mat4x4<f32>,projection:mat4x4<f32>,gl:mat4x4<f32>}
@group(0) @binding(0) var<uniform> u:U;
@group(0) @binding(1) var map:texture_3d<f32>;
@group(0) @binding(2) var map_sampler:sampler;
struct O{@builtin(position) clip:vec4<f32>,@location(0) screen:vec4<f32>,@location(1) @interpolate(flat) m0:vec4<f32>,@location(2) @interpolate(flat) m1:vec4<f32>,@location(3) @interpolate(flat) m2:vec4<f32>,@location(4) @interpolate(flat) m3:vec4<f32>}
@vertex fn vs(@location(0) position:vec3<f32>,@location(1) i0:vec4<f32>,@location(2) i1:vec4<f32>,@location(3) i2:vec4<f32>,@location(4) i3:vec4<f32>)->O{
 let mv=u.view*mat4x4(i0,i1,i2,i3);let mvp=mv*vec4(position,1.0);let gl=u.gl*mvp;
 return O(u.projection*mvp,vec4(gl.xy,0.0,gl.w),mv[0],mv[1],mv[2],mv[3]);}
fn inverse4(m:mat4x4<f32>)->mat4x4<f32>{
 let a00=m[0][0];let a01=m[0][1];let a02=m[0][2];let a03=m[0][3];let a10=m[1][0];let a11=m[1][1];let a12=m[1][2];let a13=m[1][3];
 let a20=m[2][0];let a21=m[2][1];let a22=m[2][2];let a23=m[2][3];let a30=m[3][0];let a31=m[3][1];let a32=m[3][2];let a33=m[3][3];
 let b00=a00*a11-a01*a10;let b01=a00*a12-a02*a10;let b02=a00*a13-a03*a10;let b03=a01*a12-a02*a11;let b04=a01*a13-a03*a11;let b05=a02*a13-a03*a12;
 let b06=a20*a31-a21*a30;let b07=a20*a32-a22*a30;let b08=a20*a33-a23*a30;let b09=a21*a32-a22*a31;let b10=a21*a33-a23*a31;let b11=a22*a33-a23*a32;
 let det=b00*b11-b01*b10+b02*b09+b03*b08-b04*b07+b05*b06;
 return mat4x4(vec4(a11*b11-a12*b10+a13*b09,a02*b10-a01*b11-a03*b09,a31*b05-a32*b04+a33*b03,a22*b04-a21*b05-a23*b03),
  vec4(a12*b08-a10*b11-a13*b07,a00*b11-a02*b08+a03*b07,a32*b02-a30*b05-a33*b01,a20*b05-a22*b02+a23*b01),
  vec4(a10*b10-a11*b08+a13*b06,a01*b08-a00*b10-a03*b06,a30*b04-a31*b02+a33*b00,a21*b02-a20*b04-a23*b00),
  vec4(a11*b07-a10*b09-a12*b06,a00*b09-a01*b07+a02*b06,a31*b01-a30*b03-a32*b00,a20*b03-a21*b01+a22*b00))*(1.0/det);}
fn hit_box(o:vec3<f32>,d:vec3<f32>)->vec2<f32>{let inv=1.0/d;let a=(vec3(-0.5)-o)*inv;let b=(vec3(0.5)-o)*inv;let lo=min(a,b);let hi=max(a,b);return vec2(max(lo.x,max(lo.y,lo.z)),min(hi.x,min(hi.y,hi.z)));}
struct F{@location(0) color:vec4<f32>,@builtin(frag_depth) depth:f32}
@fragment fn fs(i:O)->F{
 let instance_to_view=mat4x4(i.m0,i.m1,i.m2,i.m3);
 let screen=i.screen.xy/i.screen.w;let inv_projection=inverse4(u.gl);let inv_instance=inverse4(instance_to_view);
 var t=inv_projection*vec4(screen,-1.0,1.0);let cam_origin=t.xyz/t.w;
 t=inv_projection*vec4(screen,1.0,1.0);let cam_end=t.xyz/t.w;
 let origin=(inv_instance*vec4(cam_origin,1.0)).xyz;let end=(inv_instance*vec4(cam_end,1.0)).xyz;let dir=normalize(end-origin);
 var bounds=hit_box(origin,dir);let hit=bounds.x<=bounds.y;
 bounds.x=max(bounds.x,0.0);let step=(bounds.y-bounds.x)/100.0;
 // The original samples with implicit derivatives inside the loop; the
 // screen derivatives of p = origin + ( start + k × step ) × dir follow
 // from those of its terms, taken before the loop.
 let ox=dpdx(origin);let oy=dpdy(origin);let bx=dpdx(bounds.x);let by=dpdy(bounds.x);
 let sx=dpdx(step);let sy=dpdy(step);let dx=dpdx(dir);let dy=dpdy(dir);
 if !hit {discard;}
 var color=vec4(0.0);var p=vec3(0.0);
 for(var k=0.0;k<100.0;k+=1.0){let d=bounds.x+k*step;p=origin+d*dir;
  let gx=ox+(bx+k*sx)*dir+d*dx;let gy=oy+(by+k*sy)*dir+d*dy;
  if textureSampleGrad(map,map_sampler,p+0.5,gx,gy).r>0.5 {color=vec4(p*2.0,1.0);break;}}
 if color.a==0.0 {discard;}
 var ndc=u.gl*instance_to_view*vec4(p,1.0);ndc/=ndc.w;
 return F(color,((1.0-0.0)*ndc.z+0.0+1.0)/2.0);}";

pub(super) struct Demo {
    controls: Controls,
    last: f64,
    pending: Option<f64>,
    gpu: Option<Gpu>,
    target: Option<(RenderTarget, wgpu::TextureView)>,
    _volume: wgpu::Texture,
    vertices: wgpu::Buffer,
    indices: wgpu::Buffer,
    instances: wgpu::Buffer,
    uniforms: wgpu::Buffer,
    bind: wgpu::BindGroup,
    layout: wgpu::BindGroupLayout,
    module: wgpu::ShaderModule,
}
struct Gpu {
    format: wgpu::TextureFormat,
    pipeline: wgpu::RenderPipeline,
}
/// VOXLoader.parse's first SIZE/XYZI model and buildData3DTexture: 255
/// where a voxel is set.
fn volume(data: &[u8]) -> Result<(u32, u32, u32, Vec<u8>)> {
    let bad = |m: &'static str| Error::Asset(format!("VOX: {m}"));
    let int = |i: usize| -> Result<u32> {
        data.get(i..i + 4)
            .map(|b| u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
            .ok_or(bad("truncated"))
    };
    if data.get(..4) != Some(b"VOX ") {
        return Err(bad("invalid file"));
    }
    let mut i = 8;
    let mut size = None;
    while i + 12 <= data.len() {
        let id = &data[i..i + 4];
        let length = int(i + 4)? as usize;
        i += 12;
        match id {
            b"MAIN" => {}
            b"SIZE" => {
                size = Some((int(i)?, int(i + 4)?, int(i + 8)?));
                i += length;
            }
            b"XYZI" => {
                let (x, y, z) = size.ok_or(bad("XYZI before SIZE"))?;
                let count = int(i)? as usize;
                let voxels = data.get(i + 4..i + 4 + count * 4).ok_or(bad("truncated"))?;
                let mut array = vec![0u8; (x * y * z) as usize];
                for v in voxels.as_chunks::<4>().0 {
                    let index = v[0] as usize
                        + v[1] as usize * x as usize
                        + v[2] as usize * (x * y) as usize;
                    *array.get_mut(index).ok_or(bad("voxel outside the model"))? = 255;
                }
                return Ok((x, y, z, array));
            }
            _ => i += length,
        }
    }
    Err(bad("no model"))
}
fn buffer(r: &Renderer, label: &str, data: &[u8], usage: wgpu::BufferUsages) -> wgpu::Buffer {
    use wgpu::util::DeviceExt;
    r.device
        .create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(label),
            contents: data,
            usage,
        })
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, _id: u32, r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 60.,
            near: 0.1,
            far: 1000.,
            aspect,
            ..Default::default()
        }));
        let n = s.get_mut(c)?;
        n.position = Vector3::new(0., 0., 4.);
        n.quaternion = Quaternion::IDENTITY;
        s.background = Color::BLACK;
        let mut controls = Controls::new(Some(0.05), (0., f64::INFINITY), PI, true);
        controls.auto_rotate = Some(-1.);
        controls.update(s, c)?;
        let (x, y, z, voxels) = volume(&fetch("/web/gallery/assets/vox/menger.vox").await?)?;
        let size = wgpu::Extent3d {
            width: x,
            height: y,
            depth_or_array_layers: z,
        };
        let texture = r.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("menger volume"),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D3,
            format: wgpu::TextureFormat::R8Unorm,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        r.queue.write_texture(
            texture.as_image_copy(),
            &voxels,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(x),
                rows_per_image: Some(y),
            },
            size,
        );
        // minFilter: Nearest, magFilter: Linear; the ray samples level 0.
        let sampler = r.device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("menger volume"),
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });
        // BoxGeometry( 1, 1, 1 ): 24 vertices, 36 indices.
        let g = crate::geometry::BoxGeometry::build(1., 1., 1.)?;
        let positions = g.positions()?;
        let vertices: Vec<f32> = positions
            .iter()
            .flat_map(|p| [p.x as f32, p.y as f32, p.z as f32])
            .collect();
        let indices: Vec<u32> = g.index.clone().ok_or(Error::Invalid("box index"))?;
        // The instances: position.random() − 0.5, × 150; rotation x, y, z × π.
        let mut seed = 186u32;
        let mut random = move || {
            seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
            seed as f64 / 4294967296.
        };
        let mut matrices = Vec::with_capacity(COUNT as usize * 16);
        for _ in 0..COUNT {
            let p = Vector3::new(random(), random(), random()) - Vector3::splat(0.5);
            let (rx, ry, rz) = (random() * PI, random() * PI, random() * PI);
            let q = Euler {
                angles: Vector3::new(rx, ry, rz),
                order: EulerOrder::XYZ,
            }
            .quaternion();
            let m = Matrix4::from_scale_rotation_translation(Vector3::ONE, q, p * 150.);
            matrices.extend(m.to_cols_array().map(|v| v as f32));
        }
        use wgpu::BufferUsages as B;
        let vertices = buffer(r, "volume box", bytemuck::cast_slice(&vertices), B::VERTEX);
        let indices = buffer(
            r,
            "volume box index",
            bytemuck::cast_slice(&indices),
            B::INDEX,
        );
        let instances = buffer(
            r,
            "volume instances",
            bytemuck::cast_slice(&matrices),
            B::VERTEX,
        );
        let uniforms = r.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("volume uniforms"),
            size: 192,
            usage: B::UNIFORM | B::COPY_DST,
            mapped_at_creation: false,
        });
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
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                            view_dimension: wgpu::TextureViewDimension::D3,
                            multisampled: false,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 2,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
            });
        let view = texture.create_view(&Default::default());
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
                    resource: wgpu::BindingResource::TextureView(&view),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
            ],
        });
        let module = r.device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("volume instancing"),
            source: wgpu::ShaderSource::Wgsl(SHADER.into()),
        });
        Ok(Self {
            controls,
            last: 0.,
            pending: None,
            gpu: None,
            target: None,
            _volume: texture,
            vertices,
            indices,
            instances,
            uniforms,
            bind,
            layout,
            module,
        })
    }
    /// animate(): controls.update( delta ) with the damped autoRotate.
    fn step(&mut self, s: &mut Scene, c: Object3D, dt: f64) -> Result<()> {
        self.controls.frame_update_dt(s, c, dt)
    }
    pub fn update(&mut self, s: &mut Scene, c: Object3D, dt: f64, animate: bool) -> Result<()> {
        if animate {
            self.step(s, c, dt)?;
        }
        Ok(())
    }
    pub fn prepare(&mut self, s: &mut Scene, c: Object3D, _r: &Renderer) -> Result<()> {
        if let Some(t) = self.pending.take() {
            let dt = t - self.last;
            self.last = t;
            self.step(s, c, dt)?;
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
                    label: Some("volume depth"),
                    size: wgpu::Extent3d {
                        width: out.width,
                        height: out.height,
                        depth_or_array_layers: 1,
                    },
                    mip_level_count: 1,
                    sample_count: 1,
                    dimension: wgpu::TextureDimension::D2,
                    format: wgpu::TextureFormat::Depth32Float,
                    usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                    view_formats: &[],
                })
                .create_view(&Default::default());
            self.target = Some((target, depth));
        }
        let (target, depth) = self
            .target
            .as_ref()
            .ok_or(Error::Invalid("volume target"))?;
        let format = target.options.format;
        if self.gpu.as_ref().is_none_or(|g| g.format != format) {
            let layout = r
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: None,
                    bind_group_layouts: &[&self.layout],
                    push_constant_ranges: &[],
                });
            let pipeline = r
                .device
                .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    label: Some("volume instancing"),
                    layout: Some(&layout),
                    vertex: wgpu::VertexState {
                        module: &self.module,
                        entry_point: Some("vs"),
                        compilation_options: Default::default(),
                        buffers: &[
                            wgpu::VertexBufferLayout {
                                array_stride: 12,
                                step_mode: wgpu::VertexStepMode::Vertex,
                                attributes: &wgpu::vertex_attr_array![0 => Float32x3],
                            },
                            wgpu::VertexBufferLayout {
                                array_stride: 64,
                                step_mode: wgpu::VertexStepMode::Instance,
                                attributes: &wgpu::vertex_attr_array![1 => Float32x4, 2 => Float32x4, 3 => Float32x4, 4 => Float32x4],
                            },
                        ],
                    },
                    fragment: Some(wgpu::FragmentState {
                        module: &self.module,
                        entry_point: Some("fs"),
                        compilation_options: Default::default(),
                        targets: &[Some(format.into())],
                    }),
                    // side: BackSide.
                    primitive: wgpu::PrimitiveState {
                        cull_mode: Some(wgpu::Face::Front),
                        ..Default::default()
                    },
                    depth_stencil: Some(wgpu::DepthStencilState {
                        format: wgpu::TextureFormat::Depth32Float,
                        depth_write_enabled: true,
                        depth_compare: wgpu::CompareFunction::LessEqual,
                        stencil: Default::default(),
                        bias: Default::default(),
                    }),
                    multisample: Default::default(),
                    multiview: None,
                    cache: None,
                });
            self.gpu = Some(Gpu { format, pipeline });
        }
        let gpu = self.gpu.as_ref().ok_or(Error::Invalid("volume pipeline"))?;
        s.update()?;
        let (camera, world) = s.camera(c)?;
        let Camera::Perspective(p) = camera else {
            return Err(Error::Invalid("volume camera"));
        };
        // makePerspective in WebGL's clip space ( z from −1 to 1 ).
        let top = p.near * (0.5 * p.fov.to_radians()).tan();
        let (height, width) = (2. * top, p.aspect * 2. * top);
        let x = 2. * p.near / width;
        let y = 2. * p.near / height;
        let cc = -(p.far + p.near) / (p.far - p.near);
        let d = -2. * p.far * p.near / (p.far - p.near);
        let gl = Matrix4::from_cols_array(&[
            x, 0., 0., 0., 0., y, 0., 0., 0., 0., cc, -1., 0., 0., d, 0.,
        ]);
        let mut data = vec![];
        for m in [world.inverse(), camera.projection_matrix()?, gl] {
            data.extend(m.to_cols_array().map(|v| v as f32));
        }
        r.queue
            .write_buffer(&self.uniforms, 0, bytemuck::cast_slice(&data));
        let mut encoder = r.device.create_command_encoder(&Default::default());
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("volume instancing"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &target.view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
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
            pass.set_pipeline(&gpu.pipeline);
            pass.set_bind_group(0, &self.bind, &[]);
            pass.set_vertex_buffer(0, self.vertices.slice(..));
            pass.set_vertex_buffer(1, self.instances.slice(..));
            pass.set_index_buffer(self.indices.slice(..), wgpu::IndexFormat::Uint32);
            pass.draw_indexed(0..36, 0, 0..COUNT);
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
        let camera = super::controls_attributes::camera_state(s, c)?;
        if wheel != 0. {
            self.controls.dolly(wheel, &camera, Vector2::ZERO);
        } else if pan {
            self.controls.pan(&camera, dx, dy, height);
        } else {
            self.controls.rotate(dx, dy, height);
        }
        Ok(())
    }
    pub fn parameter(&mut self, _index: usize, _value: f32) -> Result<()> {
        Err(Error::Invalid("volume instancing parameter"))
    }
    /// A still frame at time t: one animate() with the elapsed delta.
    pub fn seek(&mut self, t: f64) {
        self.pending = Some(t);
    }
}
