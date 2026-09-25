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

// Three.js AgX (MIT): Rec.2020 inset, log2 encoding, contrast curve and outset.
fn agx_output(value:vec3<f32>,exposure:f32)->vec3<f32> {
    let inset=mat3x3<f32>(vec3(0.856627153315983,0.137318972929847,0.11189821299995),vec3(0.0951212405381588,0.761241990602591,0.0767994186031903),vec3(0.0482516061458583,0.101439036467562,0.811302368396859));
    let outset=mat3x3<f32>(vec3(1.1271005818144368,-0.1413297634984383,-0.14132976349843826),vec3(-0.11060664309660323,1.157823702216272,-0.11060664309660294),vec3(-0.016493938717834573,-0.016493938717834257,1.2519364065950405));
    let to2020=mat3x3<f32>(vec3(0.6274,0.0691,0.0164),vec3(0.3293,0.9195,0.0880),vec3(0.0433,0.0113,0.8956));
    let from2020=mat3x3<f32>(vec3(1.6605,-0.1246,-0.0182),vec3(-0.5876,1.1329,-0.1006),vec3(-0.0728,-0.0083,1.1187));
    var c=to2020*(value*exposure);
    c=max(inset*c,vec3(1e-10));
    c=clamp((log2(c)-(-12.47393))/(4.026069-(-12.47393)),vec3(0.0),vec3(1.0));
    let x2=c*c;let x4=x2*x2;
    c=15.5*x4*x2-40.14*x4*c+(31.96*x4-6.868*x2*c+(0.4298*x2+(0.1191*c-0.00232)));
    c=pow(max(vec3(0.0),outset*c),vec3(2.2));
    return clamp(from2020*c,vec3(0.0),vec3(1.0));
}

fn tone_output(value:vec3<f32>,exposure:f32,mode:f32)->vec3<f32> {
    if mode>5.5 {return agx_output(value,exposure);}
    if mode>4.5 {
        // Cineon (Richard Burgess-Dawson).
        let c=max(value*exposure-0.004,vec3(0.0));
        return pow((c*(c*6.2+0.5))/(c*(c*6.2+1.7)+0.06),vec3(2.2));
    }
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
