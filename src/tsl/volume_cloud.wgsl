fn tsl_volume_cloud(map:texture_3d<f32>,s:sampler,origin:vec3<f32>,position:vec3<f32>,options:vec4<f32>)->vec4<f32>{
 let direction=normalize(position-origin);let bounds=tsl_volume_bounds(origin,direction);if bounds.x>bounds.y {discard;}
 let start=max(bounds.x,0.0);let inc=1.0/abs(direction);let stepSize=min(inc.x,min(inc.y,inc.z))/options.w;
 var ray=origin+start*direction;var color=vec4(0.0);
 for(var t=start;t<bounds.y;t+=stepSize){
  let value=smoothstep(options.x-options.z,options.x+options.z,textureSampleLevel(map,s,ray+0.5,0.0).r)*options.y;
  let shading=textureSampleLevel(map,s,ray-0.01,0.0).r-textureSampleLevel(map,s,ray+0.01,0.0).r;
  let col=shading*3.0+(ray.x+ray.y)*0.25+0.2;let opacity=(1.0-color.a)*value;
  color=vec4(color.rgb+opacity*col,color.a+opacity);if color.a>=0.95{break;}ray+=direction*stepSize;
 }return color;
}
