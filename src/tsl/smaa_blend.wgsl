fn weights(p:vec2<f32>)->vec4<f32>{return textureSampleLevel(history_texture,input_sampler,p,0.0);}
fn effect(p:vec2<f32>)->vec4<f32>{
 let d=1.0/vec2<f32>(textureDimensions(input_texture));let current=weights(p);let a=vec4(current.r,weights(p+vec2(0.0,d.y)).g,current.b,weights(p+vec2(d.x,0.0)).a);let c=textureSampleLevel(input_texture,input_sampler,p,0.0);
 if dot(a,vec4(1.0))<0.00001 {return c;}
 var offset=vec2(select(-a.b,a.a,a.a>a.b),select(-a.r,a.g,a.g>a.r));if abs(offset.x)>abs(offset.y){offset.y=0.0;}else{offset.x=0.0;}
 return mix(c,textureSampleLevel(input_texture,input_sampler,p+sign(offset)*d,0.0),max(abs(offset.x),abs(offset.y)));
}
