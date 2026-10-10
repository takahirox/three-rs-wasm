//! `transpiler/WGSLEncoder.js`: emits WGSL from the AST.
use super::ast::*;
use super::js::*;
use super::tsl_encoder::{emit_comment, emit_extra_line, statements};

/// A JS value printed by a template literal: `undefined`, `null` or a string.
#[derive(Clone, Debug, PartialEq)]
enum Js {
    Undefined,
    Null,
    S(String),
}
impl Js {
    fn text(&self) -> &str {
        match self {
            Js::Undefined => "undefined",
            Js::Null => "null",
            Js::S(s) => s,
        }
    }
    /// `Array.prototype.join` prints undefined and null as ''.
    fn join_text(&self) -> &str {
        match self {
            Js::S(s) => s,
            _ => "",
        }
    }
    fn from_type(t: Option<String>) -> Js {
        t.map_or(Js::Null, Js::S)
    }
}

fn type_map(t: &str) -> Option<&'static str> {
    Some(match t {
        "float" => "f32",
        "int" => "i32",
        "uint" => "u32",
        "bool" => "bool",
        "vec2" => "vec2f",
        "ivec2" => "vec2i",
        "uvec2" => "vec2u",
        "bvec2" => "vec2b",
        "vec3" => "vec3f",
        "ivec3" => "vec3i",
        "uvec3" => "vec3u",
        "bvec3" => "vec3b",
        "vec4" => "vec4f",
        "ivec4" => "vec4i",
        "uvec4" => "vec4u",
        "bvec4" => "vec4b",
        "mat3" => "mat3x3<f32>",
        "mat4" => "mat4x4<f32>",
        "texture" => "texture_2d<f32>",
        "textureCube" => "texture_cube<f32>",
        "texture3D" => "texture_3d<f32>",
        _ => return None,
    })
}

/// `getWgslType( type )`.
fn wgsl_type(t: &Js) -> Js {
    match t {
        Js::S(s) => Js::S(type_map(s).map_or_else(|| s.clone(), str::to_string)),
        other => other.clone(),
    }
}

fn wgsl_lib(name: &str) -> Option<&'static str> {
    Some(match name {
        "abs" => "abs",
        "acos" => "acos",
        "asin" => "asin",
        "atan" => "atan",
        "atan2" => "atan2",
        "ceil" => "ceil",
        "clamp" => "clamp",
        "cos" => "cos",
        "cross" => "cross",
        "degrees" => "degrees",
        "distance" => "distance",
        "dot" => "dot",
        "exp" => "exp",
        "exp2" => "exp2",
        "faceforward" => "faceForward",
        "floor" => "floor",
        "fract" => "fract",
        "inverse" => "inverse",
        "inversesqrt" => "inverseSqrt",
        "length" => "length",
        "log" => "log",
        "log2" => "log2",
        "max" => "max",
        "min" => "min",
        "mix" => "mix",
        "normalize" => "normalize",
        "pow" => "pow",
        "radians" => "radians",
        "reflect" => "reflect",
        "refract" => "refract",
        "round" => "round",
        "sign" => "sign",
        "sin" => "sin",
        "smoothstep" => "smoothstep",
        "sqrt" => "sqrt",
        "step" => "step",
        "tan" => "tan",
        "transpose" => "transpose",
        "trunc" => "trunc",
        "dFdx" => "dpdx",
        "dFdy" => "dpdy",
        "fwidth" => "fwidth",
        "texture" | "texture2D" | "texture3D" | "textureCube" => "textureSample",
        "textureLod" => "textureSampleLevel",
        "texelFetch" => "textureLoad",
        "textureGrad" => "textureSampleGrad",
        "floatBitsToInt" => "bitcast<i32>",
        "floatBitsToUint" => "bitcast<u32>",
        "intBitsToFloat" | "uintBitsToFloat" => "bitcast<f32>",
        _ => return None,
    })
}

/// `String.prototype.substring( a, b )`: clamped, and swapped when a > b.
fn substring(s: &str, a: usize, b: usize) -> String {
    let n = s.chars().count();
    let (a, b) = (a.min(n), b.min(n));
    let (a, b) = if a > b { (b, a) } else { (a, b) };
    s.chars().skip(a).take(b - a).collect()
}

pub struct WgslEncoder {
    tab: String,
    uniforms: Vec<Id>,
    varyings: Vec<Id>,
    polyfills: Vec<(String, String)>,
}

impl WgslEncoder {
    pub fn new() -> Self {
        Self {
            tab: String::new(),
            uniforms: vec![],
            varyings: vec![],
            polyfills: vec![],
        }
    }

