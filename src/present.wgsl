@group(0) @binding(0) var source:texture_2d<f32>;
@group(0) @binding(1) var<uniform> options:vec4<f32>;
@vertex fn vs(@builtin(vertex_index) i:u32)->@builtin(position) vec4<f32> {
    let x=f32((i<<1u)&2u);let y=f32(i&2u);return vec4(x*2.0-1.0,1.0-y*2.0,0.0,1.0);
}
@fragment fn fs(@builtin(position) p:vec4<f32>)->@location(0) vec4<f32> {
    let value=textureLoad(source,vec2<i32>(p.xy),0);
    if options.y<0.5 {return value;}
    var c=value.rgb*options.x/0.6;
    c=mat3x3<f32>(vec3(0.59719,0.07600,0.02840),vec3(0.35458,0.90834,0.13383),vec3(0.04823,0.01566,0.83777))*c;
    c=(c*(c+0.0245786)-0.000090537)/(c*(0.983729*c+0.4329510)+0.238081);
    c=mat3x3<f32>(vec3(1.60475,-0.10208,-0.00327),vec3(-0.53108,1.10813,-0.07276),vec3(-0.07367,-0.00605,1.07602))*c;
    return vec4(clamp(c,vec3(0.0),vec3(1.0)),value.a);
}
