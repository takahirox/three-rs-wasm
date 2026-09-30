//! webgpu_compute_sort_bitonic: two bitonic sorts of 16,384 shuffled values,
//! stepped every 100 ms (1 s after a finished sort, which then restarts). The
//! left half is BitonicSort: local swaps in 128-element workgroup arrays with
//! global flips and disperses between them; the right half runs every step as
//! a global swap into a temporary buffer, an align pass and a one-thread pass
//! that sets the next step. Each half shows its buffer as a 128 × 128 grid
//! with the swap zones highlighted, as two canvases would. All passes are
//! compute dispatches over resident storage buffers, as in the original.
use crate::{Error, Result, camera::*, math::*, render_target::*, renderer::*, scene::*};

const SIZE: u32 = 16384;
const WORKGROUP: u32 = 64;
/// BitonicSort's passes (the addon's TSL) and the global-only page's.
const COMPUTE: &str = "@group(0) @binding(0) var<storage,read_write> data:array<u32>;
@group(0) @binding(1) var<storage,read_write> temp:array<u32>;
@group(0) @binding(2) var<storage,read_write> info:array<u32>;
@group(0) @binding(3) var<storage,read_write> randomized:array<u32>;
@group(0) @binding(4) var<storage,read_write> next_block:array<u32>;
var<workgroup> local:array<u32,128>;
fn flip_indices(index:u32,block:u32)->vec2<u32>{
 let offset=((index*2u)/block)*block;let half=block/2u;
 return vec2<u32>(index%half,(block-(index%half))-1u)+vec2<u32>(offset);}
fn disperse_indices(index:u32,span:u32)->vec2<u32>{
 let offset=((index*2u)/span)*span;let half=span/2u;
 return vec2<u32>(index%half,(index%half)+half)+vec2<u32>(offset);}
fn local_swap(a:u32,b:u32){let x=local[a];let y=local[b];local[a]=min(x,y);local[b]=max(x,y);}
@compute @workgroup_size(64) fn swap_local(@builtin(workgroup_id) group:vec3<u32>,@builtin(local_invocation_index) index:u32){
 let offset=128u*group.x;let a=index*2u;let b=a+1u;
 local[a]=data[offset+a];local[b]=data[offset+b];workgroupBarrier();
 var flip=2u;
 for(;flip<=128u;flip=flip<<1u){
  workgroupBarrier();let f=flip_indices(index,flip);local_swap(f.x,f.y);
  var height=flip/2u;
  for(;height>1u;height=height>>1u){workgroupBarrier();let d=disperse_indices(index,height);local_swap(d.x,d.y);}
 }
 workgroupBarrier();data[offset+a]=local[a];data[offset+b]=local[b];}
fn disperse_local_in(group:u32,index:u32,from_temp:bool){
 let offset=128u*group;let a=index*2u;let b=a+1u;
 if from_temp {local[a]=temp[offset+a];local[b]=temp[offset+b];} else {local[a]=data[offset+a];local[b]=data[offset+b];}
 workgroupBarrier();
 var height=128u;
 for(;height>1u;height=height>>1u){workgroupBarrier();let d=disperse_indices(index,height);local_swap(d.x,d.y);}
 workgroupBarrier();
 if from_temp {temp[offset+a]=local[a];temp[offset+b]=local[b];} else {data[offset+a]=local[a];data[offset+b]=local[b];}}
@compute @workgroup_size(64) fn disperse_local_data(@builtin(workgroup_id) group:vec3<u32>,@builtin(local_invocation_index) index:u32){disperse_local_in(group.x,index,false);}
@compute @workgroup_size(64) fn disperse_local_temp(@builtin(workgroup_id) group:vec3<u32>,@builtin(local_invocation_index) index:u32){disperse_local_in(group.x,index,true);}
fn global_swap(i:vec2<u32>,from_temp:bool){
 if from_temp {let x=temp[i.x];let y=temp[i.y];data[i.x]=min(x,y);data[i.y]=max(x,y);}
 else {let x=data[i.x];let y=data[i.y];temp[i.x]=min(x,y);temp[i.y]=max(x,y);}}
