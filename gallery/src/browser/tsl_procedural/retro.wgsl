// r186 hashBlur, 32 samples, straight alpha. Preserve scalar operation grouping.
fn retro_mod(x:f32,y:f32)->f32{return x-y*floor(x/y);}
fn retro_hash_blur(t:texture_2d<f32>,s:sampler,uv:vec2<f32>,amount:f32)->vec3<f32>{
 var color=vec4(0.0);
 for(var i=0.0;i<32.0;i+=1.0){
  let angle=(i/32.0)*6.283185307179586;
  color+=textureSample(t,s,uv+((vec2(cos(angle),sin(angle))*vec2(fract(sin(retro_mod(dot(vec2(i,uv.x+uv.y),vec2(12.9898,78.233)),3.141592653589793))*43758.5453)+amount))*vec2(amount)));
 }
 return (color/vec4(32.0)).rgb;
}
