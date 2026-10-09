// The built-in materials' LightProbeGrid binding. Custom shader programs
// declare the stub in shader.rs instead, so their pipelines leave the atlas
// out of their group 0 ( keeping within 16 sampled textures per stage ).
@group(0) @binding(27) var grid_atlas: texture_3d<f32>;
@group(0) @binding(28) var grid_sampler: sampler;
// LightProbeGridNode: the atlas's L2 SH at the normal-offset position, as
// irradiance ( u.probe_grid: min and intensity, max and falloff, counts ).
fn probe_grid_irradiance(position:vec3<f32>,normal:vec3<f32>)->vec3<f32>{
 if u.probe_grid[2].w<0.5 {return vec3(0.0);}
 let mn=u.probe_grid[0].xyz;let mx=u.probe_grid[1].xyz;let res=u.probe_grid[2].xyz;
 let range=mx-mn;let rm1=res-1.0;let spacing=range/rm1;
 let p=position+normal*spacing*0.5;
 let uvw=clamp((p-mn)/range,vec3(0.0),vec3(1.0))*rm1/res+vec3(0.5,0.5,0.5)/res;
 let base=uvw.z*res.z+1.0;let padded=res.z+2.0;let depth=padded*7.0;
 var s:array<vec4<f32>,7>;
 for(var t=0;t<7;t++){s[t]=textureSampleLevel(grid_atlas,grid_sampler,vec3(uvw.xy,(base+padded*f32(t))/depth),0.0);}
 let sh=array<vec3<f32>,9>(s[0].xyz,vec3(s[0].w,s[1].xy),vec3(s[1].zw,s[2].x),s[2].yzw,s[3].xyz,vec3(s[3].w,s[4].xy),vec3(s[4].zw,s[5].x),s[5].yzw,s[6].xyz);
 let n=normal;
 let e=sh[0]*0.886227+sh[1]*1.023328*n.y+sh[2]*1.023328*n.z+sh[3]*1.023328*n.x+sh[4]*0.858086*n.x*n.y+sh[5]*0.858086*n.y*n.z+sh[6]*(n.z*n.z*0.743125-0.247708)+sh[7]*0.858086*n.x*n.z+sh[8]*0.429043*(n.x*n.x-n.y*n.y);
 var irradiance=max(e,vec3(0.0))*u.probe_grid[0].w;
 if u.probe_grid[1].w>0.0 {
  let outside=max(mn-position,vec3(0.0))+max(position-mx,vec3(0.0));
  irradiance*=1.0-smoothstep(0.0,u.probe_grid[1].w,length(outside));
 }
 return irradiance;
}
