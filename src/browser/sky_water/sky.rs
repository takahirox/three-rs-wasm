//! SkyMesh: the Preetham sky with the r186 cloud layer, as a raw material whose
//! output goes through the renderer's tone mapping like a NodeMaterial.
use crate::{
    Result, geometry::BoxGeometry, material::*, math::*, renderer::Renderer, scene::*,
    shader::ShaderProgram, tsl::*,
};
use std::sync::Arc;

/// The varyings of SkyMesh's vertex node are uniform-only, so the fragment
/// evaluates them per pixel with the same f32 arithmetic.
const SKY: &str = r#"
fn sky_gradient(i:vec2<f32>)->vec2<f32>{var p=fract(i.xyx*vec3(0.1031,0.1030,0.0973));p+=dot(p,p.yzx+vec3(33.33));return fract((p.xx+p.yz)*p.zy)*2.0-1.0;}
fn sky_noise(p:vec2<f32>)->f32{let i=floor(p);let f=fract(p);let u=f*f*f*(f*(f*6.0-15.0)+10.0);let a=dot(sky_gradient(i),f);let b=dot(sky_gradient(i+vec2(1.0,0.0)),f-vec2(1.0,0.0));let c=dot(sky_gradient(i+vec2(0.0,1.0)),f-vec2(0.0,1.0));let d=dot(sky_gradient(i+vec2(1.0,1.0)),f-vec2(1.0,1.0));return mix(mix(a,b,u.x),mix(c,d,u.x),u.y)*1.6;}
fn sky_fbm(position:vec2<f32>,drift:f32)->f32{var p=position;var result=0.0;var amplitude=1.0;for(var k=0;k<4;k++){result+=amplitude*sky_noise(p);amplitude*=0.5;p=p*2.0+vec2(drift);}return result;}
fn sky_color(world:vec3<f32>,a:vec4<f32>,b:vec4<f32>,c:vec4<f32>,d:vec4<f32>)->vec4<f32>{
 let e=2.718281828459045;
 let totalRayleigh=vec3(5.804542996261093E-6,1.3562911419845635E-5,3.0265902468824876E-5);
 let MieConst=vec3(1.8399918514433978E14,2.7798023919660528E14,4.0790479543861094E14);
 let sunDirection=normalize(b.xyz);
 let zenithAngleCos=clamp(sunDirection.y,-1.0,1.0);
 let sunE=1000.0*max(0.0,1.0-pow(e,-((1.6110731556870734-acos(zenithAngleCos))/1.5)));
 let sunfade=1.0-clamp(1.0-exp(b.y/450000.0),0.0,1.0);
 let betaR=totalRayleigh*(a.y-(1.0-sunfade));
 let betaM=0.434*(0.2*a.x*10E-18)*MieConst*a.z;
 let pi=3.141592653589793;
 let direction=normalize(world-u.camera.xyz);
 let zenithAngle=acos(max(0.0,direction.y));
 let inverse=1.0/(cos(zenithAngle)+0.15*pow(93.885-zenithAngle*180.0/pi,-1.253));
 let Fex=exp(-(betaR*(8.4E3*inverse)+betaM*(1.25E3*inverse)));
 let cosTheta=dot(direction,sunDirection);
 let rPhase=0.05968310365946075*(1.0+pow(cosTheta*0.5+0.5,2.0));
 let betaRTheta=betaR*rPhase;
 let g2=pow(a.w,2.0);
 let mPhase=0.07957747154594767*(1.0-g2)*(1.0/pow(1.0-2.0*a.w*cosTheta+g2,1.5));
 let betaMTheta=betaM*mPhase;
 var Lin=pow(sunE*((betaRTheta+betaMTheta)/(betaR+betaM))*(1.0-Fex),vec3(1.5));
 Lin*=mix(vec3(1.0),pow(sunE*((betaRTheta+betaMTheta)/(betaR+betaM))*Fex,vec3(0.5)),clamp(pow(1.0-sunDirection.y,5.0),0.0,1.0));
 let L0=vec3(0.1)*Fex;
 let sundisc=clamp((cosTheta-0.9999566769464484)*50000.0,0.0,1.0)*b.w;
 let sundiscColor=min(sunE*Fex,vec3(80.0))*760.0*sundisc;
 var texColor=(Lin+L0)*0.04+sundiscColor+vec3(0.0,0.0003,0.00075);
 if direction.y>0.0 && c.z>0.0 {
  let elevation=mix(1.0,0.1,d.x);
  var cloudUV=direction.xz/(direction.y*elevation);
  cloudUV*=c.x;
  cloudUV+=vec2(d.y*c.y);
  let evolve=d.y*c.y*300.0;
  let cloudNoise=clamp(sky_fbm(cloudUV*1000.0,evolve)*0.7+0.5,0.0,1.0);
  let region=sky_noise(cloudUV*300.0)*0.37+0.5;
  let cov=clamp(c.z+(region-0.5)*0.6,0.0,1.0);
  let threshold=1.0-cov;
  var cloudMask=smoothstep(threshold,threshold+0.3,cloudNoise);
  let horizonFade=smoothstep(0.0,0.03+0.06*d.x,direction.y);
  cloudMask*=horizonFade;
  let dayFactor=smoothstep(-0.08,0.3,sunDirection.y);
  let sunColor=sunE*Fex*0.22*0.04;
  let skyAmbient=Lin*0.04+vec3(0.0,0.0003,0.00075);
  let depth=max(0.0,cloudNoise-threshold);
  let beer=exp(depth*-4.0);
  let powder=1.0-beer*beer;
  let shade=mix(0.45,1.0,clamp(beer*powder*2.6,0.0,1.0));
  let silver=clamp(0.51/pow(1.49-cosTheta*1.4,1.5),0.0,3.0);
  let edge=cloudMask*(1.0-cloudMask)*4.0;
  var cloudColor=skyAmbient+sunColor*shade;
  cloudColor+=sunColor*silver*edge*0.6;
  cloudColor*=max(dayFactor,0.03);
  let alpha=(1.0-exp(depth*c.w*-12.0))*horizonFade;
  texColor-=(L0*0.04+sundiscColor)*alpha;
  let cloudAerial=mix(texColor,cloudColor,Fex);
  texColor=mix(texColor,cloudAerial,alpha);
 }
 return vec4(texColor,1.0);
}"#;

