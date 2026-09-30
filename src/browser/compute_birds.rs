//! webgpu_compute_birds: 8,192 birds flocking by the page's two compute
//! passes (velocity from every other bird, then position and wing phase)
//! over resident storage buffers, drawn as instanced three-triangle birds
//! whose vertex stage reads the buffers, inside the vertex-colored
//! icosahedron sky with linear fog, NeutralToneMapping and OrbitControls. The
//! pointer's ray pushes the birds away. The compute and draw stages follow the
//! WGSL three.js generates for the page's TSL.
use super::controls_attributes::{Controls, camera_state};
use crate::{
    Error, Result, camera::*, geometry::*, math::*, render_target::*, renderer::*, scene::*,
};
use std::f64::consts::PI;

const BIRDS: u32 = 8192;
const BOUNDS: f64 = 800.;
/// The flock's compute passes: Birds Velocity and Birds Position.
const COMPUTE: &str = "struct U{separation:f32,alignment:f32,cohesion:f32,delta_time:f32,ray_origin:vec3<f32>,pad0:f32,ray_direction:vec3<f32>,pad1:f32}
@group(0) @binding(0) var<uniform> u:U;
@group(0) @binding(1) var<storage,read_write> positions:array<vec3<f32>>;
@group(0) @binding(2) var<storage,read_write> velocities:array<vec3<f32>>;
@group(0) @binding(3) var<storage,read_write> phases:array<f32>;
@compute @workgroup_size(64) fn velocity(@builtin(global_invocation_id) id:vec3<u32>){
 let bird=id.x;if bird>=8192u {return;}
 var limit=9.0;
 let zone=((u.separation+u.alignment)+u.cohesion);
 let separation_thresh=(u.separation/zone);
 let alignment_thresh=((u.separation+u.alignment)/zone);
 let zone_sq=(zone*zone);
 let position=positions[bird];var velocity=velocities[bird];
 let to_ray=(u.ray_origin-position);let projection=dot(to_ray,u.ray_direction);
 let closest=(u.ray_origin-(u.ray_direction*vec3<f32>(projection)));
 let to_closest=(closest-position);let distance=length(to_closest);let distance_sq=(distance*distance);
 let ray_radius=150.0;let ray_radius_sq=(ray_radius*ray_radius);
 if (distance_sq<ray_radius_sq) {
  velocity=(velocity+(normalize(to_closest)*vec3<f32>(((((distance_sq/ray_radius_sq)-1.0)*u.delta_time)*100.0))));
  limit=(limit+5.0);
 }
 var to_center=position;to_center.y=(to_center.y*2.5);
 velocity=(velocity-((normalize(to_center)*vec3<f32>(u.delta_time))*vec3<f32>(5.0)));
 for(var i:u32=0u;i<8192u;i++){
  if (i==bird) {continue;}
  let to_bird=(positions[i]-position);let dist=length(to_bird);
  if (dist<0.0001) {continue;}
  let dist_sq=(dist*dist);
  if (dist_sq>zone_sq) {continue;}
  let percent=(dist_sq/zone_sq);
  if (percent<separation_thresh) {
   velocity=(velocity-(normalize(to_bird)*vec3<f32>((((separation_thresh/percent)-1.0)*u.delta_time))));
  } else {
   if (percent<alignment_thresh) {
    velocity=(velocity+(normalize(velocities[i])*vec3<f32>((((0.5-(cos((((percent-separation_thresh)/(alignment_thresh-separation_thresh))*(3.141592653589793*2.0)))*0.5))+0.5)*u.delta_time))));
   } else {
    let thresh_delta=(1.0-alignment_thresh);var adjusted:f32;
    if (thresh_delta==0.0) {adjusted=1.0;} else {adjusted=((percent-alignment_thresh)/thresh_delta);}
    velocity=(velocity+(normalize(to_bird)*vec3<f32>(((0.5-((cos((adjusted*(3.141592653589793*2.0)))*-0.5)+0.5))*u.delta_time))));
   }
  }
 }
 if (length(velocity)>limit) {velocity=(normalize(velocity)*vec3<f32>(limit));}
 velocities[bird]=velocity;
}
fn tsl_mod_float(x:f32,y:f32)->f32{return x-y*floor(x/y);}
@compute @workgroup_size(64) fn position(@builtin(global_invocation_id) id:vec3<u32>){
 let i=id.x;if i>=8192u {return;}
 positions[i]=(positions[i]+((velocities[i]*vec3<f32>(u.delta_time))*vec3<f32>(15.0)));
 phases[i]=tsl_mod_float((((phases[i]+u.delta_time)+((length(velocities[i].xz)*u.delta_time)*3.0))+((max(velocities[i].y,0.0)*u.delta_time)*6.0)),62.83);
}";
/// The sky (MeshBasicNodeMaterial with the vertex color varying, BackSide)
/// and the birds (NodeMaterial: black, the vertexNode), both with the
/// scene's linear fog; then the output pass (NeutralToneMapping, sRGB).
const DRAW: &str = "struct R{projection:mat4x4<f32>,view:mat4x4<f32>,inverse_projection:mat4x4<f32>,sky:mat4x4<f32>,birds:mat4x4<f32>,fog:vec4<f32>,params:vec4<f32>}
@group(0) @binding(0) var<uniform> r:R;
@group(0) @binding(1) var<storage,read> positions:array<vec3<f32>>;
@group(0) @binding(2) var<storage,read> velocities:array<vec3<f32>>;
@group(0) @binding(3) var<storage,read> phases:array<f32>;
fn fog(color:vec3<f32>,depth:f32)->vec4<f32>{return vec4<f32>(mix(color,r.fog.xyz,smoothstep(r.params.x,r.params.y,depth)),1.0);}
struct Sky{@builtin(position) clip:vec4<f32>,@location(0) view:vec3<f32>,@location(1) color:vec4<f32>}
@vertex fn sky_vs(@location(0) position:vec3<f32>)->Sky{
 var out:Sky;out.color=vec4<f32>((0.25-position.y),(-0.25-position.y),(1.5+position.y),1.0);
 let model_view=(r.view*r.sky);out.view=(model_view*vec4<f32>(position,1.0)).xyz;
 out.clip=(r.projection*vec4<f32>(out.view,1.0));return out;}
