// Three.js r186 FXAANode (MIT). Preserve early exit and bounded adaptive search.
fn tsl_fxaa_sample(t:texture_2d<f32>,s:sampler,uv:vec2<f32>)->vec4<f32>{
    return textureSampleLevel(t,s,uv,0.0);
}
fn tsl_fxaa_luma(t:texture_2d<f32>,s:sampler,uv:vec2<f32>)->f32{
    return dot(tsl_fxaa_sample(t,s,uv).rgb,vec3(0.3,0.59,0.11));
}
fn tsl_fxaa(t:texture_2d<f32>,samp:sampler,coordinate:vec2<f32>,size:vec2<f32>)->vec4<f32>{
    let uv=coordinate;
    let m=tsl_fxaa_luma(t,samp,uv);
    let n=tsl_fxaa_luma(t,samp,uv+size*vec2(0.0,-1.0));
    let e=tsl_fxaa_luma(t,samp,uv+size*vec2(1.0,0.0));
    let s=tsl_fxaa_luma(t,samp,uv+size*vec2(0.0,1.0));
    let w=tsl_fxaa_luma(t,samp,uv+size*vec2(-1.0,0.0));
    let highest=max(s,max(e,max(n,max(w,m))));let lowest=min(s,min(e,min(n,min(w,m))));
    let contrast=highest-lowest;
    if contrast<max(0.0312,0.063*highest){return tsl_fxaa_sample(t,samp,uv);}
    let ne=tsl_fxaa_luma(t,samp,uv+size*vec2(1.0,-1.0));
    let nw=tsl_fxaa_luma(t,samp,uv+size*vec2(-1.0,-1.0));
    let se=tsl_fxaa_luma(t,samp,uv+size*vec2(1.0,1.0));
    let sw=tsl_fxaa_luma(t,samp,uv+size*vec2(-1.0,1.0));
    let f=clamp(abs((2.0*(s+e+n+w)+(se+sw+ne+nw))/12.0-m)/max(contrast,0.0),0.0,1.0);
    let blend_factor=smoothstep(0.0,1.0,f);
    let pixel_blend=blend_factor*blend_factor;
    let horizontal=abs(s+n-m*2.0)*2.0+(abs(se+ne-e*2.0)+abs(sw+nw-w*2.0));
    let vertical=abs(e+w-m*2.0)*2.0+(abs(se+sw-s*2.0)+abs(ne+nw-n*2.0));
    let is_horizontal=horizontal>=vertical;
    let pl=select(e,s,is_horizontal);let nl=select(w,n,is_horizontal);
    let pg=abs(pl-m);let ng=abs(nl-m);
    var pixel_step=select(size.x,size.y,is_horizontal);
    var opposite=pl;var gradient=pg;
    if pg<ng{pixel_step=-pixel_step;opposite=nl;gradient=ng;}
    let edge_uv=uv+select(vec2(pixel_step*0.5,0.0),vec2(0.0,pixel_step*0.5),is_horizontal);
    let step_uv=select(vec2(0.0,size.y),vec2(size.x,0.0),is_horizontal);
    let edge_luma=(m+opposite)*0.5;let threshold=gradient*0.25;
    let steps=array<f32,6>(1.0,1.5,2.0,2.0,2.0,4.0);
    var puv=edge_uv+step_uv;var pd=tsl_fxaa_luma(t,samp,puv)-edge_luma;var pend=abs(pd)>=threshold;
    for(var i=1;i<6;i++){if pend{break;}puv+=step_uv*steps[i];pd=tsl_fxaa_luma(t,samp,puv)-edge_luma;pend=abs(pd)>=threshold;}
    if !pend{puv+=step_uv*8.0;}
    var nuv=edge_uv-step_uv;var nd=tsl_fxaa_luma(t,samp,nuv)-edge_luma;var nend=abs(nd)>=threshold;
    for(var i=1;i<6;i++){if nend{break;}nuv-=step_uv*steps[i];nd=tsl_fxaa_luma(t,samp,nuv)-edge_luma;nend=abs(nd)>=threshold;}
    if !nend{nuv-=step_uv*8.0;}
    let p_distance=select(puv.y-uv.y,puv.x-uv.x,is_horizontal);
    let n_distance=select(uv.y-nuv.y,uv.x-nuv.x,is_horizontal);
    let shortest=select(n_distance,p_distance,p_distance<=n_distance);
    let delta_sign=select(nd>=0.0,pd>=0.0,p_distance<=n_distance);
    var edge_blend=0.0;
    if delta_sign != (m-edge_luma>=0.0){edge_blend=0.5-shortest/(p_distance+n_distance);}
    let blend=max(pixel_blend,edge_blend);
    return tsl_fxaa_sample(t,samp,uv+select(vec2(pixel_step*blend,0.0),vec2(0.0,pixel_step*blend),is_horizontal));
}
