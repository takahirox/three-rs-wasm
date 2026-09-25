override INSTANCED:bool=false;
override ENCODE_SRGB:bool=false;
// Specialize material families so simple lit meshes do not execute PBR extensions.
override MATERIAL_KIND:f32=-1.0;
override PHYSICAL:bool=true;
override EXTENSION_MAP_MASK:u32=0u;
override LIGHT_COUNT:i32=-1;
override LIGHT_TYPES:u32=0u;
fn light_count()->u32 {if LIGHT_COUNT<0 {return u32(u.material.w);}return u32(LIGHT_COUNT);}
fn light_type(i:u32)->f32 {if LIGHT_COUNT<0 {return u.light_position[i].w;}return f32((LIGHT_TYPES>>(i*3u))&7u);}
override COLOR_MAP:bool=true;
override MR_MAP:bool=true;
override NORMAL_MAP:bool=true;
override AO_MAP:bool=true;
override EMISSIVE_MAP:bool=true;
override RECEIVE_SHADOW:bool=true;
fn material_kind()->f32 {if MATERIAL_KIND<0.0 {return u.material.x;}return MATERIAL_KIND;}
// Specialize away discard for opaque draws: helper-lane derivatives must remain intact.
override ALPHA_MASK: bool = false;
override CLIPPING:bool=false;
override LINE_DASH:bool=false;
struct Uniforms {
    mvp: mat4x4<f32>, model: mat4x4<f32>, normal: mat4x4<f32>, view:mat4x4<f32>, projection:mat4x4<f32>,
    color: vec4<f32>, camera: vec4<f32>, material: vec4<f32>, emissive: vec4<f32>, ambient: vec4<f32>, point:vec4<f32>, pbr:vec4<f32>, environment:vec4<f32>, maps:vec4<f32>,
    light_position: array<vec4<f32>,8>, light_color: array<vec4<f32>,8>, light_params: array<vec4<f32>,8>, light_direction:array<vec4<f32>,8>,
    specular:vec4<f32>, flags:vec4<f32>, fog_color:vec4<f32>, fog_params:vec4<f32>,
    shadow_matrices:array<mat4x4<f32>,48>,shadow_params:array<vec4<f32>,8>,shadow_filters:array<vec4<f32>,8>,shadow_cascades:array<vec4<f32>,16>,custom:array<vec4<f32>,16>,clipping_planes:array<vec4<f32>,16>,clipping_params:vec4<f32>,physical:array<vec4<f32>,4>,uv_transforms:array<vec4<f32>,15>,transmission:array<vec4<f32>,3>,extension_matrices:array<vec4<f32>,36>,extension_sizes:array<vec4<f32>,12>,extension_wraps:array<vec4<f32>,12>,extension_sampling:array<vec4<f32>,12>,coat_normal:vec4<f32>,iridescence:vec4<f32>,line:array<vec4<f32>,2>,output:vec4<f32>,
};
@group(0) @binding(0) var<uniform> u: Uniforms;
@group(0) @binding(1) var color_map: texture_2d<f32>;
@group(0) @binding(2) var color_sampler: sampler;
@group(0) @binding(3) var mr_map: texture_2d<f32>;
@group(0) @binding(4) var mr_sampler: sampler;
@group(0) @binding(5) var normal_map: texture_2d<f32>;
@group(0) @binding(6) var normal_sampler: sampler;
@group(0) @binding(7) var ao_map: texture_2d<f32>;
@group(0) @binding(8) var ao_sampler: sampler;
@group(0) @binding(9) var emissive_map: texture_2d<f32>;
@group(0) @binding(10) var emissive_sampler: sampler;
@group(0) @binding(11) var environment_map: texture_2d<f32>;
@group(0) @binding(12) var environment_source: texture_2d<f32>;
@group(0) @binding(13) var environment_sampler: sampler;
@group(0) @binding(14) var dfg_map: texture_2d<f32>;
@group(0) @binding(15) var shadow_atlas:texture_depth_2d_array;
@group(0) @binding(16) var shadow_sampler:sampler_comparison;
@group(0) @binding(24) var ltc_sampler:sampler;
@group(0) @binding(17) var ltc_tables:texture_2d_array<f32>;
@group(0) @binding(18) var transmission_map:texture_2d<f32>;
@group(0) @binding(19) var transmission_sampler:sampler;
@group(0) @binding(20) var extension_maps:texture_2d_array<f32>;
@group(0) @binding(25) var viewport_color: texture_2d<f32>;
@group(0) @binding(26) var viewport_depth: texture_2d<f32>;
fn extension_uv(index:u32,surface:VertexOut)->vec2<f32> {
    let t=u.extension_matrices;let i=index*3u;
    let uv=select(surface.uv,surface.uv1,t[i+2u].w>0.5);
    return (mat3x3(t[i].xyz,t[i+1u].xyz,t[i+2u].xyz)*vec3(uv,1.0)).xy;
}
fn wrap_texel(coordinate:i32,size:i32,mode:f32)->i32 {
    if mode==0.0 {return clamp(coordinate,0,size-1);}
    if mode==1.0 {return ((coordinate%size)+size)%size;}
    let p=((coordinate%(size*2))+size*2)%(size*2);
    return select(p,size*2-1-p,p>=size);
}
// Derivatives are evaluated before material-dependent fragment control flow.
var<private> extension_lod:array<f32,12>;
fn extension_texel(index:u32,p:vec2<i32>,level:i32)->vec4<f32> {
    let info=u.extension_sizes[index];let size=max(vec2(1.0),floor(info.xy/exp2(f32(level))));let mode=u.extension_wraps[index];
    let value=textureLoad(extension_maps,vec2(wrap_texel(p.x,i32(size.x),mode.x),wrap_texel(p.y,i32(size.y),mode.y)),i32(u.extension_sampling[index].w),level);
    if info.w>0.5 {
        let linear=select(value.rgb/12.92,pow((value.rgb+0.055)/1.055,vec3(2.4)),value.rgb>vec3(0.04045));
        return vec4(linear,value.a);
    }
    return value;
}
fn extension_level(index:u32,uv:vec2<f32>,level:i32,linear:bool)->vec4<f32> {
    let size=max(vec2(1.0),floor(u.extension_sizes[index].xy/exp2(f32(level))));let pixel=uv*size;
    if !linear {return extension_texel(index,vec2<i32>(floor(pixel)),level);}
    let base=vec2<i32>(floor(pixel-0.5));let f=fract(pixel-0.5);
    return mix(mix(extension_texel(index,base,level),extension_texel(index,base+vec2(1,0),level),f.x),
        mix(extension_texel(index,base+vec2(0,1),level),extension_texel(index,base+vec2(1,1),level),f.x),f.y);
}
fn extension_sample(index:u32,surface:VertexOut)->vec4<f32> {
    if (EXTENSION_MAP_MASK&(1u<<index))==0u {return vec4(1.0);}
    let size=u.extension_sizes[index];
    let sampling=u.extension_sampling[index];let uv=extension_uv(index,surface);let lod=extension_lod[index];
    let linear=select(size.z,sampling.x,lod>0.0)>0.5;
    let mip=clamp(lod,0.0,sampling.z);
    if sampling.y<0.5 {return extension_level(index,uv,i32(floor(mip+0.5)),linear);}
    let low=floor(mip);return mix(extension_level(index,uv,i32(low),linear),extension_level(index,uv,i32(ceil(mip)),linear),mip-low);
}
// Thin-film model ported from Three.js r186 (MIT), Belcour/Barla 2017.
fn sensitivity(opd:f32,shift:vec3<f32>)->vec3<f32> {
    let phase=6.28318530718*opd*1e-9;
    let value=vec3(5.4856e-13,4.4201e-13,5.2481e-13);
    let pos=vec3(1.6810e6,1.7953e6,2.2084e6);
    let variance=vec3(4.3278e9,9.3046e9,6.6121e9);
    var xyz=value*sqrt(6.28318530718*variance)*cos(pos*phase+shift)*exp(-phase*phase*variance);
    xyz.x+=9.7470e-14*sqrt(6.28318530718*4.5282e9)*cos(2.2399e6*phase+shift.x)*exp(-4.5282e9*phase*phase);
    return mat3x3(vec3(3.2404542,-0.9692660,0.0556434),vec3(-1.5371385,1.8760108,-0.2040259),vec3(-0.4985314,0.0415560,1.0572252))*(xyz/1.0685e-7);
}
fn iridescent(eta:f32,cos1:f32,thickness:f32,base_f0:vec3<f32>)->vec3<f32> {
    let ior=mix(1.0,eta,smoothstep(0.0,0.03,thickness));
    let cos2sq=1.0-(1.0-cos1*cos1)/(ior*ior);
    if cos2sq<0.0 {return vec3(1.0);}
    let cos2=sqrt(cos2sq);
    let ratio=(ior-1.0)/(ior+1.0);let r0=ratio*ratio;
    let r12=r0+(1.0-r0)*exp2((-5.55473*cos1-6.98316)*cos1);
    let transmission=1.0-r12;let phi21=select(3.14159265359,0.0,ior<1.0);
    let root=sqrt(clamp(base_f0,vec3(0.0),vec3(0.9999)));
    let base_ior=(vec3(1.0)+root)/(vec3(1.0)-root);
    let r=(base_ior-ior)/(base_ior+ior);let r1=r*r;
    let r23=r1+(vec3(1.0)-r1)*exp2((-5.55473*cos2-6.98316)*cos2);
    let phi=vec3(phi21)+select(vec3(0.0),vec3(3.14159265359),base_ior<vec3(ior));
    let opd=2.0*ior*thickness*cos2;
    let r123=clamp(r12*r23,vec3(1e-5),vec3(0.9999));let root123=sqrt(r123);
    let rs=transmission*transmission*r23/(vec3(1.0)-r123);
    var result=vec3(r12)+rs;var cm=rs-transmission;
    for(var m=1;m<=2;m++) {cm*=root123;result+=cm*2.0*sensitivity(f32(m)*opd,f32(m)*phi);}
    return max(result,vec3(0.0));
}
fn fresnel_to_f0(f:vec3<f32>,nv:f32)->vec3<f32> {
    let x=clamp(1.0-nv,0.0,1.0);let x5=clamp(x*x*x*x*x,0.0,0.9999);
    return (f-x5)/(1.0-x5);
}
// Mipped bicubic B-spline filtering, matching Three.js TextureBicubic.
fn cubic_weights(a:vec2<f32>)->mat4x2<f32> {
    return mat4x2((a*(a*(-a+3.0)-3.0)+1.0)/6.0,
        (a*a*(3.0*a-6.0)+4.0)/6.0,
        (a*(a*(-3.0*a+3.0)+3.0)+1.0)/6.0,a*a*a/6.0);
}
fn transmission_bicubic(uv:vec2<f32>,level:i32)->vec3<f32> {
    let size=vec2<f32>(textureDimensions(transmission_map,level));
    let pixel=uv*size+0.5;let base=floor(pixel);let w=cubic_weights(fract(pixel));
    let g0=w[0]+w[1];let g1=w[2]+w[3];
    let p0=(base-1.0+w[1]/g0-0.5)/size;let p1=(base+1.0+w[3]/g1-0.5)/size;
    return g0.y*(g0.x*textureSampleLevel(transmission_map,transmission_sampler,p0,f32(level)).rgb+g1.x*textureSampleLevel(transmission_map,transmission_sampler,vec2(p1.x,p0.y),f32(level)).rgb)
        +g1.y*(g0.x*textureSampleLevel(transmission_map,transmission_sampler,vec2(p0.x,p1.y),f32(level)).rgb+g1.x*textureSampleLevel(transmission_map,transmission_sampler,p1,f32(level)).rgb);
}
fn transmitted(surface:VertexOut,n:vec3<f32>,v:vec3<f32>,ior:f32,roughness:f32)->vec3<f32> {
    let world_normal=(transpose(u.view)*vec4(n,0.0)).xyz;
    let world_view=(transpose(u.view)*vec4(v,0.0)).xyz;
    let ray=normalize(refract(-world_view,world_normal,1.0/ior))*u.transmission[0].y*extension_sample(9u,surface).g*vec3(length(u.model[0].xyz),length(u.model[1].xyz),length(u.model[2].xyz));
    let clip=u.projection*u.view*vec4(surface.position+ray,1.0);
    let screen=clip.xy/clip.w*0.5+0.5;
    let uv=u.transmission[2].xy+vec2(screen.x,1.0-screen.y)*u.transmission[2].zw;
    let size=vec2<f32>(textureDimensions(transmission_map));
    let lod=clamp(log2(size.x*u.transmission[2].z)*roughness*clamp(ior*2.0-2.0,0.0,1.0),0.0,f32(textureNumLevels(transmission_map)-1u));
    var color=mix(transmission_bicubic(uv,i32(floor(lod))),transmission_bicubic(uv,i32(ceil(lod))),fract(lod));
    if u.transmission[0].z<1e30 {
        color*=pow(max(u.transmission[1].rgb,vec3(0.000001)),vec3(length(ray)/u.transmission[0].z));
    }
    return color;
}
fn ltc_uv(n:vec3<f32>,v:vec3<f32>,roughness:f32)->vec2<f32> {return vec2(roughness,sqrt(1.0-clamp(dot(n,v),0.0,1.0)))*(63.0/64.0)+0.5/64.0;}
fn ltc_edge(a:vec3<f32>,b:vec3<f32>)->vec3<f32> {
 let x=dot(a,b);let y=abs(x);let v=(0.8543985+(0.4965155+0.0145206*y)*y)/(3.417594+(4.1616724+y)*y);
 return cross(a,b)*select(0.5*inverseSqrt(max(1.0-x*x,1e-7))-v,v,x>0.0);
}
fn ltc_evaluate(n:vec3<f32>,v:vec3<f32>,p:vec3<f32>,inverse:mat3x3<f32>,rect:array<vec3<f32>,4>)->f32 {
 if dot(cross(rect[1]-rect[0],rect[3]-rect[0]),p-rect[0])<0.0 {return 0.0;}
 var tangent=v-n*dot(v,n);if dot(tangent,tangent)<1e-10 {tangent=cross(n,select(vec3(0.0,0.0,1.0),vec3(0.0,1.0,0.0),abs(n.z)>0.9));}tangent=normalize(tangent);
 let basis=inverse*transpose(mat3x3(tangent,-cross(n,tangent),n));
 let a=normalize(basis*(rect[0]-p));let b=normalize(basis*(rect[1]-p));let c=normalize(basis*(rect[2]-p));let d=normalize(basis*(rect[3]-p));
 let form=ltc_edge(a,b)+ltc_edge(b,c)+ltc_edge(c,d)+ltc_edge(d,a);let length=length(form);
 return max((length*length+form.z)/(length+1.0),0.0);
}
fn shadow_map_uv(i:u32,uv:vec2<f32>)->vec2<f32>{let halfTexel=0.5/vec2<f32>(textureDimensions(shadow_atlas));return clamp(uv,halfTexel,vec2(u.shadow_filters[i].z)-halfTexel);}
fn shadow_visibility(i:u32,position:vec3<f32>,normal:vec3<f32>)->f32 {
    let settings=u.shadow_params[i];
    if settings.x==0.0 || u.flags.y==0.0 {return 1.0;}
    var layer=u32(settings.x)-1u;
    if settings.w>1.5 {return sun_shadow_visibility(i,position,normal);}
    if settings.w>0.5 {
        let d=position-u.light_position[i].xyz;let a=abs(d);
        if a.x>=a.y && a.x>=a.z {layer+=select(1u,0u,d.x>=0.0);}
        else if a.y>=a.z {layer+=select(3u,2u,d.y>=0.0);}
        else {layer+=select(5u,4u,d.z>=0.0);}
    }
    let projected=u.shadow_matrices[layer]*vec4(position+normal*settings.z,1.0);
    let ndc=projected.xyz/projected.w;var uv=ndc.xy*vec2(0.5,-0.5)+0.5;
    if projected.w<=0.0 || any(uv<vec2(0.0)) || any(uv>vec2(1.0)) || ndc.z<0.0 || ndc.z>1.0 {return 1.0;}
    uv*=u.shadow_filters[i].z;
    let size=vec2<f32>(textureDimensions(shadow_atlas));
    if u.shadow_filters[i].y>0.5 {return textureSampleCompareLevel(shadow_atlas,shadow_sampler,shadow_map_uv(i,(floor(uv*size)+0.5)/size),i32(layer),ndc.z+settings.y);}
    let phi=fract(52.9829189*fract(dot(fragment_surface.clip.xy,vec2(0.06711056,0.00583715))))*6.28318530718;
    var value=0.0;
    if settings.w>0.5 {
        // r186 getPointShadow: Vogel-disk offsets in the tangent frame of the light-to-fragment
        // direction, radius / mapSize apart, each looked up in its own cube face against the
        // fragment's face depth.
        let base=u32(settings.x)-1u;
        let light=u.light_position[i].xyz;
        let bd=normalize(position+normal*settings.z-light);let ad=abs(bd);
        let tangent=normalize(cross(bd,select(vec3(1.0,0.0,0.0),vec3(0.0,1.0,0.0),ad.x>ad.z)));
        let bitangent=cross(bd,tangent);
        let texel=u.shadow_filters[i].x/(size.x*u.shadow_filters[i].z);
        for(var j=0u;j<5u;j++){
            let angle=f32(j)*2.399963229728653+phi;let r=sqrt((f32(j)+0.5)/5.0);
            let dir=bd+(tangent*cos(angle)+bitangent*sin(angle))*r*texel;let da=abs(dir);
            var face=base;
            if da.x>=da.y && da.x>=da.z {face+=select(1u,0u,dir.x>=0.0);}
            else if da.y>=da.z {face+=select(3u,2u,dir.y>=0.0);}
            else {face+=select(5u,4u,dir.z>=0.0);}
            let q=u.shadow_matrices[face]*vec4(light+dir,1.0);
            let quv=(q.xy/q.w*vec2(0.5,-0.5)+0.5)*u.shadow_filters[i].z;
            value+=textureSampleCompareLevel(shadow_atlas,shadow_sampler,shadow_map_uv(i,quv),i32(face),ndc.z+settings.y);
        }
        return value*0.2;
    }
    for(var j=0u;j<5u;j++){let angle=f32(j)*2.399963229728653+phi;let offset=vec2(cos(angle),sin(angle))*sqrt((f32(j)+0.5)/5.0)*u.shadow_filters[i].x/size;value+=textureSampleCompareLevel(shadow_atlas,shadow_sampler,shadow_map_uv(i,uv+offset),i32(layer),ndc.z+settings.y);}
    return value*0.2;
}
fn default_environment_sample(direction:vec3<f32>,roughness:f32)->vec3<f32> {
 let world=normalize((transpose(u.view)*vec4(direction,0.0)).xyz);
 let c=cos(u.environment.y);let s=sin(u.environment.y);
 let d=vec3(c*world.x-s*world.z,-world.y,s*world.x+c*world.z);
 let mip=clamp(roughness_mip(roughness),-2.0,u.environment.z);let lo=floor(mip);
 return mix(textureSampleLevel(environment_map,environment_sampler,cube_uv(d,lo,u.environment.z),0.0).rgb,
 textureSampleLevel(environment_map,environment_sampler,cube_uv(d,lo+1.0,u.environment.z),0.0).rgb,fract(mip))*u.environment.x;
}
fn multiscattering(f0:vec3<f32>,dfg:vec2<f32>,f90:f32)->vec3<f32>{
 let single=f0*dfg.x+f90*dfg.y;let average=f0+(vec3(f90)-f0)*0.047619;let missing=1.0-dfg.x-dfg.y;
 return single*average/(1.0-missing*average)*missing;
}
fn sheen_albedo(nv:f32,r:f32)->f32 {
 let a=-1.9362+1.0678*r+0.4573*r*r-0.8469/(r+0.1);
 let b=-0.6014+0.5538*r-0.4670*r*r-0.1255/(r+0.1);
 return clamp(exp(a*nv+b),0.0,1.0);
}
fn ggx(alpha:f32,nl:f32,nv:f32,nh:f32)->f32 {
 let a2=alpha*alpha;let denom=nh*nh*(a2-1.0)+1.0;
 let distribution=a2/(3.14159265359*denom*denom);
 let visibility=0.5/max(nl*sqrt(nv*nv*(1.0-a2)+a2)+nv*sqrt(nl*nl*(1.0-a2)+a2),0.000001);
 return distribution*visibility;
}
fn transform_vertex_normal(normal:vec3<f32>)->vec3<f32>{return normal;}
var<private> vertex_instance_index:u32;
var<private> fragment_position_world:vec3<f32>;
var<private> fragment_view_z:f32;
struct VertexOut {
    @location(11) local_normal:vec3<f32>,
    @location(10) local_position:vec3<f32>,
    @builtin(position) clip: vec4<f32>, @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>, @location(2) uv: vec2<f32>, @location(3) color: vec4<f32>, @location(4) tangent:vec4<f32>, @location(5) bitangent:vec3<f32>, @location(6) view_position:vec3<f32>,@location(7) uv1:vec2<f32>,@location(8) line_distance:f32, @location(9) @interpolate(flat) instance_index:u32,
};
var<private> tsl_vertex_index:u32;
@vertex fn vs_main(@builtin(vertex_index) vertex:u32,@builtin(instance_index) instance_index:u32,@location(0) input_position:vec3<f32>, @location(1) normal:vec3<f32>, @location(2) uv:vec2<f32>, @location(3) color:vec4<f32>, @location(4) corner:vec2<f32>, @location(5) tangent:vec4<f32>,@location(6) i0:vec4<f32>,@location(7) i1:vec4<f32>,@location(8) i2:vec4<f32>,@location(9) i3:vec4<f32>,@location(10) instance_color:vec4<f32>,@location(11) uv1:vec2<f32>)->VertexOut {
    tsl_vertex_index=vertex;
    vertex_instance_index=instance_index;
    let animated=skin_morph(select(vertex,u32(tangent.z),u.line[0].x>0.0),input_position,normal,color,tangent);
    var instance=mat4x4<f32>(vec4(1.0,0.0,0.0,0.0),vec4(0.0,1.0,0.0,0.0),vec4(0.0,0.0,1.0,0.0),vec4(0.0,0.0,0.0,1.0));if INSTANCED {instance=mat4x4(i0,i1,i2,i3);}
    let position=(instance*vec4(deform(animated.position,animated.normal,uv),1.0)).xyz;
    let deformed_normal=transform_vertex_normal(animated.normal);
    let cofactor=mat3x3(cross(i1.xyz,i2.xyz),cross(i2.xyz,i0.xyz),cross(i0.xyz,i1.xyz));
    let instance_normal=select(animated.normal,normalize(cofactor*animated.normal),INSTANCED);
    var out:VertexOut;out.local_normal=deformed_normal;out.local_position=position;out.instance_index=instance_index;let model_view=u.view*u.model;out.view_position=(model_view*vec4(position,1.0)).xyz;out.clip=u.projection*vec4(out.view_position,1.0);out.position=(u.model*vec4(position,1.0)).xyz;
    if u.point.z>0.0 {
        var size=u.point.z;if u.point.w>0.0 {size*=u.point.y*0.5/out.clip.w;}
        out.clip=vec4(out.clip.xy+corner*size/u.point.xy*out.clip.w,out.clip.zw);
    }
    out.normal=normalize((u.view*vec4((u.normal*vec4(instance_normal,0.0)).xyz,0.0)).xyz);out.uv=uv;out.uv1=uv1;out.color=select(color,animated.color,u.flags.z>0.5);if INSTANCED {out.color*=instance_color;}out.tangent=vec4((model_view*instance*vec4(animated.tangent.xyz,0.0)).xyz,animated.tangent.w);out.bitangent=cross(normalize(out.normal),normalize(out.tangent.xyz))*tangent.w;out.line_distance=corner.x;
    if u.line[0].x>0.0 {
        let other=skin_morph(u32(tangent.w),normal,vec3(0.0,0.0,1.0),color,tangent);
        let world_a=u.model*instance*vec4(animated.position,1.0);let world_b=u.model*instance*vec4(other.position,1.0);
        let a=u.projection*u.view*world_a;let b=u.projection*u.view*world_b;
        var low=0.0;var high=1.0;
        if a.z<0.0 && b.z<0.0 {out.clip=vec4(0.0,0.0,2.0,1.0);return out;}
        if a.z<0.0 {low=-a.z/(b.z-a.z);}if b.z<0.0 {high=a.z/(a.z-b.z);}
        let p=mix(a,b,low);let q=mix(a,b,high);
        let direction=(q.xy/q.w-p.xy/p.w)*u.point.xy;
        let safe=direction/max(length(direction),0.000001);
        let offset=vec2(-safe.y,safe.x)*u.line[0].x/u.point.xy;
        let clip=mix(p,q,corner.x);out.clip=vec4(clip.xy+offset*corner.y*clip.w,clip.zw);
        let fraction=mix(low,high,corner.x);let world=mix(world_a,world_b,fraction);
        out.position=world.xyz;out.view_position=(u.view*world).xyz;
        out.line_distance=mix(tangent.x,tangent.y,fraction);
    }
    return project_vertex(out,animated.position);
}
fn map_uv(index:u32,surface:VertexOut)->vec2<f32> {
 let start=index*3u;let t=u.uv_transforms;
 let uv=select(surface.uv,surface.uv1,t[start+2u].w>0.5);
 return (mat3x3(t[start].xyz,t[start+1u].xyz,t[start+2u].xyz)*vec3(uv,1.0)).xy;
}
fn apply_fog(color:vec4<f32>,depth:f32)->vec4<f32> {
    // WebGL's in-shader output applies fog_fragment after colorspace_fragment.
    if ENCODE_SRGB {return color;}
    return fog_mix(color,depth,u.fog_color.rgb);
}
fn fog_mix(color:vec4<f32>,depth:f32,fog:vec3<f32>)->vec4<f32> {
    var factor=0.0;
    if u.fog_params.x==1.0 {factor=smoothstep(u.fog_params.y,u.fog_params.z,depth);}
    if u.fog_params.x==2.0 {factor=1.0-exp(-u.fog_params.y*u.fog_params.y*depth*depth);}
    return vec4(mix(color.rgb,fog,factor),color.a);
}
var<private> fragment_surface:VertexOut;
var<private> fragment_front:bool;
var<private> fragment_normal:vec3<f32>;
var<private> fragment_diffuse:vec4<f32>;
var<private> fragment_emissive:vec3<f32>;
@fragment fn fs_main(in:VertexOut,@builtin(front_facing) front:bool)->@location(0) vec4<f32> {
    fragment_surface=in;fragment_front=front;
    fragment_normal=normalize(in.normal)*select(-1.0,1.0,front);fragment_diffuse=vec4(0.0);fragment_emissive=vec3(0.0);
    fragment_position_world=in.position;fragment_view_z=-in.view_position.z;
    let color=transform_output(shade_fragment(in,front));
    if ENCODE_SRGB {
        var rgb=color.rgb;
        if u.output.y>0.5 {rgb=tone_output(rgb,u.output.x,u.output.y);}
        // Its fog color uniform is also converted to the output color space.
        return fog_mix(vec4(srgb_output(rgb),color.a),-in.view_position.z,srgb_output(u.fog_color.rgb));
    }
    return color;
}
struct LitSurface {normal:vec3<f32>,roughness:f32,metalness:f32,emissive:vec3<f32>,specular:vec4<f32>,light_map:vec3<f32>,backdrop:vec4<f32>,thickness_color:vec3<f32>,thickness:vec4<f32>,thickness_scale:f32,shadow_position:vec3<f32>,}
fn shade_fragment(in:VertexOut,front:bool)->vec4<f32> {
    if PHYSICAL {
        for(var i=0u;i<12u;i++) {
            if (EXTENSION_MAP_MASK&(1u<<i))==0u {continue;}
            let uv=extension_uv(i,in)*u.extension_sizes[i].xy;
            let dx=dpdx(uv);let dy=dpdy(uv);
            extension_lod[i]=0.5*log2(max(max(dot(dx,dx),dot(dy,dy)),1e-10));
        }
    }
    let q0=dpdx(in.view_position);let q1=-dpdy(in.view_position);var normal_uv=select(in.uv,map_uv(2u,in),u.maps.x>0.5);if u.uv_transforms[6u].w>0.5 {normal_uv.y=1.0-normal_uv.y;}let st0=dpdx(normal_uv);let st1=-dpdy(normal_uv);
    var geometry_normal=normalize(in.normal);
    if u.flags.x>0.5 {geometry_normal=normalize(cross(q0,q1));}
    let view_normal=geometry_normal;
    let derivative=max(abs(dpdx(view_normal)),abs(dpdy(view_normal)));
    var normal_sample=vec3(1.0);if NORMAL_MAP {normal_sample=textureSample(normal_map,normal_sampler,map_uv(2u,in)).xyz*2.0-1.0;}
    var base=u.color*in.color;if COLOR_MAP {base*=textureSample(color_map,color_sampler,map_uv(0u,in));}
    var clipping_opacity=1.0;
    if CLIPPING {
        let global=u32(u.clipping_params.x);let local=u32(u.clipping_params.y);
        if u.clipping_params.w>0.5 {
            var intersection_opacity=1.0;
            for(var i=0u;i<global+local;i++) {
                let distance=dot(u.clipping_planes[i],vec4(in.position,1.0));
                let gradient=max(fwidth(distance)*0.5,1e-8);
                let coverage=smoothstep(-gradient,gradient,distance);
                if i<global || u.clipping_params.z==0.0 {clipping_opacity*=coverage;}
                else {intersection_opacity*=1.0-coverage;}
            }
            if local>0u && u.clipping_params.z>0.0 {clipping_opacity*=1.0-intersection_opacity;}
            if clipping_opacity==0.0 {discard;}
        } else {
            for(var i=0u;i<global;i++){if dot(u.clipping_planes[i],vec4(in.position,1.0))<0.0 {discard;}}
            var all_outside=local>0u;
            for(var i=global;i<global+local;i++) {let outside=dot(u.clipping_planes[i],vec4(in.position,1.0))<0.0;if outside && u.clipping_params.z==0.0 {discard;}all_outside=all_outside&&outside;}
            if all_outside && u.clipping_params.z>0.0 {discard;}
        }
    }
    if LINE_DASH {let distance=in.line_distance*u.line[0].w+u.line[1].x;let period=u.line[0].y+u.line[0].z;if distance-floor(distance/period)*period>u.line[0].y {discard;}}
    if material_kind()!=5.0 {base=shade(in,base);}
    if ALPHA_MASK && material_kind()!=5.0 && base.a<u.pbr.w {discard;}
    if u.maps.y<0.5 {base.a=1.0;}
    base.a*=clipping_opacity;
    if material_kind()==5.0 {let shaded=shade(in,base);if ALPHA_MASK && shaded.a<=u.pbr.w {discard;}return apply_fog(shaded,-in.view_position.z);}
    fragment_diffuse=base;
    if material_kind()<0.5 {let surf=transform_surface(in,LitSurface(vec3(0.0),0.0,0.0,vec3(0.0),vec4(0.0),vec3(0.0),vec4(0.0,0.0,0.0,-1.0),vec3(0.0),vec4(0.1,0.0,0.1,2.0),10.0,in.position));if surf.backdrop.a>=0.0 {base=vec4(mix(base.rgb,surf.backdrop.rgb,surf.backdrop.a),base.a);}return apply_fog(max(base,vec4(0.0)),-in.view_position.z);}
    let face=select(-1.0,1.0,front);
    var n=geometry_normal*face;
    let v=select(normalize(-in.view_position),vec3(0.0,0.0,1.0),u.projection[3][3]!=0.0);
    if NORMAL_MAP && u.maps.x>0.5 {
        var t:vec3<f32>;var b:vec3<f32>;
        if abs(in.tangent.w)>0.5 {t=normalize(in.tangent.xyz);b=normalize(in.bitangent);}
        else {
            let q1perp=cross(q1,geometry_normal);let q0perp=cross(geometry_normal,q0);
            t=q1perp*st0.x+q0perp*st1.x;b=q1perp*st0.y+q0perp*st1.y;
            let scale=inverseSqrt(max(max(dot(t,t),dot(b,b)),1e-20));t*=scale;b*=scale;
        }
        let sample=normal_sample;
        n=normalize(t*sample.x*u.pbr.x*face+b*sample.y*u.pbr.y*face+n*sample.z);
    }
    if material_kind()==8.0 {return vec4(vec3(1.0-in.clip.z),base.a);}
    if material_kind()==7.0 {
        let x=normalize(vec3(v.z,0.0,-v.x));let y=cross(v,x);
        let uv=vec2(dot(x,n),dot(y,n))*0.495+0.5;
        let matcap=textureSample(mr_map,mr_sampler,vec2(uv.x,1.0-uv.y)).rgb;
        let sample=select(vec3(mix(0.2,0.8,uv.y)),matcap,u.custom[0].x>0.5);
        return apply_fog(vec4(base.rgb*sample,base.a),-in.view_position.z);
    }
    if material_kind()==4.0 {return vec4(n*0.5+0.5,base.a);}
    var mr=vec4(1.0);if MR_MAP {mr=textureSample(mr_map,mr_sampler,map_uv(1u,in));}

    var emissive_sample=vec3(1.0);if EMISSIVE_MAP {emissive_sample=textureSample(emissive_map,emissive_sampler,map_uv(4u,in)).rgb;}
    let surface=transform_surface(in,LitSurface(n,u.material.y*mr.g,u.material.z*mr.b,u.emissive.xyz*emissive_sample,u.specular,vec3(0.0),vec4(0.0,0.0,0.0,-1.0),vec3(0.0),vec4(0.1,0.0,0.1,2.0),10.0,in.position));
    n=surface.normal;fragment_normal=n;fragment_emissive=surface.emissive;
    let geometry_roughness=max(derivative.x,max(derivative.y,derivative.z));
    let roughness=min(max(surface.roughness,0.0525)+geometry_roughness,1.0);
    let metalness=clamp(surface.metalness,0.0,1.0);
    var transmission=0.0;if PHYSICAL {transmission=u.transmission[0].x*extension_sample(8u,in).r;}
    let diffuse=base.rgb*(1.0-metalness)*(1.0-transmission);
    let specular_color=u.physical[2].rgb*extension_sample(7u,in).rgb;let specular_intensity=u.physical[2].w*extension_sample(6u,in).a;
    var dielectric=vec3(0.04);var f90=1.0;
    if PHYSICAL && u.physical[3].y>0.5 {let ratio=(u.physical[0].w-1.0)/(u.physical[0].w+1.0);dielectric=min(vec3(ratio*ratio)*specular_color,vec3(1.0))*specular_intensity;f90=mix(specular_intensity,1.0,metalness);}
    let f0=mix(dielectric,base.rgb,vec3(metalness));
    let film_thickness=mix(u.iridescence.z,u.iridescence.w,extension_sample(11u,in).g);
    let film=select(clamp(u.iridescence.x*extension_sample(10u,in).r,0.0,1.0),0.0,film_thickness==0.0);
    let film_nv=clamp(dot(n,v),0.0,1.0);
    var film_d=dielectric;var film_m=base.rgb;var film_f=vec3(0.0);
    if film>0.0 {
        let d=iridescent(u.iridescence.y,film_nv,film_thickness,dielectric);
        let m=iridescent(u.iridescence.y,film_nv,film_thickness,base.rgb);
        film_f=mix(d,m,metalness);film_d=mix(dielectric,fresnel_to_f0(d,film_nv),film);film_m=mix(base.rgb,fresnel_to_f0(m,film_nv),film);
    }
    var cc=0.0;if PHYSICAL {cc=u.physical[0].x*extension_sample(0u,in).r;}let ccrough=min(max(u.physical[0].y*extension_sample(1u,in).g,0.0525)+geometry_roughness,1.0);
    var sheen=vec3(0.0);if PHYSICAL {sheen=u.physical[1].rgb*extension_sample(3u,in).rgb;}let sheen_max=max(sheen.r,max(sheen.g,sheen.b));let sheenrough=max(u.physical[0].z*extension_sample(4u,in).a,0.07);
    var coat_n=geometry_normal*face;
    var coat_uv=extension_uv(2u,in);if u.extension_matrices[6u].w>0.5 {coat_uv.y=1.0-coat_uv.y;}let coat_st0=dpdx(coat_uv);let coat_st1=-dpdy(coat_uv);
    if PHYSICAL && u.extension_sizes[2].x>0.0 {
        var t=normalize(in.tangent.xyz);var b=normalize(in.bitangent);
        if abs(in.tangent.w)<0.5 {
            t=cross(q1,geometry_normal)*coat_st0.x+cross(geometry_normal,q0)*coat_st1.x;
            b=cross(q1,geometry_normal)*coat_st0.y+cross(geometry_normal,q0)*coat_st1.y;
            let scale=inverseSqrt(max(max(dot(t,t),dot(b,b)),1e-20));t*=scale;b*=scale;
        }
        let sample=extension_sample(2u,in).xyz*2.0-1.0;
        coat_n=normalize(t*sample.x*u.coat_normal.x*face+b*sample.y*u.coat_normal.y*face+coat_n*sample.z);
    }
var coat=vec3(0.0);var sheen_light=vec3(0.0);
    var anisotropy_t=normalize(in.tangent.xyz);var anisotropy_b=normalize(in.bitangent);
    if abs(in.tangent.w)<0.5 {
        let tangent=cross(q1,geometry_normal)*st0.x+cross(geometry_normal,q0)*st1.x;
        let bitangent=cross(q1,geometry_normal)*st0.y+cross(geometry_normal,q0)*st1.y;
        let scale=inverseSqrt(max(max(dot(tangent,tangent),dot(bitangent,bitangent)),1e-20));
        anisotropy_t=tangent*scale;anisotropy_b=bitangent*scale;
    }
    let aniso_sample=extension_sample(5u,in);
    let rotation=u.physical[3].x+select(0.0,atan2(aniso_sample.y*2.0-1.0,aniso_sample.x*2.0-1.0),u.extension_sizes[5].x>0.0);let tangent=anisotropy_t;
    anisotropy_t=(tangent*cos(rotation)+anisotropy_b*sin(rotation))*face;
    anisotropy_b=(anisotropy_b*cos(rotation)-tangent*sin(rotation))*face;
    var indirect_energy=vec3(1.0);var direct_energy=vec3(1.0);
    if u.physical[3].z>0.5 && material_kind()==1.0 {
        let dfg=textureSampleLevel(dfg_map,environment_sampler,vec2(roughness,clamp(dot(n,v),0.0,1.0)),0.0).rg;
        indirect_energy=vec3(1.0)-(film_d*dfg.x+f90*dfg.y+multiscattering(film_d,dfg,f90));
        direct_energy=vec3(1.0)+f0*(1.0/(dfg.x+dfg.y)-1.0);
    }
    var ao=1.0;if AO_MAP {ao=(textureSample(ao_map,ao_sampler,map_uv(3u,in)).r-1.0)*u.pbr.z+1.0;}
    var result=ao*indirect_energy*diffuse*(u.ambient.xyz+surface.light_map)/3.14159265359*(1.0-sheen_max*sheen_albedo(clamp(dot(n,v),0.0,1.0),sheenrough))+surface.emissive;
    var total_diffuse=result-surface.emissive;
    sheen_light+=ao*u.ambient.xyz*sheen*sheen_albedo(clamp(dot(n,v),0.0,1.0),sheenrough)/3.14159265359;
    if u.environment.x>0.0 && material_kind()==1.0 {
        let nv=clamp(dot(n,v),0.0,1.0);
        let dfg=textureSampleLevel(dfg_map,environment_sampler,vec2(roughness,nv),0.0).rg;
        let sd=film_d*dfg.x+f90*dfg.y;let sm=film_m*dfg.x+f90*dfg.y;
        let md=multiscattering(film_d,dfg,f90);let mm=multiscattering(film_m,dfg,f90);
        // Three.js/Filament anisotropic IBL: bend the radiance normal along
        // the bitangent, retaining the shading normal for diffuse irradiance.
        var radiance_normal=n;
        if PHYSICAL && u.physical[1].w>0.0 {
            let strength=clamp(u.physical[1].w*aniso_sample.b,0.0,1.0);
            let bent=normalize(cross(cross(anisotropy_b,v),anisotropy_b));
            radiance_normal=normalize(mix(bent,n,pow(1.0-strength*(1.0-roughness),4.0)));
        }
        let radiance=environment_sample(normalize(mix(reflect(-v,radiance_normal),radiance_normal,pow(roughness,4.0))),roughness);
        let irradiance=environment_sample(n,1.0);
        let specular_ao=clamp(pow(nv+ao,exp2(-16.0*roughness-1.0))-1.0+ao,0.0,1.0);
        let sheen_comp=1.0-sheen_max*sheen_albedo(nv,sheenrough);
        total_diffuse+=diffuse*(1.0-sd-md)*irradiance*ao*sheen_comp;
        result+=(diffuse*(1.0-sd-md)*irradiance*ao+(radiance*mix(sd,sm,metalness)+irradiance*mix(md,mm,metalness))*specular_ao)*sheen_comp;
        sheen_light+=irradiance*sheen*sheen_albedo(nv,sheenrough)*ao;
        if cc>0.0 {
            let coatdfg=textureSampleLevel(dfg_map,environment_sampler,vec2(ccrough,clamp(dot(coat_n,v),0.0,1.0)),0.0).rg;
            coat+=environment_sample(normalize(mix(reflect(-v,coat_n),coat_n,pow(ccrough,4.0))),ccrough)*(vec3(0.04)*coatdfg.x+coatdfg.y)*ao;
        }
    }
    for(var i=0u;i<light_count();i++) {
        let light_color=transform_light_color(in,i,u.light_color[i].xyz);
        var light=(u.view*vec4(u.light_position[i].xyz,0.0)).xyz;var attenuation=1.0;
        if light_type(i)==4.0 {
            if material_kind()!=1.0 {continue;}
            let center=(u.view*vec4(u.light_position[i].xyz,1.0)).xyz;
            let width=(u.view*vec4(u.light_direction[i].xyz,0.0)).xyz;let height=(u.view*vec4(u.light_params[i].xyz,0.0)).xyz;
            let rect=array<vec3<f32>,4>(center+width-height,center-width-height,center-width+height,center+width+height);
            let uv=ltc_uv(n,v,roughness);let t1=textureSample(ltc_tables,ltc_sampler,uv,0);let t2=textureSample(ltc_tables,ltc_sampler,uv,1);
            let inverse=mat3x3(vec3(t1.x,0.0,t1.y),vec3(0.0,1.0,0.0),vec3(t1.z,0.0,t1.w));
            let identity=mat3x3(vec3(1.0,0.0,0.0),vec3(0.0,1.0,0.0),vec3(0.0,0.0,1.0));
            total_diffuse+=light_color*diffuse*ltc_evaluate(n,v,in.view_position,identity,rect);
            result+=light_color*((f0*t2.x+(vec3(f90)-f0)*t2.y)*ltc_evaluate(n,v,in.view_position,inverse,rect)+diffuse*ltc_evaluate(n,v,in.view_position,identity,rect));
                let uvcc=ltc_uv(coat_n,v,ccrough);let a=textureSample(ltc_tables,ltc_sampler,uvcc,0);let b=textureSample(ltc_tables,ltc_sampler,uvcc,1);
            if cc>0.0 {
                let inversecc=mat3x3(vec3(a.x,0.0,a.y),vec3(0.0,1.0,0.0),vec3(a.z,0.0,a.w));
                coat+=light_color*(0.04*b.x+0.96*b.y)*ltc_evaluate(coat_n,v,in.view_position,inversecc,rect);
            }
            continue;
        }
        if light_type(i)==3.0 {
            let weight=dot(n,light)*0.5+0.5;
            let hemi=ao*indirect_energy*diffuse*mix(u.light_params[i].xyz,light_color,weight)/3.14159265359;total_diffuse+=hemi;result+=hemi;
            continue;
        }
        if light_type(i)>0.5 {
            let delta=(u.view*vec4(u.light_position[i].xyz,1.0)).xyz-in.view_position;let distance=length(delta);light=delta/max(distance,0.00001);
            attenuation=1.0/max(pow(distance,u.light_params[i].y),0.01);
            let cutoff=u.light_params[i].x;
            if cutoff>0.0 {let falloff=clamp(1.0-pow(distance/cutoff,4.0),0.0,1.0);attenuation*=falloff*falloff;}
        }
        if light_type(i)==2.0 {
            let direction=(u.view*vec4(u.light_direction[i].xyz,0.0)).xyz;
            let cone=dot(light,direction);let outer=u.light_params[i].z;let inner=u.light_params[i].w;
            attenuation*=select(smoothstep(outer,max(inner,outer+0.000001),cone),select(0.0,1.0,cone>=outer),inner==outer);
        }
        if RECEIVE_SHADOW {attenuation*=shadow_visibility(i,surface.shadow_position,normalize((transpose(u.view)*vec4(n,0.0)).xyz));}
        let scattering_half=normalize(light+n*surface.thickness.x);
        let scattering_dot=pow(clamp(dot(v,-scattering_half),0.0,1.0),surface.thickness.w)*surface.thickness_scale;
        let scattering=(scattering_dot+surface.thickness.y)*surface.thickness_color*surface.thickness.z*light_color*attenuation;
        total_diffuse+=scattering;result+=scattering;
        let h=normalize(light+v);let nl=clamp(dot(n,light),0.0,1.0);let nv=clamp(dot(n,v),0.0,1.0);
        let nh=clamp(dot(n,h),0.0,1.0);let vh=clamp(dot(v,h),0.0,1.0);
        if material_kind()==6.0 {
            let coordinate=dot(n,light)*0.5+0.5;let width=fwidth(coordinate)*0.5;
            let gradient=textureSampleLevel(mr_map,mr_sampler,vec2(coordinate,0.0),0.0).r;
            let toon=select(mix(0.7,1.0,smoothstep(0.7-width,0.7+width,coordinate)),gradient,u.custom[0].x>0.5);
            let d=diffuse*light_color*attenuation*toon/3.14159265359;total_diffuse+=d;result+=d;continue;
        }
        if material_kind()>=2.0 {
            var brdf=diffuse/3.14159265359;
            if material_kind()==3.0 {
                let fresnel=surface.specular.rgb+(vec3(1.0)-surface.specular.rgb)*exp2((-5.55473*vh-6.98316)*vh);
                brdf+=fresnel*(surface.specular.w+2.0)/(8.0*3.14159265359)*pow(nh,surface.specular.w)*mr.r;
            }
            total_diffuse+=diffuse/3.14159265359*light_color*attenuation*nl;
            result+=brdf*light_color*attenuation*nl;
            continue;
        }
        let alpha=roughness*roughness;let a2=alpha*alpha;let denom=nh*nh*(a2-1.0)+1.0;
        var distribution=a2/(3.14159265359*denom*denom);
        var visibility=0.5/max(nl*sqrt(nv*nv*(1.0-a2)+a2)+nv*sqrt(nl*nl*(1.0-a2)+a2),0.000001);
        var anisotropy=0.0;if PHYSICAL {anisotropy=u.physical[1].w*aniso_sample.b;}
        if anisotropy>0.0 {
            let at=mix(alpha,1.0,anisotropy*anisotropy);let ab=alpha;
            let projected=vec3(ab*dot(anisotropy_t,h),at*dot(anisotropy_b,h),at*ab*nh);
            let w2=at*ab/max(dot(projected,projected),0.00000001);
            distribution=at*ab*w2*w2/3.14159265359;
            let gv=nl*length(vec3(at*dot(anisotropy_t,v),ab*dot(anisotropy_b,v),nv));
            let gl=nv*length(vec3(at*dot(anisotropy_t,light),ab*dot(anisotropy_b,light),nl));
            visibility=0.5/max(gv+gl,0.000001);
        }
        let fresnel=mix(f0+(vec3(f90)-f0)*exp2((-5.55473*vh-6.98316)*vh),film_f,film);
        let irradiance=light_color*attenuation*nl;
        let sheen_comp=1.0-sheen_max*max(sheen_albedo(nv,sheenrough),sheen_albedo(nl,sheenrough));
        var diffuse_energy=vec3(1.0);
        if u.physical[3].z>0.5 { diffuse_energy=vec3(1.0)-(dielectric+(vec3(f90)-dielectric)*exp2((-5.55473*vh-6.98316)*vh)); }
        var specular_brdf=fresnel*distribution*visibility;
        if PHYSICAL && u.physical[3].w>0.0 {
            let retro_v=reflect(-v,n);let retro_h=normalize(light+retro_v);
            let retro_nv=clamp(dot(n,retro_v),0.0,1.0);let retro_nh=clamp(dot(n,retro_h),0.0,1.0);let retro_vh=clamp(dot(retro_v,retro_h),0.0,1.0);
            let retro_denom=retro_nh*retro_nh*(a2-1.0)+1.0;
            var retro_distribution=a2/(3.14159265359*retro_denom*retro_denom);
            var retro_visibility=0.5/max(nl*sqrt(retro_nv*retro_nv*(1.0-a2)+a2)+retro_nv*sqrt(nl*nl*(1.0-a2)+a2),0.000001);
            if anisotropy>0.0 {
                let at=mix(alpha,1.0,anisotropy*anisotropy);let ab=alpha;
                let projected=vec3(ab*dot(anisotropy_t,retro_h),at*dot(anisotropy_b,retro_h),at*ab*retro_nh);
                let w2=at*ab/max(dot(projected,projected),0.00000001);
                retro_distribution=at*ab*w2*w2/3.14159265359;
                let gv=nl*length(vec3(at*dot(anisotropy_t,retro_v),ab*dot(anisotropy_b,retro_v),retro_nv));
                let gl=retro_nv*length(vec3(at*dot(anisotropy_t,light),ab*dot(anisotropy_b,light),nl));
                retro_visibility=0.5/max(gv+gl,0.000001);
            }
            let retro_weight=exp2((-5.55473*retro_vh-6.98316)*retro_vh);
            let retro_f=mix(f0+(vec3(f90)-f0)*retro_weight,film_f,film);
            specular_brdf=mix(specular_brdf,retro_f*retro_distribution*retro_visibility,u.physical[3].w);
            if u.physical[3].z>0.5 {diffuse_energy=mix(diffuse_energy,vec3(1.0)-(dielectric+(vec3(f90)-dielectric)*retro_weight),u.physical[3].w);}
        }
        total_diffuse+=diffuse_energy*diffuse/3.14159265359*irradiance*sheen_comp;
        result+=(diffuse_energy*diffuse/3.14159265359+direct_energy*specular_brdf)*irradiance*sheen_comp;
        if sheen_max>0.0 {
            let inv=1.0/(sheenrough*sheenrough);let charlie=(2.0+inv)*pow(max(1.0-nh*nh,0.0078125),inv*0.5)/(2.0*3.14159265359);
            let neubelt=clamp(1.0/(4.0*max(nl+nv-nl*nv,0.000001)),0.0,1.0);
            sheen_light+=sheen*charlie*neubelt*irradiance;
        }
        if cc>0.0 {
            let coatnl=clamp(dot(coat_n,light),0.0,1.0);let coatnv=clamp(dot(coat_n,v),0.0,1.0);let coatnh=clamp(dot(coat_n,h),0.0,1.0);
            let coatf=0.04+0.96*exp2((-5.55473*vh-6.98316)*vh);
            coat+=vec3(coatf*ggx(ccrough*ccrough,coatnl,coatnv,coatnh))*light_color*attenuation*coatnl;
        }
    }
    if transmission>0.0 {
        let ior=u.physical[0].w;
        var color=transmitted(in,n,v,ior,roughness);
        if u.transmission[0].w>0.0 {
            let spread=(ior-1.0)*0.025*u.transmission[0].w;
            color=vec3(transmitted(in,n,v,max(1.0,ior-spread),roughness).r,color.g,transmitted(in,n,v,ior+spread,roughness).b);
        }
        let dfg=textureSampleLevel(dfg_map,environment_sampler,vec2(roughness,clamp(dot(n,v),0.0,1.0)),0.0).rg;
        let fresnel=f0*dfg.x+vec3(f90)*dfg.y;
        result+=(vec3(1.0)-fresnel)*color*base.rgb*(1.0-metalness)*transmission;
    }
    if surface.backdrop.a>=0.0 {result+=mix(total_diffuse,surface.backdrop.rgb,surface.backdrop.a)-total_diffuse;}
    result+=sheen_light;
    if cc>0.0 {let nv=clamp(dot(coat_n,v),0.0,1.0);let fcc=0.04+0.96*exp2((-5.55473*nv-6.98316)*nv);result=result*(1.0-cc*fcc)+coat*cc;}
    return apply_fog(max(vec4(result,base.a),vec4(0.0)),-in.view_position.z);
}

