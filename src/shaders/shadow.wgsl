struct Uniform {matrix:mat4x4<f32>,alpha:vec4<f32>,model:mat4x4<f32>,planes:array<vec4<f32>,16>,params:vec4<f32>,uv_transform:array<vec4<f32>,3>};
@group(0) @binding(0) var<uniform> u:Uniform;
@group(0) @binding(1) var color_map:texture_2d<f32>;
@group(0) @binding(2) var color_sampler:sampler;
struct Out {@builtin(position) position:vec4<f32>,@location(0) uv:vec2<f32>,@location(1) world:vec3<f32>};
@vertex fn vs_main(@builtin(vertex_index) vertex:u32,@location(0) position:vec3<f32>,@location(1) uv:vec2<f32>,@location(2) uv1:vec2<f32>,@location(6) i0:vec4<f32>,@location(7) i1:vec4<f32>,@location(8) i2:vec4<f32>,@location(9) i3:vec4<f32>)->Out {
    let animated=skin_morph(vertex,position,vec3(0.0),vec4(1.0),vec4(0.0));
    let p=mat4x4(i0,i1,i2,i3)*vec4(animated.position,1.0);
    let t=u.uv_transform;let raw=select(uv,uv1,t[2].w>0.5);
    var out:Out;out.position=u.matrix*p;out.uv=(mat3x3(t[0].xyz,t[1].xyz,t[2].xyz)*vec3(raw,1.0)).xy;out.world=(u.model*p).xyz;return out;
}
@fragment fn fs_main(in:Out) {
    let alpha=textureSample(color_map,color_sampler,in.uv).a*u.alpha.x;
    let global=u32(u.params.x);let local=u32(u.params.y);
    for(var i=0u;i<global;i++){if dot(u.planes[i],vec4(in.world,1.0))<0.0 {discard;}}
    var all_outside=local>0u;
    for(var i=global;i<global+local;i++){let outside=dot(u.planes[i],vec4(in.world,1.0))<0.0;if outside && u.params.z==0.0 {discard;}all_outside=all_outside&&outside;}
    if all_outside && u.params.z>0.0 {discard;}
if alpha<u.alpha.y {discard;}}
