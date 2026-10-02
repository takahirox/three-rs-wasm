//! webgl_postprocessing_gtao: the animated Littlest Tokyo under the
//! RoomEnvironment PMREM through an EffectComposer: RenderPass, GTAOPass
//! ( showing the denoised AO by default ) and OutputPass. The scene and its
//! override normal pass render on the engine; GTAOPass's passes are WGSL
//! translations of the GLSL the page's ShaderMaterials compile: the
//! MeshNormalMaterial normals and depth ( over the scene background ), GTAOShader
//! ( the 5 × 5 magic-square rotation noise and the scene clip box ),
//! PoissonDenoiseShader ( the 64 × 64 SimplexNoise rotation, whose
//! permutation the addon draws from Math.random; the fixture seeds it ),
//! the output selection ( copy, blend, depth or normal view ) and
//! OutputPass's sRGB transfer. Depth reconstruction uses WebGPU's 0..1 depth
//! with the matching inverse projection.
use super::controls_attributes::{Controls, camera_state};
use super::gltf_viewer::load_asset;
use super::gtao::magic_square_noise;
use super::ssao::{Random, Simplex};
use crate::{
    Error, Result, camera::*, geometry::*, material::*, math::*, render_target::*, renderer::*,
    scene::*,
};
use std::collections::HashMap;
use std::f64::consts::PI;
use std::sync::Arc;
use wgpu::util::DeviceExt;