@compute @workgroup_size(64) fn flip_data(@builtin(global_invocation_id) id:vec3<u32>){if id.x>=8192u {return;}global_swap(flip_indices(id.x,info[1]),false);}
@compute @workgroup_size(64) fn flip_temp(@builtin(global_invocation_id) id:vec3<u32>){if id.x>=8192u {return;}global_swap(flip_indices(id.x,info[1]),true);}
@compute @workgroup_size(64) fn disperse_data(@builtin(global_invocation_id) id:vec3<u32>){if id.x>=8192u {return;}global_swap(disperse_indices(id.x,info[1]),false);}
@compute @workgroup_size(64) fn disperse_temp(@builtin(global_invocation_id) id:vec3<u32>){if id.x>=8192u {return;}global_swap(disperse_indices(id.x,info[1]),true);}
@compute @workgroup_size(64) fn align(@builtin(global_invocation_id) id:vec3<u32>){if id.x>=16384u {return;}data[id.x]=temp[id.x];}
@compute @workgroup_size(64) fn copy_init(@builtin(global_invocation_id) id:vec3<u32>){if id.x>=16384u {return;}randomized[id.x]=data[id.x];}
@compute @workgroup_size(64) fn copy_reset(@builtin(global_invocation_id) id:vec3<u32>){if id.x>=16384u {return;}data[id.x]=randomized[id.x];}
@compute @workgroup_size(1) fn reset(){info[0]=1u;info[1]=2u;info[2]=2u;}
@compute @workgroup_size(1) fn set_algo(){
 if info[0]==1u {info[0]=3u;info[1]=256u;info[2]=256u;}
 else if info[0]==2u {info[0]=3u;let next=info[2]*2u;info[1]=next;info[2]=next;}
 else {let next=info[1]/2u;info[0]=select(4u,2u,next<=128u);info[1]=next;}}
// The global-only page: info[0] the step, [2] the highest block; next_block[0].
@compute @workgroup_size(64) fn global_step(@builtin(global_invocation_id) id:vec3<u32>){
 if id.x>=8192u {return;}
 let height=next_block[0];let algo=info[0];
 if algo==3u {let i=flip_indices(id.x,height);
  if data[i.y]<data[i.x] {temp[i.x]=data[i.y];temp[i.y]=data[i.x];} else {temp[i.x]=data[i.x];temp[i.y]=data[i.y];}}
 else if algo==4u {let i=disperse_indices(id.x,height);
  if data[i.y]<data[i.x] {temp[i.x]=data[i.y];temp[i.y]=data[i.x];} else {temp[i.x]=data[i.x];temp[i.y]=data[i.y];}}}
@compute @workgroup_size(1) fn global_set_algo(){
 var height=next_block[0];var highest=info[2];height=height/2u;
 if height==1u {highest=highest*2u;
  if highest==32768u {info[0]=0u;height=0u;} else {info[0]=3u;height=highest;}}
 else {info[0]=4u;}
 next_block[0]=height;info[2]=highest;}
@compute @workgroup_size(1) fn global_reset(){info[0]=3u;next_block[0]=2u;info[2]=2u;}";
/// The display (MeshBasicNodeMaterial's colorNode) and the output pass.
const DRAW: &str = "struct U{model_view_projection:mat4x4<f32>,params:vec4<f32>,source:vec4<f32>}
@group(0) @binding(0) var<uniform> u:U;
@group(0) @binding(1) var<storage,read> elements:array<u32>;
@group(0) @binding(2) var<storage,read> info:array<u32>;
@group(0) @binding(3) var<storage,read> block:array<u32>;
struct V{@builtin(position) clip:vec4<f32>,@location(0) uv:vec2<f32>}
@vertex fn vs(@location(0) uv:vec2<f32>,@location(1) position:vec3<f32>)->V{return V(u.model_view_projection*vec4<f32>(position,1.0),uv);}
fn element_index(uv:vec2<f32>,width:u32,height:u32)->u32{
 let p=(uv*vec2<f32>(f32(width),f32(height)));let pixel=vec2<u32>(u32(floor(p.x)),u32(floor(p.y)));return ((width*pixel.y)+pixel.x);}
