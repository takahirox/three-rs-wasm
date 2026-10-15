// Three.js r186 webgpu_tsl_vfx_flames. Textures are resident; distortion is per fragment.
fn flame(uv:vec2<f32>,t:f32,kind:f32,cell:texture_2d<f32>,cellSampler:sampler,perlin:texture_2d<f32>,perlinSampler:sampler,gradient:texture_2d<f32>,gradientSampler:sampler)->vec4<f32>{
 let delta=uv-0.5;let delta2=dot(delta,delta);
 var p=(uv+delta*(delta2*delta2*10.0))*0.6+0.2;
 if kind<0.5 {p=vec2(p.x,p.y*p.y);}else{p=pow(abs(p),vec2(1.0,3.0))*sign(p);}
 p=p*2.0-vec2(0.5,0.0);
 if kind>0.5 {let q=fract(p-vec2(0.0,t));p.x+=(textureSampleLevel(perlin,perlinSampler,vec2(q.x,1.0-q.y),0.0).x-0.5)*0.5;}
 let g1=sin(t*10.0-p.y*6.283185307179586*2.0);let g2=smoothstep(0.0,1.0,p.y);let g3=smoothstep(0.0,0.3,1.0-p.y);
 p.x+=g1*g2*0.2;
 if kind<0.5{
  let q=fract(p*0.5-vec2(0.0,t*0.5));let noise=(1.0-smoothstep(0.0,0.5,1.0-textureSampleLevel(cell,cellSampler,vec2(q.x,1.0-q.y),0.0).r))*g2;
  let shape=1.0-length((p-0.5)*vec2(3.0,2.0))-noise;
  let color=textureSample(gradient,gradientSampler,vec2(shape,0.0)).rgb;
  return vec4(mix(color,vec3(1.0),step(0.8,shape)),smoothstep(0.0,0.3,shape));
 }
 let a=fract(p*0.5-vec2(0.0,t*0.25));let displacement=textureSampleLevel(perlin,perlinSampler,vec2(a.x,1.0-a.y),0.0).xy-0.5;
 let b=fract(p-vec2(0.0,t*0.5)+displacement);p.x+=(textureSampleLevel(perlin,perlinSampler,vec2(b.x,1.0-b.y),0.0).x-0.5)*0.5;
 let c=fract(p-vec2(0.0,t*1.5));let noise=smoothstep(0.25,1.0,1.0-textureSampleLevel(cell,cellSampler,vec2(c.x,1.0-c.y),0.0).r);
 let shape=step(length((p-0.5)*vec2(6.0,1.0)),0.5)*noise*g3;
 return vec4(vec3(1.0),step(0.01,shape));
}
