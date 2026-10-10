//! `transpiler/TSLEncoder.js`: emits TSL ( JS ) from the AST.
use super::ast::*;
use super::js::*;

/// The r186 `three/tsl` exports `addImport()` tests ( `TSL[ name ] !==
/// undefined` ), sorted.
const TSL_NAMES: &str = include_str!("tsl_names.txt");

fn tsl_has(name: &str) -> bool {
    TSL_NAMES.lines().any(|n| n == name)
}

fn op_fn(op: &str) -> &str {
    match op {
        "=" => "assign",
        "+" => "add",
        "-" => "sub",
        "*" => "mul",
        "/" => "div",
        "%" => "remainder",
        "<" => "lessThan",
        ">" => "greaterThan",
        "<=" => "lessThanEqual",
        ">=" => "greaterThanEqual",
        "==" => "equal",
        "!=" => "notEqual",
        "&&" => "and",
        "||" => "or",
        "^^" => "xor",
        "&" => "bitAnd",
        "|" => "bitOr",
        "^" => "bitXor",
        "<<" => "shiftLeft",
        ">>" => "shiftRight",
        "+=" => "addAssign",
        "-=" => "subAssign",
        "*=" => "mulAssign",
        "/=" => "divAssign",
        "%=" => "remainderAssign",
        "^=" => "bitXorAssign",
        "&=" => "bitAndAssign",
        "|=" => "bitOrAssign",
        "<<=" => "shiftLeftAssign",
        ">>=" => "shiftRightAssign",
        other => other,
    }
}

fn unary_fn(op: &str) -> &str {
    match op {
        "+" => "",
        "-" => "negate",
        "~" => "bitNot",
        "!" => "not",
        "++" => "increment",
        "--" => "decrement",
        _ => "undefined",
    }
}

const TEXTURE_LOOKUP: [&str; 7] = [
    "texture",
    "texture2D",
    "texture3D",
    "textureCube",
    "textureLod",
    "texelFetch",
    "textureGrad",
];

/// An insertion-ordered `Set`.
#[derive(Default)]
struct Set(Vec<String>);
impl Set {
    fn add(&mut self, s: &str) {
        if !self.has(s) {
            self.0.push(s.to_string());
        }
    }
    fn has(&self, s: &str) -> bool {
        self.0.iter().any(|v| v == s)
    }
}

pub struct TslEncoder {
    tab: String,
    imports: Set,
    global: Set,
    overloadings: Vec<(String, Vec<Id>)>,
    block: Option<Id>,
}

/// A possibly missing parameter in a template ( `undefined` ).
fn at(v: &[String], i: usize) -> &str {
    v.get(i).map_or("undefined", |s| s.as_str())
}

impl TslEncoder {
    pub fn new() -> Self {
        Self {
            tab: String::new(),
            imports: Set::default(),
            global: Set::default(),
            overloadings: vec![],
            block: None,
        }
    }
    fn add_import(&mut self, name: &str) {
        let name = name.split('.').next().unwrap_or("");
        if tsl_has(name) && !self.global.has(name) {
            self.imports.add(name);
        }
    }
    fn tab_pop(&mut self) {
        self.tab.pop();
    }

    fn emit_uniform(&mut self, ty: &str, name: &str) -> String {
        let mut code = format!("const {name} = ");
        self.global.add(name);
        match ty {
            "texture" => {
                self.add_import("texture");
                code += "texture( /* <THREE.Texture> */ )";
            }
            "cubeTexture" => {
                self.add_import("cubeTexture");
                code += "cubeTexture( /* <THREE.CubeTexture> */ )";
            }
            "texture3D" => {
                self.add_import("texture3D");
                code += "texture3D( /* <THREE.Data3DTexture> */ )";
            }
            _ => {
                self.add_import("uniform");
                code += &format!("uniform( '{ty}' )");
            }
        }
        code
    }

