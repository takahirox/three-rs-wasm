@group(0) @binding(0) var<uniform> params:vec4<f32>;
@group(0) @binding(1) var cloud:texture_storage_3d<rgba8unorm,write>;
@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) global:vec3<u32>,@builtin(num_workgroups) groups:vec3<u32>){
 let id=global.x+global.y*(64u*groups.x);if id>=8000000u{return;}
 let coord=vec3(id%200u,(id/200u)%200u,id/40000u);let p=vec3<f32>(coord);let d=1.0-length((p-100.0)/200.0);
 let value=tsl_mx_noise3(p*(0.05/1.5)+params.x)*d*d;
 textureStore(cloud,vec3<i32>(coord),vec4(vec3(value),1.0));
}
