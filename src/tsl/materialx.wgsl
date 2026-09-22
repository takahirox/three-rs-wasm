// MaterialX noise adapted from Three.js r186 MaterialXNoise.js.
// Modified: native WGSL implementation with shared 2D/3D kernels.
// MaterialX is Apache-2.0; see LICENSE-MATERIALX. Three.js is MIT.
fn mxn_rot(x:u32,k:u32)->u32 {return (x<<k)|(x>>(32u-k));}
fn mxn_final(v:vec3<u32>)->u32 {
 var a=v.x;var b=v.y;var c=v.z;
 c=(c^b)-mxn_rot(b,14u);a=(a^c)-mxn_rot(c,11u);
 b=(b^a)-mxn_rot(a,25u);c=(c^b)-mxn_rot(b,16u);
 a=(a^c)-mxn_rot(c,4u);b=(b^a)-mxn_rot(a,14u);
 return (c^b)-mxn_rot(b,24u);
}
fn mxn_mix(v:vec3<u32>)->vec3<u32> {
 var a=v.x;var b=v.y;var c=v.z;
 a=(a-c)^mxn_rot(c,4u);c+=b;b=(b-a)^mxn_rot(a,6u);a+=c;
 c=(c-b)^mxn_rot(b,8u);b+=a;a=(a-c)^mxn_rot(c,16u);c+=b;
 b=(b-a)^mxn_rot(a,19u);a+=c;c=(c-b)^mxn_rot(b,4u);b+=a;
 return vec3(a,b,c);
}
fn mxn_hash(p:vec3<i32>,dim:u32)->u32 {
 let seed=0xdeadbeefu+dim*4u+13u;
 return mxn_final(vec3(seed)+vec3<u32>(p));
}
fn mxn_cell(p:vec3<f32>,dim:u32)->f32 {
 return f32(mxn_hash(vec3<i32>(floor(p)),dim))/4294967295.0;
}
fn mxn_cell_vector(p:vec3<i32>,dim:u32)->vec3<f32> {
 if dim==2u {
  return vec3(f32(mxn_hash(vec3(p.xy,0),3u)),f32(mxn_hash(vec3(p.xy,1),3u)),f32(mxn_hash(vec3(p.xy,2),3u)))/4294967295.0;
 }
 let seed=0xdeadbeefu+16u+13u;
 let v=mxn_mix(vec3(seed)+vec3<u32>(p));
 return vec3(f32(mxn_final(v)),f32(mxn_final(v+vec3(1u,0u,0u))),f32(mxn_final(v+vec3(2u,0u,0u))))/4294967295.0;
}
fn mxn_gradient(hash:u32,f:vec3<f32>,dim:u32)->f32 {
 var h=hash&15u;var a=select(f.y,f.x,h<8u);
 var b=select(select(f.z,f.x,h==12u||h==14u),f.y,h<4u);
 if dim==2u { h=hash&7u;a=select(f.y,f.x,h<4u);b=2.0*select(f.x,f.y,h<4u); }
 return select(a,-a,(h&1u)!=0u)+select(b,-b,(h&2u)!=0u);
}
fn mxn_perlin(p:vec3<f32>,dim:u32,vector:bool)->vec3<f32> {
 let cell=vec3<i32>(floor(p));let f=p-vec3<f32>(cell);
 let q=f*f*f*(f*(f*6.0-15.0)+10.0);
 var corners:array<vec3<f32>,8>;
 for(var z=0;z<i32(dim)-1;z++) {for(var y=0;y<2;y++){for(var x=0;x<2;x++){
  let off=vec3(x,y,z);let h=mxn_hash(cell+off,dim);let delta=f-vec3<f32>(off);
  var v=vec3(mxn_gradient(h,delta,dim));
  if vector {v=vec3(mxn_gradient(h&255u,delta,dim),mxn_gradient((h>>8u)&255u,delta,dim),mxn_gradient((h>>16u)&255u,delta,dim));}
  corners[x+2*y+4*z]=v;
 }}}
 let xy=(1.0-q.y)*(corners[0]*(1.0-q.x)+corners[1]*q.x)+q.y*(corners[2]*(1.0-q.x)+corners[3]*q.x);
 if dim==2u {return xy*0.6616;}
 let zw=(1.0-q.y)*(corners[4]*(1.0-q.x)+corners[5]*q.x)+q.y*(corners[6]*(1.0-q.x)+corners[7]*q.x);
 return ((1.0-q.z)*xy+q.z*zw)*0.982;
}
fn mxn_worley(p:vec3<f32>,dim:u32,jitter:f32,style:u32,metric:u32)->vec3<f32> {
 let cell=vec3<i32>(floor(p));let local=p-vec3<f32>(cell);
 var distances=vec3(1e6);var nearest=vec3(0.0);
 let zmax=select(0,1,dim==3u);
 for(var x=-1;x<=1;x++){for(var y=-1;y<=1;y++){for(var z=-zmax;z<=zmax;z++){
  let n=vec3(x,y,z);var off=(mxn_cell_vector(cell+n,dim)-0.5)*jitter+0.5;
  if dim==2u {off.z=0.0;}
  let delta=vec3<f32>(n)+off-local;let a=abs(delta);
  var d=dot(delta,delta);
  if metric==2u {d=a.x+a.y+a.z;}else if metric==3u {d=max(a.x,max(a.y,a.z));}
  if d<distances.x {distances=vec3(d,distances.xy);nearest=delta;}
  else if d<distances.y {distances=vec3(distances.x,d,distances.y);}
  else if d<distances.z {distances.z=d;}
 }}}
 if style==1u {return vec3(mxn_cell(nearest+p,dim));}
 if metric==0u {return sqrt(distances);}
 return distances;
}
// kind: 0 scalar Perlin, 1 vector Perlin, 2 cell, 3 scalar fractal,
// 4 vector fractal, 5 Worley; options carry octave/jitter and lacunarity/style.
fn mxn_noise(p0:vec3<f32>,dim:u32,kind:u32,options:vec4<f32>)->vec3<f32> {
 if kind==0u||kind==1u {return mxn_perlin(p0,dim,kind==1u);}
 if kind==2u {return vec3(mxn_cell(p0,dim));}
 if kind==5u {return mxn_worley(p0,dim,options.x,u32(options.y),u32(options.z));}
 var p=p0;var amplitude=1.0;var result=vec3(0.0);
 for(var i=0;i<i32(options.x);i++){result+=amplitude*mxn_perlin(p,dim,kind==4u);amplitude*=options.z;p*=options.y;}
 return result;
}
