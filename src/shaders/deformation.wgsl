@group(0) @binding(21) var<storage,read> deformation_data:array<f32>;
@group(0) @binding(22) var<storage,read> deformation_pose:array<f32>;
struct DeformationInfo {counts:vec4<u32>,packing:vec4<u32>};
@group(0) @binding(23) var<uniform> deformation_info:DeformationInfo;
struct Deformed {position:vec3<f32>,normal:vec3<f32>,color:vec4<f32>,tangent:vec4<f32>};
fn morph_vec(i:u32)->vec4<f32>{return vec4(deformation_data[i],deformation_data[i+1u],deformation_data[i+2u],deformation_data[i+3u]);}
fn bone_matrix(index:u32)->mat4x4<f32>{
 let i=index*16u;
 return mat4x4(vec4(deformation_pose[i],deformation_pose[i+1u],deformation_pose[i+2u],deformation_pose[i+3u]),vec4(deformation_pose[i+4u],deformation_pose[i+5u],deformation_pose[i+6u],deformation_pose[i+7u]),vec4(deformation_pose[i+8u],deformation_pose[i+9u],deformation_pose[i+10u],deformation_pose[i+11u]),vec4(deformation_pose[i+12u],deformation_pose[i+13u],deformation_pose[i+14u],deformation_pose[i+15u]));
}
fn skin_morph(vertex:u32,position:vec3<f32>,normal:vec3<f32>,color:vec4<f32>,tangent:vec4<f32>)->Deformed {
 var out=Deformed(position,normal,color,tangent);let id=vertex/deformation_info.counts.w;
 for(var morph_id=0u;morph_id<deformation_info.counts.z;morph_id++){
  let weight=deformation_pose[deformation_info.counts.y*16u+morph_id];
  if weight==0.0 {continue;}
  var base=deformation_info.packing.x+(morph_id*deformation_info.counts.x+id)*deformation_info.packing.y;
  let mask=deformation_info.packing.z;
  if (mask&1u)!=0u {out.position+=morph_vec(base).xyz*weight;base+=4u;}
  if (mask&2u)!=0u {out.normal+=morph_vec(base).xyz*weight;base+=4u;}
  if (mask&4u)!=0u {out.color+=morph_vec(base)*weight;base+=4u;}
  if (mask&8u)!=0u {out.tangent=vec4(out.tangent.xyz+morph_vec(base).xyz*weight,out.tangent.w);}
 }
 if deformation_info.counts.y>0u {
  var skin=mat4x4(vec4(0.0),vec4(0.0),vec4(0.0),vec4(0.0));
  for(var c=0u;c<4u;c++){let weight=deformation_data[id*8u+4u+c];if weight>0.0{skin+=bone_matrix(u32(deformation_data[id*8u+c]))*weight;}}
  out.position=(skin*vec4(out.position,1.0)).xyz;out.normal=(skin*vec4(out.normal,0.0)).xyz;out.tangent=vec4((skin*vec4(out.tangent.xyz,0.0)).xyz,out.tangent.w);
 }
 return out;
}
