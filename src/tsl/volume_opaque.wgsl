fn tsl_volume_opaque(map:texture_3d<f32>,s:sampler,origin:vec3<f32>,position:vec3<f32>,options:vec4<f32>)->vec4<f32>{
 let direction=normalize(position-origin);let bounds=tsl_volume_bounds(origin,direction);if bounds.x>bounds.y {discard;}
 let start=max(bounds.x,0.0);let inc=1.0/abs(direction);let stepSize=min(inc.x,min(inc.y,inc.z))/options.y;
 var ray=origin+start*direction;var previous=vec3(0.0);var hasPrevious=false;
 for(var t=start;t<bounds.y;t+=stepSize){
  let value=textureSampleLevel(map,s,ray+0.5,0.0).r;
  if value>options.x {var surface=ray;
   if options.z>0.5 && hasPrevious {var p0=previous;var p1=ray;for(var i=0;i<4;i++){let p=(p0+p1)*0.5;let above=textureSampleLevel(map,s,p+0.5,0.0).r>options.x;p1=select(p1,p,above);p0=select(p,p0,above);}surface=p1;}
   return vec4(tsl_volume_normal(map,s,surface+0.5)*0.5+surface*1.5+0.25,1.0);
  }
  previous=ray;hasPrevious=true;ray+=direction*stepSize;
 }return vec4(0.0);
}