    /// `emitExpression( node )`: `''` for a missing node, None for the
    /// `undefined` a switch case's `break` returns.
    fn expr(&mut self, ast: &mut Ast, node: Option<Id>) -> Result<Js> {
        let Some(id) = node else {
            return Ok(Js::S(String::new()));
        };
        let kind = ast.kind(id).clone();
        let code = match kind {
            Kind::Accessor { property } => {
                let uniform = self.uniforms.iter().find_map(|&u| match ast.kind(u) {
                    Kind::Uniform { ty, name } if *name == property => Some(ty.clone()),
                    _ => None,
                });
                if uniform.is_some_and(|ty| !ty.contains("texture")) {
                    return Ok(Js::S(format!("uniforms.{property}")));
                }
                property
            }
            Kind::Number { value, ty } => {
                let mut code = value;
                if ty == "float" && !code.contains('.') {
                    code += ".0";
                }
                code
            }
            Kind::Operator { op, left, right } => {
                let l = self.expr(ast, left)?;
                let r = self.expr(ast, right)?;
                let mut code = format!("{} {op} {}", l.text(), r.text());
                let parent = ast.parent(id).ok_or_else(|| read_null("isAssignment"))?;
                if !ast.is_assignment(parent) && matches!(ast.kind(parent), Kind::Operator { .. }) {
                    code = format!("( {code} )");
                }
                code
            }
            Kind::FunctionCall { name, params } => self.function_call(ast, &name, &params)?,
            Kind::Return { value } => {
                let mut code = "return".to_string();
                if value.is_some() {
                    code += " ";
                    code += self.expr(ast, value)?.text();
                }
                code
            }
            Kind::Discard => "discard".into(),
            Kind::Break => {
                let parent = ast.parent(id).ok_or_else(|| read_null("isSwitchCase"))?;
                if matches!(ast.kind(parent), Kind::SwitchCase { .. }) {
                    return Ok(Js::Undefined);
                }
                "break".into()
            }
            Kind::Continue => "continue".into(),
            Kind::AccessorElements { object, elements } => {
                let mut code = self.expr(ast, object)?.text().to_string();
                for e in elements {
                    match ast.kind(e).clone() {
                        Kind::StaticElement { value } => {
                            let v = self.expr(ast, value)?;
                            code += ".";
                            code += v.text();
                        }
                        Kind::DynamicElement { value } => {
                            let v = self.expr(ast, value)?;
                            code += &format!("[{}]", v.text());
                        }
                        _ => {}
                    }
                }
                code
            }
            Kind::For {
                initialization,
                condition,
                afterthought,
                ..
            } => {
                let init = self.expr(ast, initialization)?;
                let cond = self.expr(ast, condition)?;
                let after = self.expr(ast, afterthought)?;
                let body = self.body(ast, &statements(ast, id))?;
                format!(
                    "for ( {}; {}; {} ) {{\n\n{body}\n\n{}}}",
                    init.text(),
                    cond.text(),
                    after.text(),
                    self.tab
                )
            }
            Kind::While {
                condition,
                do_while,
                ..
            } => {
                let cond = self.expr(ast, condition)?;
                let body = self.body(ast, &statements(ast, id))?;
                let t = &self.tab;
                if do_while {
                    format!(
                        "loop {{\n\n{body}\n\n{t}\tcontinuing {{\n{t}\t\tbreak if ( ! ( {} ) );\n{t}\t}}\n{t}}}",
                        cond.text()
                    )
                } else {
                    format!("while ( {} ) {{\n\n{body}\n\n{t}}}", cond.text())
                }
            }
            Kind::Switch {
                discriminant,
                cases,
            } => {
                let d = self.expr(ast, discriminant)?;
                let mut s = format!("switch ( {} ) {{\n\n", d.text());
                self.tab.push('\t');
                for c in cases {
                    let Kind::SwitchCase { body, conditions } = ast.kind(c).clone() else {
                        continue;
                    };
                    let b = self.body(ast, &body)?;
                    match conditions {
                        None => s += &format!("{t}default: {{\n\n{b}\n\n{t}}}\n\n", t = self.tab),
                        Some(conditions) => {
                            let mut cs = vec![];
                            for condition in conditions {
                                cs.push(self.expr(ast, Some(condition))?.join_text().to_string());
                            }
                            s += &format!(
                                "{t}case {}: {{\n\n{b}\n\n{t}}}\n\n",
                                cs.join(", "),
                                t = self.tab
                            );
                        }
                    }
                }
                self.tab.pop();
                s += &format!("{}}}", self.tab);
                s
            }
            Kind::VariableDeclaration { .. } => self.variables(ast, id)?,
            Kind::Uniform { .. } => {
                self.uniforms.push(id);
                String::new()
            }
            Kind::Varying { .. } => {
                self.varyings.push(id);
                String::new()
            }
            Kind::StructDefinition { name, members } => {
                let mut s = format!("struct {name} {{\n");
                for (i, m) in members.iter().enumerate() {
                    s += &format!(
                        "{}\t{}: {}",
                        self.tab,
                        m.name,
                        wgsl_type(&Js::S(m.ty.clone())).text()
                    );
                    s += if i != members.len() - 1 { ",\n" } else { "\n" };
                }
                s += &format!("{}}}", self.tab);
                s
            }
            Kind::Ternary { cond, left, right } => {
                let c = self.expr(ast, cond)?;
                let l = self.expr(ast, left)?;
                let r = self.expr(ast, right)?;
                format!("select( {}, {}, {} )", r.text(), l.text(), c.text())
            }
            Kind::Conditional {
                cond,
                body,
                else_conditional,
            } => {
                let c = self.expr(ast, cond)?;
                let b = self.body(ast, &body)?;
                let mut s = format!("if ( {} ) {{\n\n{b}\n\n{}}}", c.text(), self.tab);
                let mut current = else_conditional;
                while let Some(e) = current {
                    let Kind::Conditional {
                        cond,
                        body,
                        else_conditional,
                    } = ast.kind(e).clone()
                    else {
                        break;
                    };
                    let eb = self.body(ast, &body)?;
                    if cond.is_some() {
                        let ec = self.expr(ast, cond)?;
                        s += &format!(" else if ( {} ) {{\n\n{eb}\n\n{}}}", ec.text(), self.tab);
                    } else {
                        s += &format!(" else {{\n\n{eb}\n\n{}}}", self.tab);
                    }
                    current = else_conditional;
                }
                s
            }
            Kind::Unary { op, expression, .. } => {
                let e = self.expr(ast, expression)?;
                if op == "++" || op == "--" {
                    let o = if op == "++" { '+' } else { '-' };
                    format!("{e} = {e} {o} 1", e = e.text())
                } else {
                    format!("{op}{}", e.text())
                }
            }
            other => {
                return error(format!(
                    "THREE.WGSLEncoder: Unknown AST node type \"{}\"",
                    other.class_name()
                ));
            }
        };
        Ok(Js::S(code))
    }

