//! Fixed-cost GPU box raymarchers corresponding to r186's volume examples.
use super::*;
/// `options` is (threshold, steps, refine, unused). Origin is in object space.
pub fn opaque(texture: Texture, origin: Node, position: Node, options: Node) -> Node {
    WgslFn::new(
        "tsl_volume_opaque",
        &format!(
            "{}\n{}",
            include_str!("volume_common.wgsl"),
            include_str!("volume_opaque.wgsl")
        ),
        &[
            Type::Texture3D,
            Type::Sampler,
            Type::Vec3,
            Type::Vec3,
            Type::Vec4,
        ],
        Type::Vec4,
    )
    .unwrap()
    .call(&[texture.node(), texture.sampler(), origin, position, options])
}
/// `options` is (threshold, opacity, range, steps).
pub fn cloud(texture: Texture, origin: Node, position: Node, options: Node) -> Node {
    WgslFn::new(
        "tsl_volume_cloud",
        &format!(
            "{}\n{}",
            include_str!("volume_common.wgsl"),
            include_str!("volume_cloud.wgsl")
        ),
        &[
            Type::Texture3D,
            Type::Sampler,
            Type::Vec3,
            Type::Vec3,
            Type::Vec4,
        ],
        Type::Vec4,
    )
    .unwrap()
    .call(&[texture.node(), texture.sampler(), origin, position, options])
}

/// Animated compute-volume variant of the cloud shader (same GPU ray integration).
pub fn computed_cloud(texture: Texture, origin: Node, position: Node, options: Node) -> Node {
    let code = include_str!("volume_cloud.wgsl")
        .replace("tsl_volume_cloud", "tsl_volume_computed_cloud")
        .replace(
            "shading*3.0+(ray.x+ray.y)*0.25+0.2",
            "shading*4.0+(ray.x+ray.y)*0.5+0.3",
        );
    WgslFn::new(
        "tsl_volume_computed_cloud",
        &format!("{}\n{}", include_str!("volume_common.wgsl"), code),
        &[
            Type::Texture3D,
            Type::Sampler,
            Type::Vec3,
            Type::Vec3,
            Type::Vec4,
        ],
        Type::Vec4,
    )
    .unwrap()
    .call(&[texture.node(), texture.sampler(), origin, position, options])
}
