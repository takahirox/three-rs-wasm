struct Uniforms {
    mvp: mat4x4<f32>, model: mat4x4<f32>, normal: mat4x4<f32>,
    color: vec4<f32>, camera: vec4<f32>, material: vec4<f32>, emissive: vec4<f32>, ambient: vec4<f32>, point:vec4<f32>,
    light_position: array<vec4<f32>,8>, light_color: array<vec4<f32>,8>, light_params: array<vec4<f32>,8>,
};
@group(0) @binding(0) var<uniform> u: Uniforms;
@group(0) @binding(1) var color_map: texture_2d<f32>;
@group(0) @binding(2) var color_sampler: sampler;
struct VertexOut {
    @builtin(position) clip: vec4<f32>, @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>, @location(2) uv: vec2<f32>, @location(3) color: vec4<f32>,
};
@vertex fn vs_main(@location(0) position:vec3<f32>, @location(1) normal:vec3<f32>, @location(2) uv:vec2<f32>, @location(3) color:vec4<f32>, @location(4) corner:vec2<f32>)->VertexOut {
    var out:VertexOut; out.clip=u.mvp*vec4(position,1.0);out.position=(u.model*vec4(position,1.0)).xyz;
    if u.point.z>0.0 {
        var size=u.point.z;if u.point.w>0.0 {size*=u.point.y*0.5/out.clip.w;}
        out.clip=vec4(out.clip.xy+corner*size/u.point.xy*out.clip.w,out.clip.zw);
    }
    out.normal=(u.normal*vec4(normal,0.0)).xyz;out.uv=uv;out.color=color;return out;
}
@fragment fn fs_main(in:VertexOut,@builtin(front_facing) front:bool)->@location(0) vec4<f32> {
    let base=u.color*in.color*textureSample(color_map,color_sampler,in.uv);
    if u.material.x<0.5 {return base;}
    let n=normalize(in.normal)*select(-1.0,1.0,front);let v=normalize(u.camera.xyz-in.position);
    let roughness=max(u.material.y,0.0525);let metalness=clamp(u.material.z,0.0,1.0);
    let diffuse=base.rgb*(1.0-metalness);let f0=mix(vec3(0.04),base.rgb,vec3(metalness));
    var result=diffuse*u.ambient.xyz/3.14159265359+u.emissive.xyz;
    for(var i=0u;i<u32(u.material.w);i++) {
        var light=u.light_position[i].xyz;var attenuation=1.0;
        if u.light_position[i].w>0.5 {
            let delta=light-in.position;let distance=length(delta);light=delta/max(distance,0.00001);
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
