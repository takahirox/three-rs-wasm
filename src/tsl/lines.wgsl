struct Segment {start:vec4<f32>,end:vec4<f32>,color_start:vec4<f32>,color_end:vec4<f32>};
@group(1) @binding(0) var<storage,read> segments:array<Segment>;
fn deform(position:vec3<f32>,normal:vec3<f32>,uv:vec2<f32>)->vec3<f32>{return position;}
fn shade(surface:VertexOut,base:vec4<f32>)->vec4<f32>{
 let v=surface.uv;let dashed=u.custom[0].z>0.5;let coverage=u.custom[2].x>0.5;
 if dashed {if abs(v.y)>1.0 {discard;}let d=surface.line_distance;let period=u.custom[1].z+u.custom[1].w;if d-floor(d/period)*period>u.custom[1].z {discard;}}
 var alpha=1.0;
 if u.custom[0].y>0.5 {
  let p1=surface.tangent.xyz;let p2=surface.bitangent;let ray=normalize(surface.view_position)*1e5;
  let dir=p2-p1;let a=dot(p1,ray);let b=dot(ray,dir);let c=dot(p1,dir);let d=dot(ray,ray);let e=dot(dir,dir);
  let mua=clamp((a*b-c*d)/(e*d-b*b),0.0,1.0);let mub=clamp((a+b*mua)/d,0.0,1.0);
  let n=length(p1+dir*mua-ray*mub)/u.custom[0].x;let dn=fwidth(n);
  if !dashed {if coverage {alpha=1.0-smoothstep(0.5-dn,0.5+dn,n);}else if n>0.5 {discard;}}
 }else{
  let b=select(v.y+1.0,v.y-1.0,v.y>0.0);let len2=v.x*v.x+b*b;let dl=fwidth(len2);
  if abs(v.y)>1.0 {if coverage {alpha=1.0-smoothstep(1.0-dl,1.0+dl,len2);}else if len2>1.0 {discard;}}
 }
 return vec4(surface.color.rgb,alpha);
}
