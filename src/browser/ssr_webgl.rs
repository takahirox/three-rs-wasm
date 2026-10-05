//! webgl_postprocessing_ssr: the Draco bunny, a box, an icosphere and a
//! cone on a Phong ground in fog, a hemisphere and a spot light, through
//! SSRPass and OutputPass, with the ReflectorForSSRPass ground mirror. Each
//! frame, as SSRPass.render does:
//!
//! 1. the ground reflector renders the scene from its mirrored camera,
//!    clipped by the global plane y ≥ −clipBias, into its half-float target
//!    with a 16-bit depth texture ( unless seen from behind );
//! 2. the beauty render with the reflector drawn ( its color faded by the
//!    reflected depth's height, the fresnel and the opacity );
//! 3. the normal render ( MeshNormalMaterial override ) and, with selects,
//!    the metalness render ( white for the selects, black otherwise, without
//!    background or fog );
//! 4. SSRShader's screen-space march, SSRBlurShader twice, and the output
//!    mode's copies ( the reflections blended over the beauty, through the
//!    previous frame's result when bouncing ).
use super::controls_attributes::{Controls, camera_state};
use super::mirror_webgl::gl_perspective;
use super::ssao::{COPY, Pass, TEXEL, half, pass};
use crate::shader::ShaderProgram;
use crate::tsl::{NodeMaterial, Type, WgslFn};
use crate::{
    Error, Result, camera::*, geometry::*, material::*, math::*, render_target::*, renderer::*,
    scene::*,
};
use std::f64::consts::PI;
use std::sync::Arc;

