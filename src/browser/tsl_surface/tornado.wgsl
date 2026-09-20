fn tornado_sample(map:texture_2d<f32>,s:sampler,p:vec2<f32>)->vec3<f32>{return (textureSampleLevel(map,s,vec2(p.x,1.0-p.y),1.0).rgb-0.45)/0.25;}
fn tornado_radial(uv:vec2<f32>,scale:vec2<f32>,rotation:f32,offset:f32)->vec2<f32>{let p=uv-0.5;return vec2((atan2(p.y,p.x)+3.141592653589793)/6.283185307179586,length(p))*scale+vec2(rotation,offset);}
fn tornado_color(uv:vec2<f32>,time:f32,speed:f32,color:vec3<f32>,kind:f32,map:texture_2d<f32>,s:sampler)->vec4<f32>{
 let t=time*speed+select(0.0,123.4,kind>1.5);
 if kind<0.5{
  let a=tornado_radial(uv,vec2(0.5),t,t);let b=tornado_radial(uv,vec2(2.0,8.0),t*2.0,t*8.0);
  let n1=tornado_sample(map,s,vec2(a.x-a.y,a.y)*vec2(4.0,1.0)).r;let n2=tornado_sample(map,s,vec2(b.x-b.y*0.25,b.y)*vec2(2.0,0.25)).b;
  let d=length(uv-0.5);let fade=min(smoothstep(0.5,0.9,1.0-d),smoothstep(0.0,0.2,d));let value=n1*n2*fade;
  return vec4(color*step(0.2,value)*3.0,smoothstep(0.0,0.01,value));
 }
 let a=uv+vec2(t,-t);let b=uv+vec2(t*0.5,-t);
 let aSample=tornado_sample(map,s,vec2(a.x-a.y,a.y)*vec2(2.0,0.25));let bSample=tornado_sample(map,s,vec2(b.x-b.y,b.y)*vec2(5.0,1.0));
 let n1=select(aSample.r,aSample.g,kind>1.5);let n2=select(bSample.g,bSample.b,kind>1.5);
 let fade=min(smoothstep(0.0,select(0.1,0.2,kind>1.5),uv.y),smoothstep(0.0,0.4,1.0-uv.y));let value=n1*n2*fade;
 if kind>1.5{return vec4(vec3(0.0),smoothstep(0.0,0.01,value));}
 return vec4(color*1.2/dot(color,vec3(0.2126,0.7152,0.0722)),smoothstep(0.0,0.1,value));
}
