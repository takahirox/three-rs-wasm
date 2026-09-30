//! webgpu_tsl_compute_attractors_particles: 262,144 particles pulled and spun
//! by three rotating attractors in one compute pass per frame (the page's
//! fixed 1/60 s step), drawn as additive sprites colored by speed, with the
//! attractors' ring-and-arrow helpers and a rotate TransformControls on each.
//! The particles live in resident storage buffers; their init and update
//! passes and the sprite stage follow the WGSL three.js generates for the
//! page's TSL. The helpers and gizmos are scene meshes: the helpers are drawn
//! before the sprites and the gizmos after, as in the original's render lists.
use super::controls_attributes::{Controls, camera_state, viewport_css};
use super::transform_controls::{Event, Mode, TransformControls};
use crate::{
    Error, Result, camera::*, geometry::*, material::*, math::*, render_target::*, renderer::*,
    scene::*,
};
use std::f64::consts::PI;
use std::sync::Arc;

const COUNT: u32 = 262144;
/// The init and update passes. u: time scale, attractor count, attractor
/// mass, particle mass, spinning strength, max speed, damping, bound extent.
const COMPUTE: &str = "struct U{time_scale:f32,count:u32,attractor_mass:f32,particle_mass:f32,spinning:f32,max_speed:f32,damping:f32,extent:f32,positions:array<vec4<f32>,3>,axes:array<vec4<f32>,3>}
@group(0) @binding(0) var<uniform> u:U;
@group(0) @binding(1) var<storage,read_write> positions:array<vec3<f32>>;
@group(0) @binding(2) var<storage,read_write> velocities:array<vec3<f32>>;
fn hash(seed:u32)->f32{let s=((seed*747796405u)+2891336453u);let w=(((s>>((s>>28u)+4u))^s)*277803737u);return (f32(((w>>22u)^w))*2.3283064365386963e-10);}
fn tsl_mod_vec3(x:vec3<f32>,y:vec3<f32>)->vec3<f32>{return x-y*floor(x/y);}
@compute @workgroup_size(64) fn init(@builtin(global_invocation_id) id:vec3<u32>){
 let i=id.x;if i>=262144u {return;}
 positions[i]=((vec3<f32>(hash((i+SEED0u)),hash((i+SEED1u)),hash((i+SEED2u)))-vec3<f32>(0.5))*vec3<f32>(5.0,0.2,5.0));
 let phi=((hash((i+SEED3u))*3.141592653589793)*2.0);let sin_phi=sin(phi);
 let theta=(hash((i+SEED4u))*3.141592653589793);
 velocities[i]=(vec3<f32>((sin_phi*sin(theta)),cos(phi),(sin_phi*cos(theta)))*vec3<f32>(0.05));}
@compute @workgroup_size(64) fn update(@builtin(global_invocation_id) id:vec3<u32>){
 let index=id.x;if index>=262144u {return;}
 let delta=(0.016666666666666666*u.time_scale);var force=vec3<f32>(0.0,0.0,0.0);
 for(var i:i32=0;i<i32(u.count);i++){
  let multiplier=((((hash((index+SEED5u))-0.25)/(1.0-0.25))*(1.0-0.0))+0.0);
  let mass=(multiplier*u.particle_mass);
  let to_attractor=(u.positions[i].xyz-positions[index]);
  let gravity=(((u.attractor_mass*mass)*6.67e-11)/pow(length(to_attractor),2.0));
  force=(force+(normalize(to_attractor)*vec3<f32>(gravity)));
  force=(force+cross(((u.axes[i].xyz*vec3<f32>(gravity))*vec3<f32>(u.spinning)),to_attractor));
 }
 velocities[index]=(velocities[index]+(force*vec3<f32>(delta)));
 if ((length(velocities[index])>u.max_speed)) {velocities[index]=(normalize(velocities[index])*vec3<f32>(u.max_speed));}
 velocities[index]=(velocities[index]*vec3<f32>((1.0-u.damping)));
 positions[index]=(positions[index]+(velocities[index]*vec3<f32>(delta)));
 let half=(u.extent/2.0);
 positions[index]=(tsl_mod_vec3((positions[index]+vec3<f32>(half)),vec3<f32>(u.extent))-vec3<f32>(half));}";