// r186 SunShadowNode: two view-depth cascades with five rotated Vogel PCF taps.
fn sun_cascade_sample(i:u32,layer:u32,position:vec3<f32>,normal:vec3<f32>,inset:f32)->f32 {
 let settings=u.shadow_params[i];
 let projected=u.shadow_matrices[layer]*vec4(position+normal*settings.z,1.0);
 let ndc=projected.xyz/projected.w;
 var uv=(ndc.xy*vec2(0.5,-0.5)+0.5)*(1.0-2.0*inset)+inset;
 if projected.w<=0.0 || any(uv<vec2(0.0)) || any(uv>vec2(1.0)) || ndc.z>1.0 {return 1.0;}
 uv*=u.shadow_filters[i].z;
    let size=vec2<f32>(textureDimensions(shadow_atlas));
 if u.shadow_filters[i].y>0.5 {return textureSampleCompareLevel(shadow_atlas,shadow_sampler,shadow_map_uv(i,(floor(uv*size)+0.5)/size),i32(layer),ndc.z+settings.y);}
 let phi=fract(52.9829189*fract(dot(fragment_surface.clip.xy,vec2(0.06711056,0.00583715))))*6.28318530718;
 // Original atlas is 2 tiles wide: the scalar filter radius uses its width.
 let scale=u.shadow_filters[i].x/vec2<f32>(textureDimensions(shadow_atlas))*vec2(1.0,0.5);
 var value=0.0;
 for(var j=0u;j<5u;j++){let angle=f32(j)*2.399963229728653+phi;let offset=vec2(cos(angle),sin(angle))*sqrt((f32(j)+0.5)/5.0)*scale;value+=textureSampleCompareLevel(shadow_atlas,shadow_sampler,shadow_map_uv(i,uv+offset),i32(layer),ndc.z+settings.y);}
 return value*0.2;
}
fn sun_shadow_visibility(i:u32,position:vec3<f32>,normal:vec3<f32>)->f32 {
 let depth=-(u.view*vec4(position,1.0)).z;var value=1.0;
 for(var j=1;j>=0;j--){let range=u.shadow_cascades[i*2u+u32(j)];if depth>=range.x && depth<range.y {let sample=sun_cascade_sample(i,u32(u.shadow_params[i].x)-1u+u32(j),position,normal,range.w);value=mix(sample,value,smoothstep(range.z,range.y,depth));}}
 return value;
}

fn transform_light_color(surface:VertexOut,tsl_light_index:u32,tsl_light_color:vec3<f32>)->vec3<f32>{return tsl_light_color;}
