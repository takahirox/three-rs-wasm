// Three.js r186 webgpu_tsl_vfx_tornado: radial/skewed noise, GPU cylinder deformation.
fn tornado_position(p:vec3<f32>,params:vec4<f32>,time:f32,kind:f32)->vec3<f32>{
 let a=atan2(p.z,p.x);let e=p.y;let r=pow(params.y*(p.y-params.z),2.0)+params.w-select(0.0,0.05,kind<1.5)+sin((e-time*params.x)*20.0+a*2.0)*0.05;
 return vec3(cos(a)*r,e,sin(a)*r);
}
