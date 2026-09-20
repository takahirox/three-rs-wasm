// MaterialX scalar 3D Perlin noise, ported from Three.js r186 (MIT).
fn tsl_mx_rot(x:u32,k:u32)->u32 {return (x<<k)|(x>>(32u-k));}
fn tsl_mx_hash(p:vec3<i32>)->u32 {
 let seed=0xdeadbeefu+12u+13u;
 var a=seed+u32(p.x);var b=seed+u32(p.y);var c=seed+u32(p.z);
 c=(c^b)-tsl_mx_rot(b,14u);a=(a^c)-tsl_mx_rot(c,11u);
 b=(b^a)-tsl_mx_rot(a,25u);c=(c^b)-tsl_mx_rot(b,16u);
 a=(a^c)-tsl_mx_rot(c,4u);b=(b^a)-tsl_mx_rot(a,14u);
 return (c^b)-tsl_mx_rot(b,24u);
}
fn tsl_mx_gradient(p:vec3<i32>,f:vec3<f32>)->f32 {
 let h=tsl_mx_hash(p)&15u;
 let a=select(f.y,f.x,h<8u);let b=select(select(f.z,f.x,h==12u||h==14u),f.y,h<4u);
 return select(a,-a,(h&1u)!=0u)+select(b,-b,(h&2u)!=0u);
}
fn tsl_mx_noise3(p:vec3<f32>)->f32 {
 let i=vec3<i32>(floor(p));let f=p-vec3<f32>(i);let q=f*f*f*(f*(f*6.0-15.0)+10.0);
 let a=tsl_mx_gradient(i,f);let b=tsl_mx_gradient(i+vec3(1,0,0),f-vec3(1.0,0.0,0.0));
 let c=tsl_mx_gradient(i+vec3(0,1,0),f-vec3(0.0,1.0,0.0));let d=tsl_mx_gradient(i+vec3(1,1,0),f-vec3(1.0,1.0,0.0));
 let e=tsl_mx_gradient(i+vec3(0,0,1),f-vec3(0.0,0.0,1.0));let g=tsl_mx_gradient(i+vec3(1,0,1),f-vec3(1.0,0.0,1.0));
 let h=tsl_mx_gradient(i+vec3(0,1,1),f-vec3(0.0,1.0,1.0));let j=tsl_mx_gradient(i+vec3(1,1,1),f-vec3(1.0,1.0,1.0));
 return 0.982*((1.0-q.z)*((1.0-q.y)*(a*(1.0-q.x)+b*q.x)+q.y*(c*(1.0-q.x)+d*q.x))+q.z*((1.0-q.y)*(e*(1.0-q.x)+g*q.x)+q.y*(h*(1.0-q.x)+j*q.x)));
}
