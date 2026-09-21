struct Params{size:u32,level:u32,max_mip:u32,mode:u32,roughness:f32,pad:vec3<f32>};
@group(0) @binding(0)var source:texture_2d<f32>;
@group(0) @binding(1)var linear_sampler:sampler;
@group(0) @binding(2)var destination:texture_storage_2d<rgba16float,write>;
@group(0) @binding(3)var<uniform> p:Params;
@compute @workgroup_size(8,8)
fn main(@builtin(global_invocation_id) id:vec3<u32>){
 if id.x>=p.size*3u || id.y>=p.size*2u{return;}
 let face=id.x/p.size+(id.y/p.size)*3u;
 let uv=(vec2<f32>(id.xy%vec2(p.size))+vec2(0.5)-1.0)/(f32(p.size)-2.0);
 let direction=face_direction(uv,face);let n=vec3(direction.x,-direction.y,direction.z);var result=vec3(0.0);
 if p.mode==0u {result=textureSampleLevel(source,linear_sampler,equirect_uv(n),0.0).rgb;}
 else if p.mode==3u {result=textureSampleLevel(source,linear_sampler,(vec2<f32>(id.xy)+0.5)/vec2<f32>(textureDimensions(source)),0.0).rgb;}
 else if p.mode>=4u {
  let up=select(vec3(1.0,0.0,0.0),vec3(0.0,0.0,1.0),abs(n.z)<0.999);let tangent=normalize(cross(up,n));let bitangent=cross(n,tangent);let sigma=p.roughness;let thetaMax=min(sigma*3.0,3.14159265359);let truncation=1.0-exp(-0.5*thetaMax*thetaMax/(sigma*sigma));var weight=0.0;
  for(var i=0u;i<20u;i++){let theta=sigma*sqrt(-2.0*log(1.0-(f32(i)+0.5)/20.0*truncation));let phi=f32(i)*2.399963229728653;let direction=n*cos(theta)+(tangent*cos(phi)+bitangent*sin(phi))*sin(theta);let w=sin(theta)/theta;result+=textureSampleLevel(source,linear_sampler,cube_uv(direction,f32(p.max_mip),f32(p.max_mip)),0.0).rgb*w;weight+=w;}result/=weight;
 }
 else if p.mode==2u {result=textureSampleLevel(source,linear_sampler,cube_uv(n,f32(p.max_mip)-f32(p.level),f32(p.max_mip)),0.0).rgb;}
 else {
  let level=f32(p.max_mip)-f32(p.level)+1.0;
  let up=select(vec3(1.0,0.0,0.0),vec3(0.0,0.0,1.0),abs(n.z)<0.999);
  let tangent=normalize(cross(up,n));let bitangent=cross(n,tangent);var weight=0.0;
  for(var i=0u;i<256u;i++){
   let xi=vec2(f32(i)/256.0,f32(reverseBits(i))*2.3283064365386963e-10);
   let r=sqrt(xi.x);let phi=6.28318530718*xi.y;let alpha=p.roughness*p.roughness;
   let ht=normalize(vec3(alpha*r*cos(phi),alpha*r*sin(phi),sqrt(max(0.0,1.0-xi.x))));
   let h=normalize(tangent*ht.x+bitangent*ht.y+n*ht.z);let l=normalize(2.0*dot(n,h)*h-n);let nl=max(dot(n,l),0.0);
   if nl>0.0 {result+=textureSampleLevel(source,linear_sampler,cube_uv(l,level,f32(p.max_mip)),0.0).rgb*nl;weight+=nl;}
  }
  result/=max(weight,0.000001);
 }
 let extra=max(i32(p.level)-i32(p.max_mip)+4,0);
 let origin=vec2(extra*48,4*(i32(1u<<p.max_mip)-i32(p.size)));
 textureStore(destination,vec2<i32>(id.xy)+origin,vec4(result,1.0));
}
