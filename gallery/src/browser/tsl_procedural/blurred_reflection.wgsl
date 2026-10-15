// r186 hashBlur with a sample callback masking reflected RGB by its depth.
fn reflection_depth(depth:texture_depth_2d,uv:vec2<f32>)->f32{
 let size=vec2<i32>(textureDimensions(depth));return textureLoad(depth,clamp(vec2<i32>(floor(uv*vec2<f32>(size))),vec2(0),size-1),0);
}
fn blurred_reflection(color:texture_2d<f32>,sam:sampler,depth:texture_depth_2d,uv:vec2<f32>,roughness:f32,radius:f32)->vec3<f32>{
 let amount=mix(0.01,0.1,radius);let range=mix(0.3,0.03,roughness);var blurred=vec4(0.0);
 for(var i=0.0;i<40.0;i+=1.0){
  let angle=i/40.0*6.283185307179586;let dt=dot(vec2(i,uv.x+uv.y),vec2(12.9898,78.233));let sn=dt-floor(dt/3.141592653589793)*3.141592653589793;
  let random=fract(sin(sn)*43758.5453);let q=vec2(cos(angle),sin(angle))*(random+amount);let coord=uv+q*amount;
  let sample=textureSample(color,sam,coord);let alpha=sample.a*reflection_depth(depth,coord);blurred+=vec4(sample.rgb*alpha,alpha);
 }
 blurred/=40.0;if blurred.a>0.0{blurred=vec4(blurred.rgb/blurred.a,blurred.a);}
 let mask=clamp(blurred.a*reflection_depth(depth,uv)/range,0.0,1.0);return mix(textureSample(color,sam,uv).rgb,blurred.rgb,mask*min(roughness*2.0,1.0))*0.1;
}
