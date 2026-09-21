fn project_vertex(surface:VertexOut,position:vec3<f32>)->VertexOut {
 let segment=segments[surface.instance_index];let mv=u.view*u.model;
 var start=mv*vec4(segment.start.xyz,1.0);var end=mv*vec4(segment.end.xyz,1.0);
 var ds=segment.start.w;var de=segment.end.w;var out=surface;
 out.tangent=vec4(start.xyz,0.0);out.bitangent=end.xyz;
 if u.projection[2][3]==-1.0 {
  let a=u.projection[2][2];let b=u.projection[3][2];let near=select(-0.5*b/a,-b/(a+1.0),a>0.0);
  if start.z<0.0 && end.z>0.0 {let t=(near-start.z)/(end.z-start.z);end=vec4(mix(start.xyz,end.xyz,t),end.w);de=mix(ds,de,t);}
  else if end.z<0.0 && start.z>=0.0 {let t=(near-end.z)/(start.z-end.z);start=vec4(mix(end.xyz,start.xyz,t),start.w);ds=mix(de,ds,t);}
 }
 out.line_distance=select(ds,de,position.y>=0.5)*u.custom[1].x+u.custom[1].y;
 out.color=select(segment.color_start,segment.color_end,position.y>=0.5);
 let cs=u.projection*start;let ce=u.projection*end;let ns=cs.xyz/cs.w;let ne=ce.xyz/ce.w;
 let aspect=(u.point.x*u.transmission[2].z)/(u.point.y*u.transmission[2].w);var dir=normalize((ne.xy-ns.xy)*vec2(aspect,1.0));
 if u.custom[0].y>0.5 {
  let wd=normalize(end.xyz-start.xyz);let forward=normalize(mix(start.xyz,end.xyz,0.5));let up=normalize(cross(wd,forward));let fwd=cross(wd,up);let hw=u.custom[0].x*0.5;
  var p=select(start,end,position.y>=0.5);p+=vec4(select(up*hw,-up*hw,position.x>=0.0),0.0);
  if u.custom[0].z<0.5 {p+=vec4(select(-wd*hw,wd*hw,position.y>=0.5)+fwd*hw,0.0);if position.y>1.0 || position.y<0.0 {p-=vec4(fwd*2.0*hw,0.0);}}
  out.view_position=p.xyz;out.clip=u.projection*p;out.clip.z=select(ns.z,ne.z,position.y>=0.5)*out.clip.w;
 }else{
  var offset=vec2(dir.y,-dir.x);dir.x/=aspect;offset.x/=aspect;if position.x<0.0 {offset=-offset;}
  if position.y<0.0 {offset-=dir;}else if position.y>1.0 {offset+=dir;}
  offset*=u.custom[0].x/(u.point.y*u.transmission[2].w/u.custom[0].w);
  out.clip=select(cs,ce,position.y>=0.5);out.clip+=vec4(offset*out.clip.w,0.0,0.0);
 }
 // Line2 feeds the projected ribbon back through local position before MVP.
 // Preserve that path, including its depth rounding at overlapping round caps.
 let p=u.projection;
 var ip=mat4x4(vec4(1.0/p[0][0],0.0,0.0,0.0),vec4(0.0,1.0/p[1][1],0.0,0.0),vec4(0.0,0.0,0.0,1.0/p[3][2]),vec4(p[2][0]/p[0][0],p[2][1]/p[1][1],-1.0,p[2][2]/p[3][2]));
 if p[3][3]!=0.0 {ip=mat4x4(vec4(1.0/p[0][0],0.0,0.0,0.0),vec4(0.0,1.0/p[1][1],0.0,0.0),vec4(0.0,0.0,1.0/p[2][2],0.0),vec4(-p[3].xyz/vec3(p[0][0],p[1][1],p[2][2]),1.0));}
 let v=u.view;let cw=mat4x4(vec4(v[0].x,v[1].x,v[2].x,0.0),vec4(v[0].y,v[1].y,v[2].y,0.0),vec4(v[0].z,v[1].z,v[2].z,0.0),vec4(u.camera.xyz,1.0));
 let local=transpose(u.normal)*cw*ip*out.clip;
 out.clip=u.projection*(u.view*u.model)*vec4(local.xyz/local.w,1.0);
 return out;
}
