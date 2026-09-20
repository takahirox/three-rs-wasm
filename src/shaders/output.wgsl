// Three.js ACES filmic output transform (MIT). Keep per-fragment and
// post-resolve presentation on the same curve.
fn aces_output(value:vec3<f32>,exposure:f32)->vec3<f32> {
    var c=value*exposure/0.6;
    c=mat3x3<f32>(vec3(0.59719,0.07600,0.02840),vec3(0.35458,0.90834,0.13383),vec3(0.04823,0.01566,0.83777))*c;
    c=(c*(c+0.0245786)-0.000090537)/(c*(0.983729*c+0.4329510)+0.238081);
    c=mat3x3<f32>(vec3(1.60475,-0.10208,-0.00327),vec3(-0.53108,1.10813,-0.07276),vec3(-0.07367,-0.00605,1.07602))*c;
    return clamp(c,vec3(0.0),vec3(1.0));
}
fn srgb_output(rgb:vec3<f32>)->vec3<f32> {
    return select(1.055*pow(max(rgb,vec3(0.0)),vec3(1.0/2.4))-0.055,rgb*12.92,rgb<=vec3(0.0031308));
}

fn tone_output(value:vec3<f32>,exposure:f32,mode:f32)->vec3<f32> {
    if mode>3.5 {return clamp(value*exposure,vec3(0.0),vec3(1.0));}
    if mode>2.5 {
        var c=value*exposure;
        let x=min(c.r,min(c.g,c.b));
        let offset=select(0.04,x-6.25*x*x,x<0.08);
        c-=offset;
        let peak=max(c.r,max(c.g,c.b));
        if peak<0.76 {return c;}
        let new_peak=1.0-0.24*0.24/(peak-0.52);
        c*=new_peak/peak;
        let g=1.0-1.0/(0.15*(peak-new_peak)+1.0);
        return mix(c,vec3(new_peak),g);
    }
    if mode>1.5 {let c=value*exposure;return clamp(c/(vec3(1.0)+c),vec3(0.0),vec3(1.0));}
    if mode>0.5 {return aces_output(value,exposure);}
    return value;
}
