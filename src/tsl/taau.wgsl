// Three.js r186 TAAUNode.js, MIT (LICENSE-THREE).
@fragment fn fs_main(input:Varying)->@location(0) vec4<f32>{
 let size=vec2<f32>(textureDimensions(beauty));let input_position=input.uv*size;let texel=round(input_position-(vec2(0.5)+params.jitter.xy));
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

 let speed=clamp(length((input.uv-history_uv)*size)/params.settings.z,0.0,1.0);
 var sum=vec4(0.0);var weight_sum=0.0;var first=vec4(0.0);var second=vec4(0.0);
 for(var y=-1;y<=1;y++){for(var x=-1;x<=1;x++){
  let tap=vec2<i32>(texel)+vec2(x,y);let center=vec2<f32>(tap)+vec2(0.5)+params.jitter.xy;
  let delta=input_position-center;let weight=exp(dot(delta,delta)*(-2.29));
  let color=max(textureLoad(beauty,tap,0),vec4(0.0));sum+=color*weight;weight_sum+=weight;first+=color;second+=color*color;
 }}
 let current=sum/max(weight_sum,0.00001);let mean=first/9.0;
 let variance=sqrt(max(second/9.0-mean*mean,vec4(0.0)))*mix(0.5,1.0,(1.0-speed)*(1.0-speed));
 let old=textureSample(history,linear_sampler,history_uv);
 let low=mean-variance;let high=mean+variance;let clipped=clip_aabb(clamp(mean,low,high),old,low,high);
 let current_luma=dot(current.rgb,vec3(0.2126,0.7152,0.0722));let mean_luma=dot(mean.rgb,vec3(0.2126,0.7152,0.0722));
 let thin=smoothstep(0.0,0.2,abs(current_luma-mean_luma)/mean_luma);
 let can_lock=all(history_uv>=vec2(0.0))&&all(history_uv<=vec2(1.0))&&abs(closest-previous_z)<=params.settings.x;
 // r186 seeds history.lock to zero but copies only history.color afterwards.
 // Its otherwise-unused lock attachment therefore contributes exactly zero.
 let lock=clamp(select(0.0,thin,can_lock),0.0,1.0);
 let locked=mix(clipped,old,lock);let weight=select(1.0,clamp(0.025+speed,0.0,1.0),valid);
 return flicker(current,locked,weight);
}