fn element_color(value:f32,width:f32,height:f32)->vec3<f32>{return vec3<f32>((1.0-(value/(width*height))));}
// params: grid width, grid height, highlight, the step type that shows no zones.
@fragment fn fs(v:V)->@location(0) vec4<f32>{
 let index=element_index(v.uv,u32(u.params.x),u32(u.params.y));
 var color=element_color(f32(elements[index]),u.params.x,u.params.y);
 if ((u.params.z==1.0)&&(!(f32(info[0u])==u.params.w))) {
  let span=select(block[0u],info[1u],u.source.x==0.0);
  color.z=f32((f32(info[0u])<=2.0));
  color.x=(color.x*f32(i32(((index%span)<(span/2u)))));
  color.y=(color.y*f32(abs((i32(((index%span)<(span/2u)))-1))));
 }
 return max(vec4<f32>(color,1.0),vec4<f32>(0.0));}
@group(1) @binding(0) var scene:texture_2d<f32>;
struct O{origin:vec4<f32>}
@group(1) @binding(1) var<uniform> o:O;
@vertex fn output_vs(@builtin(vertex_index) i:u32)->@builtin(position) vec4<f32>{
 let p=array(vec2(-1.0,-1.0),vec2(3.0,-1.0),vec2(-1.0,3.0));return vec4(p[i],0.0,1.0);}
fn srgb(color:vec3<f32>)->vec3<f32>{return mix(((pow(color,vec3<f32>(0.41666))*vec3<f32>(1.055))-vec3<f32>(0.055)),(color*vec3<f32>(12.92)),vec3<f32>((color<=vec3<f32>(0.0031308))));}
@fragment fn output_fs(@builtin(position) p:vec4<f32>)->@location(0) vec4<f32>{
 let texel=textureLoad(scene,vec2<i32>(p.xy-o.origin.xy),0);let a=clamp(texel.w,0.0,1.0);
 var color=vec4<f32>(0.0);if (a!=0.0) {color=vec4<f32>((texel.xyz/vec3<f32>(a)),a);}
 return vec4<f32>((srgb(color.xyz)*vec3<f32>(color.w)),color.w);}";