    pub fn emit_expression(&mut self, ast: &mut Ast, node: Option<Id>) -> Result<String> {
        let Some(id) = node else {
            return Err(read_null("isAccessor"));
        };
        let kind = ast.kind(id).clone();
        let code = match kind {
            Kind::Accessor { property } => {
                if ast.nodes[id].linker.reference.is_none() {
                    self.add_import(&property);
                }
                property
            }
            Kind::Number { value, .. } => value,
            Kind::Str { value } => format!("'{value}'"),
            Kind::Operator { op, left, right } => {
                let f = op_fn(&op).to_string();
                let l = self.emit_expression(ast, left)?;
                let r = self.emit_expression(ast, right)?;
                if ast.is_numeric(Some(id)) {
                    return Ok(format!("{l} {op} {r}"));
                }
                if is_primitive(&l) {
                    self.add_import(&f);
                    format!("{f}( {l}, {r} )")
                } else if f == "." {
                    format!("{l}{f}{r}")
                } else {
                    format!("{l}.{f}( {r} )")
                }
            }
            Kind::FunctionCall { name, params } => {
                let mut p = vec![];
                for param in params {
                    p.push(self.emit_expression(ast, Some(param))?);
                }
                if name == "array" {
                    self.add_import("array");
                    format!("array( [ {} ] )", p.join(", "))
                } else if TEXTURE_LOOKUP.contains(&name.as_str()) {
                    let mut code = format!("{}.sample( {} )", at(&p, 0), at(&p, 1));
                    match name.as_str() {
                        "texture" | "texture2D" | "texture3D" | "textureCube" => {
                            if p.len() == 3 {
                                code += &format!(".bias( {} )", p[2]);
                            }
                        }
                        "textureLod" => code += &format!(".level( {} )", at(&p, 2)),
                        "textureGrad" => code += &format!(".grad( {}, {} )", at(&p, 2), at(&p, 3)),
                        _ => code += ".setSampler( false )",
                    }
                    code
                } else {
                    self.add_import(&name);
                    let s = if p.is_empty() {
                        String::new()
                    } else {
                        format!(" {} ", p.join(", "))
                    };
                    format!("{name}({s})")
                }
            }
            Kind::Return { value } => {
                let mut code = "return".to_string();
                if value.is_some() {
                    code += " ";
                    code += &self.emit_expression(ast, value)?;
                }
                code
            }
            Kind::Discard => {
                self.add_import("Discard");
                "Discard()".into()
            }
            Kind::Break => {
                self.add_import("Break");
                "Break()".into()
            }
            Kind::Continue => {
                self.add_import("Continue");
                "Continue()".into()
            }
            Kind::AccessorElements { object, elements } => {
                let mut code = self.emit_expression(ast, object)?;
                for e in elements {
                    match ast.kind(e).clone() {
                        Kind::StaticElement { value } => {
                            code += ".";
                            code += &self.emit_expression(ast, value)?;
                        }
                        Kind::DynamicElement { value } => {
                            let v = self.emit_expression(ast, value)?;
                            if is_primitive(&v) {
                                code += &format!("[ {v} ]");
                            } else {
                                code += &format!(".element( {v} )");
                            }
                        }
                        _ => {}
                    }
                }
                code
            }
            Kind::DynamicElement { value } | Kind::StaticElement { value } => {
                self.emit_expression(ast, value)?
            }
            Kind::For { .. } => self.emit_for(ast, id)?,
            Kind::While { .. } => self.emit_while(ast, id)?,
            Kind::Switch { .. } => self.emit_switch(ast, id)?,
            Kind::VariableDeclaration { .. } => self.emit_variables(ast, id, true)?,
            Kind::Uniform { ty, name } => self.emit_uniform(&ty, &name),
            Kind::Varying { ty, name } => {
                self.add_import("varying");
                self.add_import(&ty);
                format!("const {name} = varying( {ty}(), '{name}' )")
            }
            Kind::StructDefinition { name, members } => {
                self.add_import("struct");
                let mut s = format!("const {name} = struct( {{\n");
                for (i, m) in members.iter().enumerate() {
                    s += &format!("{}\t{}: '{}'", self.tab, m.name, m.ty);
                    if i != members.len() - 1 {
                        s += ",\n";
                    }
                }
                s += &format!("\n{}}}, '{name}' )", self.tab);
                s
            }
            Kind::Ternary { cond, left, right } => {
                let c = self.emit_expression(ast, cond)?;
                let l = self.emit_expression(ast, left)?;
                let r = self.emit_expression(ast, right)?;
                self.add_import("select");
                format!("select( {c}, {l}, {r} )")
            }
            Kind::Conditional { .. } => self.emit_conditional(ast, id)?,
            Kind::Unary {
                op,
                expression,
                after,
            } => {
                let e = expression.ok_or_else(|| read_null("isNumber"))?;
                if let (Kind::Number { value, ty }, "-") = (ast.kind(e).clone(), op.as_str()) {
                    let mut code = format!("- {value}");
                    if ty != "float" {
                        code = format!("{ty}( {code} )");
                        self.add_import(&ty);
                    }
                    code
                } else {
                    let mut ty = unary_fn(&op).to_string();
                    if ast.has_assignment(id) && !after && (op == "++" || op == "--") {
                        ty += "Before";
                    }
                    let exp = self.emit_expression(ast, expression)?;
                    if is_primitive(&exp) {
                        self.add_import(&ty);
                        format!("{ty}( {exp} )")
                    } else {
                        format!("{exp}.{ty}()")
                    }
                }
            }
            other => {
                return error(format!(
                    "THREE.TSLEncoder: Unknown AST node type \"{}\"",
                    other.class_name()
                ));
            }
        };
        Ok(code)
    }