@fragment fn sky_fs(v:Sky)->@location(0) vec4<f32>{
 let color=max(vec4<f32>(v.color.xyz,1.0),vec4<f32>(0.0));return fog(color.xyz,-v.view.z);}
struct Bird{@builtin(position) clip:vec4<f32>,@location(0) clip_space:vec4<f32>}
@vertex fn bird_vs(@builtin(instance_index) instance:u32,@builtin(vertex_index) vertex:u32,@location(0) local:vec3<f32>)->Bird{
 var position=local;let phase=phases[instance];var velocity=normalize(velocities[instance]);
 if ((f32(vertex)==4.0)||(f32(vertex)==7.0)) {position.y=(sin(phase)*5.0);}
 velocity.z=(velocity.z*-1.0);
 let xz=length(velocity.xz);let cosry=(velocity.x/xz);let sinry=(velocity.z/xz);
 let sinrz=(velocity.y/1.0);let cosrz=(sqrt((1.0-(velocity.y*velocity.y)))/1.0);
 var world=((mat3x3<f32>(cosry,0.0,(-sinry),0.0,1.0,0.0,sinry,0.0,cosry)*mat3x3<f32>(cosrz,sinrz,0.0,(-sinrz),cosrz,0.0,0.0,0.0,1.0))*(r.birds*vec4<f32>(position,1.0)).xyz);
 world=(world+positions[instance]);
 let clip=((r.projection*r.view)*vec4<f32>(world,1.0));return Bird(clip,clip);}
@fragment fn bird_fs(v:Bird)->@location(0) vec4<f32>{
 let p=(r.inverse_projection*v.clip_space);let view=(p.xyz/vec3<f32>(p.w));return fog(vec3<f32>(0.0),-view.z);}
