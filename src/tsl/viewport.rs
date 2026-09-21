//! Fragment-stage framebuffer sampling. The renderer snapshots resident GPU
//! attachments immediately before the material's draw, preserving transparent order.
use super::*;
pub fn screen_uv() -> Node {
    WgslFn::new(
        "viewport_uv",
        "fn viewport_uv()->vec2<f32>{return fragment_surface.clip.xy/u.point.xy;}",
        &[],
        Type::Vec2,
    )
    .unwrap()
    .call(&[])
}
pub fn color(coordinate: Node) -> Node {
    WgslFn::new("viewport_read_color", "fn viewport_read_color(p:vec2<f32>)->vec4<f32>{let size=vec2<i32>(textureDimensions(viewport_color));return textureLoad(viewport_color,clamp(vec2<i32>(floor(p*vec2<f32>(size))),vec2(0),size-1),0);}",&[Type::Vec2],Type::Vec4).unwrap().call(&[coordinate])
}
pub fn depth(coordinate: Node) -> Node {
    WgslFn::new("viewport_read_depth", "fn viewport_read_depth(p:vec2<f32>)->f32{let size=vec2<i32>(textureDimensions(viewport_depth));return textureLoad(viewport_depth,clamp(vec2<i32>(p*vec2<f32>(size)),vec2(0),size-1),0).r;}",&[Type::Vec2],Type::Float).unwrap().call(&[coordinate])
}
pub fn perspective_depth_to_view_z(depth: Node, near: Node, far: Node) -> Node {
    (near.clone() * far.clone()) / ((far.clone() - near) * depth - far)
}
/// Fade intersections using NVIDIA's symmetric soft-particle contrast curve.
pub fn soft_particles(
    opacity: Node,
    distance: Node,
    contrast: Node,
    near: Node,
    far: Node,
) -> Node {
    let scene_z = perspective_depth_to_view_z(depth(screen_uv()), near, far);
    let gap = ((-view_z() - scene_z) / distance).clamp(float(0.0), float(1.0));
    WgslFn::new("soft_particle_contrast", "fn soft_particle_contrast(x:f32,p:f32)->f32{let above=x>0.5;let folded=select(x,1.0-x,above);let y=0.5*pow(clamp(folded*2.0,0.0,1.0),p);return select(y,1.0-y,above);}",&[Type::Float,Type::Float],Type::Float).unwrap().call(&[gap,contrast])*opacity
}
/// Original 45-tap stochastic blur of the shared viewport, evaluated on the GPU.
pub fn hash_blur(coordinate: Node, amount: Node) -> Node {
    WgslFn::new("viewport_read_color_hash_blur",r#"
fn viewport_hash_mod(x:f32,y:f32)->f32{return x-y*floor(x/y);}
fn viewport_read_color_hash_blur(p:vec2<f32>,amount:f32)->vec4<f32>{
 var sum=vec4(0.0);for(var i=0.0;i<45.0;i+=1.0){let angle=i/45.0*6.283185307179586;
 let dt=dot(vec2(i,p.x+p.y),vec2(12.9898,78.233));let sn=viewport_hash_mod(dt,3.141592653589793);
 let rnd=fract(sin(sn)*43758.5453);let q=vec2(cos(angle),sin(angle))*(rnd+amount);
 let size=vec2<i32>(textureDimensions(viewport_color));sum+=textureLoad(viewport_color,clamp(vec2<i32>(floor((p+q*amount)*vec2<f32>(size))),vec2(0),size-1),0);
 }return sum/45.0;
}"#,&[Type::Vec2,Type::Float],Type::Vec4).unwrap().call(&[coordinate,amount])
}
/// Reject warped coordinates that would refract a surface in front of this one.
pub fn safe_uv(coordinate: Node) -> Node {
    let local_depth = WgslFn::new(
        "viewport_fragment_depth",
        "fn viewport_fragment_depth()->f32{return fragment_surface.clip.z;}",
        &[],
        Type::Float,
    )
    .unwrap()
    .call(&[]);
    depth(coordinate.clone())
        .less_than(local_depth)
        .select(screen_uv(), coordinate)
}
