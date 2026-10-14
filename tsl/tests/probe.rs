use three_tsl::reg::*;
use three_tsl::rt::*;

fn element(name: &str) -> Val {
    let o = object(&[("nodeName", Val::from(name.to_uppercase())), ("style", object(&[])), ("width", Val::Num(300.0)), ("height", Val::Num(150.0))]);
    for f in ["getContext", "addEventListener", "removeEventListener", "setAttribute", "getAttribute"] {
        set(&o, f, func(f, 0, |_t, _a| Ok(Val::Null))).unwrap();
    }
    o
}

#[test]
fn probe() {
    let doc = object(&[]);
    set(&doc, "createElementNS", func("createElementNS", 2, |_t, a| Ok(element(&display(&arg(a, 1)))))).unwrap();
    set(&doc, "createElement", func("createElement", 1, |_t, a| Ok(element(&display(&arg(a, 0)))))).unwrap();
    define_global("document", doc);
    three_tsl::three::order::init_all().unwrap();
    let three = module_namespace("Three.WebGPU.js");
    let renderer = construct(&get(&three, "WebGPURenderer").unwrap(), &[object(&[("antialias", Val::Bool(true))])]).unwrap();
    println!("ids after init: {}", display(&get(&construct(&get(&three, "Node").unwrap(), &[]).unwrap(), "id").unwrap()));
    let p = m(&renderer, "init", &[]);
    match p {
        Ok(v) => match await_val(v) {
            Ok(_) => {
                println!("init ok");
                let tsl = module_namespace("Three.TSL.js");
                let camera = construct(&get(&three, "PerspectiveCamera").unwrap(), &[70.0.into(), 1.0.into(), 0.1.into(), 10.0.into()]).unwrap();
                let scene = construct(&get(&three, "Scene").unwrap(), &[]).unwrap();
                let material = construct(&get(&three, "NodeMaterial").unwrap(), &[]).unwrap();
                let frag = call(&get(&tsl, "vec4").unwrap(), &Val::Undef, &[0.0.into(), 0.0.into(), 0.0.into(), 1.0.into()]).unwrap();
                set(&material, "fragmentNode", frag).unwrap();
                let geo = construct(&get(&three, "PlaneGeometry").unwrap(), &[1.0.into(), 1.0.into()]).unwrap();
                let mesh = construct(&get(&three, "Mesh").unwrap(), &[geo, material]).unwrap();
                m(&scene, "add", &[mesh]).unwrap();
                match m(&renderer, "render", &[scene, camera]) {
                    Ok(_) => println!("render ok"),
                    Err(e) => println!("render err {}", error_message(&e.0)),
                }
                println!("ids after render: {}", display(&get(&construct(&get(&three, "Node").unwrap(), &[]).unwrap(), "id").unwrap()));
            }
            Err(e) => println!("init rejected: {} / {}", error_message(&e.0), display(&get(&e.0, "stack").unwrap())),
        },
        Err(e) => println!("init threw: {} / {}", error_message(&e.0), display(&get(&e.0, "stack").unwrap())),
    }
}
