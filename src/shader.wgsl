// Specialize away discard for opaque draws: helper-lane derivatives must remain intact.
override ALPHA_MASK: bool = false;
struct Uniforms {
    mvp: mat4x4<f32>, model: mat4x4<f32>, normal: mat4x4<f32>, view:mat4x4<f32>, projection:mat4x4<f32>,
    color: vec4<f32>, camera: vec4<f32>, material: vec4<f32>, emissive: vec4<f32>, ambient: vec4<f32>, point:vec4<f32>, pbr:vec4<f32>, environment:vec4<f32>, maps:vec4<f32>,
    light_position: array<vec4<f32>,8>, light_color: array<vec4<f32>,8>, light_params: array<vec4<f32>,8>,
};
@group(0) @binding(0) var<uniform> u: Uniforms;
@group(0) @binding(1) var color_map: texture_2d<f32>;
@group(0) @binding(2) var color_sampler: sampler;
@group(0) @binding(3) var mr_map: texture_2d<f32>;
@group(0) @binding(4) var mr_sampler: sampler;
@group(0) @binding(5) var normal_map: texture_2d<f32>;
@group(0) @binding(6) var normal_sampler: sampler;
@group(0) @binding(7) var ao_map: texture_2d<f32>;
@group(0) @binding(8) var ao_sampler: sampler;
@group(0) @binding(9) var emissive_map: texture_2d<f32>;
@group(0) @binding(10) var emissive_sampler: sampler;
@group(0) @binding(11) var environment_map: texture_2d<f32>;
@group(0) @binding(12) var environment_source: texture_2d<f32>;
@group(0) @binding(13) var environment_sampler: sampler;
@group(0) @binding(14) var dfg_map: texture_2d<f32>;
fn environment_sample(direction:vec3<f32>,roughness:f32)->vec3<f32> {
 let world=normalize((transpose(u.view)*vec4(direction,0.0)).xyz);
 let c=cos(u.environment.y);let s=sin(u.environment.y);
 let d=vec3(c*world.x-s*world.z,-world.y,s*world.x+c*world.z);
 let mip=clamp(roughness_mip(roughness),-2.0,u.environment.z);let lo=floor(mip);
 return mix(textureSampleLevel(environment_map,environment_sampler,cube_uv(d,lo,u.environment.z),0.0).rgb,
 textureSampleLevel(environment_map,environment_sampler,cube_uv(d,lo+1.0,u.environment.z),0.0).rgb,fract(mip))*u.environment.x;
}
fn multiscattering(f0:vec3<f32>,dfg:vec2<f32>)->vec3<f32>{
 let single=f0*dfg.x+dfg.y;let average=f0+(1.0-f0)*0.047619;let missing=1.0-dfg.x-dfg.y;
 return single*average/(1.0-missing*average)*missing;
}
struct VertexOut {
    @builtin(position) clip: vec4<f32>, @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>, @location(2) uv: vec2<f32>, @location(3) color: vec4<f32>, @location(4) tangent:vec4<f32>, @location(5) bitangent:vec3<f32>, @location(6) view_position:vec3<f32>,
};
@vertex fn vs_main(@location(0) position:vec3<f32>, @location(1) normal:vec3<f32>, @location(2) uv:vec2<f32>, @location(3) color:vec4<f32>, @location(4) corner:vec2<f32>, @location(5) tangent:vec4<f32>)->VertexOut {
    var out:VertexOut;let model_view=u.view*u.model;out.view_position=(model_view*vec4(position,1.0)).xyz;out.clip=u.projection*vec4(out.view_position,1.0);out.position=(u.model*vec4(position,1.0)).xyz;
    if u.point.z>0.0 {
        var size=u.point.z;if u.point.w>0.0 {size*=u.point.y*0.5/out.clip.w;}
        out.clip=vec4(out.clip.xy+corner*size/u.point.xy*out.clip.w,out.clip.zw);
    }
    out.normal=normalize((u.view*vec4((u.normal*vec4(normal,0.0)).xyz,0.0)).xyz);out.uv=uv;out.color=color;out.tangent=vec4((model_view*vec4(tangent.xyz,0.0)).xyz,tangent.w);out.bitangent=cross(normalize(out.normal),normalize(out.tangent.xyz))*tangent.w;return out;
}
@fragment fn fs_main(in:VertexOut,@builtin(front_facing) front:bool)->@location(0) vec4<f32> {
    let q0=dpdx(in.view_position);let q1=-dpdy(in.view_position);let st0=dpdx(in.uv);let st1=-dpdy(in.uv);
    let geometry_normal=normalize(in.normal);
    let view_normal=geometry_normal;
    let derivative=max(abs(dpdx(view_normal)),abs(dpdy(view_normal)));
    let normal_sample=textureSample(normal_map,normal_sampler,in.uv).xyz*2.0-1.0;
    var base=u.color*in.color*textureSample(color_map,color_sampler,in.uv);
    if ALPHA_MASK && base.a<u.pbr.w {discard;}
    if u.maps.y<0.5 {base.a=1.0;}
    if u.material.x<0.5 {return base;}
    let face=select(-1.0,1.0,front);
    var n=geometry_normal*face;
    let v=normalize(-in.view_position);
    if u.maps.x>0.5 {
        var t:vec3<f32>;var b:vec3<f32>;
        if abs(in.tangent.w)>0.5 {t=normalize(in.tangent.xyz);b=normalize(in.bitangent);}
        else {
            let q1perp=cross(q1,geometry_normal);let q0perp=cross(geometry_normal,q0);
            t=q1perp*st0.x+q0perp*st1.x;b=q1perp*st0.y+q0perp*st1.y;
            let scale=inverseSqrt(max(max(dot(t,t),dot(b,b)),1e-20));t*=scale;b*=scale;
        }
        let sample=normal_sample;
        n=normalize(t*sample.x*u.pbr.x*face+b*sample.y*u.pbr.y*face+n*sample.z);
    }
    let mr=textureSample(mr_map,mr_sampler,in.uv);

    let geometry_roughness=max(derivative.x,max(derivative.y,derivative.z));
    let roughness=min(max(u.material.y*mr.g,0.0525)+geometry_roughness,1.0);
    let metalness=clamp(u.material.z*mr.b,0.0,1.0);
    let diffuse=base.rgb*(1.0-metalness);let f0=mix(vec3(0.04),base.rgb,vec3(metalness));
    var result=diffuse*u.ambient.xyz/3.14159265359+u.emissive.xyz*textureSample(emissive_map,emissive_sampler,in.uv).rgb;
    if u.environment.x>0.0 {
        let nv=clamp(dot(n,v),0.0,1.0);
        let dfg=textureSampleLevel(dfg_map,environment_sampler,vec2(roughness,nv),0.0).rg;
        let sd=vec3(0.04)*dfg.x+dfg.y;let sm=base.rgb*dfg.x+dfg.y;
        let md=multiscattering(vec3(0.04),dfg);let mm=multiscattering(base.rgb,dfg);
        let radiance=environment_sample(normalize(mix(reflect(-v,n),n,pow(roughness,4.0))),roughness);
        let irradiance=environment_sample(n,1.0);
        let ao=(textureSample(ao_map,ao_sampler,in.uv).r-1.0)*u.pbr.z+1.0;
        let specular_ao=clamp(pow(nv+ao,exp2(-16.0*roughness-1.0))-1.0+ao,0.0,1.0);
        result+=diffuse*(1.0-sd-md)*irradiance*ao+(radiance*mix(sd,sm,metalness)+irradiance*mix(md,mm,metalness))*specular_ao;
    }
    for(var i=0u;i<u32(u.material.w);i++) {
        var light=(u.view*vec4(u.light_position[i].xyz,0.0)).xyz;var attenuation=1.0;
        if u.light_position[i].w>0.5 {
            let delta=(u.view*vec4(u.light_position[i].xyz,1.0)).xyz-in.view_position;let distance=length(delta);light=delta/max(distance,0.00001);
            attenuation=1.0/max(pow(distance,u.light_params[i].y),0.01);
            let cutoff=u.light_params[i].x;
            if cutoff>0.0 {let falloff=clamp(1.0-pow(distance/cutoff,4.0),0.0,1.0);attenuation*=falloff*falloff;}
        }
        let h=normalize(light+v);let nl=clamp(dot(n,light),0.0,1.0);let nv=clamp(dot(n,v),0.0,1.0);
        let nh=clamp(dot(n,h),0.0,1.0);let vh=clamp(dot(v,h),0.0,1.0);
        let alpha=roughness*roughness;let a2=alpha*alpha;let denom=nh*nh*(a2-1.0)+1.0;
        let distribution=a2/(3.14159265359*denom*denom);
        let visibility=0.5/max(nl*sqrt(nv*nv*(1.0-a2)+a2)+nv*sqrt(nl*nl*(1.0-a2)+a2),0.000001);
        let fresnel=f0+(vec3(1.0)-f0)*exp2((-5.55473*vh-6.98316)*vh);
        result+=(diffuse/3.14159265359+fresnel*distribution*visibility)*u.light_color[i].xyz*attenuation*nl;
    }
    return vec4(result,base.a);
}
