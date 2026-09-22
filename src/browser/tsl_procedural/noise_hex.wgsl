// r186 MaterialXHextile.js checkerboard composition (Three.js MIT).
fn hex_hash(p:vec2<f32>)->vec2<f32>{
 let a=fract(vec3(p.x,p.y,p.x)*vec3(0.1031,0.103,0.0973));let b=a+dot(a,a.yzx+33.33);return fract((b.xx+b.yz)*b.zy);
}
fn hex_gain(x:f32,r:f32)->f32{
 let rr=clamp(r,0.001,0.999);let a=(1.0/rr-2.0)*(1.0-2.0*x);
 return mix(x/(a+1.0),(a-x)/(a-1.0),step(0.5,x));
}
fn hex_checker(uv:vec2<f32>)->vec3<f32>{
 let p=uv*2.2;let st=p*3.4641016151377544;
 let skew=vec2(st.x-0.57735027*st.y,1.15470054*st.y);
 let f=fract(skew);let z=1.0-f.x-f.y;let s=step(0.0,-z);let s2=2.0*s-1.0;
 let weights=vec3(-z*s2,s-f.y*s2,s-f.x*s2);
 let base=floor(skew);
 let ids=array<vec2<f32>,3>(base+vec2(s),base+vec2(s,1.0-s),base+vec2(1.0-s,s));
 var colors=vec3(0.0);
 for(var i=0u;i<3u;i++){
  let scaled=ids[i]/3.4641016151377544;let center=vec2(scaled.x+0.5*scaled.y,0.8660254*scaled.y);
  let random=hex_hash(ids[i]+vec2(0.12345));let rotation=mix(0.0,6.283185307179586,random.x*0.25);
  let scale=mix(1.0,mix(0.5,2.0,random.y),0.1);let offset=mix(vec2(0.0),vec2(1.0),random*0.2);
  let d=p-center;let rotated=vec2(cos(rotation)*d.x-sin(rotation)*d.y,sin(rotation)*d.x+cos(rotation)*d.y);
  let coord=rotated/max(scale,1e-6)+center+offset;let tiled=floor(coord*8.0);let sum=tiled.x+tiled.y;
  colors[i]=sum-2.0*floor(sum/2.0);
 }
 let luminance=mix(vec3(1.0),colors*(0.2722287+0.6740818+0.0536895),vec3(0.25));
 let weighted=luminance*pow(weights,vec3(7.0));let normalized=weighted/(weighted.x+weighted.y+weighted.z);
 let gain=vec3(hex_gain(normalized.x,0.35),hex_gain(normalized.y,0.35),hex_gain(normalized.z,0.35));let blend=gain/(gain.x+gain.y+gain.z);
 return vec3((blend.x*colors.x+blend.y*colors.y)+blend.z*colors.z);
}
