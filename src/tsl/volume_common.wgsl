fn tsl_volume_bounds(origin:vec3<f32>,direction:vec3<f32>)->vec2<f32>{
 let a=(-vec3(0.5)-origin)/direction;let b=(vec3(0.5)-origin)/direction;
 let low=min(a,b);let high=max(a,b);return vec2(max(low.x,max(low.y,low.z)),min(high.x,min(high.y,high.z)));
}
fn tsl_volume_normal(map:texture_3d<f32>,s:sampler,p:vec3<f32>)->vec3<f32>{
 if p.x<0.0001{return vec3(1.0,0.0,0.0);}if p.y<0.0001{return vec3(0.0,1.0,0.0);}if p.z<0.0001{return vec3(0.0,0.0,1.0);}
 if p.x>0.9999{return vec3(-1.0,0.0,0.0);}if p.y>0.9999{return vec3(0.0,-1.0,0.0);}if p.z>0.9999{return vec3(0.0,0.0,-1.0);}
 let x=textureSampleLevel(map,s,p-vec3(0.01,0.0,0.0),0.0).r-textureSampleLevel(map,s,p+vec3(0.01,0.0,0.0),0.0).r;
 let y=textureSampleLevel(map,s,p-vec3(0.0,0.01,0.0),0.0).r-textureSampleLevel(map,s,p+vec3(0.0,0.01,0.0),0.0).r;
 let z=textureSampleLevel(map,s,p-vec3(0.0,0.0,0.01),0.0).r-textureSampleLevel(map,s,p+vec3(0.0,0.0,0.01),0.0).r;
 return normalize(vec3(x,y,z));
}
