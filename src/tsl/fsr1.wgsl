// r186 FSR1Node: EASU's 12-tap edge kernel and RCAS's 5-tap cross.
fn fsr_edge(w:f32,a:f32,b:f32,c:f32,d:f32,e:f32)->vec3<f32>{
 let x=d-b;let sx=clamp(abs(x)/max(max(abs(d-c),abs(c-b)),1.0/65536.0),0.0,1.0);
 let y=e-a;let sy=clamp(abs(y)/max(max(abs(e-c),abs(c-a)),1.0/65536.0),0.0,1.0);
 return vec3(x*w,y*w,(sx*sx+sy*sy)*w);
}
fn fsr_easu(t:texture_2d<f32>,p:vec2<f32>)->vec4<f32>{
 let pp=p*vec2<f32>(textureDimensions(t))-0.5;let ip=vec2<i32>(floor(pp));let f=fract(pp);
 let offsets=array<vec2<i32>,12>(vec2(0,-1),vec2(1,-1),vec2(-1,0),vec2(0,0),vec2(1,0),vec2(2,0),vec2(-1,1),vec2(0,1),vec2(1,1),vec2(2,1),vec2(0,2),vec2(1,2));
 var c:array<vec4<f32>,12>;var l:array<f32,12>;
 for(var i=0;i<12;i++){c[i]=textureLoad(t,ip+offsets[i],0);l[i]=c[i].r*0.5+c[i].g+c[i].b*0.5;}
 var edge=fsr_edge((1.0-f.x)*(1.0-f.y),l[0],l[2],l[3],l[4],l[7]);
 edge+=fsr_edge(f.x*(1.0-f.y),l[1],l[3],l[4],l[5],l[8]);
 edge+=fsr_edge((1.0-f.x)*f.y,l[3],l[6],l[7],l[8],l[10]);
 edge+=fsr_edge(f.x*f.y,l[4],l[7],l[8],l[9],l[11]);
 var dir=edge.xy;let dirSq=dot(dir,dir);let zero=dirSq<1.0/32768.0;
 dir.x=select(dir.x,1.0,zero);dir*=select(1.0/sqrt(max(dirSq,1.0/32768.0)),1.0,zero);
 let len=(edge.z*0.5)*(edge.z*0.5);let stretch=dot(dir,dir)/max(abs(dir.x),abs(dir.y));
 let len2=vec2(1.0+(stretch-1.0)*len,1.0-len*0.5);let lob=0.5+(0.25-0.04-0.5)*len;
 var sum=vec4(0.0);var weight=0.0;
 for(var i=0;i<12;i++){let o=vec2<f32>(offsets[i])-f;let v=vec2(o.x*dir.x+o.y*dir.y,-o.x*dir.y+o.y*dir.x)*len2;let d2=min(dot(v,v),1.0/lob);let wb=d2*(2.0/5.0)-1.0;let wa=d2*lob-1.0;let w=(wb*wb*(25.0/16.0)-(25.0/16.0-1.0))*(wa*wa);sum+=c[i]*w;weight+=w;}
 return clamp(sum/weight,min(min(c[3],c[4]),min(c[7],c[8])),max(max(c[3],c[4]),max(c[7],c[8])));
}
fn fsr_rcas(t:texture_2d<f32>,uv:vec2<f32>,sharpness:f32,denoise:bool)->vec4<f32>{
 let p=vec2<i32>(floor(uv*vec2<f32>(textureDimensions(t))));
 let e=textureLoad(t,p,0);let b=textureLoad(t,p+vec2(0,-1),0);let d=textureLoad(t,p+vec2(-1,0),0);let f=textureLoad(t,p+vec2(1,0),0);let h=textureLoad(t,p+vec2(0,1),0);
 let mn=min(min(b.rgb,d.rgb),min(f.rgb,h.rgb));let mx=max(max(b.rgb,d.rgb),max(f.rgb,h.rgb));
 let hitMin=min(mn,e.rgb)/(mx*4.0);let hitMax=(vec3(1.0)-max(mx,e.rgb))/(mn*4.0-4.0);let rgb=max(-hitMin,hitMax);
 var lobe=max(-0.1875,min(max(rgb.r,max(rgb.g,rgb.b)),0.0))*exp2(-sharpness);
 if denoise {let k=vec3(0.5,1.0,0.5);let bl=dot(b.rgb,k);let dl=dot(d.rgb,k);let el=dot(e.rgb,k);let fl=dot(f.rgb,k);let hl=dot(h.rgb,k);let nz=(bl+dl+fl+hl)*0.25-el;let range=max(max(bl,dl),max(el,max(fl,hl)))-min(min(bl,dl),min(el,min(fl,hl)));lobe*=1.0-clamp(abs(nz)/max(range,1.0/65536.0),0.0,1.0)*0.5;}
 return vec4(((b.rgb+d.rgb+f.rgb+h.rgb)*lobe+e.rgb)/(lobe*4.0+1.0),e.a);
}