    fn emit_body(&mut self, ast: &mut Ast, body_of: Id) -> Result<String> {
        let mut code = String::new();
        self.tab.push('\t');
        let mut i = 0;
        loop {
            let body = statements(ast, body_of);
            let Some(&statement) = body.get(i) else { break };
            code += &emit_extra_line(ast, statement, &body)?;
            if let Kind::Comment { .. } = ast.kind(statement) {
                code += &emit_comment(ast, statement, &body, &self.tab);
                i += 1;
                continue;
            }
            if self
                .block
                .is_some_and(|b| matches!(ast.kind(b), Kind::SwitchCase { .. }))
                && matches!(ast.kind(statement), Kind::Break)
            {
                i += 1;
                continue;
            }
            let s = self.emit_expression(ast, Some(statement))?;
            code += &self.tab;
            code += &s;
            if !code.ends_with('}') {
                code += ";";
            }
            code += "\n";
            i += 1;
        }
        code.pop();
        self.tab_pop();
        Ok(code)
    }

    fn emit_conditional(&mut self, ast: &mut Ast, id: Id) -> Result<String> {
        let Kind::Conditional {
            cond,
            else_conditional,
            ..
        } = ast.kind(id).clone()
        else {
            unreachable!()
        };
        let c = self.emit_expression(ast, cond)?;
        let b = self.emit_body(ast, id)?;
        let mut s = format!("If( {c}, () => {{\n\n{b}\n\n{}}} )", self.tab);
        let mut current = else_conditional;
        while let Some(e) = current {
            let Kind::Conditional {
                cond,
                else_conditional,
                ..
            } = ast.kind(e).clone()
            else {
                break;
            };
            let eb = self.emit_body(ast, e)?;
            if cond.is_some() {
                let ec = self.emit_expression(ast, cond)?;
                s += &format!(".ElseIf( {ec}, () => {{\n\n{eb}\n\n{}}} )", self.tab);
            } else {
                s += &format!(".Else( () => {{\n\n{eb}\n\n{}}} )", self.tab);
            }
            current = else_conditional;
        }
        self.imports.add("If");
        Ok(s)
    }

