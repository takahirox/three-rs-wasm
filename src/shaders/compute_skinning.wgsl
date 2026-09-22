struct Vertex {position:vec4<f32>,joints:vec4<f32>,weights:vec4<f32>}
@group(0) @binding(0) var<storage,read> vertices:array<Vertex>;
@group(0) @binding(1) var<storage,read> palette:array<mat4x4<f32>>;
@group(0) @binding(2) var<storage,read_write> positions:array<vec4<f32>>;
@group(0) @binding(3) var<storage,read_write> displacement:array<vec4<f32>>;
@compute @workgroup_size(64) fn main(@builtin(global_invocation_id) id:vec3<u32>) {
 let i=id.x;if i>=arrayLength(&vertices){return;}
 let v=vertices[i];let j=vec4<u32>(v.joints);
 let matrix=palette[j.x]*v.weights.x+palette[j.y]*v.weights.y+palette[j.z]*v.weights.z+palette[j.w]*v.weights.w;
 let position=matrix*v.position;
 displacement[i]=vec4(position.xyz-positions[i].xyz,0.0);
 positions[i]=vec4(position.xyz,1.0);
}
