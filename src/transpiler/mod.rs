//! The r186 shader transpiler ( `examples/jsm/transpiler/` ): GLSLDecoder
//! parses GLSL into an AST, the Linker resolves its references, and
//! TSLEncoder or WGSLEncoder emits TSL or WGSL. The output text is the
//! original's, character for character.
mod ast;
mod decoder;
mod js;
mod linker;
mod preprocess;
mod tsl_encoder;
mod wgsl_encoder;

/// The encoders `webgpu_tsl_transpiler` offers.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Encoder {
    Tsl,
    Wgsl,
}

/// `new Transpiler( new GLSLDecoder(), encoder ).parse( glsl )`: the code, or
/// the error message the original throws.
pub fn transpile(glsl: &str, encoder: Encoder) -> Result<String, String> {
    let run = || -> js::Result<String> {
        let (mut ast, program) = decoder::parse(glsl)?;
        linker::process(&mut ast, program)?;
        match encoder {
            Encoder::Tsl => tsl_encoder::TslEncoder::new().emit(&mut ast, program),
            Encoder::Wgsl => wgsl_encoder::WgslEncoder::new().emit(&mut ast, program),
        }
    };
    run().map_err(|e| e.0)
}
