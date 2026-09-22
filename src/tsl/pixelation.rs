//! PixelationPassNode's depth/normal edge shading. Render the scene to a
//! low-resolution color + view-normal MRT, then upsample with nearest texels.
use super::*;
/// `texel_size` is the reciprocal low-resolution attachment size. The normal
/// input stores signed view normals; depth comes from `depth_effect`'s attachment.
pub fn pixelation(
    normal: Texture,
    texel_size: Node,
    normal_strength: Node,
    depth_strength: Node,
) -> Node {
    let coordinate = ((uv() / texel_size.clone()).floor() + float(0.5)) * texel_size.clone();
    let color = Texture::Input.sample(coordinate.clone());
    let depth = depth_texture(coordinate.clone());
    let raw_normal = normal.sample(coordinate.clone()).rgb();
    let has_normal = raw_normal.length().greater_than(float(0.));
    let n = raw_normal.normalize();
    let mut depth_sum = float(0.);
    let mut normal_sum = float(0.);
    for (x, y) in [(1., 0.), (-1., 0.), (0., 1.), (0., -1.)] {
        let p = coordinate.clone() + vec2(float(x), float(y)) * texel_size.clone();
        let diff = depth_texture(p.clone()) - depth.clone();
        let neighbor = normal.sample(p).rgb().normalize();
        depth_sum = depth_sum + diff.clone().clamp(float(0.), float(1.));
        let indicator = (n.clone() - neighbor.clone())
            .dot(vec3(float(1.), float(1.), float(1.)))
            .smoothstep(float(-0.01), float(0.01));
        normal_sum = normal_sum
            + (float(1.) - n.clone().dot(neighbor))
                * (diff * float(0.25) + float(0.0025))
                    .sign()
                    .clamp(float(0.), float(1.))
                * indicator;
    }
    let dei = depth_strength.clone().greater_than(float(0.)).select(
        (depth_sum.smoothstep(float(0.01), float(0.02)) * float(2.)).floor() / float(2.),
        float(0.),
    );
    let nei = normal_strength.clone().greater_than(float(0.)).select(
        normal_sum
            .less_than(float(0.1))
            .select(float(0.), float(1.)),
        float(0.),
    );
    let strength = dei.clone().greater_than(float(0.)).select(
        float(1.) - dei * depth_strength,
        float(1.) + has_normal.select(nei, float(0.)) * normal_strength,
    );
    vec4(color.rgb() * strength, color.swizzle("w"))
}
