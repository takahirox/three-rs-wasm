fn delta(a:vec3<f32>,b:vec3<f32>)->f32 {let t=abs(a-b);return max(max(t.x,t.y),t.z);}
fn effect(p:vec2<f32>)->vec4<f32>{
 let d=1.0/vec2<f32>(textureDimensions(input_texture));let c=textureSample(input_texture,input_sampler,p).rgb;
 let left=delta(c,textureSample(input_texture,input_sampler,p+d*vec2(-1.0,0.0)).rgb);let top=delta(c,textureSample(input_texture,input_sampler,p+d*vec2(0.0,-1.0)).rgb);
 var edges=step(vec2(0.1),vec2(left,top));if dot(edges,vec2(1.0))==0.0 {return vec4(0.0);}
 let right=delta(c,textureSampleLevel(input_texture,input_sampler,p+d*vec2(1.0,0.0),0.0).rgb);let bottom=delta(c,textureSampleLevel(input_texture,input_sampler,p+d*vec2(0.0,1.0),0.0).rgb);
 let ll=delta(c,textureSampleLevel(input_texture,input_sampler,p+d*vec2(-2.0,0.0),0.0).rgb);let tt=delta(c,textureSampleLevel(input_texture,input_sampler,p+d*vec2(0.0,-2.0),0.0).rgb);
 edges*=step(vec2(0.5*max(max(max(left,top),max(right,bottom)),max(ll,tt))),vec2(left,top));return vec4(edges,0.0,0.0);
}
