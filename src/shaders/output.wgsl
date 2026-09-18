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
