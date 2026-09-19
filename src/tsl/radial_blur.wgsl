// Three.js r186 radialBlur (MIT), straight-alpha path.
fn tsl_radial_blur(tex:texture_2d<f32>,s:sampler,uv:vec2<f32>,size:vec2<f32>,options:vec4<f32>)->vec4<f32>{
    let count=clamp(i32(options.z),1,64);
    let base=textureSampleLevel(tex,s,uv,0.0);
    let offset=(vec2(0.5)-uv)/f32(count);
    let noise=fract(52.9829189*fract(dot(uv*size,vec2(0.06711056,0.00583715))));
    var sample_uv=uv+offset*noise;
    var blur=vec4(0.0);var weight=options.x;
    for(var i=0;i<count;i++){sample_uv+=offset;blur+=textureSampleLevel(tex,s,sample_uv,0.0)*weight;weight*=options.y;}
    blur/=f32(count);blur*=options.w;
    return mix(blur,base*2.0,0.5);
}