@group(1) @binding(0) var scene:texture_2d<f32>;
@vertex fn output_vs(@builtin(vertex_index) i:u32)->@builtin(position) vec4<f32>{
 let p=array(vec2(-1.0,-1.0),vec2(3.0,-1.0),vec2(-1.0,3.0));return vec4(p[i],0.0,1.0);}
fn neutral(color:vec3<f32>,exposure:f32)->vec3<f32>{
 var c=(color*vec3<f32>(exposure));let x=min(c.x,min(c.y,c.z));var offset:f32;
 if (x<0.08) {offset=(x-(6.25*(x*x)));} else {offset=0.04;}
 c=(c-vec3<f32>(offset));let peak=max(c.x,max(c.y,c.z));if (peak<0.76) {return c;}
 let d=(1.0-0.76);let new_peak=(1.0-((d*d)/(peak+(d-0.76))));c=(c*vec3<f32>((new_peak/peak)));
 return mix(c,vec3<f32>(new_peak),(1.0-(1.0/((0.15*(peak-new_peak))+1.0))));}
fn srgb(color:vec3<f32>)->vec3<f32>{return mix(((pow(color,vec3<f32>(0.41666))*vec3<f32>(1.055))-vec3<f32>(0.055)),(color*vec3<f32>(12.92)),vec3<f32>((color<=vec3<f32>(0.0031308))));}
@fragment fn output_fs(@builtin(position) p:vec4<f32>)->@location(0) vec4<f32>{
 let texel=textureLoad(scene,vec2<i32>(p.xy),0);let a=clamp(texel.w,0.0,1.0);
 var color=vec4<f32>(0.0);if (a!=0.0) {color=vec4<f32>((texel.xyz/vec3<f32>(a)),a);}
 let mapped=neutral(color.xyz,1.0);return vec4<f32>((srgb(mapped)*vec3<f32>(color.w)),color.w);}";
