// Mipped B-spline filtering adapted from Three.js r186 TextureBicubic.js (MIT).
fn tsl_bicubic_weights(a:vec2<f32>)->mat4x2<f32> {
    return mat4x2((a*(a*(-a+3.0)-3.0)+1.0)/6.0,
        (a*a*(3.0*a-6.0)+4.0)/6.0,
        (a*(a*(-3.0*a+3.0)+3.0)+1.0)/6.0,a*a*a/6.0);
}
fn tsl_bicubic_sample(map:texture_2d<f32>,s:sampler,uv:vec2<f32>,level:i32)->vec4<f32> {
    let size=vec2<f32>(textureDimensions(map,level));
    let pixel=uv*size+0.5;let base=floor(pixel);let w=tsl_bicubic_weights(fract(pixel));
    let g0=w[0]+w[1];let g1=w[2]+w[3];
    let p0=(base-1.0+w[1]/g0-0.5)/size;let p1=(base+1.0+w[3]/g1-0.5)/size;
    return g0.y*(g0.x*textureSampleLevel(map,s,p0,f32(level))+g1.x*textureSampleLevel(map,s,vec2(p1.x,p0.y),f32(level)))
        +g1.y*(g0.x*textureSampleLevel(map,s,vec2(p0.x,p1.y),f32(level))+g1.x*textureSampleLevel(map,s,p1,f32(level)));
}
fn tsl_bicubic(map:texture_2d<f32>,s:sampler,uv:vec2<f32>,level:f32)->vec4<f32>{
 let lod=clamp(level,0.0,f32(textureNumLevels(map)-1u));
 return mix(tsl_bicubic_sample(map,s,uv,i32(floor(lod))),tsl_bicubic_sample(map,s,uv,i32(ceil(lod))),fract(lod));
}