/// SSRShader. Textures: 0 depth ( 16-bit, nearest ), 1 normal, 2 metalness,
/// 3 diffuse. u.custom[0..4] the GL projection, [4..8] its inverse,
/// [8] ( resolution, near, far ), [9] ( opacity, maxDistance, thickness,
/// MAX_STEP ), [10] ( SELECTIVE, DISTANCE_ATTENUATION, FRESNEL,
/// INFINITE_THICK ). gl_FragCoord counts from the bottom row; a fragment that
/// returns without writing outputs zero, as ANGLE initializes it.
const SSR: &str = "fn ssr_depth(uv:vec2<f32>)->f32{let d=textureLoad(tsl_texture_0,gl_texel(textureDimensions(tsl_texture_0),uv),0);return round(d*65535.0)/65535.0;}
fn ssr_view_z(depth:f32)->f32{let near=u.custom[8].z;let far=u.custom[8].w;return (near*far)/((far-near)*depth-far);}
fn ssr_view_position(uv:vec2<f32>,depth:f32,clip_w:f32)->vec3<f32>{let inverse=mat4x4<f32>(u.custom[4],u.custom[5],u.custom[6],u.custom[7]);var clip=vec4((vec3(uv,depth)-0.5)*2.0,1.0);clip*=clip_w;return (inverse*clip).xyz;}
fn ssr_normal(uv:vec2<f32>)->vec3<f32>{return 2.0*textureLoad(tsl_texture_1,gl_texel(textureDimensions(tsl_texture_1),uv),0).xyz-1.0;}
fn ssr_xy(p:vec3<f32>)->vec2<f32>{let projection=mat4x4<f32>(u.custom[0],u.custom[1],u.custom[2],u.custom[3]);let clip=projection*vec4(p,1.0);var xy=clip.xy/clip.w;xy=(xy+1.0)/2.0;return xy*u.custom[8].xy;}
fn ssr_line_distance(x0:vec3<f32>,x1:vec3<f32>,x2:vec3<f32>)->f32{return length(cross(x0-x1,x0-x2))/length(x2-x1);}
fn ssr_plane_distance(p:vec3<f32>,q:vec3<f32>,n:vec3<f32>)->f32{let d=-(n.x*q.x+n.y*q.y+n.z*q.z);return n.x*p.x+n.y*p.y+n.z*p.z+d;}
fn ssr()->vec4<f32>{let projection=mat4x4<f32>(u.custom[0],u.custom[1],u.custom[2],u.custom[3]);let resolution=u.custom[8].xy;let near=u.custom[8].z;let far=u.custom[8].w;let opacity=u.custom[9].x;let max_distance=u.custom[9].y;let thickness=u.custom[9].z;let max_step=u.custom[9].w;let flags=u.custom[10];
let v_uv=fragment_surface.uv;
if flags.x>0.5 {let metalness=textureLoad(tsl_texture_2,gl_texel(textureDimensions(tsl_texture_2),v_uv),0).r;if metalness==0.0 {return vec4(0.0);}}
let depth=ssr_depth(v_uv);let view_z=ssr_view_z(depth);if -view_z>=far {return vec4(0.0);}
let clip_w=projection[2][3]*view_z+projection[3][3];let view_position=ssr_view_position(v_uv,depth,clip_w);
let d0=vec2(fragment_surface.clip.x,resolution.y-fragment_surface.clip.y);
let view_normal=ssr_normal(v_uv);let incident=normalize(view_position);let reflect_dir=reflect(incident,view_normal);
let max_len=max_distance/dot(-incident,view_normal);var d1_view=view_position+reflect_dir*max_len;
if d1_view.z>-near {let t=(-near-view_position.z)/reflect_dir.z;d1_view=view_position+reflect_dir*t;}
let d1=ssr_xy(d1_view);let x_len=d1.x-d0.x;let y_len=d1.y-d0.y;let total=max(abs(x_len),abs(y_len));let x_span=x_len/total;let y_span=y_len/total;let s_step=1.0/total;var s=s_step;
for(var i=1.0;i<max_step;i+=1.0){
if i>=total {break;}
let xy=vec2(d0.x+i*x_span,d0.y+i*y_span);
if xy.x<0.0 || xy.x>resolution.x || xy.y<0.0 || xy.y>resolution.y {break;}
let uv=xy/resolution;let d=ssr_depth(uv);let vz=ssr_view_z(d);
if -vz>=far {continue;}
let cw=projection[2][3]*vz+projection[3][3];let vp=ssr_view_position(uv,d,cw);
let recip=1.0/view_position.z;let ray_z=1.0/(recip+s*(1.0/d1_view.z-recip));
if ray_z<=vz {
var hit=true;
if flags.w<0.5 {let away=ssr_line_distance(vp,view_position,d1_view);let neighbor=vec2(xy.x+1.0,xy.y)/resolution;let vpn=ssr_view_position(neighbor,d,cw);let min_thickness=(vpn.x-vp.x)*3.0;hit=away<=max(min_thickness,thickness);}
if hit {
let vn=ssr_normal(uv);if dot(reflect_dir,vn)>=0.0 {break;}
let distance=ssr_plane_distance(vp,view_position,view_normal);if distance>max_distance {break;}
var op=opacity;
if flags.y>0.5 {let ratio=1.0-(distance/max_distance);op=opacity*ratio*ratio;}
if flags.z>0.5 {op*=(dot(incident,reflect_dir)+1.0)/2.0;}
let color=textureLoad(tsl_texture_3,gl_texel(textureDimensions(tsl_texture_3),uv),0);
return vec4(color.xyz,op);}}
s+=s_step;}
return vec4(0.0);}";
/// SSRBlurShader: the center and its four neighbours, weighted by alpha.
const BLUR: &str = "fn ssr_blur()->vec4<f32>{let size=vec2<i32>(textureDimensions(tsl_texture_0));let c0=gl_texel(vec2<u32>(size),fragment_surface.uv);let c=textureLoad(tsl_texture_0,c0,0);let cl=textureLoad(tsl_texture_0,clamp(c0+vec2(-1,0),vec2(0),size-1),0);let cr=textureLoad(tsl_texture_0,clamp(c0+vec2(1,0),vec2(0),size-1),0);let cb=textureLoad(tsl_texture_0,clamp(c0+vec2(0,1),vec2(0),size-1),0);let ct=textureLoad(tsl_texture_0,clamp(c0+vec2(0,-1),vec2(0),size-1),0);let a=c.a*0.2+cl.a*0.2+cr.a*0.2+cb.a*0.2+ct.a*0.2;let rgb=(c.rgb*c.a*0.2+cl.rgb*cl.a*0.2+cr.rgb*cr.a*0.2+cb.rgb*cb.a*0.2+ct.rgb*ct.a*0.2)/a;return vec4(select(rgb,vec3(0.0),a==0.0),a);}";
/// SSRDepthShader: 1 − the linear depth.
const DEPTH: &str = "fn ssr_depth_view()->vec4<f32>{let near=u.custom[8].z;let far=u.custom[8].w;let d=round(textureLoad(tsl_texture_0,gl_texel(textureDimensions(tsl_texture_0),fragment_surface.uv),0)*65535.0)/65535.0;let view_z=(near*far)/((far-near)*d-far);let depth=(view_z+near)/(near-far);return vec4(vec3(1.0-depth),1.0);}";
/// ReflectorForSSRPass's shader ( useDepthTexture ). u.custom[0..4] the
/// texture matrix, [4] ( color, opacity ), [5] ( maxDistance, fresnelCoe,
/// near, far ), [6..10] the virtual camera's world matrix, [10..14] the GL
/// projection's inverse, [14] ( resolution, canvas height, P[2][3] ),
/// [15] ( P[3][3], FRESNEL, DISTANCE_ATTENUATION ).
const REFLECTOR: &str = "fn ssr_overlay(base:f32,blend:f32)->f32{return select(1.0-2.0*(1.0-base)*(1.0-blend),2.0*base*blend,base<0.5);}fn ssr_reflector()->vec4<f32>{let m=mat4x4<f32>(u.custom[0],u.custom[1],u.custom[2],u.custom[3]);let p=m*vec4(fragment_surface.local_position,1.0);let proj=p.xy/p.w;let base=textureSample(tsl_texture_0,tsl_sampler_0,vec2(proj.x,1.0-proj.y));let frag=vec2(fragment_surface.clip.x,u.custom[14].z-fragment_surface.clip.y);var uv=(frag-0.5)/u.custom[14].xy;uv.x=1.0-uv.x;let size=vec2<f32>(textureDimensions(tsl_texture_1));let texel=vec2<i32>(clamp(floor(vec2(proj.x,1.0-proj.y)*size),vec2(0.0),size-1.0));let depth=round(textureLoad(tsl_texture_1,texel,0)*65535.0)/65535.0;let near=u.custom[5].z;let far=u.custom[5].w;let view_z=(near*far)/((far-near)*depth-far);let clip_w=u.custom[14].w*view_z+u.custom[15].x;let inverse=mat4x4<f32>(u.custom[10],u.custom[11],u.custom[12],u.custom[13]);var clip=vec4((vec3(uv,depth)-0.5)*2.0,1.0);clip*=clip_w;let view_position=(inverse*clip).xyz;let world=(mat4x4<f32>(u.custom[6],u.custom[7],u.custom[8],u.custom[9])*vec4(view_position,1.0)).xyz;let max_distance=u.custom[5].x;if world.y>max_distance {discard;}var op=u.custom[4].w;if u.custom[15].z>0.5 {let ratio=1.0-(world.y/max_distance);op=u.custom[4].w*ratio*ratio;}if u.custom[15].y>0.5 {op*=u.custom[5].y;}let c=u.custom[4].rgb;return vec4(ssr_overlay(base.r,c.r),ssr_overlay(base.g,c.g),ssr_overlay(base.b,c.b),op);}";
const PROJECTION: &str =
    "fn project_vertex(surface:VertexOut,position:vec3<f32>)->VertexOut{return surface;}";
