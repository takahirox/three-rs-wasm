//! webgl_gpgpu_birds_gltf: 4,096 flocking birds in a 64 × 64
//! GPUComputationRenderer simulation, drawn as the Parrot glTF ( the page's
//! seeded Math.random picks it ) flapping through its morph targets. Each
//! frame the velocity variable gathers every other bird ( separation,
//! alignment and cohesion, flight from the pointer's predator and a pull to
//! the center ) and the position variable integrates the previous velocity;
//! both read the previous frame's textures and render into the other half of
//! their float render-target pairs, as the addon does. The page's GLSL is
//! translated to WGSL with the same expressions and loop order.
//!
//! At load the page bakes the morph animation into a float texture ( 60
//! rows per second, lerped between the targets ) and builds one geometry
//! with a copy of the bird per simulated bird, each vertex carrying its
//! bird's texel reference and seeds. The port bakes the same texture and
//! draws the bird instanced: per vertex its position, color and index, per
//! instance the bird's reference and seed, with the same values the page
//! stores. The material is MeshStandardMaterial ( vertex colors, flat,
//! roughness 1 ) with the page's onBeforeCompile vertex stage, under the
//! hemisphere and directional lights with linear fog; its program is a WGSL
//! port of WebGLRenderer's physical shading ( the r186 DFG LUT, multiple
//! scattering compensation, fog after the sRGB conversion ).
use super::controls_attributes::viewport_css;
use super::gltf_viewer::load_asset;
use crate::{Error, Result, camera::*, math::*, render_target::*, renderer::*, scene::*};
use wgpu::util::DeviceExt;