    fn function_call(&mut self, ast: &mut Ast, name: &str, params: &[Id]) -> Result<String> {
        let f = wgsl_lib(name).unwrap_or(name).to_string();
        let emitted = |enc: &mut Self, ast: &mut Ast| -> Result<Vec<Js>> {
            let mut v = vec![];
            for &p in params {
                v.push(enc.expr(ast, Some(p))?);
            }
            Ok(v)
        };
        if f == "mod" {
            let snippets = emitted(self, ast)?;
            let types: Vec<Js> = params
                .iter()
                .map(|&p| Js::from_type(ast.get_type(p)))
                .collect();
            let ty = |i: usize| types.get(i).cloned().unwrap_or(Js::Undefined);
            let joined: Vec<&str> = types.iter().map(|t| t.join_text()).collect();
            let mod_name = format!("mod_{}", joined.join("_"));
            if !self.polyfills.iter().any(|(n, _)| *n == mod_name) {
                let polyfill = format!(
                    "fn {mod_name}( x: {}, y: {} ) -> {} {{\n\n\treturn x - y * floor( x / y );\n\n}}",
                    wgsl_type(&ty(0)).text(),
                    wgsl_type(&ty(1)).text(),
                    wgsl_type(&ty(0)).text()
                );
                self.polyfills.push((mod_name.clone(), polyfill));
            }
            let s: Vec<&str> = snippets.iter().map(|s| s.join_text()).collect();
            return Ok(format!("{mod_name}( {} )", s.join(", ")));
        }
        if f.starts_with("bitcast") {
            let p = emitted(self, ast)?;
            let p: Vec<&str> = p.iter().map(|s| s.join_text()).collect();
            let t0 = params
                .first()
                .map_or(Js::Undefined, |&p| Js::from_type(ast.get_type(p)));
            let t0s = t0.text().to_string();
            // /.*vec[234]/
            let vector = ["vec2", "vec3", "vec4"].iter().any(|v| t0s.contains(v));
            let mut code = if vector {
                let conversion = substring(&f, 8, f.chars().count().saturating_sub(1));
                format!("bitcast<{t0s}<{conversion}>>")
            } else {
                f.clone()
            };
            code += &format!("( {} )", p.join(","));
            return Ok(code);
        }
        if f == "array" {
            let p = emitted(self, ast)?;
            let p: Vec<&str> = p.iter().map(|s| s.join_text()).collect();
            let first = params.first().and_then(|&p| ast.get_type(p));
            return Ok(match first {
                Some(t) => format!(
                    "array<{}, {}>( {} )",
                    wgsl_type(&Js::S(t)).text(),
                    params.len(),
                    p.join(", ")
                ),
                None => format!("array( {} )", p.join(", ")),
            });
        }
        if f.starts_with("texture") {
            return self.texture_access(ast, name, params);
        }
        let p = emitted(self, ast)?;
        let p: Vec<&str> = p.iter().map(|s| s.join_text()).collect();
        let mut code = match type_map(&f) {
            Some(t) => t.to_string(),
            None => f,
        };
        if p.is_empty() {
            code += "()";
        } else {
            code += &format!("( {} )", p.join(", "));
        }
        Ok(code)
    }

