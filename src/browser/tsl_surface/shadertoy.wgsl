// Fixed shaders from Three.js r186 webgpu_shadertoy.html.
// Water: jackdavenport, https://www.shadertoy.com/view/Mt2SzR
// Fire: trinketMage, https://www.shadertoy.com/view/3tcBzH
// The two fixed shaders in Three.js r186 webgpu_shadertoy.html.
// Water: https://www.shadertoy.com/view/Mt2SzR. Fire: pinned example's second shader.
fn water_noise(p:vec2<f32>)->f32{return fract(sin(p.x+p.y*10000.0)*10000.0);}
fn water_smooth(p:vec2<f32>)->f32{
 let f=smoothstep(vec2(0.0),vec2(1.0),fract(p));
 return mix(mix(water_noise(floor(p)),water_noise(vec2(ceil(p.x),floor(p.y))),f.x),mix(water_noise(vec2(floor(p.x),ceil(p.y))),water_noise(ceil(p)),f.x),f.y);
}
fn water_fractal(p:vec2<f32>)->f32{return (water_smooth(p)+water_smooth(p*2.0)*0.5+water_smooth(p*4.0)*0.25+water_smooth(p*8.0)*0.125+water_smooth(p*16.0)*0.0625)/1.9375;}
fn water_moving(p:vec2<f32>,t:f32)->f32{return water_fractal(p+vec2(water_fractal(p+t),water_fractal(p-t)));}
fn water_nested(p:vec2<f32>,t:f32)->f32{return water_moving(p+vec2(water_moving(p,t),water_moving(p+100.0,t)),t);}
fn fire_rand(p:vec2<f32>)->f32{return fract(sin(dot(p,vec2(12.9898,78.233)))*43758.5453);}
fn fire_noise(p:vec2<f32>,freq:f32)->f32{
 let v=p*freq;let i=floor(v);let j=floor(v+1.0);let f=fract(v);let h=f*f*(3.0-2.0*f);
 return mix(mix(fire_rand(i),fire_rand(vec2(j.x,i.y)),h.x),mix(fire_rand(vec2(i.x,j.y)),fire_rand(j),h.x),h.y);
}
fn fire_pnoise(p:vec2<f32>)->f32{
 var amplitude=1.0;var frequency=10.0;var total=0.0;var normalization=0.0;
 for(var i=0;i<5;i++){total+=fire_noise(p,frequency)*amplitude;normalization+=amplitude;frequency*=2.0;amplitude*=0.5;}return total/normalization;
}
fn toy(uv:vec2<f32>,size:vec2<f32>,t:f32)->vec4<f32>{
 let water=mix(vec3(0.4,0.6,1.0),vec3(0.1,0.2,1.0),water_nested(uv*6.0,t));
 let gradient=1.0-uv.y;let p=uv*size/size.x-vec2(0.0,t*0.3125);let n=fire_pnoise(p);
 let first=smoothstep(0.0,n,gradient);let dark=smoothstep(0.0,n,gradient-0.2);let middle=smoothstep(0.0,n,gradient-0.4);
 let brightColor=vec4(1.0,0.65,0.1,0.25);let darkColor=vec4(1.0,0.0,0.15,0.0625);
 let fire=mix(mix(brightColor,darkColor,first-dark),mix(brightColor,darkColor,0.5),dark-middle)*first;
 return mix(vec4(water,1.0),fire,sin((t*0.3+0.75)*6.283185307179586)*0.5+0.5);
}