    fn emit_loop(&mut self, ast: &mut Ast, id: Id) -> Result<String> {
        let Kind::For {
            initialization,
            condition,
            afterthought,
            ..
        } = ast.kind(id).clone()
        else {
            unreachable!()
        };
        let (init, cond, after) = (
            initialization.expect("a loop initialization"),
            condition.expect("a loop condition"),
            afterthought.expect("a loop afterthought"),
        );
        let Kind::VariableDeclaration {
            name, ty, value, ..
        } = ast.kind(init).clone()
        else {
            unreachable!()
        };
        let start = self.emit_expression(ast, value)?;
        let (condition_op, condition_right) = match ast.kind(cond) {
            Kind::Operator { op, right, .. } => (Some(op.clone()), *right),
            Kind::Ternary { right, .. } => (None, *right),
            _ => unreachable!(),
        };
        let end = self.emit_expression(ast, condition_right)?;
        let name_param = if name != "i" {
            format!(", name: '{name}'")
        } else {
            String::new()
        };
        let type_param = if ty != "int" {
            format!(", type: '{ty}'")
        } else {
            String::new()
        };
        let condition_op = condition_op.unwrap_or_else(|| "undefined".into());
        let condition_param = if condition_op != "<" {
            format!(", condition: '{condition_op}'")
        } else {
            String::new()
        };
        let mut update_param = String::new();
        match ast.kind(after).clone() {
            Kind::Unary { op, .. } => {
                if op != "++" {
                    update_param = format!(", update: '{op}'");
                }
            }
            Kind::Operator { right, .. } => {
                if right.is_some_and(|r| {
                    matches!(ast.kind(r), Kind::Accessor { .. } | Kind::Number { .. })
                }) {
                    update_param = format!(", update: {}", self.emit_expression(ast, right)?);
                } else {
                    update_param = format!(
                        ", update: ( {{ i }} ) => {}",
                        self.emit_expression(ast, Some(after))?
                    );
                }
            }
            _ => {}
        }
        let loop_params = if start == "0"
            && name_param.is_empty()
            && type_param.is_empty()
            && condition_param.is_empty()
            && update_param.is_empty()
        {
            end
        } else {
            format!(
                "{{ start: {start}, end: {end}{name_param}{type_param}{condition_param}{update_param} }}"
            )
        };
        let body = statements(ast, id);
        let used = body.iter().any(|&s| variable_used(ast, Some(s), &name));
        let callback = if used {
            format!("( {{ {name} }} ) =>")
        } else {
            "() =>".to_string()
        };
        let mut s = format!("Loop( {loop_params}, {callback} {{\n\n");
        s += &self.emit_body(ast, id)?;
        s += "\n\n";
        s += &format!("{}}} )", self.tab);
        self.imports.add("Loop");
        Ok(s)
    }

    fn emit_switch(&mut self, ast: &mut Ast, id: Id) -> Result<String> {
        let Kind::Switch {
            discriminant,
            cases,
        } = ast.kind(id).clone()
        else {
            unreachable!()
        };
        let d = self.emit_expression(ast, discriminant)?;
        self.tab.push('\t');
        let mut s = format!("Switch( {d} )\n{}", self.tab);
        let previous = self.block;
        for c in cases {
            self.block = Some(c);
            let Kind::SwitchCase { conditions, .. } = ast.kind(c).clone() else {
                continue;
            };
            let body;
            match conditions {
                Some(conditions) => {
                    let mut cs = vec![];
                    for condition in conditions {
                        cs.push(self.emit_expression(ast, Some(condition))?);
                    }
                    body = self.emit_body(ast, c)?;
                    s += &format!(".Case( {}, ", cs.join(", "));
                }
                None => {
                    body = self.emit_body(ast, c)?;
                    s += ".Default( ";
                }
            }
            s += &format!("() => {{\n\n{body}\n\n{}}} )", self.tab);
        }
        self.block = previous;
        self.tab_pop();
        self.imports.add("Switch");
        Ok(s)
    }

