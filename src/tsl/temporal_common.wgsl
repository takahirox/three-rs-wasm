// Three.js r186 TRAANode.js / TAAUtils.js, MIT (LICENSE-THREE).
struct Parameters {
 previous_to_view:mat4x4<f32>, previous_projection_inverse:mat4x4<f32>,
 camera:vec4<f32>, settings:vec4<f32>, jitter:vec4<f32>,
}
@group(0) @binding(0) var<uniform> params:Parameters;
@group(0) @binding(1) var beauty:texture_2d<f32>;
@group(0) @binding(2) var depth:texture_depth_2d;
@group(0) @binding(3) var motion:texture_2d<f32>;
@group(0) @binding(4) var history:texture_2d<f32>;
@group(0) @binding(5) var history_depth:texture_depth_2d;
@group(0) @binding(6) var linear_sampler:sampler;
struct Varying { @builtin(position) position:vec4<f32>, @location(0) uv:vec2<f32> }
@vertex fn vs_main(@builtin(vertex_index) i:u32)->Varying {
 let p=array<vec2<f32>,3>(vec2(-1.0,-1.0),vec2(3.0,-1.0),vec2(-1.0,3.0));
 var out:Varying;out.position=vec4(p[i],0.0,1.0);out.uv=p[i]*vec2(0.5,-0.5)+0.5;return out;
}
fn clip_aabb(current:vec4<f32>,old:vec4<f32>,low:vec4<f32>,high:vec4<f32>)->vec4<f32>{
 let center=(high.rgb+low.rgb)*0.5;let extent=(high.rgb-low.rgb)*0.5+1e-7;
 let delta=old-vec4(center,current.a);let unit=abs(delta.rgb/extent);let magnitude=max(max(unit.x,unit.y),unit.z);
 return select(old,vec4(center,current.a)+delta/magnitude,magnitude>1.0);
}
fn flicker(current:vec4<f32>,old:vec4<f32>,weight:f32)->vec4<f32>{
 let a=current/(max(max(current.r,current.g),current.b)+1.0);let b=old/(max(max(old.r,old.g),old.b)+1.0);
 let wa=weight/(dot(a.rgb,vec3(0.2126,0.7152,0.0722))+1.0);let wb=(1.0-weight)/(dot(b.rgb,vec3(0.2126,0.7152,0.0722))+1.0);
 return (current*wa+old*wb)/max(wa+wb,0.00001);
}
