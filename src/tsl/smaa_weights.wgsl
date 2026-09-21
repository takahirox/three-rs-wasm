@group(1) @binding(0) var area:texture_2d<f32>;
@group(1) @binding(1) var area_sampler:sampler;
@group(1) @binding(2) var search:texture_2d<f32>;
@group(1) @binding(3) var search_sampler:sampler;
fn edge(p:vec2<f32>)->vec2<f32>{return textureSampleLevel(input_texture,input_sampler,p,0.0).rg;}
fn search_edge(p:vec2<f32>,end:f32,axis:u32,sign:f32,d:vec2<f32>)->f32{
 var coord=p;var e=vec2(0.0);for(var i=0;i<8;i++){e=edge(coord);coord[axis]+=sign*2.0*d[axis];let along=select(e.g,e.r,axis==1u);let crossing=select(e.r,e.g,axis==1u);if (coord[axis]-end)*sign>=0.0 || along<=0.8281 || crossing!=0.0 {break;}}
 let sample=select(e,e.gr,axis==1u);let len=255.0*textureSampleLevel(search,search_sampler,vec2(select(0.0,0.5,sign>0.0)+sample.r*0.5,sample.g),0.0).r;
 return coord[axis]-sign*(3.25-len)*d[axis];
}
fn area_weights(dist:vec2<f32>,a:f32,b:f32)->vec2<f32>{let coord=(16.0*round(4.0*vec2(a,b))+dist+0.5)/vec2(160.0,560.0);return textureSampleLevel(area,area_sampler,coord,0.0).rg;}
fn effect(p:vec2<f32>)->vec4<f32>{
 let d=1.0/vec2<f32>(textureDimensions(input_texture));let e=edge(p);var w=vec4(0.0);
 if e.g>0.0 {let left=search_edge(p+d*vec2(-0.25,-0.125),p.x+d.x*(-0.25-16.0),0u,-1.0,d);let right=search_edge(p+d*vec2(1.25,-0.125),p.x+d.x*(1.25+16.0),0u,1.0,d);let a=edge(vec2(left,p.y-0.25*d.y)).r;let b=edge(vec2(right+d.x,p.y-0.25*d.y)).r;let weights=area_weights(sqrt(abs(vec2(left,right)/d.x-p.x/d.x)),a,b);w.r=weights.r;w.g=weights.g;}
 if e.r>0.0 {let up=search_edge(p+d*vec2(-0.125,-0.25),p.y+d.y*(-0.25-16.0),1u,-1.0,d);let down=search_edge(p+d*vec2(-0.125,1.25),p.y+d.y*(1.25+16.0),1u,1.0,d);let a=edge(vec2(p.x-0.25*d.x,up)).g;let b=edge(vec2(p.x-0.25*d.x,down+d.y)).g;let weights=area_weights(sqrt(abs(vec2(up,down)/d.y-p.y/d.y)),a,b);w.b=weights.r;w.a=weights.g;}
 return w;
}
