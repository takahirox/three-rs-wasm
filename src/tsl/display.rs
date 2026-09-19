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