const WIDTH: u32 = 64;
const BIRDS: u32 = WIDTH * WIDTH;
const BOUNDS: f64 = 800.;
const FLOAT: wgpu::TextureFormat = wgpu::TextureFormat::Rgba32Float;
const DEPTH: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;
/// The Parrot's background and fog color, and the bird size.
const COLOR: u32 = 0xccffff;
const SIZE: f64 = 0.2;
const QUAD: &str = "@vertex fn vs(@builtin(vertex_index) i:u32)->@builtin(position) vec4<f32>{let p=array(vec2(-1.0,-1.0),vec2(3.0,-1.0),vec2(-1.0,3.0));return vec4(p[i],0.0,1.0);}";
/// fragmentShaderPosition.
const POSITION: &str = "
struct U{time:f32,delta:f32,pad0:f32,pad1:f32}
@group(0) @binding(0) var texture_position:texture_2d<f32>;
@group(0) @binding(1) var texture_velocity:texture_2d<f32>;
@group(0) @binding(2) var<uniform> u:U;
fn glsl_mod(x:f32,y:f32)->f32{return x-y*floor(x/y);}
@fragment fn fs(@builtin(position) frag:vec4<f32>)->@location(0) vec4<f32>{
 let texel=vec2<i32>(frag.xy);
 let tmp_pos=textureLoad(texture_position,texel,0);
 let position=tmp_pos.xyz;
 let velocity=textureLoad(texture_velocity,texel,0).xyz;
 var phase=tmp_pos.w;
 phase=glsl_mod((phase+u.delta+length(velocity.xz)*u.delta*3.0+max(velocity.y,0.0)*u.delta*6.0),62.83);
 return vec4(position+velocity*u.delta*15.0,phase);
}";
/// fragmentShaderVelocity ( BOUNDS 800.00 ).
const VELOCITY: &str = "
struct U{time:f32,delta:f32,separation_distance:f32,alignment_distance:f32,cohesion_distance:f32,freedom_factor:f32,pad0:f32,pad1:f32,predator:vec4<f32>}
@group(0) @binding(0) var texture_position:texture_2d<f32>;
@group(0) @binding(1) var texture_velocity:texture_2d<f32>;
@group(0) @binding(2) var<uniform> u:U;
const width:f32=64.0;
const height:f32=64.0;
const PI:f32=3.141592653589793;
const PI_2:f32=PI*2.0;
const UPPER_BOUNDS:f32=800.00;
const SPEED_LIMIT:f32=9.0;
@fragment fn fs(@builtin(position) frag:vec4<f32>)->@location(0) vec4<f32>{
 let zone_radius=u.separation_distance+u.alignment_distance+u.cohesion_distance;
 let separation_thresh=u.separation_distance/zone_radius;
 let alignment_thresh=(u.separation_distance+u.alignment_distance)/zone_radius;
 let zone_radius_squared=zone_radius*zone_radius;
 let texel=vec2<i32>(frag.xy);
 let self_position=textureLoad(texture_position,texel,0).xyz;
 let self_velocity=textureLoad(texture_velocity,texel,0).xyz;
 var velocity=self_velocity;
 var limit=SPEED_LIMIT;
 var dir=u.predator.xyz*UPPER_BOUNDS-self_position;
 dir.z=0.0;
 var dist=length(dir);
 var dist_squared=dist*dist;
 let prey_radius=150.0;
 let prey_radius_sq=prey_radius*prey_radius;
 if dist<prey_radius {
  let f=(dist_squared/prey_radius_sq-1.0)*u.delta*100.0;
  velocity+=normalize(dir)*f;
  limit+=5.0;
 }
 let central=vec3(0.0,0.0,0.0);
 dir=self_position-central;
 dist=length(dir);
 dir.y*=2.5;
 velocity-=normalize(dir)*u.delta*5.0;
 for(var y=0.0;y<height;y+=1.0){
  for(var x=0.0;x<width;x+=1.0){
   let bird_position=textureLoad(texture_position,vec2<i32>(i32(x),i32(y)),0).xyz;
   dir=bird_position-self_position;
   dist=length(dir);
   if dist<0.0001 {continue;}
   dist_squared=dist*dist;
   if dist_squared>zone_radius_squared {continue;}
   let percent=dist_squared/zone_radius_squared;
   if percent<separation_thresh {
    let f=(separation_thresh/percent-1.0)*u.delta;
    velocity-=normalize(dir)*f;
   } else if percent<alignment_thresh {
    let thresh_delta=alignment_thresh-separation_thresh;
    let adjusted_percent=(percent-separation_thresh)/thresh_delta;
    let bird_velocity=textureLoad(texture_velocity,vec2<i32>(i32(x),i32(y)),0).xyz;
    let f=(0.5-cos(adjusted_percent*PI_2)*0.5+0.5)*u.delta;
    velocity+=normalize(bird_velocity)*f;
   } else {
    let thresh_delta=1.0-alignment_thresh;
    var adjusted_percent:f32;
    if thresh_delta==0.0 {adjusted_percent=1.0;} else {adjusted_percent=(percent-alignment_thresh)/thresh_delta;}
    let f=(0.5-(cos(adjusted_percent*PI_2)*-0.5+0.5))*u.delta;
    velocity+=normalize(dir)*f;
   }
  }
 }
 if length(velocity)>limit {velocity=normalize(velocity)*limit;}
 return vec4(velocity,1.0);
}";
/// MeshStandardMaterial with the page's vertex stage. WebGL's dFdy runs up
/// the screen, so the flat normal negates WGSL's.
const BIRD: &str = "
struct Frame{
 view:mat4x4<f32>,projection:mat4x4<f32>,model:mat4x4<f32>,
 // The hemisphere's view-space up, sky and ground colors ( with intensity ).
 hemi_direction:vec4<f32>,sky:vec4<f32>,ground:vec4<f32>,
 light_direction:vec4<f32>,light_color:vec4<f32>,
 // fogColor, fogNear; fogFar, time, size.
 fog:vec4<f32>,params:vec4<f32>,
}
@group(0) @binding(0) var<uniform> frame:Frame;
@group(0) @binding(1) var texture_position:texture_2d<f32>;
@group(0) @binding(2) var texture_velocity:texture_2d<f32>;
@group(0) @binding(3) var texture_animation:texture_2d<f32>;
@group(0) @binding(4) var dfg_lut:texture_2d<f32>;
@group(0) @binding(5) var dfg_sampler:sampler;
@group(0) @binding(6) var nearest:sampler;
struct Varyings{
 @builtin(position) position:vec4<f32>,
 @location(0) view_position:vec3<f32>,
 @location(1) color:vec3<f32>,
 @location(2) fog_depth:f32,
}
fn glsl_mod(x:f32,y:f32)->f32{return x-y*floor(x/y);}
// The page samples with nearest filtering at the texel's corner
// ( reference.xy = ( j % 64, floor( j / 64 ) ) / 64 ), as here.
@vertex fn vs(@location(0) position:vec3<f32>,@location(1) color:vec3<f32>,@location(2) vertex:f32,@location(3) bird:vec2<f32>)->Varyings{
 let index=u32(bird.x);
 let reference=vec2(f32(index%64u),f32(index/64u))/64.0;
 let tmp_pos=textureSampleLevel(texture_position,nearest,reference,0.0);
 let pos=tmp_pos.xyz;
 var velocity=normalize(textureSampleLevel(texture_velocity,nearest,reference,0.0).xyz);
 // reference.z = index / 512, reference.w = 72 / 128.
 let row_v=glsl_mod(frame.params.y+bird.x*((0.0004+bird.y/10000.0)+normalize(velocity).x/20000.0),72.0/128.0);
 let ani_pos=textureSampleLevel(texture_animation,nearest,vec2(vertex/512.0,row_v),0.0).xyz;
 let model3=mat3x3<f32>(frame.model[0].xyz,frame.model[1].xyz,frame.model[2].xyz);
 var new_position=position;
 new_position=model3*(new_position+ani_pos);
 new_position*=frame.params.z+bird.y*frame.params.z*0.2;
 velocity.z*=-1.0;
 let xz=length(velocity.xz);
 let xyz=1.0;
 let x=sqrt(1.0-velocity.y*velocity.y);
 let cosry=velocity.x/xz;
 let sinry=velocity.z/xz;
 let cosrz=x/xyz;
 let sinrz=velocity.y/xyz;
 let maty=mat3x3<f32>(cosry,0.0,-sinry,0.0,1.0,0.0,sinry,0.0,cosry);
 let matz=mat3x3<f32>(cosrz,sinrz,0.0,-sinrz,cosrz,0.0,0.0,0.0,1.0);
 new_position=maty*matz*new_position;
 new_position+=pos;
 let mv=frame.view*frame.model*vec4(new_position,1.0);
 var out:Varyings;
 out.position=frame.projection*mv;
 out.view_position=-mv.xyz;
 out.color=color;
 out.fog_depth=-mv.z;
 return out;
}
const RECIPROCAL_PI:f32=0.3183098861837907;
fn f_schlick(f0:vec3<f32>,f90:f32,dot_vh:f32)->vec3<f32>{
 let fresnel=exp2((-5.55473*dot_vh-6.98316)*dot_vh);
 return f0*(1.0-fresnel)+(f90*fresnel);
}
@fragment fn fs(in:Varyings)->@location(0) vec4<f32>{
 let diffuse_color=in.color;
 let fdx=dpdx(in.view_position);
 let fdy=-dpdy(in.view_position);
 let normal=normalize(cross(fdx,fdy));
 let dxy=max(abs(dpdx(normal)),abs(dpdy(normal)));
 let geometry_roughness=max(max(dxy.x,dxy.y),dxy.z);
 let roughness=min(max(1.0,0.0525)+geometry_roughness,1.0);
 let diffuse_contribution=diffuse_color;
 let specular_color=vec3(0.04);
 let view_dir=normalize(in.view_position);
 let dot_nv_ms=saturate(dot(normal,view_dir));
 let dfg=textureSampleLevel(dfg_lut,dfg_sampler,vec2(roughness,dot_nv_ms),0.0).rg;
 let ess_ms=dfg.x+dfg.y;
 let compensation=1.0+specular_color*(1.0/ess_ms-1.0);
 // RE_Direct_Physical.
 let light=frame.light_direction.xyz;
 let dot_nl=saturate(dot(normal,light));
 let irradiance=dot_nl*frame.light_color.rgb;
 let alpha=roughness*roughness;
 let half_dir=normalize(light+view_dir);
 let dot_nv=saturate(dot(normal,view_dir));
 let dot_nh=saturate(dot(normal,half_dir));
 let dot_vh=saturate(dot(view_dir,half_dir));
 let f=f_schlick(specular_color,1.0,dot_vh);
 let a2=alpha*alpha;
 let gv=dot_nl*sqrt(a2+(1.0-a2)*dot_nv*dot_nv);
 let gl=dot_nv*sqrt(a2+(1.0-a2)*dot_nl*dot_nl);
 let v=0.5/max(gv+gl,1e-6);
 let denom=dot_nh*dot_nh*(a2-1.0)+1.0;
 let d=RECIPROCAL_PI*a2/(denom*denom);
 let direct_specular=irradiance*(f*(v*d))*compensation;
 let direct_diffuse=irradiance*RECIPROCAL_PI*diffuse_contribution*(1.0-f_schlick(specular_color,1.0,dot_vh));
 // The hemisphere light's irradiance through RE_IndirectDiffuse_Physical.
 let weight=0.5*dot(normal,frame.hemi_direction.xyz)+0.5;
 let hemi=mix(frame.ground.rgb,frame.sky.rgb,weight);
 let fss_ess=specular_color*dfg.x+dfg.y;
 let ess=dfg.x+dfg.y;
 let ems=1.0-ess;
 let favg=specular_color+(1.0-specular_color)*0.047619;
 let fms=fss_ess*favg/(1.0-ems*favg);
 let indirect_diffuse=hemi*RECIPROCAL_PI*diffuse_contribution*(1.0-fss_ess-fms*ems);
 let outgoing=direct_diffuse+indirect_diffuse+direct_specular;
 let c=outgoing;
 let encoded=select(pow(c,vec3(0.41666))*1.055-vec3(0.055),c*12.92,c<=vec3(0.0031308));
 let fog_factor=smoothstep(frame.fog.w,frame.params.x,in.fog_depth);
 return vec4(mix(encoded,frame.fog.rgb,fog_factor),1.0);
}";
/// The fixture's Math.random: a 32-bit LCG seeded with 186.
struct Random(u32);
impl Random {
    fn next(&mut self) -> f64 {
        self.0 = self.0.wrapping_mul(1664525).wrapping_add(1013904223);
        f64::from(self.0) / 4294967296.
    }
}
fn srgb_to_linear(c: f64) -> f64 {
    if c < 0.04045 {
        c * 0.0773993808
    } else {
        (c * 0.9478672986 + 0.0521327014).powf(2.4)
    }
}
/// `Color.setHSL( h, s, l, SRGBColorSpace )`, stored linear.
fn hsl(h: f64, s: f64, l: f64) -> [f64; 3] {
    let hue = |p: f64, q: f64, mut t: f64| {
        if t < 0. {
            t += 1.;
        }
        if t > 1. {
            t -= 1.;
        }
        if t < 1. / 6. {
            p + (q - p) * 6. * t
        } else if t < 0.5 {
            q
        } else if t < 2. / 3. {
            p + (q - p) * 6. * (2. / 3. - t)
        } else {
            p
        }
    };
    let h = h.rem_euclid(1.);
    let (s, l) = (s.clamp(0., 1.), l.clamp(0., 1.));
    let rgb = if s == 0. {
        [l; 3]
    } else {
        let p = if l <= 0.5 {
            l * (1. + s)
        } else {
            l + s - l * s
        };
        let q = 2. * l - p;
        [hue(q, p, h + 1. / 3.), hue(q, p, h), hue(q, p, h - 1. / 3.)]
    };
    rgb.map(srgb_to_linear)
}
fn float_texture(
    r: &Renderer,
    label: &str,
    (width, height): (u32, u32),
    data: Option<&[f32]>,
) -> wgpu::Texture {
    let texture = r.device.create_texture(&wgpu::TextureDescriptor {
        label: Some(label),
        size: wgpu::Extent3d {
            width,
            height,
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
    });
    if let Some(data) = data {
        r.queue.write_texture(
            texture.as_image_copy(),
            bytemuck::cast_slice(data),
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(width * 16),
                rows_per_image: Some(height),
            },
            texture.size(),
        );
    }
    texture
}
/// A simulation variable: its program and, per current index, the bind
/// group reading that index's textures.
struct Variable {
    pipeline: wgpu::RenderPipeline,
    groups: [wgpu::BindGroup; 2],
    uniforms: wgpu::Buffer,
}
struct Targets {
    size: (u32, u32),
    samples: u32,
    format: wgpu::TextureFormat,
    color: Option<wgpu::TextureView>,
    depth: wgpu::TextureView,
    screen: RenderTarget,
    pipeline: wgpu::RenderPipeline,
}
pub(super) struct Demo {
    pending: bool,
    elapsed: f64,
    last: f64,
    /// separation, alignment, cohesion, size, count.
    params: [f64; 5],
    mouse: Option<(f64, f64)>,
    /// The current texture index of both variables.
    current: usize,
    /// Position and velocity views, by index.
    positions: [wgpu::TextureView; 2],
    velocities: [wgpu::TextureView; 2],
    velocity: Variable,
    position: Variable,
    vertices: wgpu::Buffer,
    instances: wgpu::Buffer,
    index: (wgpu::Buffer, u32),
    frame: wgpu::Buffer,
    /// The bird bind groups, by current index.
    bird_groups: [wgpu::BindGroup; 2],
    layout: wgpu::BindGroupLayout,
    targets: Option<Targets>,
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, _id: u32, r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 75.,
            near: 1.,
            far: 3000.,
            aspect,
            ..Default::default()
        }));
        s.get_mut(c)?.position = Vector3::new(0., 0., 350.);
        let mut random = Random(186);
        // selectModel: the Parrot.
        if (random.next() * 2.).floor() != 0. {
            return Err(Error::Invalid("birds model selection"));
        }
        let (asset, buffers, _) = load_asset("/web/gallery/assets/gltf/Parrot.glb").await?;
        let mesh = asset
            .default_scene()
            .and_then(|scene| scene.nodes().next())
            .and_then(|node| node.mesh())
            .ok_or(Error::Invalid("parrot mesh"))?;
        let primitive = mesh
            .primitives()
            .next()
            .ok_or(Error::Invalid("parrot primitive"))?;
        let reader = primitive.reader(|b| buffers.get(b.index()).map(Vec::as_slice));
        let positions: Vec<[f32; 3]> = reader
            .read_positions()
            .ok_or(Error::Invalid("parrot positions"))?
            .collect();
        let colors: Vec<[f32; 3]> = reader
            .read_colors(0)
            .ok_or(Error::Invalid("parrot colors"))?
            .into_rgb_f32()
            .collect();
        let index: Vec<u32> = reader
            .read_indices()
            .ok_or(Error::Invalid("parrot index"))?
            .into_u32()
            .collect();
        let morphs: Vec<Vec<[f32; 3]>> = reader
            .read_morph_targets()
            .map(|(p, _, _)| {
                p.map(Iterator::collect)
                    .ok_or(Error::Invalid("parrot morph"))
            })
            .collect::<Result<_>>()?;
        let duration = asset
            .animations()
            .next()
            .and_then(|a| {
                a.channels()
                    .filter_map(|c| {
                        c.reader(|b| buffers.get(b.index()).map(Vec::as_slice))
                            .read_inputs()
                    })
                    .flat_map(|inputs| inputs.last())
                    .reduce(f32::max)
            })
            .ok_or(Error::Invalid("parrot animation"))?;
        // The animation texture: durationAnimation rows at 60 per second.
        let duration_animation = (f64::from(duration) * 60.).round() as usize;
        let next_power = |n: usize| 2_f64.powf(((n as f64).ln() / 2_f64.ln()).ceil()) as usize;
        let (t_height, t_width) = (next_power(duration_animation), next_power(positions.len()));
        if (t_width, t_height, duration_animation) != (512, 128, 72) {
            return Err(Error::Invalid("parrot animation size"));
        }
        let mut t_data = vec![0f32; 4 * t_width * t_height];
        let targets = morphs.len();
        for i in 0..t_width {
            for j in 0..duration_animation {
                let offset = j * t_width * 4;
                let phase = j as f64 / duration_animation as f64 * targets as f64;
                let current = phase.floor() as usize;
                let next = (current + 1) % targets;
                let amount = phase % 1.;
                for k in 0..3 {
                    if i < positions.len() {
                        let (d0, d1) = (
                            f64::from(morphs[current][i][k]),
                            f64::from(morphs[next][i][k]),
                        );
                        t_data[offset + i * 4 + k] = (d0 + (d1 - d0) * amount) as f32;
                    }
                }
                t_data[offset + i * 4 + 3] = 1.;
            }
        }
        // The page's per-bird randoms ( r, then two seeds per vertex ), as
        // its geometry loop draws them.
        let mut _r = random.next();
        let mut instances: Vec<f32> = Vec::with_capacity(BIRDS as usize * 2);
        for bird in 0..BIRDS {
            _r = random.next();
            instances.extend([bird as f32, _r as f32]);
            for _ in 0..positions.len() {
                random.next();
                random.next();
            }
        }
        let mut position_data = vec![0f32; BIRDS as usize * 4];
        for texel in position_data.chunks_mut(4) {
            for v in texel.iter_mut().take(3) {
                *v = (random.next() * BOUNDS - BOUNDS / 2.) as f32;
            }
            texel[3] = 1.;
        }
        let mut velocity_data = vec![0f32; BIRDS as usize * 4];
        for texel in velocity_data.chunks_mut(4) {
            for v in texel.iter_mut().take(3) {
                *v = ((random.next() - 0.5) * 10.) as f32;
            }
            texel[3] = 1.;
        }
        let mut vertex_data: Vec<f32> = Vec::with_capacity(positions.len() * 7);
        for (k, (p, c)) in positions.iter().zip(&colors).enumerate() {
            vertex_data.extend_from_slice(p);
            vertex_data.extend_from_slice(c);
            vertex_data.push(k as f32);
        }
        let init = |label, contents: &[u8], usage| {
            r.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some(label),
                    contents,
                    usage,
                })
        };
        // GPUComputationRenderer.init(): both targets start from the data.
        let size = (WIDTH, WIDTH);
        let view = |t: wgpu::Texture| t.create_view(&Default::default());
        let positions_views = [
            view(float_texture(
                r,
                "bird positions",
                size,
                Some(&position_data),
            )),
            view(float_texture(
                r,
                "bird positions",
                size,
                Some(&position_data),
            )),
        ];
        let velocities_views = [
            view(float_texture(
                r,
                "bird velocities",
                size,
                Some(&velocity_data),
            )),
            view(float_texture(
                r,
                "bird velocities",
                size,
                Some(&velocity_data),
            )),
        ];
        let animation = view(float_texture(
            r,
            "bird animation",
            (t_width as u32, t_height as u32),
            Some(&t_data),
        ));
        let variable = |label, source: &str, uniforms_size| -> Variable {
            let module = r.device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some(label),
                source: wgpu::ShaderSource::Wgsl(format!("{QUAD}{source}").into()),
            });
            let pipeline = r
                .device
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
                });
            let uniforms = r.device.create_buffer(&wgpu::BufferDescriptor {
                label: Some(label),
                size: uniforms_size,
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });
            let groups = [0, 1].map(|k| {
                r.device.create_bind_group(&wgpu::BindGroupDescriptor {
                    label: Some(label),
                    layout: &pipeline.get_bind_group_layout(0),
                    entries: &[
                        wgpu::BindGroupEntry {
                            binding: 0,
                            resource: wgpu::BindingResource::TextureView(&positions_views[k]),
                        },
                        wgpu::BindGroupEntry {
                            binding: 1,
                            resource: wgpu::BindingResource::TextureView(&velocities_views[k]),
                        },
                        wgpu::BindGroupEntry {
                            binding: 2,
                            resource: uniforms.as_entire_binding(),
                        },
                    ],
                })
            });
            Variable {
                pipeline,
                groups,
                uniforms,
            }
        };
        let velocity = variable("bird velocity", VELOCITY, 48);
        let position = variable("bird position", POSITION, 16);
        let entry = |binding, ty| wgpu::BindGroupLayoutEntry {
            binding,
            visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
            ty,
            count: None,
        };
        let unfilterable = wgpu::BindingType::Texture {
            sample_type: wgpu::TextureSampleType::Float { filterable: false },
            view_dimension: wgpu::TextureViewDimension::D2,
            multisampled: false,
        };
        let layout = r
            .device
            .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("birds"),
                entries: &[
                    entry(
                        0,
                        wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                    ),
                    entry(1, unfilterable),
                    entry(2, unfilterable),
                    entry(3, unfilterable),
                    entry(
                        4,
                        wgpu::BindingType::Texture {
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                            view_dimension: wgpu::TextureViewDimension::D2,
                            multisampled: false,
                        },
                    ),
                    entry(
                        5,
                        wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    ),
                    entry(
                        6,
                        wgpu::BindingType::Sampler(wgpu::SamplerBindingType::NonFiltering),
                    ),
                ],
            });
        let frame = r.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("birds frame"),
            size: 64 * 3 + 16 * 7,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let dfg_sampler = r.device.create_sampler(&wgpu::SamplerDescriptor {
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        // The variables' RepeatWrapping; the animation texture clamps.
        let nearest = r.device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::Repeat,
            address_mode_v: wgpu::AddressMode::Repeat,
            ..Default::default()
        });
        // The birds read the textures the frame's compute wrote: index k
        // after a compute that started from 1 - k.
        let bird_groups = [0, 1].map(|k| {
            r.device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("birds"),
                layout: &layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: frame.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::TextureView(&positions_views[k]),
                    },
                    wgpu::BindGroupEntry {
                        binding: 2,
                        resource: wgpu::BindingResource::TextureView(&velocities_views[k]),
                    },
                    wgpu::BindGroupEntry {
                        binding: 3,
                        resource: wgpu::BindingResource::TextureView(&animation),
                    },
                    wgpu::BindGroupEntry {
                        binding: 4,
                        resource: wgpu::BindingResource::TextureView(&r.dfg),
                    },
                    wgpu::BindGroupEntry {
                        binding: 5,
                        resource: wgpu::BindingResource::Sampler(&dfg_sampler),
                    },
                    wgpu::BindGroupEntry {
                        binding: 6,
                        resource: wgpu::BindingResource::Sampler(&nearest),
                    },
                ],
            })
        });
        Ok(Self {
            pending: true,
            elapsed: 0.,
            last: 0.,
            params: [20., 20., 20., SIZE, f64::from(BIRDS / 4)],
            mouse: Some((0., 0.)),
            current: 0,
            positions: positions_views,
            velocities: velocities_views,
            velocity,
            position,
            vertices: init(
                "bird vertices",
                bytemuck::cast_slice(&vertex_data),
                wgpu::BufferUsages::VERTEX,
            ),
            instances: init(
                "bird instances",
                bytemuck::cast_slice(&instances),
                wgpu::BufferUsages::VERTEX,
            ),
            index: (
                init(
                    "bird index",
                    bytemuck::cast_slice(&index),
                    wgpu::BufferUsages::INDEX,
                ),
                index.len() as u32,
            ),
            frame,
            bird_groups,
            layout,
            targets: None,
        })
    }
    fn pipeline(
        &self,
        r: &Renderer,
        format: wgpu::TextureFormat,
        samples: u32,
    ) -> wgpu::RenderPipeline {
        let module = r.device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("birds"),
            source: wgpu::ShaderSource::Wgsl(BIRD.into()),
        });
        let layout = r
            .device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("birds"),
                bind_group_layouts: &[&self.layout],
                push_constant_ranges: &[],
            });
        let per_vertex = [
            wgpu::VertexAttribute {
                format: wgpu::VertexFormat::Float32x3,
                offset: 0,
                shader_location: 0,
            },
            wgpu::VertexAttribute {
                format: wgpu::VertexFormat::Float32x3,
                offset: 12,
                shader_location: 1,
            },
            wgpu::VertexAttribute {
                format: wgpu::VertexFormat::Float32,
                offset: 24,
                shader_location: 2,
            },
        ];
        let per_instance = [wgpu::VertexAttribute {
            format: wgpu::VertexFormat::Float32x2,
            offset: 0,
            shader_location: 3,
        }];
        r.device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("birds"),
                layout: Some(&layout),
                vertex: wgpu::VertexState {
                    module: &module,
                    entry_point: Some("vs"),
                    compilation_options: Default::default(),
                    buffers: &[
                        wgpu::VertexBufferLayout {
                            array_stride: 28,
                            step_mode: wgpu::VertexStepMode::Vertex,
                            attributes: &per_vertex,
                        },
                        wgpu::VertexBufferLayout {
                            array_stride: 8,
                            step_mode: wgpu::VertexStepMode::Instance,
                            attributes: &per_instance,
                        },
                    ],
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
                    format: DEPTH,
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
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, dt: f64, animate: bool) -> Result<()> {
        // The clock advances on animation frames; a seek sets it.
        if animate {
            self.elapsed += dt;
        }
        self.pending |= animate;
        Ok(())
    }
    pub fn prepare(&mut self, _s: &mut Scene, _c: Object3D, _r: &Renderer) -> Result<()> {
        Ok(())
    }
    pub fn render(
        &mut self,
        r: &Renderer,
        s: &mut Scene,
        c: Object3D,
        out: &RenderTarget,
    ) -> Result<bool> {
        let samples = if out.options.samples > 1 { 4 } else { 1 };
        if self.targets.as_ref().is_none_or(|t| {
            t.size != (out.width, out.height)
                || t.samples != samples
                || t.format != out.options.format
        }) {
            let attachment = |format, label| {
                r.device
                    .create_texture(&wgpu::TextureDescriptor {
                        label: Some(label),
                        size: wgpu::Extent3d {
                            width: out.width,
                            height: out.height,
                            depth_or_array_layers: 1,
                        },
                        mip_level_count: 1,
                        sample_count: samples,
                        dimension: wgpu::TextureDimension::D2,
                        format,
                        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                        view_formats: &[],
                    })
                    .create_view(&Default::default())
            };
            self.targets = Some(Targets {
                size: (out.width, out.height),
                samples,
                format: out.options.format,
                color: (samples > 1).then(|| attachment(out.options.format, "birds color")),
                depth: attachment(DEPTH, "birds depth"),
                screen: RenderTarget::with_options(
                    &r.device,
                    out.width,
                    out.height,
                    RenderTargetOptions {
                        samples: 0,
                        depth_buffer: false,
                        ..out.options.clone()
                    },
                )?,
                pipeline: self.pipeline(r, out.options.format, samples),
            });
        }
        if !std::mem::take(&mut self.pending) {
            return Ok(true);
        }
        // render(): the clock's delta ( capped at a second ), the predator
        // from the last pointer move, then gpuCompute.compute().
        let now = self.elapsed * 1000.;
        let delta = ((now - self.last) / 1000.).min(1.);
        self.last = now;
        let (w, h, _) = viewport_css();
        let (half_x, half_y) = (w / 2., h / 2.);
        let (mouse_x, mouse_y) = self.mouse.take().unwrap_or((10000., 10000.));
        let predator = [0.5 * mouse_x / half_x, -0.5 * mouse_y / half_y, 0.];
        let p = self.params;
        let velocity: [f32; 12] = [
            now as f32,
            delta as f32,
            p[0] as f32,
            p[1] as f32,
            p[2] as f32,
            0.75,
            0.,
            0.,
            predator[0] as f32,
            predator[1] as f32,
            0.,
            0.,
        ];
        r.queue
            .write_buffer(&self.velocity.uniforms, 0, bytemuck::cast_slice(&velocity));
        r.queue.write_buffer(
            &self.position.uniforms,
            0,
            bytemuck::cast_slice(&[now as f32, delta as f32, 0., 0.]),
        );
        s.update()?;
        let (camera, world) = s.camera(c)?;
        let view = world.inverse();
        let model = Matrix4::from_rotation_y(std::f64::consts::FRAC_PI_2);
        let mut data: Vec<f32> = vec![];
        for m in [view, camera.projection_matrix()?, model] {
            data.extend(m.to_cols_array().iter().map(|&v| v as f32));
        }
        let mut vec4 = |v: [f64; 4]| data.extend(v.map(|x| x as f32));
        // HemisphereLight at ( 0, 50, 0 ): its view-space up.
        let up = view.transform_vector3(Vector3::Y).normalize();
        vec4([up.x, up.y, up.z, 0.]);
        let [sr, sg, sb] = hsl(0.6, 1., 0.6).map(|v| v * 4.5);
        vec4([sr, sg, sb, 1.]);
        let [gr, gg, gb] = hsl(0.095, 1., 0.75).map(|v| v * 4.5);
        vec4([gr, gg, gb, 1.]);
        let d = view
            .transform_vector3(Vector3::new(-1., 1.75, 1.).normalize())
            .normalize();
        vec4([d.x, d.y, d.z, 0.]);
        let [lr, lg, lb] = hsl(0.1, 1., 0.95).map(|v| v * 2.);
        vec4([lr, lg, lb, 1.]);
        // refreshFogUniforms: the fog color in the output ( sRGB ) space.
        let [fr, fg, fb] = [16, 8, 0].map(|shift| f64::from((COLOR >> shift) & 255) / 255.);
        vec4([fr, fg, fb, 100.]);
        vec4([1000., now / 1000., p[3], 0.]);
        r.queue
            .write_buffer(&self.frame, 0, bytemuck::cast_slice(&data));
        let t = self
            .targets
            .as_ref()
            .ok_or(Error::Invalid("birds targets"))?;
        let (current, next) = (self.current, 1 - self.current);
        let mut encoder = r.device.create_command_encoder(&Default::default());
        for (variable, targets) in [
            (&self.velocity, &self.velocities),
            (&self.position, &self.positions),
        ] {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("birds compute"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &targets[next],
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                ..Default::default()
            });
            pass.set_pipeline(&variable.pipeline);
            pass.set_bind_group(0, &variable.groups[current], &[]);
            pass.draw(0..3, 0..1);
        }
        self.current = next;
        let [br, bg, bb] = [16, 8, 0].map(|shift| f64::from((COLOR >> shift) & 255) / 255.);
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("birds"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: t.color.as_ref().unwrap_or(&t.screen.view),
                    depth_slice: None,
                    resolve_target: t.color.as_ref().map(|_| &t.screen.view),
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: br,
                            g: bg,
                            b: bb,
                            a: 1.,
                        }),
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
            pass.set_bind_group(0, &self.bird_groups[next], &[]);
            pass.set_vertex_buffer(0, self.vertices.slice(..));
            pass.set_vertex_buffer(1, self.instances.slice(..));
            pass.set_index_buffer(self.index.0.slice(..), wgpu::IndexFormat::Uint32);
            // setDrawRange( 0, indicesPerBird × count ): whole birds.
            pass.draw_indexed(0..self.index.1, 0, 0..(p[4] as u32).min(BIRDS));
        }
        r.queue.submit([encoder.finish()]);
        Ok(true)
    }
    pub fn output(&self) -> Option<&RenderTarget> {
        self.targets.as_ref().map(|t| &t.screen)
    }
    /// onPointerMove: the pointer from the window's center; the next frame
    /// consumes it ( render() then moves the predator away ).
    pub fn draw(&mut self, kind: u32, x: f64, y: f64) {
        if kind != 0 {
            return;
        }
        let (w, h, _) = viewport_css();
        self.mouse = Some((x - w / 2., y - h / 2.));
    }
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
    /// The GUI: separation, alignment, cohesion, size, count.
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        *self
            .params
            .get_mut(index)
            .ok_or(Error::Invalid("birds parameter"))? = f64::from(value);
        Ok(())
    }
    pub fn seek(&mut self, t: f64) {
        self.elapsed = t;
        self.pending = true;
    }
}
