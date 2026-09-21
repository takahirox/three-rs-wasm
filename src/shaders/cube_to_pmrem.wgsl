struct Params {size:u32,pad:vec3<u32>};
@group(0) @binding(0) var source:texture_cube<f32>;
@group(0) @binding(1) var source_sampler:sampler;
@group(0) @binding(2) var destination:texture_storage_2d<rgba16float,write>;
@group(0) @binding(3) var<uniform> p:Params;
@compute @workgroup_size(8,8)
fn main(@builtin(global_invocation_id) id:vec3<u32>){
 if id.x>=3u*p.size||id.y>=2u*p.size{return;}
 let face=id.x/p.size+3u*(id.y/p.size);let uv=(vec2<f32>(id.xy%vec2(p.size))+0.5-1.0)/(f32(p.size)-2.0);
 let d=face_direction(uv,face);
 textureStore(destination,vec2<i32>(id.xy),textureSampleLevel(source,source_sampler,vec3(-d.x,-d.y,d.z),0.0));
}