/// The SpriteNodeMaterial: the quad turned to the view, sized by the mass
/// multiplier and scale, colored by speed. r.params: scale, max speed.
const SPRITES: &str = "struct R{projection:mat4x4<f32>,view:mat4x4<f32>,color_a:vec4<f32>,color_b:vec4<f32>,params:vec4<f32>}
@group(0) @binding(0) var<uniform> r:R;
@group(0) @binding(1) var<storage,read> positions:array<vec3<f32>>;
@group(0) @binding(2) var<storage,read> velocities:array<vec3<f32>>;
fn hash(seed:u32)->f32{let s=((seed*747796405u)+2891336453u);let w=(((s>>((s>>28u)+4u))^s)*277803737u);return (f32(((w>>22u)^w))*2.3283064365386963e-10);}
struct V{@builtin(position) clip:vec4<f32>,@location(0) velocity:vec3<f32>}
@vertex fn vs(@builtin(instance_index) instance:u32,@location(0) position:vec3<f32>)->V{
 let view=(r.view*vec4<f32>(positions[instance],1.0));
 let c=cos(0.0);let s=sin(0.0);
 let multiplier=((((hash((instance+SEED5u))-0.25)/(1.0-0.25))*(1.0-0.0))+0.0);
 let offset=(mat2x2<f32>(c,s,(-s),c)*(position.xy*(vec2<f32>(1.0,1.0)*vec2<f32>((multiplier*r.params.x)))));
 return V((r.projection*vec4<f32>((view.xy+offset),view.zw)),velocities[instance]);}
@fragment fn fs(v:V)->@location(0) vec4<f32>{
 let color=mix(r.color_a.xyz,r.color_b.xyz,smoothstep(0.0,0.5,(length(v.velocity)/r.params.y)));
 return max(vec4<f32>(color,1.0),vec4<f32>(0.0));}
@group(1) @binding(0) var scene:texture_2d<f32>;
@vertex fn output_vs(@builtin(vertex_index) i:u32)->@builtin(position) vec4<f32>{
 let p=array(vec2(-1.0,-1.0),vec2(3.0,-1.0),vec2(-1.0,3.0));return vec4(p[i],0.0,1.0);}
fn srgb(color:vec3<f32>)->vec3<f32>{return mix(((pow(color,vec3<f32>(0.41666))*vec3<f32>(1.055))-vec3<f32>(0.055)),(color*vec3<f32>(12.92)),vec3<f32>((color<=vec3<f32>(0.0031308))));}
@fragment fn output_fs(@builtin(position) p:vec4<f32>)->@location(0) vec4<f32>{
 let texel=textureLoad(scene,vec2<i32>(p.xy),0);let a=clamp(texel.w,0.0,1.0);
 var color=vec4<f32>(0.0);if (a!=0.0) {color=vec4<f32>((texel.xyz/vec3<f32>(a)),a);}
 return vec4<f32>((srgb(color.xyz)*vec3<f32>(color.w)),color.w);}";
