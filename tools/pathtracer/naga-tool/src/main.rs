use std::io::Read;
fn main() {
    let stage = match std::env::args().nth(1).as_deref() {
        Some("vert") => naga::ShaderStage::Vertex,
        _ => naga::ShaderStage::Fragment,
    };
    let mut src = String::new();
    std::io::stdin().read_to_string(&mut src).unwrap();
    let mut front = naga::front::glsl::Frontend::default();
    let module = match front.parse(&naga::front::glsl::Options::from(stage), &src) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("{}", e.emit_to_string(&src));
            std::process::exit(1);
        }
    };
    let info = naga::valid::Validator::new(naga::valid::ValidationFlags::all(), naga::valid::Capabilities::all())
        .validate(&module)
        .unwrap_or_else(|e| {
            eprintln!("validate: {e:?}");
            std::process::exit(2);
        });
    let wgsl = naga::back::wgsl::write_string(&module, &info, naga::back::wgsl::WriterFlags::empty()).unwrap();
    print!("{wgsl}");
}
