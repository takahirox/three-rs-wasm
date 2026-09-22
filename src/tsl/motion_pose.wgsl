fn FUNCTION(ignored:f32)->vec3<f32>{
 let id=tsl_vertex_index/deformation_info.counts.w;
 var p=tsl_motion_original_position;
 if (deformation_info.packing.z&1u)!=0u {
  for(var i=0u;i<deformation_info.counts.z;i++){
   let weight=POSE[deformation_info.counts.y*16u+i];
   let base=deformation_info.packing.x+(i*deformation_info.counts.x+id)*deformation_info.packing.y;
   p+=morph_vec(base).xyz*weight;
  }
 }
 if deformation_info.counts.y>0u {
  var skin=mat4x4(vec4(0.0),vec4(0.0),vec4(0.0),vec4(0.0));
  for(var c=0u;c<4u;c++){
   let weight=deformation_data[id*8u+4u+c];
   if weight>0.0 {
    let i=u32(deformation_data[id*8u+c])*16u;
    let bone=mat4x4(vec4(POSE[i],POSE[i+1u],POSE[i+2u],POSE[i+3u]),vec4(POSE[i+4u],POSE[i+5u],POSE[i+6u],POSE[i+7u]),vec4(POSE[i+8u],POSE[i+9u],POSE[i+10u],POSE[i+11u]),vec4(POSE[i+12u],POSE[i+13u],POSE[i+14u],POSE[i+15u]));
    skin+=bone*weight;
   }
  }
  p=(skin*vec4(p,1.0)).xyz;
 }
 return p;
}