/// One attractor: its reference object, helper and gizmo.
struct Attractor {
    reference: Object3D,
    helper: Object3D,
    controls: TransformControls,
    position: Vector3,
    axis: Vector3,
}
struct Targets {
    scene: RenderTarget,
    output: wgpu::BindGroup,
    screen: RenderTarget,
    sprites: wgpu::RenderPipeline,
}
pub(super) struct Demo {
    controls: Controls,
    orbit_enabled: bool,
    attractors: Vec<Attractor>,
    /// attractorMass, particleGlobalMass, maxSpeed, velocityDamping,
    /// spinningStrength, scale, boundHalfExtent.
    params: [f64; 7],
    colors: [Color; 2],
    pending: u32,
    reset: bool,
    dragging: bool,
    /// Pointer events and GUI changes, applied with the scene in prepare().
    queue: Vec<(u32, f64, f64)>,
    gizmos: bool,
    uniforms: wgpu::Buffer,
    draw_uniforms: wgpu::Buffer,
    compute_bind: wgpu::BindGroup,
    draw_bind: wgpu::BindGroup,
    init: wgpu::ComputePipeline,
    update: wgpu::ComputePipeline,
    quad: (wgpu::Buffer, wgpu::Buffer),
    module: wgpu::ShaderModule,
    draw_layout: wgpu::PipelineLayout,
    output_group: wgpu::BindGroupLayout,
    output_layout: wgpu::PipelineLayout,
    output: Option<(wgpu::TextureFormat, wgpu::RenderPipeline)>,
    targets: Option<Targets>,
}
/// The page's Math.random() seeds for its hash nodes, as TSL's uint() rounds them.
fn seeds() -> [u32; 6] {
    let mut seed = 186u32;
    [0; 6].map(|_| {
        seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
        (seed as f64 / 4294967296. * 0xffffff as f64).round() as u32
    })
}
fn with_seeds(source: &str) -> String {
    let mut out = source.to_string();
    for (i, s) in seeds().iter().enumerate() {
        out = out.replace(&format!("SEED{i}u"), &format!("{s}u"));
    }
    out
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, _id: u32, r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 25.,
            near: 0.1,
            far: 100.,
            aspect,
            ..Default::default()
        }));
        s.get_mut(c)?.position = Vector3::new(3., 5., 8.);
        s.background = Color::BLACK;
        s.insert(NodeKind::Light(Light::Ambient {
            color: Color::WHITE,
            intensity: 0.5,
        }));
        let light = s.insert(NodeKind::Light(Light::Directional {
            color: Color::WHITE,
            intensity: 1.5,
            target: Vector3::ZERO,
        }));
        s.get_mut(light)?.position = Vector3::new(4., 2., 0.);
        let mut controls = Controls::new(Some(0.05), (0.1, 50.), PI, true);
        controls.update(s, c)?;
        // The attractors, their helpers and rotate gizmos.
        let ring = Arc::new(RingGeometry::build(1., 1.02, 32, 1, 0., PI * 1.5)?);
        let arrow = Arc::new(CylinderGeometry::build(
            0.,
            0.1,
            0.4,
            12,
            1,
            false,
            0.,
            2. * PI,
        )?);
        let mut basic = MeshBasicMaterial::default();
        basic.properties.side = Side::Double;
        let material = Arc::new(Material::Basic(basic));
        let mut attractors = vec![];
        for (position, axis) in [
            (Vector3::new(-1., 0., 0.), Vector3::Y),
            (Vector3::new(1., 0., -0.5), Vector3::Y),
            (
                Vector3::new(0., 0.5, 1.),
                Vector3::new(1., 0., -0.5).normalize(),
            ),
        ] {
            let reference = s.insert(NodeKind::Group);
            let n = s.get_mut(reference)?;
            n.position = position;
            n.quaternion = Quaternion::from_rotation_arc(Vector3::Y, axis);
            let helper = s.insert(NodeKind::Group);
            s.get_mut(helper)?.scale = Vector3::splat(0.325);
            s.add(reference, helper)?;
            let ring = s.insert(NodeKind::Mesh(Mesh::new(ring.clone(), material.clone())));
            s.get_mut(ring)?.quaternion = Quaternion::from_rotation_x(-PI * 0.5);
            s.add(helper, ring)?;
            let arrow = s.insert(NodeKind::Mesh(Mesh::new(arrow.clone(), material.clone())));
            let n = s.get_mut(arrow)?;
            n.position = Vector3::new(1., 0., 0.2);
            n.quaternion = Quaternion::from_rotation_x(PI * 0.5);
            s.add(helper, arrow)?;
            let mut tc = TransformControls::new(s, c)?;
            tc.set_mode(s, Mode::Rotate)?;
            tc.set_size(s, 0.5)?;
            tc.attach(s, reference)?;
            tc.events.clear();
            tc.update(s)?;
            attractors.push(Attractor {
                reference,
                helper,
                controls: tc,
                position,
                axis,
            });
        }
        let storage = |label| {
            r.device.create_buffer(&wgpu::BufferDescriptor {
                label: Some(label),
                size: COUNT as u64 * 16,
                usage: wgpu::BufferUsages::STORAGE,
                mapped_at_creation: false,
            })
        };
        let (positions, velocities) = (
            storage("attractor positions"),
            storage("attractor velocities"),
        );
        let uniforms = r.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("attractors compute"),
            size: 32 + 96,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let draw_uniforms = r.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("attractors draw"),
            size: 128 + 48,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let layout_entries = |visibility, read_only| {
            [0, 1, 2].map(|binding| wgpu::BindGroupLayoutEntry {
                binding,
                visibility: if binding == 0 {
                    visibility | wgpu::ShaderStages::FRAGMENT
                } else {
                    visibility
                },
                ty: wgpu::BindingType::Buffer {
                    ty: if binding == 0 {
                        wgpu::BufferBindingType::Uniform
                    } else {
                        wgpu::BufferBindingType::Storage { read_only }
                    },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            })
        };
        let compute_group = r
            .device
            .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("attractors compute"),
                entries: &layout_entries(wgpu::ShaderStages::COMPUTE, false),
            });
        let draw_group = r
            .device
            .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("attractors draw"),
                entries: &layout_entries(wgpu::ShaderStages::VERTEX, true),
            });
        let bind = |layout, uniform: &wgpu::Buffer| {
            r.device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: None,
                layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: uniform.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: positions.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 2,
                        resource: velocities.as_entire_binding(),
                    },
                ],
            })
        };
        let compute_bind = bind(&compute_group, &uniforms);
        let draw_bind = bind(&draw_group, &draw_uniforms);
        let compute_module = r.device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("attractors compute"),
            source: wgpu::ShaderSource::Wgsl(with_seeds(COMPUTE).into()),
        });
        let compute_layout = r
            .device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[&compute_group],
                push_constant_ranges: &[],
            });
        let pipeline = |entry| {
            r.device
                .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                    label: Some(entry),
                    layout: Some(&compute_layout),
                    module: &compute_module,
                    entry_point: Some(entry),
                    compilation_options: Default::default(),
                    cache: None,
                })
        };
        let (init, update) = (pipeline("init"), pipeline("update"));
        let module = r.device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("attractors sprites"),
            source: wgpu::ShaderSource::Wgsl(with_seeds(SPRITES).into()),
        });
        let draw_layout = r
            .device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[&draw_group],
                push_constant_ranges: &[],
            });
        let output_group = r
            .device
            .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("attractors output"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: false },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                }],
            });
        let output_layout = r
            .device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[&draw_group, &output_group],
                push_constant_ranges: &[],
            });
        // PlaneGeometry( 1, 1 ): the sprite quad.
        let plane = PlaneGeometry::build(1., 1., 1, 1)?;
        let quad: Vec<f32> = plane
            .positions()?
            .iter()
            .flat_map(|p| [p.x as f32, p.y as f32, p.z as f32])
            .collect();
        let index = plane.index.clone().ok_or(Error::Invalid("sprite index"))?;
        use wgpu::util::DeviceExt;
        let quad = (
            r.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("sprite quad"),
                    contents: bytemuck::cast_slice(&quad),
                    usage: wgpu::BufferUsages::VERTEX,
                }),
            r.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("sprite index"),
                    contents: bytemuck::cast_slice(&index),
                    usage: wgpu::BufferUsages::INDEX,
                }),
        );
        let demo = Self {
            controls,
            orbit_enabled: true,
            attractors,
            params: [1e7, 1e4, 8., 0.1, 2.75, 0.008, 8.],
            colors: [Color::from_hex(0x5900ff), Color::from_hex(0xffa575)],
            pending: 1,
            reset: true,
            dragging: false,
            queue: vec![],
            gizmos: true,
            uniforms,
            draw_uniforms,
            compute_bind,
            draw_bind,
            init,
            update,
            quad,
            module,
            draw_layout,
            output_group,
            output_layout,
            output: None,
            targets: None,
        };
        demo.write_uniforms(r);
        Ok(demo)
    }
    fn write_uniforms(&self, r: &Renderer) {
        let f = |v: f64| v as f32;
        let [
            attractor_mass,
            particle_mass,
            max_speed,
            damping,
            spinning,
            _,
            extent,
        ] = self.params;
        let mut data: Vec<f32> = vec![
            1.,
            f32::from_bits(self.attractors.len() as u32),
            f(attractor_mass),
            f(particle_mass),
            f(spinning),
            f(max_speed),
            f(damping),
            f(extent),
        ];
        for a in &self.attractors {
            data.extend([f(a.position.x), f(a.position.y), f(a.position.z), 0.]);
        }
        for a in &self.attractors {
            data.extend([f(a.axis.x), f(a.axis.y), f(a.axis.z), 0.]);
        }
        r.queue
            .write_buffer(&self.uniforms, 0, bytemuck::cast_slice(&data));
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, _dt: f64, animate: bool) -> Result<()> {
        if animate {
            self.pending += 1;
        }
        Ok(())
    }
    pub fn prepare(&mut self, s: &mut Scene, c: Object3D, r: &Renderer) -> Result<()> {
        for (kind, x, y) in std::mem::take(&mut self.queue) {
            if kind >= 1000 {
                self.scene_parameter(s, (kind - 1000) as usize, x as f32)?;
            } else {
                self.pointer(s, kind, x, y)?;
            }
        }
        self.controls.frame_update(s, c)?;
        for a in &mut self.attractors {
            a.controls.update(s)?;
        }
        self.write_uniforms(r);
        let mut encoder = r.device.create_command_encoder(&Default::default());
        {
            let mut pass = encoder.begin_compute_pass(&Default::default());
            pass.set_bind_group(0, &self.compute_bind, &[]);
            if std::mem::take(&mut self.reset) {
                pass.set_pipeline(&self.init);
                pass.dispatch_workgroups(COUNT / 64, 1, 1);
            }
            pass.set_pipeline(&self.update);
            for _ in 0..std::mem::take(&mut self.pending) {
                pass.dispatch_workgroups(COUNT / 64, 1, 1);
            }
        }
        r.queue.submit([encoder.finish()]);
        Ok(())
    }
    fn resize(&mut self, r: &Renderer, out: &RenderTarget) -> Result<()> {
        let samples = out.options.samples.max(1);
        let scene = RenderTarget::with_options(
            &r.device,
            out.width,
            out.height,
            RenderTargetOptions {
                format: wgpu::TextureFormat::Rgba16Float,
                samples,
                ..Default::default()
            },
        )?;
        let output = r.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("attractors output"),
            layout: &self.output_group,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(&scene.view),
            }],
        });
        // Additive blending (SRC_ALPHA, ONE; ONE, ONE), no depth writes.
        let sprites = r
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("attractor sprites"),
                layout: Some(&self.draw_layout),
                vertex: wgpu::VertexState {
                    module: &self.module,
                    entry_point: Some("vs"),
                    compilation_options: Default::default(),
                    buffers: &[wgpu::VertexBufferLayout {
                        array_stride: 12,
                        step_mode: wgpu::VertexStepMode::Vertex,
                        attributes: &wgpu::vertex_attr_array![0 => Float32x3],
                    }],
                },
                fragment: Some(wgpu::FragmentState {
                    module: &self.module,
                    entry_point: Some("fs"),
                    compilation_options: Default::default(),
                    targets: &[Some(wgpu::ColorTargetState {
                        format: wgpu::TextureFormat::Rgba16Float,
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
                    format: wgpu::TextureFormat::Depth32Float,
                    depth_write_enabled: false,
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
            });
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
            scene,
            output,
            screen,
            sprites,
        });
        Ok(())
    }
    /// The scene's helpers or gizmos alone.
    fn show(&self, s: &mut Scene, helpers: bool) -> Result<()> {
        for a in &self.attractors {
            s.get_mut(a.helper)?.visible = helpers && s.get(a.helper)?.visible;
            s.get_mut(a.controls.root)?.visible = !helpers && self.gizmos;
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
        if self.targets.as_ref().is_none_or(|t| {
            t.scene.width != out.width
                || t.scene.height != out.height
                || t.scene.options.samples.max(1) != out.options.samples.max(1)
        }) {
            self.resize(r, out)?;
        }
        let format = out.options.format;
        if self.output.as_ref().is_none_or(|(f, _)| *f != format) {
            let pipeline = r
                .device
                .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    label: Some("attractors output"),
                    layout: Some(&self.output_layout),
                    vertex: wgpu::VertexState {
                        module: &self.module,
                        entry_point: Some("output_vs"),
                        compilation_options: Default::default(),
                        buffers: &[],
                    },
                    fragment: Some(wgpu::FragmentState {
                        module: &self.module,
                        entry_point: Some("output_fs"),
                        compilation_options: Default::default(),
                        targets: &[Some(format.into())],
                    }),
                    primitive: Default::default(),
                    depth_stencil: None,
                    multisample: Default::default(),
                    multiview: None,
                    cache: None,
                });
            self.output = Some((format, pipeline));
        }
        let Some(mut t) = self.targets.take() else {
            return Err(Error::Invalid("attractor targets"));
        };
        let result = self.passes(r, s, c, &mut t);
        self.targets = Some(t);
        result?;
        Ok(true)
    }
    fn passes(&mut self, r: &Renderer, s: &mut Scene, c: Object3D, t: &mut Targets) -> Result<()> {
        // Opaque list: the helpers, then the sprites; the gizmos come last.
        let visible: Vec<bool> = self
            .attractors
            .iter()
            .map(|a| s.get(a.helper).map(|n| n.visible))
            .collect::<Result<_>>()?;
        self.show(s, true)?;
        let first = r.render(s, c, &t.scene);
        s.update()?;
        let (camera, world) = s.camera(c)?;
        let (projection, view) = (camera.projection_matrix()?, world.inverse());
        for (a, v) in self.attractors.iter().zip(&visible) {
            s.get_mut(a.helper)?.visible = *v;
        }
        first?;
        let f = |v: f64| v as f32;
        let mut data: Vec<f32> = vec![];
        for m in [projection, view] {
            data.extend(m.to_cols_array().map(f));
        }
        for color in self.colors {
            data.extend([f(color.0.x), f(color.0.y), f(color.0.z), 1.]);
        }
        data.extend([f(self.params[5]), f(self.params[2]), 0., 0.]);
        r.queue
            .write_buffer(&self.draw_uniforms, 0, bytemuck::cast_slice(&data));
        let multisampled = t.scene.options.samples > 1;
        let depth = t
            .scene
            .depth_view
            .as_ref()
            .ok_or(Error::Invalid("attractor depth"))?;
        let mut encoder = r.device.create_command_encoder(&Default::default());
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("attractor sprites"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: if multisampled {
                        &t.scene.multisampled_views[0]
                    } else {
                        &t.scene.view
                    },
                    depth_slice: None,
                    resolve_target: multisampled.then_some(&t.scene.view),
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: depth,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                ..Default::default()
            });
            pass.set_pipeline(&t.sprites);
            pass.set_bind_group(0, &self.draw_bind, &[]);
            pass.set_vertex_buffer(0, self.quad.0.slice(..));
            pass.set_index_buffer(self.quad.1.slice(..), wgpu::IndexFormat::Uint32);
            pass.draw_indexed(0..6, 0, 0..COUNT);
        }
        r.queue.submit([encoder.finish()]);
        // The gizmos over the sprites.
        self.show(s, false)?;
        t.scene.set_load_color(true);
        t.scene.options.load_depth = true;
        let second = r.render(s, c, &t.scene);
        t.scene.set_load_color(false);
        t.scene.options.load_depth = false;
        for (a, v) in self.attractors.iter().zip(&visible) {
            s.get_mut(a.helper)?.visible = *v;
            s.get_mut(a.controls.root)?.visible = self.gizmos;
        }
        second?;
        let mut encoder = r.device.create_command_encoder(&Default::default());
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("attractors output"),
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
            let (_, pipeline) = self
                .output
                .as_ref()
                .ok_or(Error::Invalid("attractors output"))?;
            pass.set_pipeline(pipeline);
            pass.set_bind_group(0, &self.draw_bind, &[]);
            pass.set_bind_group(1, &t.output, &[]);
            pass.draw(0..3, 0..1);
        }
        r.queue.submit([encoder.finish()]);
        Ok(())
    }
    pub fn output(&self) -> Option<&RenderTarget> {
        self.targets.as_ref().map(|t| &t.screen)
    }
    fn events(&mut self) {
        for a in &mut self.attractors {
            for e in std::mem::take(&mut a.controls.events) {
                match e {
                    Event::DraggingChanged(v) => {
                        self.orbit_enabled = !v;
                        self.dragging = v;
                    }
                    Event::Change => {}
                    _ => {}
                }
            }
        }
    }
    /// The gizmos' pointer listeners; a drag turns its attractor.
    pub fn draw(&mut self, kind: u32, x: f64, y: f64) {
        self.queue.push((kind, x, y));
    }
    fn pointer(&mut self, s: &mut Scene, kind: u32, x: f64, y: f64) -> Result<()> {
        let (w, h, _) = viewport_css();
        let ndc = Vector2::new(x / w * 2. - 1., -(y / h) * 2. + 1.);
        for a in &mut self.attractors {
            if !a.controls.enabled {
                continue;
            }
            match kind {
                10..=12 => {
                    a.controls.pointer_hover(s, ndc)?;
                    a.controls.pointer_down(s, ndc, kind as i32 - 10)?;
                }
                0 => {
                    a.controls.pointer_hover(s, ndc)?;
                    a.controls.pointer_move(s, ndc)?;
                }
                20..=22 => a.controls.pointer_up(s, kind as i32 - 20)?,
                _ => {}
            }
            // change: the attractor follows its reference object.
            let n = s.get(a.reference)?;
            a.position = n.position;
            a.axis = n.quaternion * Vector3::Y;
        }
        self.events();
        Ok(())
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
        // The gizmos' listeners see the pointer before the orbit controls move.
        for (kind, x, y) in std::mem::take(&mut self.queue) {
            if kind >= 1000 {
                self.scene_parameter(s, (kind - 1000) as usize, x as f32)?;
            } else {
                self.pointer(s, kind, x, y)?;
            }
        }
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
    /// The GUI: mass exponents, maxSpeed, velocityDamping, spinningStrength,
    /// scale, boundHalfExtent, the two colors, the controls mode, the helpers
    /// and reset.
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        let v = value as f64;
        match index {
            0 => self.params[0] = 10f64.powi(v.round() as i32),
            1 => self.params[1] = 10f64.powi(v.round() as i32),
            2 => self.params[2] = v,
            3 => self.params[3] = v,
            4 => self.params[4] = v,
            5 => self.params[5] = v,
            6 => self.params[6] = v,
            7 | 8 => self.colors[index - 7] = Color::from_hex(value as u32),
            9..=11 => self.queue.push((1000 + index as u32, v, 0.)),
            _ => return Err(Error::Invalid("attractors parameter")),
        }
        Ok(())
    }
    fn scene_parameter(&mut self, s: &mut Scene, index: usize, value: f32) -> Result<()> {
        match index {
            9 => {
                self.gizmos = value.round() as u32 != 2;
                for a in &mut self.attractors {
                    match value.round() as u32 {
                        0 | 1 => {
                            a.controls.set_enabled(s, true)?;
                            a.controls.set_mode(
                                s,
                                if value.round() as u32 == 0 {
                                    Mode::Translate
                                } else {
                                    Mode::Rotate
                                },
                            )?;
                        }
                        _ => {
                            a.controls.set_enabled(s, false)?;
                        }
                    }
                }
            }
            10 => {
                for a in &self.attractors {
                    s.get_mut(a.helper)?.visible = value > 0.5;
                }
            }
            11 => self.reset = true,
            _ => return Err(Error::Invalid("attractors parameter")),
        }
        Ok(())
    }
    pub fn seek(&mut self, _t: f64) {
        self.pending += 1;
    }
}
