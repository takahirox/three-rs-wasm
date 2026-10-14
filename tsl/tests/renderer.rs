//! The transpiled WebGPURenderer on the headless device, natively: init, one
//! frame of the editor's scene, and as many nodes as r186 creates by then.
use three_tsl::reg::*;
use three_tsl::rt::*;

fn element(name: &str) -> Val {
    let o = object(&[
        ("nodeName", Val::from(name.to_uppercase())),
        ("style", object(&[])),
        ("width", Val::Num(300.0)),
        ("height", Val::Num(150.0)),
    ]);
    for f in ["getContext", "addEventListener", "removeEventListener", "setAttribute", "getAttribute"] {
        set(&o, f, func(f, 0, |_t, _a| Ok(Val::Null))).unwrap();
    }
    o
}

#[test]
fn editor_scene_renders_headless() {
    let doc = object(&[]);
    set(&doc, "createElementNS", func("createElementNS", 2, |_t, a| Ok(element(&display(&arg(a, 1)))))).unwrap();
    define_global("document", doc);
    three_tsl::three::order::init_all().unwrap();
    let three = module_namespace("Three.WebGPU.js");
    let tsl = module_namespace("Three.TSL.js");
    let new = |name: &str, args: &[Val]| construct(&get(&three, name).unwrap(), args).unwrap();
    // reference/three-js/tsl-corpus.html's steps.
    let camera = new("PerspectiveCamera", &[70.0.into(), 1.0.into(), 0.1.into(), 10.0.into()]);
    set(&get(&camera, "position").unwrap(), "z", Val::Num(0.72)).unwrap();
    m(&camera, "lookAt", &[0.0.into(), 0.0.into(), 0.0.into()]).unwrap();
    let renderer = new("WebGPURenderer", &[object(&[("antialias", Val::Bool(true))])]);
    m(&renderer, "setPixelRatio", &[1.0.into()]).unwrap();
    m(&renderer, "setSize", &[200.0.into(), 200.0.into()]).unwrap();
    set(&get(&get(&renderer, "debug").unwrap(), "diagnostics").unwrap(), "keywords", Val::Bool(true)).unwrap();
    let scene = new("Scene", &[]);
    set(&scene, "background", new("Color", &[Val::Num(0x222222 as f64)])).unwrap();
    let material = new("NodeMaterial", &[]);
    let fragment = call(&get(&tsl, "vec4").unwrap(), &Val::Undef, &[0.0.into(), 0.0.into(), 0.0.into(), 1.0.into()]).unwrap();
    set(&material, "fragmentNode", fragment).unwrap();
    let mesh = new("Mesh", &[new("PlaneGeometry", &[1.0.into(), 1.0.into()]), material]);
    m(&scene, "add", &[mesh]).unwrap();
    let gl = new("WebGPURenderer", &[object(&[("forceWebGL", Val::Bool(true))])]);
    set(&get(&get(&gl, "debug").unwrap(), "diagnostics").unwrap(), "keywords", Val::Bool(true)).unwrap();
    await_val(m(&renderer, "init", &[]).unwrap()).unwrap();
    m(&renderer, "render", &[scene, camera]).unwrap();
    // reference/three-js/tsl-corpus.html after its first frame: the next node id is 1136.
    assert_eq!(display(&get(&new("Node", &[]), "id").unwrap()), "1136");
}