    fn texture_access(&mut self, ast: &mut Ast, name: &str, params: &[Id]) -> Result<String> {
        let f = wgsl_lib(name).unwrap_or("undefined");
        let tex = self.expr(ast, params.first().copied())?.text().to_string();
        let uv = self.expr(ast, params.get(1).copied())?.text().to_string();
        let sampler = format!("{tex}_sampler");
        Ok(match name {
            "texture" | "texture2D" | "texture3D" | "textureCube" => {
                if params.len() == 3 {
                    let bias = self.expr(ast, Some(params[2]))?;
                    format!("textureSampleBias({tex}, {sampler}, {uv}, {})", bias.text())
                } else {
                    format!("{f}({tex}, {sampler}, {uv})")
                }
            }
            "textureLod" => {
                let lod = self.expr(ast, params.get(2).copied())?;
                format!("{f}({tex}, {sampler}, {uv}, {})", lod.text())
            }
            "textureGrad" => {
                let ddx = self.expr(ast, params.get(2).copied())?;
                let ddy = self.expr(ast, params.get(3).copied())?;
                format!(
                    "{f}({tex}, {sampler}, {uv}, {}, {})",
                    ddx.text(),
                    ddy.text()
                )
            }
            "texelFetch" => {
                let coords = self.expr(ast, params.get(1).copied())?;
                let lod = if params.len() > 2 {
                    self.expr(ast, Some(params[2]))?.text().to_string()
                } else {
                    "0".into()
                };
                format!("{f}({tex}, {}, {lod})", coords.text())
            }
            _ => format!("/* unsupported texture op: {name} */"),
        })
    }

    fn body(&mut self, ast: &mut Ast, body: &[Id]) -> Result<String> {
        let mut code = String::new();
        self.tab.push('\t');
        for &s in body {
            code += &emit_extra_line(ast, s, body)?;
            if let Kind::Comment { .. } = ast.kind(s) {
                code += &emit_comment(ast, s, body, &self.tab);
                continue;
            }
            let c = self.expr(ast, Some(s))?;
            if let Js::S(c) = c
                && !c.is_empty()
            {
                code += &self.tab;
                code += &c;
                if !c.ends_with('}') && !c.ends_with('{') {
                    code += ";";
                }
                code += "\n";
            }
        }
        self.tab.pop();
        code.pop();
        Ok(code)
    }

    fn variables(&mut self, ast: &mut Ast, id: Id) -> Result<String> {
        let mut declarations = vec![];
        let mut current = Some(id);
        while let Some(c) = current {
            let Kind::VariableDeclaration {
                ty,
                name,
                value,
                next,
                ..
            } = ast.kind(c).clone()
            else {
                break;
            };
            let t = wgsl_type(&Js::S(ty));
            let value_str = if value.is_some() {
                format!(" = {}", self.expr(ast, value)?.text())
            } else {
                String::new()
            };
            let keyword = if !ast.nodes[c].linker.assignments.is_empty() {
                "var"
            } else if ast.is_numeric(value) {
                "const"
            } else {
                "let"
            };
            let array = value.is_some_and(
                |v| matches!(ast.kind(v), Kind::FunctionCall { name, .. } if name == "array"),
            );
            let type_str = if array {
                String::new()
            } else {
                format!(": {}", t.text())
            };
            declarations.push(format!("{keyword} {name}{type_str}{value_str}"));
            current = next;
        }
        Ok(declarations.join(&format!(";\n{}", self.tab)))
    }

