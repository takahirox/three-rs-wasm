//! webgl_random_uv: the Draco ShaderBall2 under a VSM-shadowing directional
//! light over a ShadowMaterial ground, lit and backed by the blurred lobe HDR
//! under ACES Filmic tone mapping. The page's onBeforeCompile patches become
//! TSL surface graphs on the GPU: the outer shell's jade map is sampled by
//! textureNoTile ( the noise texture or three's rand value noise picks two
//! offset virtual patterns, sampled with the base uv's gradients ), and the
//! shell and the inner ball replace their alpha with the dissolve ramp of the
//! shaderball_ds map. OrbitControls render on demand.
use super::controls_attributes::{Controls, camera_state};
use super::gltf_viewer::{decode_texture_image, fetch, load_asset};
use crate::shadow::ShadowFilter;
use crate::tsl::surface::SurfaceNodes;
use crate::tsl::{Node, Texture as Tex, Type, WgslFn, base_color, float, uniform, uv, vec4};
use crate::{Result, camera::*, geometry::*, material::*, math::*, renderer::*, scene::*};
use std::f64::consts::PI;
use std::sync::Arc;

const ASSETS: &str = "/web/gallery/assets";

/// three's `rand`, directNoise and the index / offsets / blend of textureNoTile.
const NO_TILE: &str = "fn random_uv_rand(uv:vec2<f32>)->f32{let dt=dot(uv,vec2(12.9898,78.233));let sn=dt-3.141592653589793*floor(dt/3.141592653589793);return fract(sin(sn)*43758.5453);}
fn random_uv_k(noise:f32,p:vec2<f32>,use_noise:f32)->f32{
if use_noise==1.0 {return noise;}
let ip=floor(p);var u=fract(p);u=u*u*(3.0-2.0*u);
let res=mix(mix(random_uv_rand(ip),random_uv_rand(ip+vec2(1.0,0.0)),u.x),mix(random_uv_rand(ip+vec2(0.0,1.0)),random_uv_rand(ip+vec2(1.0,1.0)),u.x),u.y);
return res*res;}";

fn wgsl(name: &str, body: &str, arguments: &[Type], result: Type) -> Result<WgslFn> {
    WgslFn::new(name, &format!("{NO_TILE}\n{body}"), arguments, result)
}

/// diffuseColor.a = clamp(( vv − r ) / threshold ), r = disolve ( 1 + 2 threshold ) − threshold.
fn dissolve(texture: Tex, uniforms: &Node) -> Result<Node> {
    let f = WgslFn::new(
        "random_uv_dissolve",
        "fn random_uv_dissolve(vv:f32,disolve:f32,threshold:f32)->f32{let r=disolve*(1.0+threshold*2.0)-threshold;return clamp((vv-r)*(1.0/threshold),0.0,1.0);}",
        &[Type::Float, Type::Float, Type::Float],
        Type::Float,
    )?;
    Ok(f.call(&[
        texture.sample(uv()).y(),
        uniforms.swizzle("x"),
        uniforms.swizzle("y"),
    ]))
}