    fn emit_for(&mut self, ast: &mut Ast, id: Id) -> Result<String> {
        let Kind::For {
            initialization,
            condition,
            afterthought,
            ..
        } = ast.kind(id).clone()
        else {
            unreachable!()
        };
        let init_name = initialization.and_then(|i| match ast.kind(i) {
            Kind::VariableDeclaration {
                name, next: None, ..
            } => Some(name.clone()),
            _ => None,
        });
        let property = |n: Option<Id>| match n.map(|n| ast.kind(n)) {
            Some(Kind::Accessor { property }) => Some(property.clone()),
            _ => None,
        };
        let loop_like = init_name.as_ref().is_some_and(|name| {
            let condition_left = condition.and_then(|c| match ast.kind(c) {
                Kind::Operator { left, .. } | Kind::Ternary { left, .. } => *left,
                _ => None,
            });
            let condition_ok = property(condition_left).as_ref() == Some(name);
            let after_ok = afterthought.is_some_and(|a| match ast.kind(a) {
                Kind::Unary { expression, .. } => {
                    expression.is_some() && property(*expression).as_ref() == Some(name)
                }
                Kind::Operator { left, .. } => {
                    left.is_some() && property(*left).as_ref() == Some(name)
                }
                _ => false,
            });
            condition_ok && after_ok
        });
        if loop_like {
            return self.emit_loop(ast, id);
        }
        self.emit_for_while(ast, id)
    }

    fn emit_for_while(&mut self, ast: &mut Ast, id: Id) -> Result<String> {
        let Kind::For {
            initialization,
            condition,
            afterthought,
            ..
        } = ast.kind(id).clone()
        else {
            unreachable!()
        };
        let init = self.emit_expression(ast, initialization)?;
        let cond = self.emit_expression(ast, condition)?;
        let after = self.emit_expression(ast, afterthought)?;
        self.tab.push('\t');
        let mut s = format!("{{\n\n{}{init};\n\n", self.tab);
        s += &format!("{}Loop( {cond}, () => {{\n\n", self.tab);
        s += &self.emit_body(ast, id)?;
        s += "\n\n";
        s += &format!("{}\t{after};\n\n", self.tab);
        s += &format!("{}}} )\n\n", self.tab);
        self.tab_pop();
        s += &format!("{}}}", self.tab);
        self.imports.add("Loop");
        Ok(s)
    }

    fn emit_while(&mut self, ast: &mut Ast, id: Id) -> Result<String> {
        let Kind::While {
            condition,
            do_while,
            ..
        } = ast.kind(id).clone()
        else {
            unreachable!()
        };
        if do_while {
            let c = self.emit_expression(ast, condition)?;
            let b = self.emit_body(ast, id)?;
            let t = self.tab.clone();
            let mut s = format!(
                "Loop( () => {{\n\n{b}\n\n{t}\tIf( {c}.not(), () => {{\n\n{t}\t\tBreak();\n\n{t}\t}} );\n\n"
            );
            s += &format!("{t}}} )");
            self.imports.add("Loop");
            self.imports.add("If");
            self.imports.add("Break");
            return Ok(s);
        }
        let c = self.emit_expression(ast, condition)?;
        let mut s = format!("Loop( {c}, () => {{\n\n");
        s += &self.emit_body(ast, id)?;
        s += "\n\n";
        s += &format!("{}}} )", self.tab);
        self.imports.add("Loop");
        Ok(s)
    }

    fn emit_variables(&mut self, ast: &mut Ast, id: Id, root: bool) -> Result<String> {
        let Kind::VariableDeclaration {
            ty,
            name,
            value,
            next,
            needs_to_var,
            ..
        } = ast.kind(id).clone()
        else {
            unreachable!()
        };
        let mut s = if root {
            "const ".to_string()
        } else {
            String::new()
        };
        s += &name;
        if value.is_some() {
            let mut v = self.emit_expression(ast, value)?;
            if ast.is_numeric(value) {
                v = format!("{ty}( {v} )");
                self.add_import(&ty);
            }
            s += " = ";
            s += &v;
        } else {
            let program = ast.program(id).ok_or_else(|| read_null("structTypes"))?;
            let _ = program;
            if ast.struct_types.contains(&ty) {
                s += &format!(" = {ty}()");
            } else {
                s += &format!(" = property( '{ty}' )");
                self.add_import("property");
            }
        }
        if let Some(n) = next {
            s += ", ";
            s += &self.emit_variables(ast, n, false)?;
        }
        if needs_to_var {
            s += ".toVar()";
        }
        Ok(s)
    }

