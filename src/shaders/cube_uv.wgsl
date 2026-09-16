// Three.js r186 PMREM cube-UV layout and roughness mapping (MIT, LICENSE-THREE).
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
fn cube_uv(d:vec3<f32>,level:f32,max_mip:f32)->vec2<f32>{
 var face=cube_face(d);let extra=max(4.0-level,0.0);let size=exp2(max(level,4.0));
 var uv=face_uv(d,face)*(size-2.0)+1.0;if face>2u {uv.y+=size;face-=3u;}
 uv.x+=f32(face)*size+extra*48.0;uv.y+=4.0*(exp2(max_mip)-size);
 return uv/vec2(3.0*max(exp2(max_mip),112.0),4.0*exp2(max_mip));
}
fn roughness_mip(r:f32)->f32{
 if r>=0.8{return (1.0-r)*5.0-2.0;}if r>=0.4{return (0.8-r)*7.5-1.0;}
 if r>=0.305{return (0.4-r)/0.095+2.0;}if r>=0.21{return (0.305-r)/0.095+3.0;}return -2.0*log2(1.16*max(r,0.00001));
}
fn equirect_uv(d:vec3<f32>)->vec2<f32>{return vec2(atan2(d.z,d.x)*0.15915494309+0.5,0.5-asin(clamp(d.y,-1.0,1.0))*0.31830988618);}
