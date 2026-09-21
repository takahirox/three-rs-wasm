//! r186 weighted blended order-independent transparency (McGuire equation 9).
//! Render opaque geometry first, share its depth attachment, then draw the two
//! transparent MRT outputs with depth writes disabled. Accumulation clears to
//! zero and revealage to one. Composite in linear HDR before display encoding.
use super::*;
pub fn outputs() -> [Node; 2] {
    let alpha = output().swizzle("w");
    let z = view_z();
    let weight = alpha.clone()
        * (float(0.03) / ((z / float(200.0)).pow(float(4.0)) + float(1e-5)))
            .clamp(float(1e-2), float(3e3));
    [
        vec4(output().rgb() * alpha.clone(), alpha.clone()) * weight,
        splat(alpha, Type::Vec4),
    ]
}
pub fn blending() -> Vec<Option<wgpu::BlendState>> {
    let add = wgpu::BlendComponent {
        src_factor: wgpu::BlendFactor::One,
        dst_factor: wgpu::BlendFactor::One,
        operation: wgpu::BlendOperation::Add,
    };
    let reveal = wgpu::BlendComponent {
        src_factor: wgpu::BlendFactor::Zero,
        dst_factor: wgpu::BlendFactor::OneMinusSrc,
        operation: wgpu::BlendOperation::Add,
    };
    vec![
        Some(wgpu::BlendState {
            color: add,
            alpha: add,
        }),
        Some(wgpu::BlendState {
            color: reveal,
            alpha: reveal,
        }),
    ]
}
pub fn composite(beauty: Node, accumulation: Node, revealage: Node) -> Node {
    vec4(
        mix(
            accumulation.rgb() / accumulation.swizzle("w").max(float(1e-5)),
            beauty.rgb(),
            revealage,
        ),
        beauty.swizzle("w"),
    )
}