    fn emit_function(&mut self, ast: &mut Ast, id: Id) -> Result<String> {
        let Kind::FunctionDeclaration {
            ty, name, params, ..
        } = ast.kind(id).clone()
        else {
            unreachable!()
        };
        let mut names = vec![];
        let mut inputs = vec![];
        let mut mutable = vec![];
        let mut has_pointer = false;
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
            let mut n = pname.clone();
            if !ast.nodes[p].linker.assignments.is_empty() {
                n += "_immutable";
                mutable.push(p);
            }
            if matches!(qualifier.as_deref(), Some("inout" | "out")) {
                has_pointer = true;
            }
            inputs.push(format!("{pname}: '{pty}'"));
            names.push(n);
        }
        for p in mutable {
            let Kind::FunctionParameter {
                ty: pty,
                name: pname,
                ..
            } = ast.kind(p).clone()
            else {
                continue;
            };
            let accessor = ast.add(Kind::Accessor {
                property: format!("{pname}_immutable"),
            });
            let declaration = ast.add(Kind::VariableDeclaration {
                ty: pty,
                name: pname,
                value: Some(accessor),
                next: None,
                needs_to_var: true,
            });
            ast.nodes[declaration].parent = ast.parent(p);
            ast.nodes[declaration].linker.assignments.push(declaration);
            if let Kind::FunctionDeclaration { body, .. } = ast.kind_mut(id) {
                body.insert(0, declaration);
            }
        }
        let params_str = if names.is_empty() {
            String::new()
        } else {
            format!(" [ {} ] ", names.join(", "))
        };
        let body = self.emit_body(ast, id)?;
        let mut fn_name = name.clone();
        let mut overloading_nodes = None;
        if let Some((_, list)) = self.overloadings.iter().find(|(n, _)| *n == name)
            && list.len() > 1
        {
            let index = list.iter().position(|&n| n == id).unwrap_or(0);
            fn_name += &format!("_{index}");
            if index == list.len() - 1 {
                overloading_nodes = Some(list.clone());
            }
        }
        let mut s = format!(
            "export const {fn_name} = /*@__PURE__*/ Fn( ({params_str}) => {{\n\n{body}\n\n{}}}",
            self.tab
        );
        if !has_pointer {
            let inputs = if inputs.is_empty() {
                String::new()
            } else {
                inputs.join(", ") + ", "
            };
            s += &format!(", {{ {inputs}return: '{ty}' }}");
        }
        s += " );\n";
        self.imports.add("Fn");
        self.global.add(&name);
        if let Some(nodes) = overloading_nodes {
            self.add_import("overloadingFn");
            let list: Vec<String> = (0..nodes.len()).map(|i| format!("{name}_{i}")).collect();
            s += &format!(
                "\nexport const {name} = /*@__PURE__*/ overloadingFn( [ {} ] );\n",
                list.join(", ")
            );
        }
        Ok(s)
    }

    /// `emit( ast )`.
    pub fn emit(&mut self, ast: &mut Ast, program: Id) -> Result<String> {
        let mut code = "\n".to_string();
        let body = statements(ast, program);
        for &s in &body {
            if let Kind::FunctionDeclaration { name, .. } = ast.kind(s) {
                match self.overloadings.iter_mut().find(|(n, _)| n == name) {
                    Some((_, list)) => list.push(s),
                    None => self.overloadings.push((name.clone(), vec![s])),
                }
            }
        }
        for &s in &body {
            code += &emit_extra_line(ast, s, &body)?;
            if let Kind::Comment { .. } = ast.kind(s) {
                code += &emit_comment(ast, s, &body, &self.tab);
                continue;
            }
            if ast.is_function_declaration(s) {
                code += &self.tab.clone();
                code += &self.emit_function(ast, s)?;
            } else {
                code += &self.tab.clone();
                code += &self.emit_expression(ast, Some(s))?;
                code += ";\n";
            }
        }
        let mut header = "// Three.js Transpiler r186\n\n".to_string();
        if !self.imports.0.is_empty() {
            header += &format!(
                "import {{ {} }} from 'three/tsl';\n",
                self.imports.0.join(", ")
            );
        }
        Ok(header + &code)
    }
}