    fn function(&mut self, ast: &mut Ast, id: Id) -> Result<String> {
        let Kind::FunctionDeclaration {
            ty,
            name,
            params,
            body,
        } = ast.kind(id).clone()
        else {
            unreachable!()
        };
        let return_type = wgsl_type(&Js::S(ty));
        let mut ps = vec![];
        let mut body = body;
        for &p in &params {
            let Kind::FunctionParameter {
                ty: pty,
                name: pname,
                qualifier,
                ..
            } = ast.kind(p).clone()
            else {
                continue;
            };
            let ptype = wgsl_type(&Js::S(pty.clone()));
            if matches!(qualifier.as_deref(), Some("inout" | "out")) {
                ps.push(format!("{pname}: ptr<function, {}>", ptype.text()));
                continue;
            }
            if !ast.nodes[p].linker.assignments.is_empty() {
                let immutable = format!("{pname}_in");
                ps.push(format!("{immutable}: {}", ptype.text()));
                let accessor = ast.add(Kind::Accessor {
                    property: immutable,
                });
                let var = ast.add(Kind::VariableDeclaration {
                    ty: pty,
                    name: pname,
                    value: Some(accessor),
                    next: None,
                    needs_to_var: false,
                });
                ast.nodes[var].linker.assignments.push(var);
                body.insert(0, var);
            } else {
                ps.push(format!("{pname}: {}", ptype.text()));
            }
        }
        let params_str = if ps.is_empty() {
            String::new()
        } else {
            format!(" {} ", ps.join(", "))
        };
        let return_str = match &return_type {
            Js::S(t) if !t.is_empty() && t != "void" => format!(" -> {t}"),
            _ => String::new(),
        };
        let b = self.body(ast, &body)?;
        Ok(format!(
            "fn {name}({params_str}){return_str} {{\n\n{b}\n\n{}}}",
            self.tab
        ))
    }

    /// `emit( ast )`.
    pub fn emit(&mut self, ast: &mut Ast, program: Id) -> Result<String> {
        let header = "// Three.js Transpiler r186\n\n";
        let mut globals = String::new();
        let mut functions = String::new();
        let mut dependencies = String::new();
        let body = statements(ast, program);
        for &s in &body {
            match ast.kind(s) {
                Kind::Uniform { .. } => self.uniforms.push(s),
                Kind::Varying { .. } => self.varyings.push(s),
                _ => {}
            }
        }
        if !self.uniforms.is_empty() {
            let mut binding = 0;
            let mut members = vec![];
            let mut textures = vec![];
            for &u in &self.uniforms {
                let Kind::Uniform { ty, name } = ast.kind(u) else {
                    continue;
                };
                let t = wgsl_type(&Js::S(ty.clone()));
                if ty.contains("texture") {
                    textures.push(format!(
                        "@group(0) @binding({binding}) var {name}: {};",
                        t.text()
                    ));
                    binding += 1;
                    textures.push(format!(
                        "@group(0) @binding({binding}) var {name}_sampler: sampler;"
                    ));
                    binding += 1;
                } else {
                    members.push(format!("\t{name}: {},", t.text()));
                }
            }
            if !members.is_empty() {
                globals += "struct Uniforms {\n";
                globals += &members.join("\n");
                globals += "\n};\n";
                globals +=
                    &format!("@group(0) @binding({binding}) var<uniform> uniforms: Uniforms;\n\n");
            }
            globals += &(textures.join("\n") + "\n\n");
        }
        if !self.varyings.is_empty() {
            globals += "struct Varyings {\n";
            for (location, &v) in self.varyings.iter().enumerate() {
                if let Kind::Varying { ty, name } = ast.kind(v) {
                    globals += &format!(
                        "\t@location({location}) {name}: {},\n",
                        wgsl_type(&Js::S(ty.clone())).text()
                    );
                }
            }
            globals += "};\n\n";
        }
        for &s in &body {
            functions += &emit_extra_line(ast, s, &body)?;
            match ast.kind(s) {
                Kind::FunctionDeclaration { .. } => {
                    functions += &self.function(ast, s)?;
                    functions += "\n";
                }
                Kind::Comment { .. } => functions += &emit_comment(ast, s, &body, &self.tab),
                Kind::Uniform { .. } | Kind::Varying { .. } => {}
                _ => {
                    functions += self.expr(ast, Some(s))?.text();
                    functions += ";\n";
                }
            }
        }
        if let Some((_, value)) = self.polyfills.last() {
            dependencies = format!("{value}\n\n");
        }
        Ok(format!(
            "{header}{dependencies}{globals}{}\n",
            trim_end(&functions)
        ))
    }
}
