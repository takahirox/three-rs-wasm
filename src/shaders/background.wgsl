struct Background { inverse_projection:mat4x4<f32>, camera:mat4x4<f32>, options:vec4<f32>, intensity:vec4<f32> };
@group(0) @binding(0) var<uniform> u:Background;
@group(0) @binding(1) var source:texture_2d<f32>;
@group(0) @binding(2) var atlas:texture_2d<f32>;
@group(0) @binding(3) var linear_sampler:sampler;
struct Out { @builtin(position) position:vec4<f32>, @location(0) ndc:vec2<f32> };
@vertex fn vs(@builtin(vertex_index) i:u32)->Out {
 let xy=vec2(f32((i<<1u)&2u),f32(i&2u));var out:Out;out.ndc=xy*2.0-1.0;out.position=vec4(out.ndc,1.0,1.0);return out;
}
// Three's sharp background first converts the panorama to a cube of image.height.
// Evaluate that cube's four texels directly, avoiding a second resident 6-face texture.
fn sharp_background(d:vec3<f32>)->vec3<f32> {
 let face=cube_face(d);let size=f32(textureDimensions(source).y);
 let pixel=face_uv(d,face)*size-0.5;let base=floor(pixel);let weight=fract(pixel);
 let a=quantizeToF16(textureSampleLevel(source,linear_sampler,equirect_uv(face_direction((base+vec2(0.5,0.5))/size,face)),0.0).rgb);
 let b=quantizeToF16(textureSampleLevel(source,linear_sampler,equirect_uv(face_direction((base+vec2(1.5,0.5))/size,face)),0.0).rgb);
 let c=quantizeToF16(textureSampleLevel(source,linear_sampler,equirect_uv(face_direction((base+vec2(0.5,1.5))/size,face)),0.0).rgb);
 let e=quantizeToF16(textureSampleLevel(source,linear_sampler,equirect_uv(face_direction((base+vec2(1.5,1.5))/size,face)),0.0).rgb);
 return mix(mix(a,b,weight.x),mix(c,e,weight.x),weight.y);
}
@fragment fn fs(in:Out)->@location(0) vec4<f32> {
 let view=u.inverse_projection*vec4(in.ndc,1.0,1.0);
 let direction=normalize((u.camera*vec4(view.xyz/view.w,0.0)).xyz);
 let c=cos(u.options.x);let s=sin(u.options.x);let d=vec3(c*direction.x-s*direction.z,direction.y,s*direction.x+c*direction.z);
 if u.options.w>0.5 {return vec4(textureSampleLevel(source,linear_sampler,equirect_uv(d),0.0).rgb*u.intensity.x,1.0);}
 if u.options.y==0.0 {return vec4(sharp_background(d)*u.intensity.x,1.0);}
 let filtered=vec3(d.x,-d.y,d.z);
 let mip=clamp(roughness_mip(u.options.y),-2.0,u.options.z);let lo=floor(mip);
 return vec4(mix(textureSampleLevel(atlas,linear_sampler,cube_uv(filtered,lo,u.options.z),0.0).rgb,textureSampleLevel(atlas,linear_sampler,cube_uv(filtered,lo+1.0,u.options.z),0.0).rgb,fract(mip))*u.intensity.x,1.0);
}