/// The statement list a body-owning node holds.
pub fn statements(ast: &Ast, id: Id) -> Vec<Id> {
    match ast.kind(id) {
        Kind::Program { body }
        | Kind::FunctionDeclaration { body, .. }
        | Kind::Conditional { body, .. }
        | Kind::For { body, .. }
        | Kind::While { body, .. }
        | Kind::SwitchCase { body, .. } => body.clone(),
        _ => vec![],
    }
}

/// `isVariableUsed( node, varName )` over a node's fields, skipping `parent`.
fn variable_used(ast: &Ast, node: Option<Id>, name: &str) -> bool {
    let Some(id) = node else { return false };
    let one = |v: &Option<Id>| variable_used(ast, *v, name);
    let many = |v: &[Id]| v.iter().any(|&c| variable_used(ast, Some(c), name));
    match ast.kind(id) {
        Kind::Accessor { property } => property == name,
        Kind::Program { body } => many(body),
        Kind::VariableDeclaration { value, next, .. } => one(value) || one(next),
        Kind::FunctionDeclaration { params, body, .. } => many(params) || many(body),
        Kind::Ternary { cond, left, right } => one(cond) || one(left) || one(right),
        Kind::Operator { left, right, .. } => one(left) || one(right),
        Kind::Unary { expression, .. } => one(expression),
        Kind::Conditional {
            cond,
            body,
            else_conditional,
        } => one(cond) || many(body) || one(else_conditional),
        Kind::FunctionCall { params, .. } => many(params),
        Kind::Return { value } => one(value),
        Kind::StaticElement { value } | Kind::DynamicElement { value } => one(value),
        Kind::AccessorElements { object, elements } => one(object) || many(elements),
        Kind::For {
            initialization,
            condition,
            afterthought,
            body,
        } => one(initialization) || one(condition) || one(afterthought) || many(body),
        Kind::While {
            condition, body, ..
        } => one(condition) || many(body),
        Kind::Switch {
            discriminant,
            cases,
        } => one(discriminant) || many(cases),
        Kind::SwitchCase { body, conditions } => {
            many(body) || conditions.as_ref().is_some_and(|c| many(c))
        }
        _ => false,
    }
}

/// `emitComment( statement, body )`, shared by both encoders.
pub fn emit_comment(ast: &Ast, statement: Id, body: &[Id], tab: &str) -> String {
    let index = body.iter().position(|&s| s == statement).unwrap_or(0);
    let expression = |i: Option<usize>| {
        i.and_then(|i| body.get(i))
            .is_some_and(|&s| !matches!(ast.kind(s), Kind::Null) && is_expression(ast, s))
    };
    let Kind::Comment { comment } = ast.kind(statement) else {
        return String::new();
    };
    let mut out = String::new();
    if expression(index.checked_sub(1)) {
        out.push('\n');
    }
    out += tab;
    out += &comment.replace('\n', &format!("\n{tab}"));
    out.push('\n');
    if expression(Some(index + 1)) {
        out.push('\n');
    }
    out
}

/// `emitExtraLine( statement, body )`, shared by both encoders.
pub fn emit_extra_line(ast: &Ast, statement: Id, body: &[Id]) -> Result<String> {
    let index = body.iter().position(|&s| s == statement).unwrap_or(0);
    let Some(&previous) = index.checked_sub(1).and_then(|i| body.get(i)) else {
        return Ok(String::new());
    };
    if matches!(ast.kind(statement), Kind::Null) {
        return Err(read_null("isReturn"));
    }
    if let Kind::Return { .. } = ast.kind(statement) {
        return Ok("\n".into());
    }
    if matches!(ast.kind(previous), Kind::Null) {
        return Err(read_null("isFunctionDeclaration"));
    }
    let last = is_expression(ast, previous);
    let current = is_expression(ast, statement);
    Ok(if last != current || (!last && !current) {
        "\n".into()
    } else {
        String::new()
    })
}
