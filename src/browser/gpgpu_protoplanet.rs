//! webgl_gpgpu_protoplanet: 4,096 debris particles in a 64 × 64
//! GPUComputationRenderer simulation. Each frame the velocity variable
//! gathers every other particle's gravity (merging colliding particles into
//! the one with the lower id) and the position variable integrates the
//! previous velocity; both read the previous frame's textures and render into
//! the other half of their float render-target pairs, as the addon does. The
//! particles are drawn from those textures as points sized by their mass. The
//! page's GLSL is translated to WGSL with the same expressions; WebGL's
//! points become six-vertex billboards of the same clamped size, depth-tested
//! at their centers.
use super::controls_attributes::{Controls, camera_state, viewport_css};
use crate::{Error, Result, camera::*, math::*, render_target::*, renderer::*, scene::*};
use std::f64::consts::PI;

const WIDTH: u32 = 64;
const PARTICLES: usize = (WIDTH * WIDTH) as usize;
const FLOAT: wgpu::TextureFormat = wgpu::TextureFormat::Rgba32Float;
const DEPTH: wgpu::TextureFormat = wgpu::TextureFormat::Depth24Plus;
/// ANGLE's ALIASED_POINT_SIZE_RANGE upper bound on the reference adapter.
const MAX_POINT_SIZE: f32 = 511.;
const QUAD: &str = "@vertex fn vs(@builtin(vertex_index) i:u32)->@builtin(position) vec4<f32>{let p=array(vec2(-1.0,-1.0),vec2(3.0,-1.0),vec2(-1.0,3.0));return vec4(p[i],0.0,1.0);}";
/// computeShaderVelocity, with `resolution` 64 × 64 and gl_FragCoord's rows
/// matching the texture rows the data texture was uploaded to.
const VELOCITY: &str = "
struct U{gravity:f32,density:f32,pad0:f32,pad1:f32}
@group(0) @binding(0) var position_texture:texture_2d<f32>;
@group(0) @binding(1) var velocity_texture:texture_2d<f32>;
@group(0) @binding(2) var<uniform> u:U;
const PI:f32=3.141592653589793;
const delta:f32=1.0/60.0;
fn radius_from_mass(mass:f32)->f32{return pow((3.0/(4.0*PI))*mass/u.density,1.0/3.0);}
@fragment fn fs(@builtin(position) frag:vec4<f32>)->@location(0) vec4<f32>{
 let resolution=vec2(64.0,64.0);
 let uv=frag.xy/resolution;
 let id=uv.y*resolution.x+uv.x;
 let texel=vec2<i32>(frag.xy);
 let pos=textureLoad(position_texture,texel,0).xyz;
 let tmp_vel=textureLoad(velocity_texture,texel,0);
 var vel=tmp_vel.xyz;var mass=tmp_vel.w;
 if mass>0.0 {
  var radius=radius_from_mass(mass);
  var acceleration=vec3(0.0);
  for(var y=0.0;y<resolution.y;y+=1.0){
   for(var x=0.0;x<resolution.x;x+=1.0){
    let second=vec2(x+0.5,y+0.5)/resolution;
    let t=vec2<i32>(i32(x),i32(y));
    let pos2=textureLoad(position_texture,t,0).xyz;
    let vel_temp2=textureLoad(velocity_texture,t,0);
    let vel2=vel_temp2.xyz;let mass2=vel_temp2.w;
    let id2=second.y*resolution.x+second.x;
    if id==id2 {continue;}
    if mass2==0.0 {continue;}
    let d=pos2-pos;let distance=length(d);
    let radius2=radius_from_mass(mass2);
    if distance==0.0 {continue;}
    if distance<radius+radius2 {
     if id<id2 {
      vel=(vel*mass+vel2*mass2)/(mass+mass2);
      mass+=mass2;
      radius=radius_from_mass(mass);
     } else {
      mass=0.0;radius=0.0;vel=vec3(0.0);
      break;
     }
    }
    let distance_sq=distance*distance;
    var field=u.gravity*mass2/distance_sq;
    field=min(field,1000.0);
    acceleration+=field*normalize(d);
   }
   if mass==0.0 {break;}
  }
  vel+=delta*acceleration;
 }
 return vec4(vel,mass);
}";
/// computeShaderPosition.
const POSITION: &str = "
@group(0) @binding(0) var position_texture:texture_2d<f32>;
@group(0) @binding(1) var velocity_texture:texture_2d<f32>;
@fragment fn fs(@builtin(position) frag:vec4<f32>)->@location(0) vec4<f32>{
 let texel=vec2<i32>(frag.xy);
 var pos=textureLoad(position_texture,texel,0).xyz;
 let tmp_vel=textureLoad(velocity_texture,texel,0);
 var vel=tmp_vel.xyz;
 if tmp_vel.w==0.0 {vel=vec3(0.0);}
 pos+=vel*(1.0/60.0);
 return vec4(pos,1.0);
}";
/// particleVertexShader and particleFragmentShader: each instance is one
/// point, expanded to its gl_PointSize square in window pixels.
const PARTICLE: &str = "
struct U{projection:mat4x4<f32>,view:mat4x4<f32>,viewport:vec2<f32>,camera_constant:f32,density:f32}
@group(0) @binding(0) var position_texture:texture_2d<f32>;
@group(0) @binding(1) var velocity_texture:texture_2d<f32>;
@group(0) @binding(2) var<uniform> u:U;
const PI:f32=3.141592653589793;
struct V{@builtin(position) clip:vec4<f32>,@location(0) color:vec4<f32>,@location(1) coord:vec2<f32>}
fn radius_from_mass(mass:f32)->f32{return pow((3.0/(4.0*PI))*mass/u.density,1.0/3.0);}
@vertex fn vs(@builtin(vertex_index) v:u32,@builtin(instance_index) i:u32)->V{
 // uv = ( i / 63, j / 63 ) read with NearestFilter and clamping.
 let ij=vec2(f32(i%64u),f32(i/64u));
 let uv=ij/63.0;
 let texel=vec2<i32>(clamp(floor(uv*64.0),vec2(0.0),vec2(63.0)));
 let pos=textureLoad(position_texture,texel,0).xyz;
 let vel_temp=textureLoad(velocity_texture,texel,0);
 let mass=vel_temp.w;
 var out:V;
 out.color=vec4(1.0,mass/250.0,0.0,1.0);
 let mv=u.view*vec4(pos,1.0);
 let radius=radius_from_mass(mass);
 var size=0.0;
 if mass!=0.0 {size=radius*u.camera_constant/(-mv.z);}
 size=clamp(size,1.0," ;
const PARTICLE_TAIL: &str = ");
 let clip=u.projection*mv;
 let corner=array(vec2(-0.5,-0.5),vec2(0.5,-0.5),vec2(-0.5,0.5),vec2(-0.5,0.5),vec2(0.5,-0.5),vec2(0.5,0.5))[v];
 out.clip=clip+vec4(corner*size*2.0/u.viewport*clip.w,0.0,0.0);
 // gl_PointCoord: ( 0, 0 ) at the sprite's top-left.
 out.coord=vec2(corner.x,-corner.y)+0.5;
 return out;
}
@fragment fn fs(in:V)->@location(0) vec4<f32>{
 if in.color.y==0.0 {discard;}
 let f=length(in.coord-vec2(0.5,0.5));
 if f>0.5 {discard;}
 return in.color;
}";
struct Targets {
    width: u32,
    height: u32,
    format: wgpu::TextureFormat,
    depth: wgpu::TextureView,
    screen: RenderTarget,
    pipeline: wgpu::RenderPipeline,
    /// The particle groups reading each half of the pairs.
    groups: [wgpu::BindGroup; 2],
}
pub(super) struct Demo {
    controls: Controls,
    seed: u32,
    /// gravityConstant, density, radius, height, exponent, maxMass, velocity,
    /// velocityExponent, randVelocity.
    params: [f64; 9],
    pending: bool,
    /// restartSimulation() was pressed.
    restart: bool,
    current: usize,
    positions: [wgpu::Texture; 2],
    velocities: [wgpu::Texture; 2],
    velocity_pipeline: wgpu::RenderPipeline,
    position_pipeline: wgpu::RenderPipeline,
    /// The compute groups reading each current half.
    velocity_groups: [wgpu::BindGroup; 2],
    position_groups: [wgpu::BindGroup; 2],
    velocity_uniforms: wgpu::Buffer,
    particle_uniforms: wgpu::Buffer,
    targets: Option<Targets>,
}
fn view(t: &wgpu::Texture) -> wgpu::TextureView {
    t.create_view(&Default::default())
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, _id: u32, r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 75.,
            near: 5.,
            far: 15000.,
            aspect,
            ..Default::default()
        }));
        let n = s.get_mut(c)?;
        n.position = Vector3::new(0., 120., 400.);
        n.quaternion = Quaternion::IDENTITY;
        let mut controls = Controls::new(None, (100., 1000.), PI, true);
        controls.update(s, c)?;
        let texture = || {
            r.device.create_texture(&wgpu::TextureDescriptor {
                label: Some("protoplanet variable"),
                size: wgpu::Extent3d {
                    width: WIDTH,
                    height: WIDTH,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: FLOAT,
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                    | wgpu::TextureUsages::TEXTURE_BINDING
                    | wgpu::TextureUsages::COPY_DST,
                view_formats: &[],
            })
        };
        let positions = [texture(), texture()];
        let velocities = [texture(), texture()];
        let compute = |label, source: String| {
            let module = r.device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some(label),
                source: wgpu::ShaderSource::Wgsl(source.into()),
            });
            r.device
                .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    label: Some(label),
                    layout: None,
                    vertex: wgpu::VertexState {
                        module: &module,
                        entry_point: Some("vs"),
                        compilation_options: Default::default(),
                        buffers: &[],
                    },
                    fragment: Some(wgpu::FragmentState {
                        module: &module,
                        entry_point: Some("fs"),
                        compilation_options: Default::default(),
                        targets: &[Some(FLOAT.into())],
                    }),
                    primitive: Default::default(),
                    depth_stencil: None,
                    multisample: Default::default(),
                    multiview: None,
                    cache: None,
                })
        };
        let velocity_pipeline = compute("protoplanet velocity", format!("{QUAD}{VELOCITY}"));
        let position_pipeline = compute("protoplanet position", format!("{QUAD}{POSITION}"));
        let velocity_uniforms = r.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("protoplanet velocity uniforms"),
            size: 16,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let particle_uniforms = r.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("protoplanet particle uniforms"),
            size: 144,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let group = |p: &wgpu::RenderPipeline, k: usize, uniforms: Option<&wgpu::Buffer>| {
            let (pv, vv) = (view(&positions[k]), view(&velocities[k]));
            let mut entries = vec![
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&pv),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&vv),
                },
            ];
            if let Some(b) = uniforms {
                entries.push(wgpu::BindGroupEntry {
                    binding: 2,
                    resource: b.as_entire_binding(),
                });
            }
            r.device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: None,
                layout: &p.get_bind_group_layout(0),
                entries: &entries,
            })
        };
        let velocity_groups =
            [0, 1].map(|k| group(&velocity_pipeline, k, Some(&velocity_uniforms)));
        let position_groups = [0, 1].map(|k| group(&position_pipeline, k, None));
        let mut demo = Self {
            controls,
            seed: 186,
            params: [100., 0.45, 300., 8., 0.4, 15., 70., 0.2, 0.001],
            pending: true,
            restart: false,
            current: 0,
            positions,
            velocities,
            velocity_pipeline,
            position_pipeline,
            velocity_groups,
            position_groups,
            velocity_uniforms,
            particle_uniforms,
            targets: None,
        };
        // initComputeRenderer(): fillTextures, both halves of each pair.
        demo.fill(r);
        // initProtoplanets(): the unused position attribute draws x and z.
        for _ in 0..PARTICLES * 2 {
            demo.random();
        }
        Ok(demo)
    }
    /// The fixture's seeded Math.random.
    fn random(&mut self) -> f64 {
        self.seed = self.seed.wrapping_mul(1664525).wrapping_add(1013904223);
        self.seed as f64 / 4294967296.
    }
    /// fillTextures() into both halves of the position and velocity pairs.
    fn fill(&mut self, r: &Renderer) {
        let [
            _,
            _,
            radius,
            height,
            exponent,
            max_mass,
            max_vel,
            vel_exponent,
            rand_vel,
        ] = self.params;
        let max_mass = max_mass * 1024. / PARTICLES as f64;
        let (mut pos, mut vel) = (vec![0f32; PARTICLES * 4], vec![0f32; PARTICLES * 4]);
        for k in (0..PARTICLES * 4).step_by(4) {
            let (mut x, mut z, mut rr);
            loop {
                x = self.random() * 2. - 1.;
                z = self.random() * 2. - 1.;
                rr = x * x + z * z;
                if rr <= 1. {
                    break;
                }
            }
            let rr = rr.sqrt();
            let r_exp = radius * rr.powf(exponent);
            let v = max_vel * rr.powf(vel_exponent);
            let vx = v * z + (self.random() * 2. - 1.) * rand_vel;
            let vy = (self.random() * 2. - 1.) * rand_vel * 0.05;
            let vz = -v * x + (self.random() * 2. - 1.) * rand_vel;
            let y = (self.random() * 2. - 1.) * height;
            let mass = self.random() * max_mass + 1.;
            pos[k..k + 4].copy_from_slice(&[(x * r_exp) as f32, y as f32, (z * r_exp) as f32, 1.]);
            vel[k..k + 4].copy_from_slice(&[vx as f32, vy as f32, vz as f32, mass as f32]);
        }
        for (textures, data) in [(&self.positions, &pos), (&self.velocities, &vel)] {
            for t in textures {
                r.queue.write_texture(
                    t.as_image_copy(),
                    bytemuck::cast_slice(data),
                    wgpu::TexelCopyBufferLayout {
                        offset: 0,
                        bytes_per_row: Some(WIDTH * 16),
                        rows_per_image: Some(WIDTH),
                    },
                    wgpu::Extent3d {
                        width: WIDTH,
                        height: WIDTH,
                        depth_or_array_layers: 1,
                    },
                );
            }
        }
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
        let module = r.device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("protoplanet particles"),
            source: wgpu::ShaderSource::Wgsl(
                format!("{PARTICLE}{MAX_POINT_SIZE:?}{PARTICLE_TAIL}").into(),
            ),
        });
        let pipeline = r
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("protoplanet particles"),
                layout: None,
                vertex: wgpu::VertexState {
                    module: &module,
                    entry_point: Some("vs"),
                    compilation_options: Default::default(),
                    buffers: &[],
                },
                fragment: Some(wgpu::FragmentState {
                    module: &module,
                    entry_point: Some("fs"),
                    compilation_options: Default::default(),
                    targets: &[Some(out.options.format.into())],
                }),
                primitive: Default::default(),
                depth_stencil: Some(wgpu::DepthStencilState {
                    format: DEPTH,
                    depth_write_enabled: true,
                    depth_compare: wgpu::CompareFunction::LessEqual,
                    stencil: Default::default(),
                    bias: Default::default(),
                }),
                multisample: Default::default(),
                multiview: None,
                cache: None,
            });
        let groups = [0, 1].map(|k| {
            let (pv, vv) = (view(&self.positions[k]), view(&self.velocities[k]));
            r.device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: None,
                layout: &pipeline.get_bind_group_layout(0),
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&pv),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::TextureView(&vv),
                    },
                    wgpu::BindGroupEntry {
                        binding: 2,
                        resource: self.particle_uniforms.as_entire_binding(),
                    },
                ],
            })
        });
        let depth = r
            .device
            .create_texture(&wgpu::TextureDescriptor {
                label: Some("protoplanet depth"),
                size: wgpu::Extent3d {
                    width: out.width,
                    height: out.height,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: DEPTH,
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                view_formats: &[],
            })
            .create_view(&Default::default());
        let screen = RenderTarget::with_options(
            &r.device,
            out.width,
            out.height,
            RenderTargetOptions {
                samples: 0,
                depth_buffer: false,
                ..out.options.clone()
            },
        )?;
        self.targets = Some(Targets {
            width: out.width,
            height: out.height,
            format: out.options.format,
            depth,
            screen,
            pipeline,
            groups,
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
            t.width != out.width || t.height != out.height || t.format != out.options.format
        }) {
            self.resize(r, out)?;
        }
        if std::mem::take(&mut self.restart) {
            self.fill(r);
        }
        let mut encoder = r.device.create_command_encoder(&Default::default());
        // gpuCompute.compute(), once per animate().
        if std::mem::take(&mut self.pending) {
            r.queue.write_buffer(
                &self.velocity_uniforms,
                0,
                bytemuck::cast_slice(&[self.params[0] as f32, self.params[1] as f32, 0., 0.]),
            );
            let next = 1 - self.current;
            for (pipeline, groups, target) in [
                (
                    &self.velocity_pipeline,
                    &self.velocity_groups,
                    &self.velocities[next],
                ),
                (
                    &self.position_pipeline,
                    &self.position_groups,
                    &self.positions[next],
                ),
            ] {
                let target = view(target);
                let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("protoplanet compute"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &target,
                        depth_slice: None,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                    ..Default::default()
                });
                pass.set_pipeline(pipeline);
                pass.set_bind_group(0, &groups[self.current], &[]);
                pass.draw(0..3, 0..1);
            }
            self.current = next;
        }
        s.update()?;
        let (camera, world) = s.camera(c)?;
        let (projection, view_matrix) = (camera.projection_matrix()?, world.inverse());
        let fov = match camera {
            Camera::Perspective(p) => p.fov,
            _ => 75.,
        };
        // getCameraConstant(): the window's CSS height over tan( fov / 2 ).
        let (_, css_height, _) = viewport_css();
        let constant = css_height / (fov.to_radians() * 0.5).tan();
        let t = self
            .targets
            .as_ref()
            .ok_or(Error::Invalid("protoplanet targets"))?;
        let mut data: Vec<f32> = projection.to_cols_array().map(|v| v as f32).to_vec();
        data.extend(view_matrix.to_cols_array().map(|v| v as f32));
        data.extend([
            t.width as f32,
            t.height as f32,
            constant as f32,
            self.params[1] as f32,
        ]);
        r.queue
            .write_buffer(&self.particle_uniforms, 0, bytemuck::cast_slice(&data));
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("protoplanet particles"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &t.screen.view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
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
            pass.set_pipeline(&t.pipeline);
            pass.set_bind_group(0, &t.groups[self.current], &[]);
            pass.draw(0..6, 0..PARTICLES as u32);
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
    /// The GUI's nine parameters, then restartSimulation (index 9), which
    /// refills both halves of the pairs from the static parameters.
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        match index {
            0..=8 => self.params[index] = value as f64,
            9 => self.restart = true,
            _ => return Err(Error::Invalid("protoplanet parameter")),
        }
        Ok(())
    }
    pub fn seek(&mut self, _t: f64) {
        self.pending = true;
    }
}
