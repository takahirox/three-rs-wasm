@group(0) @binding(0) var input_texture:texture_2d<f32>;
@group(0) @binding(1) var input_sampler:sampler;
@group(0) @binding(2) var<uniform> params:array<vec4<f32>,16>;
@group(0) @binding(3) var history_texture:texture_2d<f32>;
struct Out {@builtin(position) position:vec4<f32>,@location(0) uv:vec2<f32>};
@vertex fn vs_main(@builtin(vertex_index) index:u32)->Out {
    let uv=vec2(f32((index<<1u)&2u),f32(index&2u));var out:Out;
    out.position=vec4(uv*2.0-1.0,0.0,1.0);out.uv=vec2(uv.x,1.0-uv.y);return out;
}
@fragment fn fs_main(in:Out)->@location(0) vec4<f32> {return effect(in.uv);}