/// One half of the page: its buffers, step state and display.
struct Side {
    /// The elements, temporary, info, randomized and next-block buffers.
    _buffers: [wgpu::Buffer; 5],
    compute: wgpu::BindGroup,
    draw: wgpu::BindGroup,
    uniforms: wgpu::Buffer,
    /// stepAnimation: the page's step counter and the next timer (ms).
    step: u32,
    next: f64,
    background: Color,
}
struct Targets {
    width: u32,
    height: u32,
    halves: [(wgpu::TextureView, wgpu::BindGroup); 2],
    screen: RenderTarget,
}
pub(super) struct Demo {
    sides: [Side; 2],
    /// BitonicSort's CPU state: currentDispatch, globalOpsRemaining,
    /// globalOpsInSpan and whether the temporary buffer holds the data.
    dispatch: u32,
    remaining: u32,
    in_span: u32,
    read_temp: bool,
    step_count: u32,
    pipelines: std::collections::HashMap<&'static str, wgpu::ComputePipeline>,
    plane: (wgpu::Buffer, wgpu::Buffer),
    display: wgpu::RenderPipeline,
    module: wgpu::ShaderModule,
    output_group: wgpu::BindGroupLayout,
    output_layout: wgpu::PipelineLayout,
    output: Option<(wgpu::TextureFormat, wgpu::RenderPipeline)>,
    output_uniforms: [wgpu::Buffer; 2],
    targets: Option<Targets>,
    highlight: f32,
    time: f64,
    pending: bool,
}
/// randomizeDataArray: the page's Fisher–Yates shuffle on the fixture's sequence.
fn shuffle(seed: &mut u32) -> Vec<u32> {
    let mut array: Vec<u32> = (0..SIZE).collect();
    let mut current = array.len();
    while current != 0 {
        *seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
        let random = (*seed as f64 / 4294967296. * current as f64).floor() as usize;
        current -= 1;
        array.swap(current, random);
    }
    array
}
impl Demo {
    pub async fn create(_s: &mut Scene, _c: Object3D, _id: u32, r: &Renderer) -> Result<Self> {
        let mut seed = 186u32;
        let arrays = [shuffle(&mut seed), shuffle(&mut seed)];
        use wgpu::util::DeviceExt;
        let buffer = |label, data: &[u32]| {
            r.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some(label),
                    contents: bytemuck::cast_slice(data),
                    usage: wgpu::BufferUsages::STORAGE,
                })
        };
        let storage = |binding, read_only, visibility| wgpu::BindGroupLayoutEntry {
            binding,
            visibility,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Storage { read_only },
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        };
        let compute_group = r
            .device
            .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("bitonic compute"),
                entries: &(0..5)
                    .map(|i| storage(i, false, wgpu::ShaderStages::COMPUTE))
                    .collect::<Vec<_>>(),
            });
        let fragment = wgpu::ShaderStages::FRAGMENT;
        let draw_group = r
            .device
            .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("bitonic draw"),
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
                    storage(1, true, fragment),
                    storage(2, true, fragment),
                    storage(3, true, fragment),
                ],
            });
        let mut sides = vec![];
        for (k, array) in arrays.iter().enumerate() {
            let data = buffer("bitonic elements", array);
            let temp = buffer("bitonic temp", array);
            let randomized = buffer("bitonic randomized", &vec![0; SIZE as usize]);
            // BitonicSort's info starts at SWAP_LOCAL; the global page's at FLIP_GLOBAL.
            let info = buffer("bitonic info", if k == 0 { &[1, 2, 2] } else { &[3, 2, 2] });
            let block = buffer("bitonic next block", &[2]);
            let compute = r.device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: None,
                layout: &compute_group,
                entries: &[&data, &temp, &info, &randomized, &block]
                    .iter()
                    .enumerate()
                    .map(|(i, b)| wgpu::BindGroupEntry {
                        binding: i as u32,
                        resource: b.as_entire_binding(),
                    })
                    .collect::<Vec<_>>(),
            });
            let uniforms = r.device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("bitonic display"),
                size: 96,
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });
            let draw = r.device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: None,
                layout: &draw_group,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: uniforms.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: data.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 2,
                        resource: info.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 3,
                        resource: block.as_entire_binding(),
                    },
                ],
            });
            sides.push(Side {
                _buffers: [data, temp, info, randomized, block],
                compute,
                draw,
                uniforms,
                step: 0,
                next: 0.,
                background: Color::from_hex(if k == 0 { 0x313131 } else { 0x212121 }),
            });
        }
        let compute_module = r.device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("bitonic compute"),
            source: wgpu::ShaderSource::Wgsl(COMPUTE.into()),
        });
        let compute_layout = r
            .device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[&compute_group],
                push_constant_ranges: &[],
            });
        let mut pipelines = std::collections::HashMap::new();
        for entry in [
            "swap_local",
            "disperse_local_data",
            "disperse_local_temp",
            "flip_data",
            "flip_temp",
            "disperse_data",
            "disperse_temp",
            "align",
            "copy_init",
            "copy_reset",
            "reset",
            "set_algo",
            "global_step",
            "global_set_algo",
            "global_reset",
        ] {
            pipelines.insert(
                entry,
                r.device
                    .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                        label: Some(entry),
                        layout: Some(&compute_layout),
                        module: &compute_module,
                        entry_point: Some(entry),
                        compilation_options: Default::default(),
                        cache: None,
                    }),
            );
        }
        let module = r.device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("bitonic draw"),
            source: wgpu::ShaderSource::Wgsl(DRAW.into()),
        });
        let draw_layout = r
            .device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[&draw_group],
                push_constant_ranges: &[],
            });
        let display = r
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("bitonic display"),
                layout: Some(&draw_layout),
                vertex: wgpu::VertexState {
                    module: &module,
                    entry_point: Some("vs"),
                    compilation_options: Default::default(),
                    buffers: &[
                        wgpu::VertexBufferLayout {
                            array_stride: 8,
                            step_mode: wgpu::VertexStepMode::Vertex,
                            attributes: &wgpu::vertex_attr_array![0 => Float32x2],
                        },
                        wgpu::VertexBufferLayout {
                            array_stride: 12,
                            step_mode: wgpu::VertexStepMode::Vertex,
                            attributes: &wgpu::vertex_attr_array![1 => Float32x3],
                        },
                    ],
                },
                fragment: Some(wgpu::FragmentState {
                    module: &module,
                    entry_point: Some("fs"),
                    compilation_options: Default::default(),
                    targets: &[Some(wgpu::TextureFormat::Rgba16Float.into())],
                }),
                primitive: wgpu::PrimitiveState {
                    cull_mode: Some(wgpu::Face::Back),
                    ..Default::default()
                },
                depth_stencil: None,
                multisample: Default::default(),
                multiview: None,
                cache: None,
            });
        let output_group = r
            .device
            .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("bitonic output"),
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: fragment,
                        ty: wgpu::BindingType::Texture {
                            sample_type: wgpu::TextureSampleType::Float { filterable: false },
                            view_dimension: wgpu::TextureViewDimension::D2,
                            multisampled: false,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: fragment,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                ],
            });
        let output_layout = r
            .device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[&draw_group, &output_group],
                push_constant_ranges: &[],
            });
        // PlaneGeometry( 1, 1 ): uv and position streams, one index list.
        let plane = crate::geometry::PlaneGeometry::build(1., 1., 1, 1)?;
        let mut uvs = vec![];
        let mut positions = vec![];
        let uv = plane
            .attributes
            .get("uv")
            .ok_or(Error::Invalid("plane uv"))?;
        for (i, p) in plane.positions()?.iter().enumerate() {
            uvs.extend([
                uv.get_component(i, 0)? as f32,
                uv.get_component(i, 1)? as f32,
            ]);
            positions.extend([p.x as f32, p.y as f32, p.z as f32]);
        }
        let index = plane.index.clone().ok_or(Error::Invalid("plane index"))?;
        let vertex = r
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("bitonic plane"),
                contents: bytemuck::cast_slice(&[uvs, positions].concat()),
                usage: wgpu::BufferUsages::VERTEX,
            });
        let indices = r
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("bitonic plane index"),
                contents: bytemuck::cast_slice(&index),
                usage: wgpu::BufferUsages::INDEX,
            });
        let output_uniforms = [0, 1].map(|_| {
            r.device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("bitonic output"),
                size: 16,
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            })
        });
        let sides: [Side; 2] = sides
            .try_into()
            .map_err(|_| Error::Invalid("bitonic sides"))?;
        // BitonicSort's step count for 16,384 values in 128-value workgroups.
        let flips = SIZE.ilog2() - (WORKGROUP * 2).ilog2();
        let step_count = (1..=flips)
            .fold((1, 0), |(steps, disperses), _| {
                (steps + 2 + disperses, disperses + 1)
            })
            .0;
        let demo = Self {
            sides,
            dispatch: 0,
            remaining: 0,
            in_span: 0,
            read_temp: false,
            step_count,
            pipelines,
            plane: (vertex, indices),
            display,
            module,
            output_group,
            output_layout,
            output: None,
            output_uniforms,
            targets: None,
            highlight: 1.,
            time: 0.,
            pending: true,
        };
        // computeInit on both halves.
        let mut encoder = r.device.create_command_encoder(&Default::default());
        for k in 0..2 {
            demo.dispatch(&mut encoder, k, "copy_init", SIZE / WORKGROUP);
        }
        r.queue.submit([encoder.finish()]);
        Ok(demo)
    }
    fn dispatch(&self, encoder: &mut wgpu::CommandEncoder, side: usize, entry: &str, groups: u32) {
        let mut pass = encoder.begin_compute_pass(&Default::default());
        pass.set_pipeline(&self.pipelines[entry]);
        pass.set_bind_group(0, &self.sides[side].compute, &[]);
        pass.dispatch_workgroups(groups, 1, 1);
    }
    /// BitonicSort.computeStep( renderer ).
    fn compute_step(&mut self, encoder: &mut wgpu::CommandEncoder) {
        let half = SIZE / 2 / WORKGROUP;
        if self.dispatch == 0 {
            self.dispatch(encoder, 0, "swap_local", half);
            self.remaining = 1;
            self.in_span = 1;
        } else if self.remaining > 0 {
            let flip = self.remaining == self.in_span;
            let entry = match (flip, self.read_temp) {
                (true, false) => "flip_data",
                (true, true) => "flip_temp",
                (false, false) => "disperse_data",
                (false, true) => "disperse_temp",
            };
            self.dispatch(encoder, 0, entry, half);
            self.read_temp = !self.read_temp;
            self.remaining -= 1;
        } else {
            let entry = if self.read_temp {
                "disperse_local_temp"
            } else {
                "disperse_local_data"
            };
            self.dispatch(encoder, 0, entry, half);
            self.in_span += 1;
            self.remaining = self.in_span;
        }
        self.dispatch += 1;
        if self.dispatch == self.step_count {
            if self.read_temp {
                self.dispatch(encoder, 0, "align", SIZE / WORKGROUP);
                self.read_temp = false;
            }
            self.dispatch(encoder, 0, "reset", 1);
            self.dispatch = 0;
            self.remaining = 0;
            self.in_span = 0;
        } else {
            self.dispatch(encoder, 0, "set_algo", 1);
        }
    }
    /// Both pages' stepAnimation timers up to the example time.
    fn advance(&mut self, r: &Renderer) {
        let now = self.time * 1000.;
        let mut encoder = r.device.create_command_encoder(&Default::default());
        let max_steps = SIZE.ilog2() * (SIZE.ilog2() + 1) / 2;
        for k in 0..2 {
            while self.sides[k].next <= now {
                let finished = if k == 0 {
                    if self.sides[0].step < self.step_count {
                        self.compute_step(&mut encoder);
                        self.sides[0].step += 1;
                    } else {
                        self.dispatch(&mut encoder, 0, "copy_reset", SIZE / WORKGROUP);
                        self.sides[0].step = 0;
                    }
                    self.sides[0].step == self.step_count
                } else {
                    if self.sides[1].step != max_steps {
                        self.dispatch(&mut encoder, 1, "global_step", SIZE / 2 / WORKGROUP);
                        self.dispatch(&mut encoder, 1, "align", SIZE / WORKGROUP);
                        self.dispatch(&mut encoder, 1, "global_set_algo", 1);
                        self.sides[1].step += 1;
                    } else {
                        self.dispatch(&mut encoder, 1, "copy_reset", SIZE / WORKGROUP);
                        self.dispatch(&mut encoder, 1, "global_reset", 1);
                        self.sides[1].step = 0;
                    }
                    self.sides[1].step == max_steps
                };
                self.sides[k].next += if finished { 1000. } else { 100. };
            }
        }
        r.queue.submit([encoder.finish()]);
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, dt: f64, animate: bool) -> Result<()> {
        if animate {
            self.time += dt;
            self.pending = true;
        }
        Ok(())
    }
    pub fn prepare(&mut self, _s: &mut Scene, _c: Object3D, r: &Renderer) -> Result<()> {
        if std::mem::take(&mut self.pending) {
            self.advance(r);
        }
        Ok(())
    }
    fn resize(&mut self, r: &Renderer, out: &RenderTarget) -> Result<()> {
        let half = (out.width / 2).max(1);
        let halves = [0, 1].map(|k| {
            let view = r
                .device
                .create_texture(&wgpu::TextureDescriptor {
                    label: Some("bitonic half"),
                    size: wgpu::Extent3d {
                        width: half,
                        height: out.height,
                        depth_or_array_layers: 1,
                    },
                    mip_level_count: 1,
                    sample_count: 1,
                    dimension: wgpu::TextureDimension::D2,
                    format: wgpu::TextureFormat::Rgba16Float,
                    usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                        | wgpu::TextureUsages::TEXTURE_BINDING,
                    view_formats: &[],
                })
                .create_view(&Default::default());
            r.queue.write_buffer(
                &self.output_uniforms[k],
                0,
                bytemuck::cast_slice(&[(k as u32 * half) as f32, 0., 0., 0.]),
            );
            let bind = r.device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: None,
                layout: &self.output_group,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: self.output_uniforms[k].as_entire_binding(),
                    },
                ],
            });
            (view, bind)
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
            width: out.width,
            height: out.height,
            halves,
            screen,
        });
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
            .targets
            .as_ref()
            .is_none_or(|t| t.width != out.width || t.height != out.height)
        {
            self.resize(r, out)?;
        }
        let format = out.options.format;
        if self.output.as_ref().is_none_or(|(f, _)| *f != format) {
            let pipeline = r
                .device
                .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    label: Some("bitonic output"),
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
        let t = self
            .targets
            .as_ref()
            .ok_or(Error::Invalid("bitonic targets"))?;
        let half = t.width / 2;
        // Each canvas: OrthographicCamera( -aspect, aspect, 1, -1, 0, 2 ) at z 1.
        let aspect = half as f64 / t.height as f64;
        let camera = Camera::Orthographic(OrthographicCamera {
            left: -aspect,
            right: aspect,
            top: 1.,
            bottom: -1.,
            near: 0.,
            far: 2.,
            zoom: 1.,
            view: None,
        });
        let mvp =
            camera.projection_matrix()? * Matrix4::from_translation(Vector3::new(0., 0., -1.));
        let mut encoder = r.device.create_command_encoder(&Default::default());
        for (k, side) in self.sides.iter().enumerate() {
            let mut data: Vec<f32> = mvp.to_cols_array().map(|v| v as f32).to_vec();
            // The left page shows no zones during local swaps, the right none when done.
            data.extend([128., 128., self.highlight, if k == 0 { 1. } else { 0. }]);
            // The left page highlights by the info's swap span, the right by
            // the next block height.
            data.extend([k as f32, 0., 0., 0.]);
            r.queue
                .write_buffer(&side.uniforms, 0, bytemuck::cast_slice(&data));
            let b = side.background.0;
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("bitonic display"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &t.halves[k].0,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: b.x,
                            g: b.y,
                            b: b.z,
                            a: 1.,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                ..Default::default()
            });
            pass.set_pipeline(&self.display);
            pass.set_bind_group(0, &side.draw, &[]);
            pass.set_vertex_buffer(0, self.plane.0.slice(..32));
            pass.set_vertex_buffer(1, self.plane.0.slice(32..));
            pass.set_index_buffer(self.plane.1.slice(..), wgpu::IndexFormat::Uint32);
            pass.draw_indexed(0..6, 0, 0..1);
        }
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("bitonic output"),
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
                .ok_or(Error::Invalid("bitonic output"))?;
            pass.set_pipeline(pipeline);
            for k in 0..2 {
                pass.set_viewport(
                    (k as u32 * half) as f32,
                    0.,
                    half as f32,
                    t.height as f32,
                    0.,
                    1.,
                );
                pass.set_bind_group(0, &self.sides[k].draw, &[]);
                pass.set_bind_group(1, &t.halves[k].1, &[]);
                pass.draw(0..3, 0..1);
            }
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
    /// Display Mode: Elements (0) or Swap Zone Highlight (1).
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        if index != 0 {
            return Err(Error::Invalid("bitonic parameter"));
        }
        self.highlight = value.round();
        Ok(())
    }
    /// A still frame: the step timers run up to the example time.
    pub fn seek(&mut self, t: f64) {
        self.time = t;
        self.pending = true;
    }
}
