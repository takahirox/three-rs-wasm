//! GPU PMREM sampling and direction corrections for custom PBR environments.
use super::*;

pub fn pmrem(texture: Texture, direction: Node, roughness: Node, max_mip: Node) -> Node {
    WgslFn::new("tsl_pmrem", r#"
fn tsl_pmrem(map:texture_2d<f32>,s:sampler,direction:vec3<f32>,roughness:f32,max_mip:f32)->vec3<f32>{
 let d=vec3(direction.x,-direction.y,direction.z);let mip=clamp(roughness_mip(roughness),-2.0,max_mip);let lo=floor(mip);
 return mix(textureSampleLevel(map,s,cube_uv(d,lo,max_mip),0.0).rgb,textureSampleLevel(map,s,cube_uv(d,lo+1.0,max_mip),0.0).rgb,fract(mip));
}"#, &[Type::Texture,Type::Sampler,Type::Vec3,Type::Float,Type::Float],Type::Vec3).unwrap().call(&[texture.node(),texture.sampler(),direction,roughness,max_mip])
}

/// r186 getParallaxCorrectNormal: intersect the reflected ray with a box.
pub fn parallax_correct(direction: Node, position: Node, size: Node, center: Node) -> Node {
    WgslFn::new("tsl_parallax_correct",r#"
fn tsl_parallax_correct(direction:vec3<f32>,position:vec3<f32>,size:vec3<f32>,center:vec3<f32>)->vec3<f32>{
 let n=normalize(direction);let first=(0.5*size+center-position)/n;let second=(-0.5*size+center-position)/n;
 let far=select(second,first,n>vec3(0.0));let distance=min(min(far.x,far.y),far.z);return position+n*distance-center;
}"#,&[Type::Vec3,Type::Vec3,Type::Vec3,Type::Vec3],Type::Vec3).unwrap().call(&[direction,position,size,center])
}

/// Resolved material reflection in world space, without roughness bending.
/// Available in environment and post-lighting surface graphs.
pub fn reflect_vector() -> Node {
    WgslFn::new("tsl_reflect_world",r#"
fn tsl_reflect_world(n:vec3<f32>,v:vec3<f32>)->vec3<f32>{return normalize((transpose(u.view)*vec4(reflect(-v,n),0.0)).xyz);}
"#,&[Type::Vec3,Type::Vec3],Type::Vec3).unwrap().call(&[normal_view(),position_view_direction()])
}
/// Resolved normal including the material's normal/bump map and face orientation.
pub fn material_normal_world() -> Node {
    WgslFn::new("tsl_material_normal_world",r#"
fn tsl_material_normal_world(n:vec3<f32>)->vec3<f32>{return normalize((transpose(u.view)*vec4(n,0.0)).xyz);}
"#,&[Type::Vec3],Type::Vec3).unwrap().call(&[normal_view()])
}

/// r186 GroundedSkybox: intersect the camera ray with a sphere clipped by a disk.
pub fn ground_projected_normal(position: Node, camera: Node, radius: Node, height: Node) -> Node {
    WgslFn::new("tsl_ground_projected",r#"
fn tsl_ground_projected(position:vec3<f32>,camera:vec3<f32>,radius:f32,height:f32)->vec3<f32>{
 let p=normalize(position-camera);let cam=camera-vec3(0.0,height,0.0);let b=dot(cam,p);let c=dot(cam,cam)-radius*radius;let h=b*b-c;
 var projected=vec3(0.0,1.0,0.0);if h>=0.0{let intersection=sqrt(h)-b;if intersection>0.0{var disk=1e6;if p.y<=0.0{let o=cam+vec3(0.0,height,0.0);let t=-o.y/p.y;let q=o+p*t;if dot(q,q)<radius*radius{disk=t;}}projected=(cam+p*min(intersection,disk))/radius;}}return projected;
}"#,&[Type::Vec3,Type::Vec3,Type::Float,Type::Float],Type::Vec3).unwrap().call(&[position,camera,radius,height])
}
