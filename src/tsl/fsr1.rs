//! GPU FSR1 kernels. Run EASU into a full-resolution HDR target, then RCAS.
use super::*;
pub fn easu(texture: Texture, coordinate: Node) -> Node {
    WgslFn::new(
        "fsr_easu",
        include_str!("fsr1.wgsl"),
        &[Type::Texture, Type::Vec2],
        Type::Vec4,
    )
    .unwrap()
    .call(&[texture.node(), coordinate])
}
pub fn rcas(texture: Texture, coordinate: Node, sharpness: Node, denoise: Node) -> Node {
    WgslFn::new(
        "fsr_rcas",
        include_str!("fsr1.wgsl"),
        &[Type::Texture, Type::Vec2, Type::Float, Type::Bool],
        Type::Vec4,
    )
    .unwrap()
    .call(&[texture.node(), coordinate, sharpness, denoise])
}
