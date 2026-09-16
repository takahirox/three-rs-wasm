@group(0) @binding(0) var source:texture_2d<f32>;
@group(0) @binding(1) var linear_sampler:sampler;
struct Out { @builtin(position) position:vec4<f32>, @location(0) uv:vec2<f32> };
@vertex fn vs(@builtin(vertex_index) i:u32)->Out {
 var out:Out;out.uv=vec2(f32((i<<1u)&2u),f32(i&2u));out.position=vec4(out.uv.x*2.0-1.0,1.0-out.uv.y*2.0,0.0,1.0);return out;
}
@fragment fn fs(in:Out)->@location(0) vec4<f32> {return textureSample(source,linear_sampler,in.uv);}
