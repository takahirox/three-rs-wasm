@group(0) @binding(0) var source:texture_2d<f32>;
@vertex fn vs(@builtin(vertex_index) i:u32)->@builtin(position) vec4<f32> {
    let x=f32((i<<1u)&2u);let y=f32(i&2u);return vec4(x*2.0-1.0,1.0-y*2.0,0.0,1.0);
}
@fragment fn fs(@builtin(position) p:vec4<f32>)->@location(0) vec4<f32> {
    return textureLoad(source,vec2<i32>(p.xy),0);
}
