// Three.js r187 environment sampling (MIT, LICENSE-THREE).
// envmap_common_pars_fragment's roughnessToMip: the inverse of
// PMREMGenerator.lodToRoughness ( Filament's perceptualRoughnessToLod ).
fn roughness_to_mip(roughness:f32,max_lod:f32)->f32{let r=clamp(roughness,0.0,1.0);return max_lod*r*(2.0-r);}
// The sharp background's cube of an equirectangular map: its faces and texel directions.
fn cube_face(d:vec3<f32>)->u32 {
 let a=abs(d);if a.x>a.z {if a.x>a.y {return select(3u,0u,d.x>0.0);}return select(4u,1u,d.y>0.0);}
 if a.z>a.y{return select(5u,2u,d.z>0.0);}return select(4u,1u,d.y>0.0);
}
fn face_uv(d:vec3<f32>,face:u32)->vec2<f32>{
 var v:vec2<f32>;switch face {case 0u:{v=d.zy/abs(d.x);}case 1u:{v=-d.xz/abs(d.y);}case 2u:{v=vec2(-d.x,d.y)/abs(d.z);}case 3u:{v=vec2(-d.z,d.y)/abs(d.x);}case 4u:{v=vec2(-d.x,d.z)/abs(d.y);}default:{v=d.xy/abs(d.z);}}return v*0.5+0.5;
}
fn face_direction(uv:vec2<f32>,face:u32)->vec3<f32>{
 let v=uv*2.0-1.0;switch face {case 0u:{return normalize(vec3(1.0,v.y,v.x));}case 1u:{return normalize(vec3(-v.x,1.0,-v.y));}case 2u:{return normalize(vec3(-v.x,v.y,1.0));}case 3u:{return normalize(vec3(-1.0,v.y,-v.x));}case 4u:{return normalize(vec3(-v.x,-1.0,v.y));}default:{return normalize(vec3(v.x,v.y,-1.0));}}
}
fn equirect_uv(d:vec3<f32>)->vec2<f32>{return vec2(atan2(d.z,d.x)*0.15915494309+0.5,0.5-asin(clamp(d.y,-1.0,1.0))*0.31830988618);}