const HALF: wgpu::TextureFormat = wgpu::TextureFormat::Rgba16Float;
const QUAD: &str = "@vertex fn vs(@builtin(vertex_index) i:u32)->@builtin(position) vec4<f32>{let p=array(vec2(-1.0,-1.0),vec2(3.0,-1.0),vec2(-1.0,3.0));return vec4(p[i],0.0,1.0);}";
/// Shared by the passes: the frame's camera and settings. vUv runs up the
/// screen as WebGL's does; the targets are stored from the top.
const COMMON: &str = "
struct U{
 projection:mat4x4<f32>,inverse_projection:mat4x4<f32>,camera_world:mat4x4<f32>,
 // resolution, near, far.
 screen:vec4<f32>,
 // radius, distanceExponent, thickness, distanceFallOff.
 ao:vec4<f32>,
 // scale, screenSpaceRadius, sceneClipBox, blend intensity.
 ao2:vec4<f32>,
 box_min:vec4<f32>,box_max:vec4<f32>,
 // lumaPhi, depthPhi, normalPhi, radius.
 pd:vec4<f32>,
 output:vec4<f32>,
}
@group(0) @binding(0) var<uniform> u:U;
@group(0) @binding(1) var t_normal:texture_2d<f32>;
@group(0) @binding(2) var t_depth:texture_depth_2d;
@group(0) @binding(3) var t_noise:texture_2d<f32>;
@group(0) @binding(4) var t_diffuse:texture_2d<f32>;
@group(0) @binding(5) var linear_sampler:sampler;
const PI:f32=3.141592653589793;
fn gl_uv(frag:vec4<f32>)->vec2<f32>{return vec2(frag.x/u.screen.x,1.0-frag.y/u.screen.y);}
fn texel(t:texture_2d<f32>,uv:vec2<f32>)->vec4<f32>{
 let size=vec2<f32>(textureDimensions(t));
 let p=clamp(vec2<i32>(floor(vec2(uv.x,1.0-uv.y)*size)),vec2(0),vec2<i32>(size)-1);
 return textureLoad(t,p,0);
}
fn get_depth(uv:vec2<f32>)->f32{
 let size=vec2<f32>(textureDimensions(t_depth));
 let p=clamp(vec2<i32>(floor(vec2(uv.x,1.0-uv.y)*size)),vec2(0),vec2<i32>(size)-1);
 return textureLoad(t_depth,p,0);
}
fn get_view_position(uv:vec2<f32>,depth:f32)->vec3<f32>{
 let v=u.inverse_projection*vec4(uv*2.0-1.0,depth,1.0);
 return v.xyz/v.w;
}
fn unpack_normal(rgb:vec3<f32>)->vec3<f32>{return 2.0*rgb-1.0;}
fn get_view_normal(uv:vec2<f32>)->vec3<f32>{return unpack_normal(texel(t_normal,uv).rgb);}
// Nearest, repeating noise at the WebGL fragment.
fn noise_texel(frag:vec4<f32>)->vec4<f32>{
 let size=vec2<i32>(textureDimensions(t_noise));
 let pixel=vec2<i32>(i32(floor(frag.x)),i32(u.screen.y)-1-i32(floor(frag.y)));
 return textureLoad(t_noise,((pixel%size)+size)%size,0);
}
";
/// GTAOShader ( PERSPECTIVE_CAMERA, NORMAL_VECTOR_TYPE 1 ).
const GTAO: &str = "
override SAMPLES:i32=16;
fn get_scene_uv_and_depth(sample_view_pos:vec3<f32>)->vec3<f32>{
 let clip=u.projection*vec4(sample_view_pos,1.0);
 let uv=clip.xy/clip.w*0.5+0.5;
 return vec3(uv,get_depth(uv));
}
@fragment fn fs(@builtin(position) frag:vec4<f32>)->@location(0) vec4<f32>{
 let uv=gl_uv(frag);
 let depth=get_depth(uv);
 if depth>=1.0 {discard;}
 let view_pos=get_view_position(uv,depth);
 let view_normal=get_view_normal(uv);
 var radius_to_use=u.ao.x;
 var distance_falloff_to_use=u.ao.z;
 if u.ao2.y>0.5 {
  let radius_scale=get_view_position(vec2(0.5+100.0/u.screen.x,0.0),depth).x;
  radius_to_use*=radius_scale;
  distance_falloff_to_use*=radius_scale;
 }
 var box_distance=0.0;
 if u.ao2.z>0.5 {
  let world_pos=(u.camera_world*vec4(view_pos,1.0)).xyz;
  box_distance=length(max(vec3(0.0),max(u.box_min.xyz-world_pos,world_pos-u.box_max.xyz)));
  if box_distance>radius_to_use {discard;}
 }
 let noise=noise_texel(frag);
 let random_vec=noise.xyz*2.0-1.0;
 let tangent=normalize(vec3(random_vec.xy,0.0));
 let bitangent=vec3(-tangent.y,tangent.x,0.0);
 let kernel=mat3x3<f32>(tangent,bitangent,vec3(0.0,0.0,1.0));
 let directions=select(5,3,SAMPLES<30);
 let steps=(SAMPLES+directions-1)/directions;
 var ao=0.0;
 for(var i=0;i<directions;i++){
  let angle=f32(i)/f32(directions)*PI;
  var sample_dir=vec4(cos(angle),sin(angle),0.0,0.5+0.5*noise.w);
  sample_dir=vec4(normalize(kernel*sample_dir.xyz),sample_dir.w);
  let view_dir=normalize(-view_pos);
  let slice_bitangent=normalize(cross(sample_dir.xyz,view_dir));
  let slice_tangent=cross(slice_bitangent,view_dir);
  let normal_in_slice=normalize(view_normal-slice_bitangent*dot(view_normal,slice_bitangent));
  let tangent_to_normal_in_slice=cross(normal_in_slice,slice_bitangent);
  var cos_horizons=vec2(dot(view_dir,tangent_to_normal_in_slice),dot(view_dir,-tangent_to_normal_in_slice));
  for(var j=0;j<steps;j++){
   let offset=sample_dir.xyz*radius_to_use*sample_dir.w*pow(f32(j+1)/f32(steps),u.ao.y);
   var s=get_scene_uv_and_depth(view_pos+offset);
   var delta=get_view_position(s.xy,s.z)-view_pos;
   if abs(delta.z)<u.ao.z {
    let c=dot(view_dir,normalize(delta));
    cos_horizons.x+=max(0.0,(c-cos_horizons.x)*mix(1.0,2.0/f32(j+2),u.ao.w));
   }
   s=get_scene_uv_and_depth(view_pos-offset);
   delta=get_view_position(s.xy,s.z)-view_pos;
   if abs(delta.z)<u.ao.z {
    let c=dot(view_dir,normalize(delta));
    cos_horizons.y+=max(0.0,(c-cos_horizons.y)*mix(1.0,2.0/f32(j+2),u.ao.w));
   }
  }
  let sin_horizons=sqrt(1.0-cos_horizons*cos_horizons);
  let nx=dot(normal_in_slice,slice_tangent);
  let ny=dot(normal_in_slice,view_dir);
  let nxb=1.0/2.0*(acos(cos_horizons.y)-acos(cos_horizons.x)+sin_horizons.x*cos_horizons.x-sin_horizons.y*cos_horizons.y);
  let nyb=1.0/2.0*(2.0-cos_horizons.x*cos_horizons.x-cos_horizons.y*cos_horizons.y);
  ao+=nx*nxb+ny*nyb;
 }
 ao=clamp(ao/f32(directions),0.0,1.0);
 if u.ao2.z>0.5 {ao=mix(ao,1.0,smoothstep(0.0,radius_to_use,box_distance));}
 ao=pow(ao,u.ao2.x);
 return vec4(vec3(ao),1.0);
}";
/// PoissonDenoiseShader ( NORMAL_VECTOR_TYPE 1, depth from .r ).
const DENOISE: &str = "
struct Disk{points:array<vec4<f32>,32>}
@group(0) @binding(6) var<uniform> disk:Disk;
override SAMPLES:i32=16;
fn luminance(a:vec3<f32>)->f32{return dot(vec3(0.2125,0.7154,0.0721),a);}
@fragment fn fs(@builtin(position) frag:vec4<f32>)->@location(0) vec4<f32>{
 let uv=gl_uv(frag);
 let depth=get_depth(uv);
 let view_normal=get_view_normal(uv);
 if depth==1.0 || dot(view_normal,view_normal)==0.0 {discard;}
 let center=textureSampleLevel(t_diffuse,linear_sampler,vec2(uv.x,1.0-uv.y),0.0).rgb;
 let view_pos=get_view_position(uv,depth);
 let noise=noise_texel(frag);
 let noise_vec=vec2(sin(noise[0]*2.0*PI),cos(noise[0]*2.0*PI));
 let rotation=mat2x2<f32>(noise_vec.x,-noise_vec.y,noise_vec.x,noise_vec.y);
 var total_weight=1.0;
 var denoised=center;
 for(var i=0;i<SAMPLES;i++){
  let sample_dir=disk.points[i].xyz;
  let offset=rotation*(sample_dir.xy*(1.0+sample_dir.z*(u.pd.w-1.0))/u.screen.xy);
  let sample_uv=uv+offset;
  let neighbor=textureSampleLevel(t_diffuse,linear_sampler,vec2(sample_uv.x,1.0-sample_uv.y),0.0).rgb;
  let sample_depth=get_depth(sample_uv);
  let sample_normal=get_view_normal(sample_uv);
  let view_pos_sample=get_view_position(sample_uv,sample_depth);
  let normal_similarity=pow(max(dot(view_normal,sample_normal),0.0),u.pd.z);
  let luma_similarity=max(1.0-abs(luminance(neighbor)-luminance(center))/u.pd.x,0.0);
  let depth_similarity=max(1.0-abs(dot(view_pos-view_pos_sample,view_normal))/u.pd.y,0.0);
  let w=luma_similarity*depth_similarity*normal_similarity;
  denoised+=w*neighbor;
  total_weight+=w;
 }
 if total_weight>0.0 {denoised/=total_weight;}
 return vec4(denoised,1.0);
}";
/// The output selection into the composer's write buffer: CopyShader, or
/// GTAODepthShader's view when u.output.x is set; GTAOBlendShader.
const OUTPUT: &str = "
@fragment fn fs(@builtin(position) frag:vec4<f32>)->@location(0) vec4<f32>{
 let uv=gl_uv(frag);
 if u.output.x>0.5 {
  // perspectiveDepthToViewZ, then viewZToOrthographicDepth.
  let view_z=get_view_position(uv,get_depth(uv)).z;
  let linear=(view_z+u.screen.z)/(u.screen.z-u.screen.w);
  return vec4(vec3(1.0-linear),1.0);
 }
 return textureSampleLevel(t_diffuse,linear_sampler,vec2(uv.x,1.0-uv.y),0.0);
}
@fragment fn blend_fs(@builtin(position) frag:vec4<f32>)->@location(0) vec4<f32>{
 let uv=gl_uv(frag);
 let t=textureSampleLevel(t_diffuse,linear_sampler,vec2(uv.x,1.0-uv.y),0.0);
 return vec4(mix(vec3(1.0),t.rgb,u.ao2.w),t.a);
}";
/// OutputPass: no tone mapping, the sRGB transfer.
const SRGB: &str = "
@fragment fn fs(@builtin(position) frag:vec4<f32>)->@location(0) vec4<f32>{
 let c=textureSampleLevel(t_diffuse,linear_sampler,frag.xy/u.screen.xy,0.0);
 return vec4(select(pow(c.rgb,vec3(0.41666))*1.055-vec3(0.055),c.rgb*12.92,c.rgb<=vec3(0.0031308)),c.a);
}";
/// GTAOPass.OUTPUT: Off, Default, Diffuse, Depth, Normal, AO, Denoise.
#[derive(Clone, Copy, PartialEq)]
enum Output {
    Default,
    Diffuse,
    Ao,
    Denoise,
    Depth,
    Normal,
}
struct Targets {
    size: (u32, u32),
    format: wgpu::TextureFormat,
    beauty: RenderTarget,
    normal: RenderTarget,
    gtao: wgpu::TextureView,
    pd: wgpu::TextureView,
    write: wgpu::TextureView,
    screen: RenderTarget,
    /// Bind groups reading ( beauty, gtao, pd, normal, write ) as t_diffuse.
    groups: HashMap<&'static str, wgpu::BindGroup>,
}
pub(super) struct Demo {
    controls: Controls,
    mixer: Option<crate::animation::AnimationMixer>,
    time: f64,
    meshes: Vec<Object3D>,
    normal_material: Arc<Material>,
    box_min: Vector3,
    box_max: Vector3,
    output: Output,
    /// blendIntensity; radius, distanceExponent, thickness, distanceFallOff,
    /// scale, samples, screenSpaceRadius; lumaPhi, depthPhi, normalPhi,
    /// radius, radiusExponent, rings, samples.
    params: [f64; 15],
    uniforms: wgpu::Buffer,
    disk: wgpu::Buffer,
    /// The denoise sample points to upload after a rings, radius exponent or
    /// samples change ( the page recompiles the shader with them ).
    pending_disk: Option<Vec<f32>>,
    noise: [wgpu::TextureView; 2],
    sampler: wgpu::Sampler,
    layout: wgpu::BindGroupLayout,
    pipelines: HashMap<(&'static str, i32, wgpu::TextureFormat), wgpu::RenderPipeline>,
    targets: Option<Targets>,
}
fn texture(
    r: &Renderer,
    label: &str,
    (width, height): (u32, u32),
    format: wgpu::TextureFormat,
) -> wgpu::TextureView {
    r.device
        .create_texture(&wgpu::TextureDescriptor {
            label: Some(label),
            size: wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        })
        .create_view(&Default::default())
}
fn data_texture(r: &Renderer, label: &str, size: u32, data: &[u8]) -> wgpu::TextureView {
    r.device
        .create_texture_with_data(
            &r.queue,
            &wgpu::TextureDescriptor {
                label: Some(label),
                size: wgpu::Extent3d {
                    width: size,
                    height: size,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba8Unorm,
                usage: wgpu::TextureUsages::TEXTURE_BINDING,
                view_formats: &[],
            },
            wgpu::util::TextureDataOrder::LayerMajor,
            data,
        )
        .create_view(&Default::default())
}
/// generateDenoiseSamples( samples, rings, radiusExponent ).
fn disk(samples: usize, rings: f64, exponent: f64) -> Vec<f32> {
    let mut out = vec![0f32; 32 * 4];
    for i in 0..samples.min(32) {
        let angle = 2. * PI * rings * i as f64 / samples as f64;
        let radius = (i as f64 / (samples - 1) as f64).powf(exponent);
        out[i * 4..i * 4 + 3].copy_from_slice(&[
            angle.cos() as f32,
            angle.sin() as f32,
            radius as f32,
        ]);
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
            fov: 40.,
            near: 1.,
            far: 100.,
            aspect,
            ..Default::default()
        }));
        s.get_mut(c)?.position = Vector3::new(5., 2., 8.);
        s.background = Color::from_hex(0xbfe3dd);
        s.environment = Some(super::room_environment::environment(r)?);
        let (a, b, i) =
            load_asset("/web/gallery/assets/tsl-viewport/models/gltf/LittlestTokyo.glb").await?;
        let instance = crate::gltf::import_animated_decoded(&a, &b, &i)?.instantiate(s)?;
        let group = s.insert(NodeKind::Group);
        let n = s.get_mut(group)?;
        n.position = Vector3::new(1., 1., 0.);
        n.scale = Vector3::splat(0.01);
        for h in instance.roots {
            s.add(group, h)?;
        }
        let mut mixer = crate::animation::AnimationMixer::default();
        mixer.play(instance.clips[0].clone())?;
        // Box3.setFromObject( scene ): each mesh's geometry box in its world
        // matrix, before the animation runs.
        s.update()?;
        let (mut box_min, mut box_max) = (
            Vector3::splat(f64::INFINITY),
            Vector3::splat(f64::NEG_INFINITY),
        );
        for &h in &instance.meshes {
            let node = s.get(h)?;
            let NodeKind::Mesh(mesh) = &node.kind else {
                continue;
            };
            let Some(Attribute::F32(p)) = mesh.geometry.attributes.get("position") else {
                continue;
            };
            let (lo, hi) = p.array().chunks(3).fold(
                (
                    Vector3::splat(f64::INFINITY),
                    Vector3::splat(f64::NEG_INFINITY),
                ),
                |(lo, hi), v| {
                    let v = Vector3::new(f64::from(v[0]), f64::from(v[1]), f64::from(v[2]));
                    (lo.min(v), hi.max(v))
                },
            );
            let world = node.matrix_world;
            for k in 0..8 {
                let corner = Vector3::new(
                    if k & 1 == 0 { lo.x } else { hi.x },
                    if k & 2 == 0 { lo.y } else { hi.y },
                    if k & 4 == 0 { lo.z } else { hi.z },
                );
                let w = world.transform_point3(corner);
                box_min = box_min.min(w);
                box_max = box_max.max(w);
            }
        }
        let mut controls = Controls::new(Some(0.05), (0., f64::INFINITY), PI, true);
        controls.set_target(Vector3::new(0., 0.5, 0.));
        controls.update(s, c)?;
        // The addon's noise: the 5 × 5 magic square, and SimplexNoise over
        // the seeded Math.random for the denoise rotation.
        let simplex = Simplex::new(&mut Random(186));
        let mut pd_noise = vec![0u8; 64 * 64 * 4];
        let byte = |v: f64| ((v * 0.5 + 0.5) * 255.) as u8;
        for i in 0..64 {
            for j in 0..64 {
                let (x, y) = (i as f64, j as f64);
                let at = (i * 64 + j) * 4;
                pd_noise[at] = byte(simplex.noise(x, y));
                pd_noise[at + 1] = byte(simplex.noise(x + 64., y));
                pd_noise[at + 2] = byte(simplex.noise(x, y + 64.));
                pd_noise[at + 3] = byte(simplex.noise(x + 64., y + 64.));
            }
        }
        let noise = [
            data_texture(r, "gtao noise", 5, &magic_square_noise()),
            data_texture(r, "denoise noise", 64, &pd_noise),
        ];
        let entry = |binding, ty| wgpu::BindGroupLayoutEntry {
            binding,
            visibility: wgpu::ShaderStages::FRAGMENT,
            ty,
            count: None,
        };
        let tex = |sample_type| wgpu::BindingType::Texture {
            sample_type,
            view_dimension: wgpu::TextureViewDimension::D2,
            multisampled: false,
        };
        let uniform = wgpu::BindingType::Buffer {
            ty: wgpu::BufferBindingType::Uniform,
            has_dynamic_offset: false,
            min_binding_size: None,
        };
        let layout = r
            .device
            .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("gtao"),
                entries: &[
                    entry(0, uniform),
                    entry(1, tex(wgpu::TextureSampleType::Float { filterable: false })),
                    entry(2, tex(wgpu::TextureSampleType::Depth)),
                    entry(3, tex(wgpu::TextureSampleType::Float { filterable: false })),
                    entry(4, tex(wgpu::TextureSampleType::Float { filterable: true })),
                    entry(
                        5,
                        wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    ),
                    entry(6, uniform),
                ],
            });
        let sampler = r.device.create_sampler(&wgpu::SamplerDescriptor {
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        let uniforms = r.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("gtao uniforms"),
            size: 64 * 3 + 16 * 7,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let params = [
            1., 0.25, 1., 1., 1., 1., 16., 0., 10., 2., 3., 4., 1., 2., 16.,
        ];
        let disk = r
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("denoise samples"),
                contents: bytemuck::cast_slice(&disk(16, 2., 1.)),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            });
        Ok(Self {
            controls,
            mixer: Some(mixer),
            time: 0.,
            meshes: instance.meshes,
            normal_material: Arc::new(Material::Normal(MeshNormalMaterial::default())),
            box_min,
            box_max,
            output: Output::Denoise,
            params,
            uniforms,
            disk,
            pending_disk: None,
            noise,
            sampler,
            layout,
            pipelines: HashMap::new(),
            targets: None,
        })
    }
    fn pipeline(
        &mut self,
        r: &Renderer,
        pass: &'static str,
        samples: i32,
        format: wgpu::TextureFormat,
    ) -> wgpu::RenderPipeline {
        if let Some(p) = self.pipelines.get(&(pass, samples, format)) {
            return p.clone();
        }
        let source = match pass {
            "gtao" => GTAO,
            "denoise" => DENOISE,
            "output" | "blend" => OUTPUT,
            _ => SRGB,
        };
        let module = r.device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some(pass),
            source: wgpu::ShaderSource::Wgsl(format!("{QUAD}{COMMON}{source}").into()),
        });
        let layout = r
            .device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some(pass),
                bind_group_layouts: &[&self.layout],
                push_constant_ranges: &[],
            });
        let constants: Vec<(&str, f64)> = if matches!(pass, "gtao" | "denoise") {
            vec![("SAMPLES", f64::from(samples))]
        } else {
            vec![]
        };
        let options = wgpu::PipelineCompilationOptions {
            constants: &constants,
            ..Default::default()
        };
        // GTAOBlendShader multiplies the destination ( DstColor, Zero ).
        let blend = (pass == "blend").then_some(wgpu::BlendState {
            color: wgpu::BlendComponent {
                src_factor: wgpu::BlendFactor::Dst,
                dst_factor: wgpu::BlendFactor::Zero,
                operation: wgpu::BlendOperation::Add,
            },
            alpha: wgpu::BlendComponent {
                src_factor: wgpu::BlendFactor::DstAlpha,
                dst_factor: wgpu::BlendFactor::Zero,
                operation: wgpu::BlendOperation::Add,
            },
        });
        let module_ref = &module;
        let pipeline = r
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some(pass),
                layout: Some(&layout),
                vertex: wgpu::VertexState {
                    module: module_ref,
                    entry_point: Some("vs"),
                    compilation_options: options.clone(),
                    buffers: &[],
                },
                fragment: Some(wgpu::FragmentState {
                    module: module_ref,
                    entry_point: Some(if pass == "blend" { "blend_fs" } else { "fs" }),
                    compilation_options: options,
                    targets: &[Some(wgpu::ColorTargetState {
                        format,
                        blend,
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                }),
                primitive: Default::default(),
                depth_stencil: None,
                multisample: Default::default(),
                multiview: None,
                cache: None,
            });
        self.pipelines
            .insert((pass, samples, format), pipeline.clone());
        pipeline
    }
    fn resize(&mut self, r: &Renderer, out: &RenderTarget) -> Result<()> {
        let size = (out.width, out.height);
        let half = RenderTargetOptions {
            samples: 1,
            format: HALF,
            depth_buffer: true,
            ..Default::default()
        };
        let beauty = RenderTarget::with_options(&r.device, size.0, size.1, half.clone())?;
        let normal = RenderTarget::with_options(&r.device, size.0, size.1, half)?;
        let gtao = texture(r, "gtao", size, HALF);
        let pd = texture(r, "gtao denoise", size, HALF);
        let write = texture(r, "composer write", size, HALF);
        let depth = normal
            .depth_view
            .as_ref()
            .ok_or(Error::Invalid("gtao depth"))?;
        let mut groups = HashMap::new();
        for (name, diffuse) in [
            ("beauty", &beauty.view),
            ("gtao", &gtao),
            ("pd", &pd),
            ("normal", &normal.view),
            ("write", &write),
        ] {
            for (noise_name, noise) in [("", &self.noise[0]), ("pd-noise", &self.noise[1])] {
                let key: &'static str = Box::leak(format!("{name}{noise_name}").into_boxed_str());
                groups.insert(
                    key,
                    r.device.create_bind_group(&wgpu::BindGroupDescriptor {
                        label: Some("gtao"),
                        layout: &self.layout,
                        entries: &[
                            wgpu::BindGroupEntry {
                                binding: 0,
                                resource: self.uniforms.as_entire_binding(),
                            },
                            wgpu::BindGroupEntry {
                                binding: 1,
                                resource: wgpu::BindingResource::TextureView(&normal.view),
                            },
                            wgpu::BindGroupEntry {
                                binding: 2,
                                resource: wgpu::BindingResource::TextureView(depth),
                            },
                            wgpu::BindGroupEntry {
                                binding: 3,
                                resource: wgpu::BindingResource::TextureView(noise),
                            },
                            wgpu::BindGroupEntry {
                                binding: 4,
                                resource: wgpu::BindingResource::TextureView(diffuse),
                            },
                            wgpu::BindGroupEntry {
                                binding: 5,
                                resource: wgpu::BindingResource::Sampler(&self.sampler),
                            },
                            wgpu::BindGroupEntry {
                                binding: 6,
                                resource: self.disk.as_entire_binding(),
                            },
                        ],
                    }),
                );
            }
        }
        self.targets = Some(Targets {
            size,
            format: out.options.format,
            beauty,
            normal,
            gtao,
            pd,
            write,
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
            groups,
        });
        Ok(())
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
        s: &mut Scene,
        c: Object3D,
        out: &RenderTarget,
    ) -> Result<bool> {
        if self
            .targets
            .as_ref()
            .is_none_or(|t| t.size != (out.width, out.height) || t.format != out.options.format)
        {
            self.resize(r, out)?;
        }
        // animate(): the mixer by the timer, controls.update(), composer.render().
        if let Some(m) = &mut self.mixer {
            for a in &mut m.actions {
                a.time = self.time;
            }
            m.update(s, 0.)?;
        }
        self.controls.update(s, c)?;
        s.update()?;
        let t = self
            .targets
            .as_ref()
            .ok_or(Error::Invalid("gtao targets"))?;
        // RenderPass: the scene into the composer's read buffer.
        r.render(s, c, &t.beauty)?;
        // GTAOPass: the override normal material; the scene's background
        // colour replaces the pass's 0x7777ff clear.
        let mut originals = vec![];
        for &h in &self.meshes {
            if let NodeKind::Mesh(mesh) = &mut s.get_mut(h)?.kind {
                let count = mesh.materials.len();
                originals.push(std::mem::replace(
                    &mut mesh.materials,
                    vec![self.normal_material.clone(); count],
                ));
            }
        }
        let environment = s.environment.take();
        let result = r.render(s, c, &t.normal);
        s.environment = environment;
        let mut originals = originals.into_iter();
        for &h in &self.meshes {
            if let NodeKind::Mesh(mesh) = &mut s.get_mut(h)?.kind {
                mesh.materials = originals.next().unwrap_or_default();
            }
        }
        result?;
        let (camera, world) = s.camera(c)?;
        let projection = camera.projection_matrix()?;
        let (near, far) = match camera {
            Camera::Perspective(p) => (p.near, p.far),
            _ => (1., 100.),
        };
        let p = self.params;
        let mut data: Vec<f32> = vec![];
        for m in [projection, projection.inverse(), world] {
            data.extend(m.to_cols_array().iter().map(|&v| v as f32));
        }
        let mut vec4 = |v: [f64; 4]| data.extend(v.map(|x| x as f32));
        vec4([f64::from(t.size.0), f64::from(t.size.1), near, far]);
        vec4([p[1], p[2], p[3], p[4]]);
        vec4([p[5], p[7], 1., p[0]]);
        vec4([self.box_min.x, self.box_min.y, self.box_min.z, 0.]);
        vec4([self.box_max.x, self.box_max.y, self.box_max.z, 0.]);
        vec4([p[8], p[9], p[10], p[11]]);
        let mode = if self.output == Output::Depth { 1. } else { 0. };
        vec4([mode, 0., 0., 0.]);
        r.queue
            .write_buffer(&self.uniforms, 0, bytemuck::cast_slice(&data));
        if let Some(points) = self.pending_disk.take() {
            r.queue
                .write_buffer(&self.disk, 0, bytemuck::cast_slice(&points));
        }
        let (gtao_samples, pd_samples) = (p[6] as i32, p[14] as i32);
        let gtao = self.pipeline(r, "gtao", gtao_samples, HALF);
        let denoise = self.pipeline(r, "denoise", pd_samples, HALF);
        let copy = self.pipeline(r, "output", 0, HALF);
        let blend = self.pipeline(r, "blend", 0, HALF);
        let srgb = self.pipeline(r, "srgb", 0, out.options.format);
        let t = self
            .targets
            .as_ref()
            .ok_or(Error::Invalid("gtao targets"))?;
        let group = |name: &str| t.groups.get(name).ok_or(Error::Invalid("gtao group"));
        let mut encoder = r.device.create_command_encoder(&Default::default());
        let mut pass = |target: &wgpu::TextureView,
                        pipeline: &wgpu::RenderPipeline,
                        group: &wgpu::BindGroup,
                        clear: Option<wgpu::Color>| {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("gtao pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: target,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: clear.map_or(wgpu::LoadOp::Load, wgpu::LoadOp::Clear),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                ..Default::default()
            });
            pass.set_pipeline(pipeline);
            pass.set_bind_group(0, group, &[]);
            pass.draw(0..3, 0..1);
        };
        pass(&t.gtao, &gtao, group("beauty")?, Some(wgpu::Color::WHITE));
        pass(
            &t.pd,
            &denoise,
            group("gtaopd-noise")?,
            Some(wgpu::Color::WHITE),
        );
        // The output selection into the write buffer ( not cleared: the
        // composer's buffers keep their last contents ).
        match self.output {
            Output::Diffuse => pass(&t.write, &copy, group("beauty")?, None),
            Output::Ao => pass(&t.write, &copy, group("gtao")?, None),
            Output::Denoise => pass(&t.write, &copy, group("pd")?, None),
            Output::Depth => pass(&t.write, &copy, group("beauty")?, None),
            Output::Normal => pass(&t.write, &copy, group("normal")?, None),
            Output::Default => {
                pass(&t.write, &copy, group("beauty")?, None);
                pass(&t.write, &blend, group("pd")?, None);
            }
        }
        // OutputPass to the screen.
        pass(
            &t.screen.view,
            &srgb,
            group("write")?,
            Some(wgpu::Color::BLACK),
        );
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
        } else if !pan {
            // enablePan = false.
            self.controls.rotate(dx, dy, height);
        }
        Ok(())
    }
    /// The GUI: output, blendIntensity, the AO parameters, then the denoise
    /// parameters, in the page's order.
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        if index == 0 {
            self.output = match value as u32 {
                0 => Output::Default,
                1 => Output::Diffuse,
                2 => Output::Ao,
                4 => Output::Depth,
                5 => Output::Normal,
                _ => Output::Denoise,
            };
            return Ok(());
        }
        *self
            .params
            .get_mut(index - 1)
            .ok_or(Error::Invalid("gtao parameter"))? = f64::from(value);
        if index >= 13 {
            // generatePdSamplePointInitializer( samples, rings, radiusExponent ).
            let p = self.params;
            self.pending_disk = Some(disk(p[14] as usize, p[13], p[12]));
        }
        Ok(())
    }
    pub fn seek(&mut self, t: f64) {
        self.time = t;
    }
}