struct Targets {
    samples: u32,
    width: u32,
    height: u32,
    /// The half-float scene target (multisampled, with its resolve) and depth.
    color: wgpu::TextureView,
    resolve: Option<wgpu::TextureView>,
    depth: wgpu::TextureView,
    output: wgpu::BindGroup,
    pipelines: [wgpu::RenderPipeline; 2],
    screen: RenderTarget,
}
pub(super) struct Demo {
    controls: Controls,
    /// separation, alignment, cohesion.
    params: [f64; 3],
    last: f64,
    time: f64,
    pending: bool,
    delta: f64,
    pointer: Vector2,
    uniforms: wgpu::Buffer,
    draw_uniforms: wgpu::Buffer,
    compute_bind: wgpu::BindGroup,
    draw_bind: wgpu::BindGroup,
    velocity: wgpu::ComputePipeline,
    position: wgpu::ComputePipeline,
    sky: (wgpu::Buffer, u32),
    bird: wgpu::Buffer,
    module: wgpu::ShaderModule,
    draw_layout: wgpu::PipelineLayout,
    output_group: wgpu::BindGroupLayout,
    output_layout: wgpu::PipelineLayout,
    targets: Option<Targets>,
    format: Option<wgpu::TextureFormat>,
    output_pipeline: Option<wgpu::RenderPipeline>,
}
fn storage_entry(
    binding: u32,
    read_only: bool,
    visibility: wgpu::ShaderStages,
) -> wgpu::BindGroupLayoutEntry {
    wgpu::BindGroupLayoutEntry {
        binding,
        visibility,
        ty: wgpu::BindingType::Buffer {
            ty: wgpu::BufferBindingType::Storage { read_only },
            has_dynamic_offset: false,
            min_binding_size: None,
        },
        count: None,
    }
}
fn uniform_entry(visibility: wgpu::ShaderStages) -> wgpu::BindGroupLayoutEntry {
    wgpu::BindGroupLayoutEntry {
        binding: 0,
        visibility,
        ty: wgpu::BindingType::Buffer {
            ty: wgpu::BufferBindingType::Uniform,
            has_dynamic_offset: false,
            min_binding_size: None,
        },
        count: None,
    }
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, _id: u32, r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 50.,
            near: 1.,
            far: 5000.,
            aspect,
            ..Default::default()
        }));
        let n = s.get_mut(c)?;
        n.position = Vector3::new(0., 0., 1000.);
        n.quaternion = Quaternion::IDENTITY;
        // The fixture's Math.random: positions and velocities, bird by bird.
        let mut seed = 186u32;
        let mut random = || {
            seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
            seed as f64 / 4294967296.
        };
        let (mut positions, mut velocities) = (vec![], vec![]);
        for _ in 0..BIRDS {
            let p = [0; 3].map(|_| (random() * BOUNDS - BOUNDS / 2.) as f32);
            let v = [0; 3].map(|_| ((random() - 0.5) * 10.) as f32);
            // array<vec3<f32>> has a 16-byte stride.
            positions.extend([p[0], p[1], p[2], 0.]);
            velocities.extend([v[0], v[1], v[2], 0.]);
        }
        use wgpu::util::DeviceExt;
        let storage = |label, data: &[f32]| {
            r.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some(label),
                    contents: bytemuck::cast_slice(data),
                    usage: wgpu::BufferUsages::STORAGE,
                })
        };
        let position_buffer = storage("bird positions", &positions);
        let velocity_buffer = storage("bird velocities", &velocities);
        let phase_buffer = storage("bird phases", &vec![1f32; BIRDS as usize]);
        let uniforms = r.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("birds compute"),
            size: 48,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let draw_uniforms = r.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("birds draw"),
            size: 64 * 5 + 32,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let compute_stage = wgpu::ShaderStages::COMPUTE;
        let compute_group = r
            .device
            .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("birds compute"),
                entries: &[
                    uniform_entry(compute_stage),
                    storage_entry(1, false, compute_stage),
                    storage_entry(2, false, compute_stage),
                    storage_entry(3, false, compute_stage),
                ],
            });
        let draw_stage = wgpu::ShaderStages::VERTEX_FRAGMENT;
        let draw_group = r
            .device
            .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("birds draw"),
                entries: &[
                    uniform_entry(draw_stage),
                    storage_entry(1, true, wgpu::ShaderStages::VERTEX),
                    storage_entry(2, true, wgpu::ShaderStages::VERTEX),
                    storage_entry(3, true, wgpu::ShaderStages::VERTEX),
                ],
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
                        resource: position_buffer.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 2,
                        resource: velocity_buffer.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 3,
                        resource: phase_buffer.as_entire_binding(),
                    },
                ],
            })
        };
        let compute_bind = bind(&compute_group, &uniforms);
        let draw_bind = bind(&draw_group, &draw_uniforms);
        let compute_module = r.device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("birds compute"),
            source: wgpu::ShaderSource::Wgsl(COMPUTE.into()),
        });
        let compute_layout = r
            .device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[&compute_group],
                push_constant_ranges: &[],
            });
        let compute = |entry| {
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
        let (velocity, position) = (compute("velocity"), compute("position"));
        let module = r.device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("birds draw"),
            source: wgpu::ShaderSource::Wgsl(DRAW.into()),
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
                label: Some("birds output"),
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
        // IcosahedronGeometry( 1, 6 ) and BirdGeometry (scaled by 0.2).
        let sky: Vec<f32> = IcosahedronGeometry::build(1., 6)?
            .positions()?
            .iter()
            .flat_map(|p| [p.x as f32, p.y as f32, p.z as f32])
            .collect();
        let bird: Vec<f32> = [
            0., 0., -20., 0., -8., 10., 0., 0., 30., 0., 0., -15., -20., 0., 5., 0., 0., 15., 0.,
            0., 15., 20., 0., 5., 0., 0., -15.,
        ]
        .map(|v: f64| (v * 0.2) as f32)
        .to_vec();
        let vertex = |label, data: &[f32]| {
            r.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some(label),
                    contents: bytemuck::cast_slice(data),
                    usage: wgpu::BufferUsages::VERTEX,
                })
        };
        let mut controls = Controls::new(None, (0., f64::INFINITY), PI, true);
        controls.update(s, c)?;
        Ok(Self {
            controls,
            params: [15., 20., 20.],
            last: 0.,
            time: 0.,
            pending: true,
            delta: 0.,
            pointer: Vector2::ZERO,
            uniforms,
            draw_uniforms,
            compute_bind,
            draw_bind,
            velocity,
            position,
            sky: (vertex("sky", &sky), (sky.len() / 3) as u32),
            bird: vertex("bird", &bird),
            module,
            draw_layout,
            output_group,
            output_layout,
            targets: None,
            format: None,
            output_pipeline: None,
        })
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, dt: f64, animate: bool) -> Result<()> {
        if animate {
            self.time += dt;
            self.pending = true;
        }
        Ok(())
    }
    /// render(): deltaTime from performance.now(), capped at 1 s.
    pub fn prepare(&mut self, _s: &mut Scene, _c: Object3D, _r: &Renderer) -> Result<()> {
        if std::mem::take(&mut self.pending) {
            let now = self.time * 1000.;
            let mut delta = (now - self.last) / 1000.;
            if delta > 1. {
                delta = 1.;
            }
            self.last = now;
            self.delta = delta;
        } else {
            self.delta = -1.;
        }
        Ok(())
    }
    fn resize(&mut self, r: &Renderer, out: &RenderTarget) -> Result<()> {
        let samples = out.options.samples.max(1);
        let texture = |format, samples, usage| {
            r.device
                .create_texture(&wgpu::TextureDescriptor {
                    label: Some("birds target"),
                    size: wgpu::Extent3d {
                        width: out.width,
                        height: out.height,
                        depth_or_array_layers: 1,
                    },
                    mip_level_count: 1,
                    sample_count: samples,
                    dimension: wgpu::TextureDimension::D2,
                    format,
                    usage,
                    view_formats: &[],
                })
                .create_view(&Default::default())
        };
        let attachment = wgpu::TextureUsages::RENDER_ATTACHMENT;
        let sampled = attachment | wgpu::TextureUsages::TEXTURE_BINDING;
        let half = wgpu::TextureFormat::Rgba16Float;
        let color = texture(
            half,
            samples,
            if samples > 1 { attachment } else { sampled },
        );
        let resolve = (samples > 1).then(|| texture(half, 1, sampled));
        let depth = texture(wgpu::TextureFormat::Depth24Plus, samples, attachment);
        let output = r.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("birds output"),
            layout: &self.output_group,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(resolve.as_ref().unwrap_or(&color)),
            }],
        });
        let pipeline = |vs, fs, cull, stride| {
            r.device
                .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    label: Some(vs),
                    layout: Some(&self.draw_layout),
                    vertex: wgpu::VertexState {
                        module: &self.module,
                        entry_point: Some(vs),
                        compilation_options: Default::default(),
                        buffers: &[wgpu::VertexBufferLayout {
                            array_stride: stride,
                            step_mode: wgpu::VertexStepMode::Vertex,
                            attributes: &wgpu::vertex_attr_array![0 => Float32x3],
                        }],
                    },
                    fragment: Some(wgpu::FragmentState {
                        module: &self.module,
                        entry_point: Some(fs),
                        compilation_options: Default::default(),
                        targets: &[Some(half.into())],
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
        // BackSide culls front faces; the birds are DoubleSide.
        let pipelines = [
            pipeline("sky_vs", "sky_fs", Some(wgpu::Face::Front), 12),
            pipeline("bird_vs", "bird_fs", None, 12),
        ];
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
            samples,
            width: out.width,
            height: out.height,
            color,
            resolve,
            depth,
            output,
            pipelines,
            screen,
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
            t.width != out.width
                || t.height != out.height
                || t.samples != out.options.samples.max(1)
        }) {
            self.resize(r, out)?;
        }
        let format = out.options.format;
        if self.format != Some(format) {
            self.output_pipeline = Some(r.device.create_render_pipeline(
                &wgpu::RenderPipelineDescriptor {
                    label: Some("birds output"),
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
                },
            ));
            self.format = Some(format);
        }
        s.update()?;
        let (camera, world) = s.camera(c)?;
        let Camera::Perspective(p) = camera else {
            return Err(Error::Invalid("birds camera"));
        };
        let projection = camera.projection_matrix()?;
        let view = world.inverse();
        // raycaster.setFromCamera( pointer, camera ).
        let origin = world.w_axis.truncate();
        let tan = (p.fov.to_radians() * 0.5).tan();
        let direction = world
            .transform_vector3(Vector3::new(
                self.pointer.x * tan * p.aspect,
                self.pointer.y * tan,
                -1.,
            ))
            .normalize();
        let f = |v: f64| v as f32;
        let mut encoder = r.device.create_command_encoder(&Default::default());
        if self.delta >= 0. {
            let [separation, alignment, cohesion] = self.params;
            let data = [
                f(separation),
                f(alignment),
                f(cohesion),
                f(self.delta),
                f(origin.x),
                f(origin.y),
                f(origin.z),
                0.,
                f(direction.x),
                f(direction.y),
                f(direction.z),
                0.,
            ];
            r.queue
                .write_buffer(&self.uniforms, 0, bytemuck::cast_slice(&data));
            let mut pass = encoder.begin_compute_pass(&Default::default());
            pass.set_bind_group(0, &self.compute_bind, &[]);
            pass.set_pipeline(&self.velocity);
            pass.dispatch_workgroups(BIRDS / 64, 1, 1);
            pass.set_pipeline(&self.position);
            pass.dispatch_workgroups(BIRDS / 64, 1, 1);
            // The pointer moves off screen after each render.
            self.pointer.y = 10.;
        }
        let mut data: Vec<f32> = vec![];
        let sky = Matrix4::from_scale_rotation_translation(
            Vector3::splat(1200.),
            Quaternion::from_rotation_z(0.75),
            Vector3::ZERO,
        );
        let birds = Matrix4::from_rotation_y(PI / 2.);
        for m in [projection, view, projection.inverse(), sky, birds] {
            data.extend(m.to_cols_array().map(f));
        }
        data.extend([1., 1., 1., 0., 700., 3000., 0., 0.]);
        r.queue
            .write_buffer(&self.draw_uniforms, 0, bytemuck::cast_slice(&data));
        let t = self
            .targets
            .as_ref()
            .ok_or(Error::Invalid("birds targets"))?;
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("birds scene"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &t.color,
                    depth_slice: None,
                    resolve_target: t.resolve.as_ref(),
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
            pass.set_bind_group(0, &self.draw_bind, &[]);
            pass.set_pipeline(&t.pipelines[0]);
            pass.set_vertex_buffer(0, self.sky.0.slice(..));
            pass.draw(0..self.sky.1, 0..1);
            pass.set_pipeline(&t.pipelines[1]);
            pass.set_vertex_buffer(0, self.bird.slice(..));
            pass.draw(0..9, 0..BIRDS);
        }
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("birds output"),
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
            pass.set_pipeline(
                self.output_pipeline
                    .as_ref()
                    .ok_or(Error::Invalid("birds output"))?,
            );
            pass.set_bind_group(0, &self.draw_bind, &[]);
            pass.set_bind_group(1, &t.output, &[]);
            pass.draw(0..3, 0..1);
        }
        r.queue.submit([encoder.finish()]);
        Ok(true)
    }
    pub fn output(&self) -> Option<&RenderTarget> {
        self.targets.as_ref().map(|t| &t.screen)
    }
    /// pointermove: the pointer in normalized device coordinates.
    pub fn draw(&mut self, kind: u32, x: f64, y: f64) {
        if kind != 0 {
            return;
        }
        if let Some(window) = web_sys::window() {
            let w = window
                .inner_width()
                .ok()
                .and_then(|v| v.as_f64())
                .unwrap_or(1.);
            let h = window
                .inner_height()
                .ok()
                .and_then(|v| v.as_f64())
                .unwrap_or(1.);
            self.pointer = Vector2::new(x / w * 2. - 1., 1. - y / h * 2.);
        }
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
    /// Separation, alignment and cohesion.
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        *self
            .params
            .get_mut(index)
            .ok_or(Error::Invalid("birds parameter"))? = value as f64;
        Ok(())
    }
    /// A still frame: one render() at the example time.
    pub fn seek(&mut self, t: f64) {
        self.time = t;
        self.pending = true;
    }
}