/// The shell's graph: textures are jade ( 0 ), noise ( 1 ) and shaderball_ds ( 2 );
/// custom[0] = ( disolve, threshold, enableRandom, useNoiseMap ),
/// custom[1] = ( useSuslikMethod, debugNoise ).
fn shell_graph() -> Result<SurfaceNodes> {
    let p = uniform(0, Type::Vec4);
    let q = uniform(1, Type::Vec4);
    // vMapUv: the map's repeat ( 20, 20 ).
    let map_uv = uv() * float(20.);
    // noise.png keeps TextureLoader's flipY.
    let noise_uv = map_uv.clone() * float(0.005);
    let noise = Tex::External(1).sample(vec2_flip(noise_uv)?).x();
    let k = wgsl(
        "random_uv_k_value",
        "fn random_uv_k_value(noise:f32,p:vec2<f32>,use_noise:f32)->f32{return random_uv_k(noise,p,use_noise);}",
        &[Type::Float, Type::Vec2, Type::Float],
        Type::Float,
    )?
    .call(&[noise, map_uv.clone(), p.swizzle("w")]);
    // ( offa, offb ) and f.
    let offsets = WgslFn::new(
        "random_uv_offsets",
        "fn random_uv_offsets(k:f32,suslik:f32)->vec4<f32>{let index=k*8.0;var ia=floor(index);var ib=ia+1.0;if suslik==1.0 {ia=floor(index+0.5);ib=floor(index);}return vec4(sin(vec2(3.0,7.0)*ia),sin(vec2(3.0,7.0)*ib));}",
        &[Type::Float, Type::Float],
        Type::Vec4,
    )?
    .call(&[k.clone(), q.swizzle("x")]);
    let f = WgslFn::new(
        "random_uv_f",
        "fn random_uv_f(k:f32,suslik:f32)->f32{let f=fract(k*8.0);if suslik==1.0 {return min(f,1.0-f)*2.0;}return f;}",
        &[Type::Float, Type::Float],
        Type::Float,
    )?
    .call(&[k, q.swizzle("x")]);
    let derivative = |name: &str, op: &str| {
        WgslFn::new(
            name,
            &format!("fn {name}(v:vec2<f32>)->vec2<f32>{{return {op}(v);}}"),
            &[Type::Vec2],
            Type::Vec2,
        )
        .map(|f| f.call(std::slice::from_ref(&map_uv)))
    };
    let dx = derivative("random_uv_dx", "dpdx")?;
    let dy = derivative("random_uv_dy", "dpdy")?;
    let cola = Tex::External(0).sample_grad(
        map_uv.clone() + offsets.swizzle("xy"),
        dx.clone(),
        dy.clone(),
    );
    let colb = Tex::External(0).sample_grad(map_uv.clone() + offsets.swizzle("zw"), dx, dy);
    let plain = Tex::External(0).sample(map_uv);
    let map = WgslFn::new(
        "random_uv_map",
        "fn random_uv_map(a:vec4<f32>,b:vec4<f32>,f:f32,debug:f32,plain:vec4<f32>,enabled:f32)->vec4<f32>{
if enabled!=1.0 {return plain;}
var cola=a;var colb=b;
if debug==1.0 {cola=vec4(0.1,0.0,0.0,1.0);colb=vec4(0.0,0.0,1.0,1.0);}
let d=cola-colb;
return mix(cola,colb,smoothstep(0.2,0.8,f-0.1*(d.x+d.y+d.z)));}",
        &[Type::Vec4, Type::Vec4, Type::Float, Type::Float, Type::Vec4, Type::Float],
        Type::Vec4,
    )?
    .call(&[cola, colb, f, q.swizzle("y"), plain, p.swizzle("z")]);
    Ok(SurfaceNodes {
        color: Some(vec4(
            base_color().rgb() * map.rgb(),
            dissolve(Tex::External(2), &p)?,
        )),
        ..Default::default()
    })
}

fn vec2_flip(v: Node) -> Result<Node> {
    Ok(WgslFn::new(
        "random_uv_flip",
        "fn random_uv_flip(v:vec2<f32>)->vec2<f32>{return vec2(v.x,1.0-v.y);}",
        &[Type::Vec2],
        Type::Vec2,
    )?
    .call(&[v]))
}

pub struct Demo {
    controls: Controls,
    shell: Option<Object3D>,
    inner: Option<Object3D>,
    ground: Object3D,
    /// disolve, threshold, enableRandom, useNoiseMap, useSuslikMethod, debugNoise.
    settings: [f32; 6],
    roughness: f64,
    metalness: f64,
    /// A GUI change waiting for the next prepare.
    dirty: bool,
}

impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, _id: u32, r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 45.,
            near: 0.1,
            far: 20.,
            aspect,
            ..Default::default()
        }));
        s.get_mut(c)?.position = Vector3::new(-0.8, 0.6, 1.5);
        s.aces_tone_mapping = true;
        s.exposure = 0.7;
        let light = s.insert(NodeKind::Light(Light::Directional {
            color: Color::WHITE,
            intensity: 3.,
            target: Vector3::ZERO,
        }));
        let n = s.get_mut(light)?;
        n.position = Vector3::new(-0.5, 1., 0.8);
        n.cast_shadow = true;
        n.shadow = crate::shadow::Shadow {
            near: 0.5,
            far: 3.,
            extent: 2.,
            map_size: Some(1024),
            radius: 16.,
            bias: -0.0005,
            filter: ShadowFilter::Vsm,
            ..Default::default()
        };
        let mut plane = PlaneGeometry::build(2., 2., 1, 1)?;
        plane.rotate_x(-PI * 0.5)?;
        let mut shadow = ShadowMaterial::default();
        shadow.properties.opacity = 0.5;
        let ground = s.insert(NodeKind::Mesh(Mesh {
            geometry: Arc::new(plane),
            materials: vec![Arc::new(Material::Shadow(shadow))],
        }));
        let n = s.get_mut(ground)?;
        n.receive_shadow = true;
        n.position.z = -0.5;
        let texture = |name: &str, srgb: bool, repeat: bool| {
            let name = name.to_owned();
            async move {
                let mut t =
                    decode_texture_image(&fetch(&format!("{ASSETS}/random-uv/{name}")).await?)
                        .await?;
                t.srgb = srgb;
                t.mipmap_filter = Some(Filter::Linear);
                if repeat {
                    t.wrap_s = Wrapping::Repeat;
                    t.wrap_t = Wrapping::Repeat;
                }
                Ok::<_, crate::Error>(Arc::new(t))
            }
        };
        let jade = r.upload_texture(&texture("jade.jpg", true, true).await?)?;
        let noise = r.upload_texture(&texture("noise.png", false, false).await?)?;
        let disolve = r.upload_texture(&texture("shaderball_ds.jpg", false, false).await?)?;
        let mut env = crate::environment::EnvironmentMap::from_hdr(
            &fetch(&format!("{ASSETS}/spot-skinning/lobe.hdr")).await?,
        )?;
        env.prefilter(r)?;
        s.environment = Some(Arc::new(env));
        s.background_environment = true;
        s.background_blur = 0.5;
        s.background_intensity = 1.;
        s.environment_intensity = 1.5;
        let mut controls = Controls::new(None, (0.3, 10.), PI, true);
        controls.set_target(Vector3::new(0., 0.4, 0.));
        controls.update(s, c)?;
        let mut demo = Self {
            controls,
            shell: None,
            inner: None,
            ground,
            settings: [0., 0.2, 1., 1., 0., 0.],
            roughness: 0.,
            metalness: 0.,
            dirty: false,
        };
        let (a, b, i) = load_asset(&format!("{ASSETS}/random-uv/ShaderBall2.glb")).await?;
        let model = crate::gltf::import_animated_decoded(&a, &b, &i)?.instantiate(s)?;
        // The base, the inside and the logo; renderOrder counts down from the last child.
        let count = model.meshes.len();
        for (index, &mesh) in model.meshes.iter().enumerate() {
            let n = s.get_mut(mesh)?;
            n.cast_shadow = true;
            n.receive_shadow = true;
            n.render_order = (count - 1 - index) as i32;
        }
        let shell_program = shell_graph()?
            .build(
                r,
                &[],
                &[
                    (&jade.view, &jade.sampler),
                    (&noise.view, &noise.sampler),
                    (&disolve.view, &disolve.sampler),
                ],
            )
            .await?;
        let inner_program = SurfaceNodes {
            color: Some(vec4(
                base_color().rgb(),
                dissolve(Tex::External(0), &uniform(0, Type::Vec4))?,
            )),
            ..Default::default()
        }
        .build(r, &[], &[(&disolve.view, &disolve.sampler)])
        .await?;
        // Their shadows are the default depth: the patches leave the shadow pass alone.
        let shadow_program =
            Arc::new(crate::shadow::ShadowProgram::new(r, crate::shader::DEFAULT_HOOKS).await?);
        for (index, program) in [shell_program, inner_program].into_iter().enumerate() {
            let Some(&mesh) = model.meshes.get(index) else {
                continue;
            };
            if let NodeKind::Mesh(m) = &mut s.get_mut(mesh)?.kind {
                let mut material = (*m.materials[0]).clone();
                if let Material::Standard(standard) = &mut material
                    && index == 0
                {
                    demo.roughness = standard.roughness;
                    demo.metalness = standard.metalness;
                }
                let properties = material.properties_mut();
                properties.transparent = true;
                properties.vertex_program = Some(Arc::new(program));
                properties.shadow_program = Some(shadow_program.clone());
                m.materials[0] = Arc::new(material);
            }
            if index == 0 {
                demo.shell = Some(mesh);
            } else {
                demo.inner = Some(mesh);
            }
        }
        demo.apply(s)?;
        Ok(demo)
    }
    /// The uniforms and GUI material values, written once per change.
    fn apply(&self, s: &mut Scene) -> Result<()> {
        let v = self.settings;
        for (handle, shell) in [(self.shell, true), (self.inner, false)] {
            let Some(handle) = handle else { continue };
            if let NodeKind::Mesh(m) = &mut s.get_mut(handle)?.kind {
                let material = Arc::make_mut(&mut m.materials[0]);
                if shell && let Material::Standard(standard) = material {
                    standard.roughness = self.roughness;
                    standard.metalness = self.metalness;
                }
                let uniforms = &mut material.properties_mut().vertex_uniforms;
                uniforms[0] = [v[0], v[1], v[2], v[3]];
                uniforms[1] = [v[4], v[5], 0., 0.];
            }
        }
        if let NodeKind::Mesh(m) = &mut s.get_mut(self.ground)?.kind {
            Arc::make_mut(&mut m.materials[0]).properties_mut().opacity =
                (1. - f64::from(v[0])) * 0.5;
        }
        Ok(())
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, _dt: f64, _animate: bool) -> Result<()> {
        Ok(())
    }
    pub fn prepare(&mut self, s: &mut Scene, c: Object3D, _r: &Renderer) -> Result<()> {
        if std::mem::take(&mut self.dirty) {
            self.apply(s)?;
        }
        self.controls.update(s, c)
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
        Ok(())
    }
    /// roughness, metalness, disolve, threshold, Enabled, UseNoiseMap, SuslikMethod, DebugNoise.
    pub fn parameter(&mut self, index: usize, value: f32) -> Result<()> {
        match index {
            0 => self.roughness = f64::from(value),
            1 => self.metalness = f64::from(value),
            2..=7 => self.settings[index - 2] = value,
            _ => return Ok(()),
        }
        self.dirty = true;
        Ok(())
    }
    pub fn seek(&mut self, _t: f64) {}
}
