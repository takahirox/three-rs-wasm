@fragment fn fs_main(input:Varying)->@location(0) vec4<f32>{
 let size=vec2<f32>(textureDimensions(beauty));let texel=input.uv*size;
 var closest=2.0;var farthest=-1.0;var coordinate=vec2<i32>(0);
 for(var x=-1;x<=1;x++){for(var y=-1;y<=1;y++){
  let p=vec2<i32>(texel+vec2<f32>(f32(x),f32(y)));let z=textureLoad(depth,p,0);
  if z<closest {closest=z;coordinate=p;}farthest=max(farthest,z);
 }}
 let offset=textureLoad(motion,coordinate,0).xy*vec2(0.5,-0.5);let history_uv=input.uv-offset;
 let history_size=vec2<i32>(textureDimensions(history_depth));let sample_position=clamp(vec2<i32>(floor(history_uv*vec2<f32>(history_size))),vec2(0),history_size-1);
 let old_z=textureLoad(history_depth,sample_position,0);
 let clip=vec4(history_uv*vec2(2.0,-2.0)+vec2(-1.0,1.0),old_z,1.0);
 let previous_view=params.previous_projection_inverse*clip;
 let view=params.previous_to_view*vec4(previous_view.xyz/previous_view.w,1.0);
 let near=params.camera.x;let far=params.camera.y;let previous_z=select(((near+view.z)*far)/((far-near)*view.z),(view.z+near)/(near-far),params.camera.z>0.5);
 let valid=all(history_uv>=vec2(0.0))&&all(history_uv<=vec2(1.0))&&((farthest-closest)>params.settings.y||(closest-previous_z)<=params.settings.x);
 let current=textureSample(beauty,linear_sampler,input.uv);let old=textureSample(history,linear_sampler,history_uv);
 let speed=clamp(length((input.uv-history_uv)*size)/params.settings.z,0.0,1.0);
 let phase=abs(fract(offset*size));let subpixel=max(phase,1.0-phase);
 let weight=select(1.0,clamp(0.05+(1.0-subpixel.x*subpixel.y)/0.75*0.25*params.settings.w+speed,0.0,1.0),valid);
 var first=current;var second=current*current;
 let offsets=array<vec2<i32>,8>(vec2(-1,-1),vec2(-1,1),vec2(1,-1),vec2(1,1),vec2(1,0),vec2(0,-1),vec2(0,1),vec2(-1,0));
 for(var i=0;i<8;i++){let sample=max(textureLoad(beauty,vec2<i32>(texel)+offsets[i],0),vec4(0.0));first+=sample;second+=sample*sample;}
 let mean=first/9.0;let variance=sqrt(max(second/9.0-mean*mean,vec4(0.0)))*mix(0.5,1.0,(1.0-speed)*(1.0-speed));
 let low=mean-variance;let high=mean+variance;let clipped=clip_aabb(clamp(mean,low,high),old,low,high);
 return flicker(current,clipped,weight);
}
