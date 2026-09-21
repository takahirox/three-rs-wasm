// r186 getAlphaHashThreshold, adapted from Wyman 2017 hashed alpha testing.
fn alpha_hash2(v:vec2<f32>)->f32{return fract(1.0e4*sin(17.0*v.x+0.1*v.y)*(0.1+abs(sin(13.0*v.y+v.x))));}
fn alpha_hash3(v:vec3<f32>)->f32{return alpha_hash2(vec2(alpha_hash2(v.xy),v.z));}
fn tsl_alpha_hash(opacity:f32,position:vec3<f32>,enabled:bool)->f32{
 if !enabled{return opacity;}
 let derivative=max(length(dpdx(position)),length(dpdy(position)));let pix_scale=1.0/(0.05*derivative);let log_scale=log2(pix_scale);
 let scales=vec2(exp2(floor(log_scale)),exp2(ceil(log_scale)));let alpha=vec2(alpha_hash3(floor(scales.x*position)),alpha_hash3(floor(scales.y*position)));
 let t=fract(log_scale);let x=(1.0-t)*alpha.x+t*alpha.y;let a=min(t,1.0-t);
 let cases=vec3(x*x/(2.0*a*(1.0-a)),(x-0.5*a)/(1.0-a),1.0-(1.0-x)*(1.0-x)/(2.0*a*(1.0-a)));
 let threshold=clamp(select(cases.z,select(cases.y,cases.x,x<a),x<1.0-a),1e-6,1.0);
 if opacity<threshold{discard;}return opacity;
}
