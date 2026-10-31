// Three.js r187 PMREMGenerator ( WebGL, src/extras/PMREMGenerator.js; MIT,
// LICENSE-THREE ): its materials as fragment passes over one cube face each.
// The box CubeCamera draws gives every fragment the direction through its
// face point ( cube_vert's vWorldDirection ), which `direction` computes from
// the face and the fragment position in the cube map convention WebGL and
// WebGPU share.
// source_size is a uniform, as three's, so the loops are not unrolled.
struct Pass { face:u32, size:f32, roughness:f32, lod_bias:f32, source_lod:f32, sigma:f32, source_size:i32, pad:f32 };
@group(0) @binding(0) var<uniform> p:Pass;
@group(0) @binding(2) var linear_sampler:sampler;
const PI:f32=3.141592653589793;
const GGX_SAMPLES:u32=256u;
const BLUR_SAMPLES:i32=20;
const GOLDEN_ANGLE:f32=2.39996322973;
@vertex fn vs(@builtin(vertex_index) i:u32)->@builtin(position) vec4<f32> {
 let xy=vec2(f32((i<<1u)&2u),f32(i&2u));return vec4(xy*2.0-1.0,0.0,1.0);
}
fn face_point(face:u32,uv:vec2<f32>)->vec3<f32> {
 let a=uv.x*2.0-1.0;let b=uv.y*2.0-1.0;
 switch face {
  case 0u:{return vec3(1.0,-b,-a);}
  case 1u:{return vec3(-1.0,-b,a);}
  case 2u:{return vec3(a,1.0,b);}
  case 3u:{return vec3(a,-1.0,-b);}
  case 4u:{return vec3(a,-b,1.0);}
  default:{return vec3(-a,-b,-1.0);}
 }
}
fn direction(position:vec4<f32>)->vec3<f32> {return normalize(face_point(p.face,position.xy/p.size));}
// Three's equirectUv for a texture stored from its top row.
fn equirect_uv(d:vec3<f32>)->vec2<f32> {return vec2(atan2(d.z,d.x)*0.15915494309189535+0.5,0.5-asin(clamp(d.y,-1.0,1.0))*0.3183098861837907);}

// PMREMEquirectangularToCubemap: four subpixel samples.
@group(0) @binding(1) var equirect:texture_2d<f32>;
@fragment fn fs_equirect(@builtin(position) position:vec4<f32>)->@location(0) vec4<f32> {
 let d=direction(position);let dx=dpdx(d)*0.25;let dy=dpdy(d)*0.25;
 var color=textureSampleLevel(equirect,linear_sampler,equirect_uv(normalize(d-dx-dy)),0.0).rgb;
 color+=textureSampleLevel(equirect,linear_sampler,equirect_uv(normalize(d+dx-dy)),0.0).rgb;
 color+=textureSampleLevel(equirect,linear_sampler,equirect_uv(normalize(d-dx+dy)),0.0).rgb;
 color+=textureSampleLevel(equirect,linear_sampler,equirect_uv(normalize(d+dx+dy)),0.0).rgb;
 return vec4(color*0.25,1.0);
}

// PMREMCubemapToCubemap. The engine's cubes hold the direction ( -x, y, z ) at
// x, as uploaded cube textures do in three ( flipEnvMap -1 ).
@group(0) @binding(1) var env_map:texture_cube<f32>;
@fragment fn fs_cube(@builtin(position) position:vec4<f32>)->@location(0) vec4<f32> {
 let d=direction(position);
 return vec4(textureSample(env_map,linear_sampler,vec3(-d.x,d.yz)).rgb,1.0);
}

// generateMipmap: the box filter over the level above.
@group(0) @binding(1) var level_above:texture_2d<f32>;
@fragment fn fs_mipmap(@builtin(position) position:vec4<f32>)->@location(0) vec4<f32> {
 let t=vec2<i32>(position.xy)*2;
 return (textureLoad(level_above,t,0)+textureLoad(level_above,t+vec2(1,0),0)+textureLoad(level_above,t+vec2(0,1),0)+textureLoad(level_above,t+vec2(1,1),0))*0.25;
}

