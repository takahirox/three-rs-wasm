//! Native WebGPU gather operations. Offsets/components are shader constants.
use super::*;
fn valid(offset: [i32; 2]) -> bool {
    offset.into_iter().all(|x| (-8..=7).contains(&x))
}
pub fn gather(texture: Texture, uv: Node, component: u32, offset: [i32; 2]) -> Result<Node> {
    if component > 3 || !valid(offset) {
        return Err(Error::Invalid("TSL gather component/offset"));
    }
    let name = format!("tsl_gather_{component}_{}_{}", offset[0] + 8, offset[1] + 8);
    let source = format!(
        "fn {name}(t:texture_2d<f32>,s:sampler,p:vec2<f32>)->vec4<f32>{{return textureGather({component},t,s,p,vec2<i32>({},{}));}}",
        offset[0], offset[1]
    );
    Ok(WgslFn::new(
        &name,
        &source,
        &[Type::Texture, Type::Sampler, Type::Vec2],
        Type::Vec4,
    )?
    .call(&[texture.node(), texture.sampler(), uv]))
}
pub fn gather_compare(
    texture: Texture,
    uv: Node,
    reference: Node,
    offset: [i32; 2],
) -> Result<Node> {
    if !valid(offset) {
        return Err(Error::Invalid("TSL gather comparison offset"));
    }
    let name = format!("tsl_gather_compare_{}_{}", offset[0] + 8, offset[1] + 8);
    let source = format!(
        "fn {name}(t:texture_depth_2d,s:sampler_comparison,p:vec2<f32>,r:f32)->vec4<f32>{{return textureGatherCompare(t,s,p,r,vec2<i32>({},{}));}}",
        offset[0], offset[1]
    );
    Ok(WgslFn::new(
        &name,
        &source,
        &[
            Type::DepthTexture,
            Type::ComparisonSampler,
            Type::Vec2,
            Type::Float,
        ],
        Type::Vec4,
    )?
    .call(&[texture.node(), texture.sampler(), uv, reference]))
}

/// Sample a resident cubemap in its native GPU cube topology.
pub fn cube(texture: Texture, direction: Node) -> Node {
    WgslFn::new("tsl_cube_sample", "fn tsl_cube_sample(t:texture_cube<f32>,s:sampler,d:vec3<f32>)->vec4<f32>{return textureSample(t,s,d);}", &[Type::TextureCube,Type::Sampler,Type::Vec3],Type::Vec4).expect("static cube function").call(&[texture.node(),texture.sampler(),direction])
}

/// Linearly filtered three-dimensional texture sampling in a fragment graph.
pub fn volume(texture: Texture, coordinate: Node) -> Node {
    WgslFn::new("tsl_sample_volume", "fn tsl_sample_volume(t:texture_3d<f32>,s:sampler,p:vec3<f32>)->vec4<f32>{return textureSample(t,s,p);}", &[Type::Texture3D,Type::Sampler,Type::Vec3], Type::Vec4).unwrap().call(&[texture.node(),texture.sampler(),coordinate])
}
