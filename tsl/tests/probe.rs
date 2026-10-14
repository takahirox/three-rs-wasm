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
    let p = m(&renderer, "init", &[]);
    match p {
        Ok(v) => match await_val(v) {
            Ok(_) => println!("init ok"),
            Err(e) => println!("init rejected: {} / {}", error_message(&e.0), display(&get(&e.0, "stack").unwrap())),
        },
        Err(e) => println!("init threw: {} / {}", error_message(&e.0), display(&get(&e.0, "stack").unwrap())),
    }
}