// PMREMSphericalGaussianBlur.
@fragment fn fs_blur(@builtin(position) position:vec4<f32>)->@location(0) vec4<f32> {
 let output_direction=direction(position);let sigma=p.sigma;
 let up=select(vec3(1.0,0.0,0.0),vec3(0.0,0.0,1.0),abs(output_direction.z)<0.999);
 let tangent=normalize(cross(up,output_direction));let bitangent=cross(output_direction,tangent);
 let theta_max=min(3.0*sigma,PI);let truncation=1.0-exp(-0.5*theta_max*theta_max/(sigma*sigma));
 var accum_color=vec3(0.0);var accum_weight=0.0;
 for(var i=0;i<BLUR_SAMPLES;i++){
  let stratum=(f32(i)+0.5)/f32(BLUR_SAMPLES);
  let theta=sigma*sqrt(-2.0*log(1.0-stratum*truncation));let phi=f32(i)*GOLDEN_ANGLE;
  let offset=cos(phi)*tangent+sin(phi)*bitangent;let sample_direction=cos(theta)*output_direction+sin(theta)*offset;
  let weight=sin(theta)/theta;
  accum_color+=weight*textureSampleLevel(env_map,linear_sampler,sample_direction,0.0).rgb;accum_weight+=weight;
 }
 return vec4(accum_color/accum_weight,1.0);
}

// PMREMGGXConvolution: GGX VNDF importance sampling with V = N.
@fragment fn fs_ggx(@builtin(position) position:vec4<f32>)->@location(0) vec4<f32> {
 let n=direction(position);let roughness=p.roughness;
 if roughness<0.001 {return vec4(textureSampleLevel(env_map,linear_sampler,n,0.0).rgb,1.0);}
 let alpha=roughness*roughness;let alpha2=alpha*alpha;
 let up=select(vec3(1.0,0.0,0.0),vec3(0.0,0.0,1.0),abs(n.z)<0.999);
 let tangent=normalize(cross(up,n));let bitangent=cross(n,tangent);
 var prefiltered=vec3(0.0);var total_weight=0.0;
 for(var i=0u;i<GGX_SAMPLES;i++){
  // Hammersley: the Van der Corput radical inverse.
  let xi=vec2(f32(i)/f32(GGX_SAMPLES),f32(reverseBits(i))*2.3283064365386963e-10);
  let inv_q=1.0/(1.0-xi.x+alpha2*xi.x);let n_dot_l=(1.0-xi.x-alpha2*xi.x)*inv_q;
  if n_dot_l>0.0 {
   let phi=2.0*PI*xi.y;let sin_theta=2.0*alpha*sqrt(xi.x*(1.0-xi.x))*inv_q;
   let l=n*n_dot_l+(tangent*cos(phi)+bitangent*sin(phi))*sin_theta;
   let d=alpha2*inv_q;let lod=max(log2(d)+p.lod_bias,0.0);
   prefiltered+=textureSampleLevel(env_map,linear_sampler,l,lod).rgb*n_dot_l;total_weight+=n_dot_l;
  }
 }
 return vec4(prefiltered/total_weight,1.0);
}

// PMREMGGXIntegration: every texel of the 16-texel source mip, GGX-weighted.
@fragment fn fs_integration(@builtin(position) position:vec4<f32>)->@location(0) vec4<f32> {
 let n=direction(position);let alpha=p.roughness*p.roughness;let alpha2=alpha*alpha;
 let source_size=p.source_size;let texel_size=2.0/f32(source_size);
 var prefiltered=vec3(0.0);var total_weight=0.0;
 for(var face=0;face<3;face++){
  for(var y=0;y<source_size;y++){
   for(var x=0;x<source_size;x++){
    let uv=(vec2(f32(x),f32(y))+0.5)*texel_size-1.0;
    var texel_direction=select(select(vec3(uv,1.0),vec3(uv.x,1.0,uv.y),face==1),vec3(1.0,uv),face==0);
    let inv_distance=inverseSqrt(1.0+dot(uv,uv));var n_dot_l=dot(n,texel_direction);
    texel_direction*=select(1.0,-1.0,n_dot_l<0.0);n_dot_l=abs(n_dot_l)*inv_distance;
    let d=1.0+alpha2+(alpha2-1.0)*n_dot_l;
    let weight=n_dot_l*inv_distance*inv_distance*inv_distance/(d*d);
    prefiltered+=textureSampleLevel(env_map,linear_sampler,texel_direction,p.source_lod).rgb*weight;total_weight+=weight;
   }
  }
 }
 return vec4(prefiltered/total_weight,1.0);
}