/// The GUI, in the page's order.
#[derive(Clone, Copy)]
struct Params {
    enable_ssr: bool,
    ground_reflector: bool,
    resolution_scale: f64,
    thickness: f64,
    infinite_thick: bool,
    auto_rotate: bool,
    fresnel: bool,
    distance_attenuation: bool,
    max_distance: f64,
    other_meshes: bool,
    bouncing: bool,
    output: u32,
    opacity: f64,
    blur: bool,
}
struct Targets {
    width: u32,
    height: u32,
    beauty: RenderTarget,
    normal: RenderTarget,
    metalness: RenderTarget,
    ssr: RenderTarget,
    blur: RenderTarget,
    blur2: RenderTarget,
    prev: RenderTarget,
    write: RenderTarget,
}
/// The copies SSRPass draws, by source ( and blending ).
struct Copies {
    beauty: Pass,
    ssr: Pass,
    blur2: Pass,
    prev: Pass,
    normal: Pass,
    metalness: Pass,
    ssr_blend: Pass,
    blur2_blend: Pass,
}
pub(super) struct Demo {
    controls: Controls,
    params: Params,
    time: f64,
    steps: u32,
    /// The selects: the bunny, then the box, icosphere and cone ( otherMeshes ).
    selects: Vec<Object3D>,
    reflector: Object3D,
    reflector_target: RenderTarget,
    reflector_camera: Object3D,
    /// The override renders: the meshes with MeshNormalMaterial, and with
    /// the metalness materials ( no background or fog ); their nodes in the
    /// order ground, selects; their cameras.
    normal_scene: (Scene, Object3D, Vec<Object3D>),
    metal_scene: (Scene, Object3D, Vec<Object3D>),
    targets: Option<Targets>,
    /// SSRShader reading the beauty, and ( bouncing ) the previous result.
    ssr: [Pass; 2],
    blurs: [Pass; 2],
    depth: Pass,
    copies: Copies,
    output: Pass,
    nearest: wgpu::Sampler,
    comparison: wgpu::Sampler,
    linear: wgpu::Sampler,
}
fn named(name: &str) -> Color {
    Color::from_hex(match name {
        "green" => 0x008000,
        "cyan" => 0x00ffff,
        _ => 0xffff00,
    })
}
/// Pass with explicit sample types ( a depth texture among them ).
async fn typed_pass(
    r: &Renderer,
    name: &str,
    source: &str,
    textures: &[(
        &wgpu::TextureView,
        &wgpu::Sampler,
        Type,
        wgpu::TextureSampleType,
    )],
    blending: Option<wgpu::BlendState>,
) -> Result<Pass> {
    let mut p = pass(r, name, source, textures).await?;
    let m = p.material()?;
    m.properties.blending = blending;
    m.properties.transparent = blending.is_some();
    Ok(p)
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, _id: u32, r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 35.,
            near: 0.1,
            far: 15.,
            aspect,
            ..Default::default()
        }));
        s.get_mut(c)?.position =
            Vector3::new(0.13271600513224902, 0.3489546826045913, 0.43921296427927076);
        s.background = Color::from_hex(0x443333);
        s.fog = Some(Fog::Linear {
            color: Color::from_hex(0x443333),
            near: 1.,
            far: 4.,
        });
        let mut phong = MeshPhongMaterial::default();
        phong.properties.color = Color::from_hex(0xcbcbcb);
        let ground = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(PlaneGeometry::build(8., 8., 1, 1)?),
            Arc::new(Material::Phong(phong)),
        )));
        let n = s.get_mut(ground)?;
        n.quaternion = Quaternion::from_rotation_x(-PI / 2.);
        n.position.y = -0.0001;
        let hemi = s.insert(NodeKind::Light(Light::Hemisphere {
            sky: Color::from_hex(0x8d7c7c),
            ground: Color::from_hex(0x494966),
            intensity: 3.,
        }));
        s.get_mut(hemi)?.position = Vector3::Y;
        let spot = s.insert(NodeKind::Light(Light::Spot {
            color: Color::WHITE,
            intensity: 8.,
            target: Vector3::ZERO,
            distance: 0.,
            decay: 2.,
            angle: PI / 16.,
            penumbra: 0.5,
            ies: false,
        }));
        s.get_mut(spot)?.position = Vector3::new(-1., 1., 1.);
        let standard = |color: Color| {
            let mut m = MeshStandardMaterial::default();
            m.properties.color = color;
            Arc::new(Material::Standard(m))
        };
        // DRACOLoader, then computeVertexNormals().
        let mut geometry = crate::compression::decode_draco(
            &super::gltf_viewer::fetch("/web/gallery/assets/draco-variants/bunny.drc").await?,
        )?;
        geometry.compute_vertex_normals()?;
        let bunny = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(geometry),
            standard(Color::from_hex(0xa5a5a5)),
        )));
        s.get_mut(bunny)?.position.y = -0.0365;
        let mut selects = vec![bunny];
        for (geometry, color, position) in [
            (
                BoxGeometry::build(0.05, 0.05, 0.05)?,
                "green",
                Vector3::new(-0.12, 0.025, 0.015),
            ),
            (
                IcosahedronGeometry::build(0.025, 4)?,
                "cyan",
                Vector3::new(-0.05, 0.025, 0.08),
            ),
            (
                // ConeGeometry( 0.025, 0.05, 64 ).
                CylinderGeometry::build(0., 0.025, 0.05, 64, 1, false, 0., 2. * PI)?,
                "yellow",
                Vector3::new(-0.05, 0.025, -0.055),
            ),
        ] {
            let mesh = s.insert(NodeKind::Mesh(Mesh::new(
                Arc::new(geometry),
                standard(named(color)),
            )));
            s.get_mut(mesh)?.position = position;
            selects.push(mesh);
        }
        // The ground reflector: its target at the window's CSS size.
        let (css_w, css_h, _) = super::controls_attributes::viewport_css();
        let reflector_target = RenderTarget::with_options(
            &r.device,
            (css_w as u32).max(1),
            (css_h as u32).max(1),
            RenderTargetOptions {
                format: wgpu::TextureFormat::Rgba16Float,
                depth_buffer: true,
                ..Default::default()
            },
        )?;
        let linear = r.device.create_sampler(&wgpu::SamplerDescriptor {
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        let nearest = r.device.create_sampler(&Default::default());
        let comparison = r.device.create_sampler(&wgpu::SamplerDescriptor {
            compare: Some(wgpu::CompareFunction::LessEqual),
            ..Default::default()
        });
        let reflector_color = reflector_target.texture.create_view(&Default::default());
        let reflector_depth = reflector_target
            .depth_view
            .clone()
            .ok_or(Error::Invalid("reflector depth"))?;
        let node = WgslFn::new("ssr_reflector", REFLECTOR, &[], Type::Vec4)?.call(&[]);
        let program = ShaderProgram::with_projection_and_sample_types(
            r,
            &NodeMaterial::new(node)
                .wgsl_with_texture_types(&[Type::Texture, Type::DepthTexture], &[])?,
            &[(&reflector_color, &linear), (&reflector_depth, &comparison)],
            &[wgpu::TextureViewDimension::D2; 2],
            &[
                wgpu::TextureSampleType::Float { filterable: true },
                wgpu::TextureSampleType::Depth,
            ],
            PROJECTION,
        )
        .await?;
        let mut material = ShaderMaterial::new(Arc::new(program));
        material.properties.transparent = true;
        material.properties.depth_write = false;
        let reflector = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(PlaneGeometry::build(1., 1., 1, 1)?),
            Arc::new(Material::Shader(material)),
        )));
        let n = s.get_mut(reflector)?;
        n.quaternion = Quaternion::from_rotation_x(-PI / 2.);
        n.visible = false;
        let reflector_camera = s.insert(NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 35.,
            near: 0.1,
            far: 15.,
            aspect,
            ..Default::default()
        })));
        let mut controls = Controls::new(Some(0.05), (0., f64::INFINITY), PI, true);
        controls.set_target(Vector3::new(0., 0.0635, 0.));
        controls.update(s, c)?;
        let metal = |color: Color| {
            let mut m = MeshBasicMaterial::default();
            m.properties.color = color;
            Arc::new(Material::Basic(m))
        };
        let normal_material = Arc::new(Material::Normal(MeshNormalMaterial::default()));
        let (on, off) = (metal(Color::WHITE), metal(Color::BLACK));
        // The override scenes mirror the ground and the selects.
        let mirror = |s: &Scene,
                      background: Color,
                      alpha: f64,
                      material: &dyn Fn(usize) -> Arc<Material>|
         -> Result<(Scene, Object3D, Vec<Object3D>)> {
            let mut scene = Scene::new();
            scene.background = background;
            scene.background_alpha = alpha;
            let camera = scene.insert(NodeKind::Camera(Camera::Perspective(
                PerspectiveCamera::default(),
            )));
            let mut nodes = vec![];
            for (i, &h) in [ground].iter().chain(selects.iter()).enumerate() {
                let n = s.get(h)?;
                let NodeKind::Mesh(m) = &n.kind else {
                    return Err(Error::Invalid("ssr mesh"));
                };
                let node = scene.insert(NodeKind::Mesh(Mesh::new(m.geometry.clone(), material(i))));
                let (position, quaternion, scale) = (n.position, n.quaternion, n.scale);
                let c = scene.get_mut(node)?;
                c.position = position;
                c.quaternion = quaternion;
                c.scale = scale;
                nodes.push(node);
            }
            Ok((scene, camera, nodes))
        };
        let normal_scene = mirror(s, Color::from_hex(0x443333), 1., &|_| {
            normal_material.clone()
        })?;
        let metal_scene = mirror(s, Color::BLACK, 0., &|i| {
            if i == 0 { off.clone() } else { on.clone() }
        })?;
        let placeholder = super::ssao::target(r, 1, 1, true)?;
        let view = placeholder.texture.create_view(&Default::default());
        let depth_view = placeholder
            .depth_view
            .clone()
            .ok_or(Error::Invalid("ssr depth"))?;
        let color = |v| (v, &nearest, Type::Texture, half());
        let depth = (
            &depth_view,
            &comparison,
            Type::DepthTexture,
            wgpu::TextureSampleType::Depth,
        );
        let ssr_source = format!("{TEXEL}{SSR}");
        let ssr_textures = [depth, color(&view), color(&view), color(&view)];
        let one = [color(&view)];
        let ssr = || typed_pass(r, "ssr", &ssr_source, &ssr_textures, None);
        let blur_source = format!("{TEXEL}{BLUR}");
        let copy_source = format!("{TEXEL}{COPY}");
        let copy = || typed_pass(r, "ssao_copy", &copy_source, &one, None);
        let normal_blend = wgpu::BlendState {
            color: wgpu::BlendComponent {
                src_factor: wgpu::BlendFactor::SrcAlpha,
                dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                operation: wgpu::BlendOperation::Add,
            },
            alpha: wgpu::BlendComponent {
                src_factor: wgpu::BlendFactor::One,
                dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                operation: wgpu::BlendOperation::Add,
            },
        };
        let blend = || typed_pass(r, "ssao_copy", &copy_source, &one, Some(normal_blend));
        let copies = Copies {
            beauty: copy().await?,
            ssr: copy().await?,
            blur2: copy().await?,
            prev: copy().await?,
            normal: copy().await?,
            metalness: copy().await?,
            ssr_blend: blend().await?,
            blur2_blend: blend().await?,
        };
        Ok(Self {
            controls,
            params: Params {
                enable_ssr: true,
                ground_reflector: true,
                resolution_scale: 1.,
                thickness: 0.018,
                infinite_thick: false,
                auto_rotate: true,
                fresnel: true,
                distance_attenuation: true,
                max_distance: 0.1,
                other_meshes: true,
                bouncing: false,
                output: 0,
                opacity: 1.,
                blur: true,
            },
            time: 0.,
            steps: 1,
            selects,
            reflector,
            reflector_target,
            reflector_camera,
            normal_scene,
            metal_scene,
            targets: None,
            ssr: [ssr().await?, ssr().await?],
            blurs: [
                typed_pass(r, "ssr_blur", &blur_source, &one, None).await?,
                typed_pass(r, "ssr_blur", &blur_source, &one, None).await?,
            ],
            depth: typed_pass(
                r,
                "ssr_depth_view",
                &format!("{TEXEL}{DEPTH}"),
                &[depth],
                None,
            )
            .await?,
            copies,
            output: copy().await?,
            nearest,
            comparison,
            linear,
        })
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, dt: f64, animate: bool) -> Result<()> {
        if animate {
            self.time += dt;
        }
        self.steps = self.steps.max(u32::from(animate));
        Ok(())
    }
    /// render(): the auto-rotating camera follows Date.now() * 0.0003, or the
    /// damped controls update once per frame.
    pub fn prepare(&mut self, s: &mut Scene, c: Object3D, _r: &Renderer) -> Result<()> {
        let steps = std::mem::take(&mut self.steps);
        if self.params.auto_rotate {
            let timer = self.time * 1000. * 0.0003;
            s.get_mut(c)?.position = Vector3::new(timer.sin() * 0.5, 0.2135, timer.cos() * 0.5);
            s.look_at(c, Vector3::new(0., 0.0635, 0.))?;
        } else {
            for _ in 0..steps {
                self.controls.update(s, c)?;
            }
        }
        for &mesh in &self.selects[1..] {
            s.get_mut(mesh)?.visible = self.params.other_meshes;
        }
        Ok(())
    }
    fn resize(&mut self, r: &Renderer, w: u32, h: u32) -> Result<()> {
        let scale = self.params.resolution_scale;
        let (ew, eh) = (
            ((scale * w as f64).round() as u32).max(1),
            ((scale * h as f64).round() as u32).max(1),
        );
        let half_target = |w, h, depth| super::ssao::target(r, w, h, depth);
        let byte = |w, h| {
            RenderTarget::with_options(
                &r.device,
                w,
                h,
                RenderTargetOptions {
                    format: wgpu::TextureFormat::Rgba8Unorm,
                    depth_buffer: false,
                    ..Default::default()
                },
            )
        };
        let t = Targets {
            width: w,
            height: h,
            beauty: half_target(w, h, true)?,
            normal: half_target(w, h, true)?,
            metalness: half_target(w, h, true)?,
            ssr: byte(ew, eh)?,
            blur: byte(ew, eh)?,
            blur2: byte(ew, eh)?,
            prev: byte(ew, eh)?,
            write: half_target(w, h, false)?,
        };
        let view = |t: &RenderTarget| t.texture.create_view(&Default::default());
        let depth = t
            .beauty
            .depth_view
            .clone()
            .ok_or(Error::Invalid("ssr beauty depth"))?;
        let n = &self.nearest;
        let rebind =
            |p: &mut Pass, textures: &[(&wgpu::TextureView, &wgpu::Sampler)]| -> Result<()> {
                Arc::make_mut(&mut p.material()?.program).rebind(r, &[], textures)
            };
        let (beauty, normal, metalness, ssr, blur, blur2, prev, write) = (
            view(&t.beauty),
            view(&t.normal),
            view(&t.metalness),
            view(&t.ssr),
            view(&t.blur),
            view(&t.blur2),
            view(&t.prev),
            view(&t.write),
        );
        for (p, diffuse) in self.ssr.iter_mut().zip([&beauty, &prev]) {
            rebind(
                p,
                &[
                    (&depth, &self.comparison),
                    (&normal, n),
                    (&metalness, n),
                    (diffuse, n),
                ],
            )?;
        }
        rebind(&mut self.blurs[0], &[(&ssr, n)])?;
        rebind(&mut self.blurs[1], &[(&blur, n)])?;
        rebind(&mut self.depth, &[(&depth, &self.comparison)])?;
        let c = &mut self.copies;
        for (p, source) in [
            (&mut c.beauty, &beauty),
            (&mut c.ssr, &ssr),
            (&mut c.blur2, &blur2),
            (&mut c.prev, &prev),
            (&mut c.normal, &normal),
            (&mut c.metalness, &metalness),
            (&mut c.ssr_blend, &ssr),
            (&mut c.blur2_blend, &blur2),
        ] {
            rebind(p, &[(source, n)])?;
        }
        rebind(&mut self.output, &[(&write, n)])?;
        self.targets = Some(t);
        Ok(())
    }
    /// ReflectorForSSRPass.doRender(): the mirrored camera's render into the
    /// reflector's target, clipped by the global plane, and its uniforms.
    fn render_reflector(
        &mut self,
        r: &Renderer,
        s: &mut Scene,
        c: Object3D,
        projection: Matrix4,
        canvas_height: f64,
    ) -> Result<()> {
        let (camera, world) = s.camera(c)?;
        let Camera::Perspective(p) = camera.clone() else {
            return Err(Error::Invalid("ssr camera"));
        };
        let camera_position = world.w_axis.truncate();
        let position = camera_position.normalize();
        let reflected = position - Vector3::Y * (2. * position.dot(Vector3::Y));
        let fresnel = (position.dot(reflected) + 1.) / 2.;
        // rotation.x = −π / 2 at the origin.
        let model = Matrix4::from_rotation_x(-PI / 2.);
        let normal = model.transform_vector3(Vector3::Z).normalize();
        let mirror = model.w_axis.truncate();
        let reflect = |v: Vector3| v - normal * (2. * v.dot(normal));
        let to_mirror = mirror - camera_position;
        let mut values = [[0f32; 4]; 16];
        let (css_w, css_h, _) = super::controls_attributes::viewport_css();
        let inverse = projection.inverse();
        let e = projection.to_cols_array();
        if to_mirror.dot(normal) <= 0. {
            let eye = -reflect(to_mirror) + mirror;
            let rotation = Matrix4::from_mat3(glam::DMat3::from_mat4(world));
            let look = rotation.transform_vector3(Vector3::new(0., 0., -1.)) + camera_position;
            let target = -reflect(mirror - look) + mirror;
            let up = reflect(rotation.transform_vector3(Vector3::Y));
            let n = s.get_mut(self.reflector_camera)?;
            n.position = eye;
            n.up = up;
            n.kind = NodeKind::Camera(Camera::Perspective(p));
            s.look_at(self.reflector_camera, target)?;
            s.update()?;
            let virtual_world = s.camera(self.reflector_camera)?.1;
            let texture_matrix = Matrix4::from_cols_array(&[
                0.5, 0., 0., 0., 0., 0.5, 0., 0., 0., 0., 0.5, 0., 0.5, 0.5, 0.5, 1.,
            ]) * projection
                * virtual_world.inverse()
                * model;
            let columns = |m: Matrix4, out: &mut [[f32; 4]]| {
                let a = m.to_cols_array();
                for k in 0..4 {
                    out[k] = [
                        a[k * 4] as f32,
                        a[k * 4 + 1] as f32,
                        a[k * 4 + 2] as f32,
                        a[k * 4 + 3] as f32,
                    ];
                }
            };
            columns(texture_matrix, &mut values[0..4]);
            columns(virtual_world, &mut values[6..10]);
            columns(inverse, &mut values[10..14]);
            s.clipping_planes = vec![Plane {
                normal: Vector3::Y,
                constant: 0.0003,
            }];
            let result = r.render(s, self.reflector_camera, &self.reflector_target);
            s.clipping_planes.clear();
            result?;
        } else if let NodeKind::Mesh(mesh) = &s.get(self.reflector)?.kind
            && let Material::Shader(m) = mesh.materials[0].as_ref()
        {
            values = m.uniforms;
        }
        let color = Color::from_hex(0x888888);
        let pr = &self.params;
        values[4] = [
            color.0.x as f32,
            color.0.y as f32,
            color.0.z as f32,
            pr.opacity as f32,
        ];
        values[5] = [pr.max_distance as f32, fresnel as f32, 0.1, 15.];
        values[14] = [
            css_w as f32,
            css_h as f32,
            canvas_height as f32,
            e[11] as f32,
        ];
        values[15] = [
            e[15] as f32,
            f32::from(u8::from(pr.fresnel)),
            f32::from(u8::from(pr.distance_attenuation)),
            0.,
        ];
        if let NodeKind::Mesh(mesh) = &mut s.get_mut(self.reflector)?.kind
            && let Material::Shader(m) = Arc::make_mut(&mut mesh.materials[0])
        {
            m.uniforms = values;
        }
        Ok(())
    }
    /// An override render: the mirror scene with the camera and the other
    /// meshes' visibility.
    fn render_mirror(
        mirror: &mut (Scene, Object3D, Vec<Object3D>),
        r: &Renderer,
        s: &Scene,
        c: Object3D,
        other_meshes: bool,
        target: &RenderTarget,
    ) -> Result<()> {
        let n = s.get(c)?;
        let (kind, position, quaternion) = (n.kind.clone(), n.position, n.quaternion);
        let (scene, camera, nodes) = mirror;
        let n = scene.get_mut(*camera)?;
        n.kind = kind;
        n.position = position;
        n.quaternion = quaternion;
        for &node in &nodes[2..] {
            scene.get_mut(node)?.visible = other_meshes;
        }
        r.render(scene, *camera, target)
    }
    pub fn render(
        &mut self,
        r: &Renderer,
        s: &mut Scene,
        c: Object3D,
        out: &RenderTarget,
    ) -> Result<bool> {
        if !self.params.enable_ssr {
            r.render(s, c, out)?;
            return Ok(true);
        }
        if self
            .targets
            .as_ref()
            .is_none_or(|t| t.width != out.width || t.height != out.height)
        {
            self.resize(r, out.width, out.height)?;
        }
        s.update()?;
        let Camera::Perspective(p) = s.camera(c)?.0.clone() else {
            return Err(Error::Invalid("ssr camera"));
        };
        let projection = gl_perspective(p.fov, p.aspect, p.near, p.far);
        // onWindowResize(): the reflector's target follows the window's CSS size.
        let (css_w, css_h, _) = super::controls_attributes::viewport_css();
        let size = ((css_w as u32).max(1), (css_h as u32).max(1));
        if (self.reflector_target.width, self.reflector_target.height) != size {
            self.reflector_target = RenderTarget::with_options(
                &r.device,
                size.0,
                size.1,
                RenderTargetOptions {
                    format: wgpu::TextureFormat::Rgba16Float,
                    depth_buffer: true,
                    ..Default::default()
                },
            )?;
            let color = self
                .reflector_target
                .texture
                .create_view(&Default::default());
            let depth = self
                .reflector_target
                .depth_view
                .clone()
                .ok_or(Error::Invalid("reflector depth"))?;
            if let NodeKind::Mesh(mesh) = &mut s.get_mut(self.reflector)?.kind
                && let Material::Shader(m) = Arc::make_mut(&mut mesh.materials[0])
            {
                Arc::make_mut(&mut m.program).rebind(
                    r,
                    &[],
                    &[(&color, &self.linear), (&depth, &self.comparison)],
                )?;
            }
        }
        let selective = self.params.ground_reflector;
        if selective {
            self.render_reflector(r, s, c, projection, out.height as f64)?;
            s.get_mut(self.reflector)?.visible = true;
        }
        let mut t = self.targets.take().ok_or(Error::Invalid("ssr targets"))?;
        let result = self.passes(r, s, c, out, &mut t, projection, selective);
        self.targets = Some(t);
        s.get_mut(self.reflector)?.visible = false;
        result?;
        Ok(true)
    }
    #[allow(clippy::too_many_arguments)]
    fn passes(
        &mut self,
        r: &Renderer,
        s: &mut Scene,
        c: Object3D,
        out: &RenderTarget,
        t: &mut Targets,
        projection: Matrix4,
        selective: bool,
    ) -> Result<()> {
        r.render(s, c, &t.beauty)?;
        s.get_mut(self.reflector)?.visible = false;
        // The normal override, cleared by the background color as WebGLBackground does.
        Self::render_mirror(
            &mut self.normal_scene,
            r,
            s,
            c,
            self.params.other_meshes,
            &t.normal,
        )?;
        if selective {
            // _renderMetalness: no background or fog, cleared transparent.
            Self::render_mirror(
                &mut self.metal_scene,
                r,
                s,
                c,
                self.params.other_meshes,
                &t.metalness,
            )?;
        }
        let pr = self.params;
        let mut uniforms = [[0f32; 4]; 16];
        let columns = |m: Matrix4, out: &mut [[f32; 4]]| {
            let a = m.to_cols_array();
            for k in 0..4 {
                out[k] = [
                    a[k * 4] as f32,
                    a[k * 4 + 1] as f32,
                    a[k * 4 + 2] as f32,
                    a[k * 4 + 3] as f32,
                ];
            }
        };
        columns(projection, &mut uniforms[0..4]);
        columns(projection.inverse(), &mut uniforms[4..8]);
        let (ew, eh) = (t.ssr.width as f32, t.ssr.height as f32);
        uniforms[8] = [ew, eh, 0.1, 15.];
        uniforms[9] = [
            pr.opacity as f32,
            pr.max_distance as f32,
            pr.thickness as f32,
            (ew * ew + eh * eh).sqrt(),
        ];
        let flag = |b: bool| f32::from(u8::from(b));
        uniforms[10] = [
            flag(selective),
            flag(pr.distance_attenuation),
            flag(pr.fresnel),
            flag(pr.infinite_thick),
        ];
        self.ssr[usize::from(pr.bouncing)].render(r, &t.ssr, &uniforms)?;
        if pr.blur {
            self.blurs[0].render(r, &t.blur, &uniforms)?;
            self.blurs[1].render(r, &t.blur2, &uniforms)?;
        }
        // The blended copies draw over their target without a clear.
        let blended = |p: &mut Pass, target: &mut RenderTarget| -> Result<()> {
            target.set_load_color(true);
            let result = p.render(r, target, &uniforms);
            target.set_load_color(false);
            result
        };
        match pr.output {
            0 => {
                if pr.bouncing {
                    self.copies.beauty.render(r, &t.prev, &uniforms)?;
                    if pr.blur {
                        blended(&mut self.copies.blur2_blend, &mut t.prev)?;
                    } else {
                        blended(&mut self.copies.ssr_blend, &mut t.prev)?;
                    }
                    self.copies.prev.render(r, &t.write, &uniforms)?;
                } else {
                    self.copies.beauty.render(r, &t.write, &uniforms)?;
                    if pr.blur {
                        blended(&mut self.copies.blur2_blend, &mut t.write)?;
                    } else {
                        blended(&mut self.copies.ssr_blend, &mut t.write)?;
                    }
                }
            }
            1 => {
                if pr.blur {
                    self.copies.blur2.render(r, &t.write, &uniforms)?;
                } else {
                    self.copies.ssr.render(r, &t.write, &uniforms)?;
                }
                if pr.bouncing {
                    if pr.blur {
                        self.copies.blur2.render(r, &t.prev, &uniforms)?;
                    } else {
                        self.copies.beauty.render(r, &t.prev, &uniforms)?;
                    }
                    blended(&mut self.copies.ssr_blend, &mut t.prev)?;
                }
            }
            3 => self.copies.beauty.render(r, &t.write, &uniforms)?,
            4 => self.depth.render(r, &t.write, &uniforms)?,
            5 => self.copies.normal.render(r, &t.write, &uniforms)?,
            _ => self.copies.metalness.render(r, &t.write, &uniforms)?,
        }
        // OutputPass.
        self.output.render(r, out, &uniforms)
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
        // controls.enabled = !autoRotate.
        if self.params.auto_rotate {
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
        Ok(())
    }
    /// The GUI, in the page's order.
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        let on = value > 0.5;
        let p = &mut self.params;
        match index {
            0 => p.enable_ssr = on,
            1 => p.ground_reflector = on,
            2 => {
                p.resolution_scale = value as f64;
                self.targets = None;
            }
            3 => p.thickness = value as f64,
            4 => p.infinite_thick = on,
            5 => p.auto_rotate = on,
            6 => p.fresnel = on,
            7 => p.distance_attenuation = on,
            8 => p.max_distance = value as f64,
            9 => p.other_meshes = on,
            10 => p.bouncing = on,
            11 => {
                p.output = [0, 1, 3, 4, 5, 7]
                    .get(value.round() as usize)
                    .copied()
                    .unwrap_or(0)
            }
            12 => p.opacity = value as f64,
            13 => p.blur = on,
            _ => return Err(Error::Invalid("ssr parameter")),
        }
        Ok(())
    }
    pub fn seek(&mut self, t: f64) {
        self.time = t;
        self.steps += 1;
    }
}