/// SkyMesh's uniforms, with its defaults.
#[derive(Clone, Copy)]
pub(super) struct Sky {
    pub turbidity: f64,
    pub rayleigh: f64,
    pub mie_coefficient: f64,
    pub mie_directional_g: f64,
    pub sun: Vector3,
    pub cloud_scale: f64,
    pub cloud_speed: f64,
    pub cloud_coverage: f64,
    pub cloud_density: f64,
    pub cloud_elevation: f64,
    pub show_sun_disc: bool,
}
impl Default for Sky {
    fn default() -> Self {
        Self {
            turbidity: 2.,
            rayleigh: 1.,
            mie_coefficient: 0.005,
            mie_directional_g: 0.8,
            sun: Vector3::ZERO,
            cloud_scale: 0.0002,
            cloud_speed: 0.00002,
            cloud_coverage: 0.4,
            cloud_density: 0.4,
            cloud_elevation: 0.5,
            show_sun_disc: true,
        }
    }
}
impl Sky {
    /// The node uniforms, with TSL `time` in seconds.
    pub fn uniforms(&self, time: f64) -> [[f32; 4]; 4] {
        let f = |v: f64| v as f32;
        [
            [
                f(self.turbidity),
                f(self.rayleigh),
                f(self.mie_coefficient),
                f(self.mie_directional_g),
            ],
            [
                f(self.sun.x),
                f(self.sun.y),
                f(self.sun.z),
                f(f64::from(u8::from(self.show_sun_disc))),
            ],
            [
                f(self.cloud_scale),
                f(self.cloud_speed),
                f(self.cloud_coverage),
                f(self.cloud_density),
            ],
            [f(self.cloud_elevation), f(time), 0., 0.],
        ]
    }
    /// Write the uniforms into a sky mesh's material.
    pub fn apply(&self, s: &mut Scene, sky: Object3D, time: f64) -> Result<()> {
        if let NodeKind::Mesh(m) = &mut s.get_mut(sky)?.kind
            && let Material::Shader(m) = Arc::make_mut(&mut m.materials[0])
        {
            m.uniforms[..4].copy_from_slice(&self.uniforms(time));
        }
        Ok(())
    }
}
/// The sky program: z pinned to the far plane, back faces, no depth writes.
pub(super) async fn sky_program(r: &Renderer) -> Result<Arc<ShaderProgram>> {
    let color = WgslFn::new(
        "sky_color",
        SKY,
        &[Type::Vec3, Type::Vec4, Type::Vec4, Type::Vec4, Type::Vec4],
        Type::Vec4,
    )?
    .call(&[
        position_world(),
        uniform(0, Type::Vec4),
        uniform(1, Type::Vec4),
        uniform(2, Type::Vec4),
        uniform(3, Type::Vec4),
    ]);
    Ok(Arc::new(
        ShaderProgram::with_projection(
            r,
            &NodeMaterial::new(color).wgsl(0)?,
            &[],
            &[],
            "fn project_vertex(surface:VertexOut,position:vec3<f32>)->VertexOut{var out=surface;out.clip=vec4(out.clip.xy,out.clip.w,out.clip.w);return out;}",
        )
        .await?,
    ))
}
/// A SkyMesh node: the unit box scaled by `scale`.
pub(super) fn sky_mesh(
    s: &mut Scene,
    program: &Arc<ShaderProgram>,
    scale: f64,
) -> Result<Object3D> {
    let mut m = ShaderMaterial::new(program.clone());
    m.properties.side = Side::Back;
    m.properties.depth_write = false;
    m.properties.fog = false;
    let h = s.insert(NodeKind::Mesh(Mesh::new(
        Arc::new(BoxGeometry::build(1., 1., 1.)?),
        Arc::new(Material::Shader(m)),
    )));
    // Frustum culling stays on, as for SkyMesh: an oblique reflector projection culls it.
    s.get_mut(h)?.scale = Vector3::splat(scale);
    Ok(h)
}
