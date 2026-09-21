//! Reusable ports of r186 display nodes. Adaptive loops execute in WGSL, not Rust.
use super::*;

pub fn srgb(color: Node) -> Node {
    WgslFn::new("tsl_srgb", "fn tsl_srgb(c:vec3<f32>)->vec3<f32>{return select(1.055*pow(max(c,vec3(0.0)),vec3(0.41666))-0.055,c*12.92,c<=vec3(0.0031308));}", &[Type::Vec3], Type::Vec3).unwrap().call(&[color])
}
/// r186 radialBlur. `size` is the output pixel size and options are
/// (weight, decay, integer sample count, exposure). Count is clamped to 1–64
/// and remains a runtime GPU loop.
pub fn radial_blur(texture: Texture, coordinate: Node, size: Node, options: Node) -> Node {
    WgslFn::new(
        "tsl_radial_blur",
        include_str!("radial_blur.wgsl"),
        &[
            Type::Texture,
            Type::Sampler,
            Type::Vec2,
            Type::Vec2,
            Type::Vec4,
        ],
        Type::Vec4,
    )
    .unwrap()
    .call(&[texture.node(), texture.sampler(), coordinate, size, options])
}
/// r186 FXAANode including adaptive edge search. Input must already be sRGB.
pub fn fxaa(texture: Texture, coordinate: Node, inverse_size: Node) -> Node {
    WgslFn::new(
        "tsl_fxaa",
        include_str!("fxaa.wgsl"),
        &[Type::Texture, Type::Sampler, Type::Vec2, Type::Vec2],
        Type::Vec4,
    )
    .unwrap()
    .call(&[texture.node(), texture.sampler(), coordinate, inverse_size])
}
pub fn transition(
    a: Node,
    b: Node,
    mask: Node,
    ratio: Node,
    threshold: Node,
    use_texture: Node,
) -> Node {
    let r = ratio.clone() * (threshold.clone() * float(2.0) + float(1.0)) - threshold.clone();
    let amount = ((mask - r) / threshold).clamp(float(0.0), float(1.0));
    use_texture
        .greater_than(float(0.5))
        .select(mix(a.clone(), b.clone(), amount), mix(b, a, ratio))
}
/// Encode premultiplied linear color for a premultiplied sRGB canvas. Unpremultiply
/// before the nonlinear conversion, then premultiply in the output color space.
pub fn premultiplied_srgb(color: Node) -> Node {
    let alpha = color.swizzle("w");
    vec4(
        srgb(color.rgb() / alpha.clone().max(float(1e-6))) * alpha.clone(),
        alpha,
    )
}

/// r186 Sobel edge magnitude. Pass display-encoded input when matching
/// sobel(renderOutput(scenePass)); only the eight nonzero kernel taps are read.
pub fn sobel(texture: Texture, coordinate: Node, inverse_size: Node) -> Node {
    WgslFn::new("tsl_sobel",r#"
fn tsl_sobel(t:texture_2d<f32>,s:sampler,p:vec2<f32>,d:vec2<f32>)->vec4<f32>{
 let l=vec3(0.2126,0.7152,0.0722);
 let a=dot(textureSample(t,s,p+d*vec2(-1.0,-1.0)).rgb,l);let b=dot(textureSample(t,s,p+d*vec2(0.0,-1.0)).rgb,l);let c=dot(textureSample(t,s,p+d*vec2(1.0,-1.0)).rgb,l);
 let e=dot(textureSample(t,s,p+d*vec2(-1.0,0.0)).rgb,l);let f=dot(textureSample(t,s,p+d*vec2(1.0,0.0)).rgb,l);
 let g=dot(textureSample(t,s,p+d*vec2(-1.0,1.0)).rgb,l);let h=dot(textureSample(t,s,p+d*vec2(0.0,1.0)).rgb,l);let i=dot(textureSample(t,s,p+d*vec2(1.0,1.0)).rgb,l);
 let x=-a-2.0*e-g+c+2.0*f+i;let y=-a-2.0*b-c+g+2.0*h+i;return vec4(vec3(sqrt(x*x+y*y)),1.0);
}"#,&[Type::Texture,Type::Sampler,Type::Vec2,Type::Vec2],Type::Vec4).unwrap().call(&[texture.node(),texture.sampler(),coordinate,inverse_size])
}

/// r186 ChromaticAberrationNode: independently scaled, radial RGB samples.
pub fn chromatic_aberration(
    texture: Texture,
    coordinate: Node,
    strength: Node,
    center: Node,
    scale: Node,
) -> Node {
    let offset = coordinate.clone() - center.clone();
    let shift = scale * float(0.02) * strength.clone() + strength * offset.length() * float(0.01);
    let red = texture.sample(center.clone() + offset.clone() * (float(1.0) + shift.clone()));
    let green = texture.sample(coordinate);
    let blue = texture.sample(center + offset * (float(1.0) - shift));
    vec4(
        vec3(red.x(), green.y(), blue.swizzle("z")),
        green.swizzle("w"),
    )
}

/// r186 boxBlur. Kernel size and separation are runtime GPU inputs; the input
/// must contain straight alpha unless `premultiplied_alpha` is requested.
pub fn box_blur(
    texture: Texture,
    coordinate: Node,
    size: Node,
    separation: Node,
    premultiplied_alpha: Node,
) -> Node {
    WgslFn::new("tsl_box_blur",r#"
fn tsl_box_blur(t:texture_2d<f32>,s:sampler,p:vec2<f32>,size:f32,separation:f32,premultiplied:bool)->vec4<f32>{
 let radius=max(i32(size),0);let step=vec2(max(separation,1.0))/vec2<f32>(textureDimensions(t));var sum=vec4(0.0);var count=0.0;
 for(var i= -radius;i<=radius;i++){for(var j= -radius;j<=radius;j++){
 var sample=textureSampleLevel(t,s,p+vec2<f32>(f32(i),f32(j))*step,0.0);if premultiplied{sample=vec4(sample.rgb*sample.a,sample.a);}sum+=sample;count+=1.0;
 }}sum/=count;if premultiplied{sum=vec4(sum.rgb/max(sum.a,1e-6),sum.a);}return sum;
}"#,&[Type::Texture,Type::Sampler,Type::Vec2,Type::Float,Type::Float,Type::Bool],Type::Vec4).unwrap().call(&[texture.node(),texture.sampler(),coordinate,size,separation,premultiplied_alpha])
}

/// r186 lensflare ghost sampling. Render this at the chosen downsampled size;
/// options are (threshold, sample count, spacing, attenuation exponent).
pub fn lensflare(texture: Texture, coordinate: Node, tint: Node, options: Node) -> Node {
    WgslFn::new("tsl_lensflare",r#"
fn tsl_lensflare(t:texture_2d<f32>,s:sampler,p:vec2<f32>,tint:vec3<f32>,options:vec4<f32>)->vec4<f32>{
 let uv=vec2(1.0)-p;let ghost=(vec2(0.5)-uv)*options.z;var result=vec4(0.0,0.0,0.0,1.0);
 for(var i=0;i<i32(options.y);i++) {let sample_uv=fract(uv+ghost*f32(i));let weight=pow(1.0-distance(sample_uv,vec2(0.5)),options.w);
 let color=max(textureSampleLevel(t,s,sample_uv,0.0).rgb-vec3(options.x),vec3(0.0))*tint*weight;result+=vec4(color,1.0);}
 return result;
}"#,&[Type::Texture,Type::Sampler,Type::Vec2,Type::Vec3,Type::Vec4],Type::Vec4).unwrap().call(&[texture.node(),texture.sampler(),coordinate,tint,options])
}
