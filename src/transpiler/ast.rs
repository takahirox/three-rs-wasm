//! `transpiler/AST.js`: the language-independent syntax tree. Nodes live in
//! an arena and refer to each other, their parent and the linker's
//! references by index, as the JS objects refer to each other.

pub type Id = usize;

/// The linker's record on a node: the declaration an accessor refers to,
/// and the accesses and assignments of a declaration.
#[derive(Clone, Debug, Default)]
pub struct Linker {
    pub reference: Option<Id>,
    pub accesses: Vec<Id>,
    pub assignments: Vec<Id>,
}

#[derive(Clone, Debug)]
pub struct StructMember {
    pub ty: String,
    pub name: String,
}

#[derive(Clone, Debug)]
pub enum Kind {
    /// A `null` the decoder leaves in a statement list or case conditions.
    Null,
    Comment {
        comment: String,
    },
    Program {
        body: Vec<Id>,
    },
    VariableDeclaration {
        ty: String,
        name: String,
        value: Option<Id>,
        next: Option<Id>,
        /// TSLEncoder's `needsToVar`.
        needs_to_var: bool,
    },
    Uniform {
        ty: String,
        name: String,
    },
    Varying {
        ty: String,
        name: String,
    },
    FunctionParameter {
        ty: String,
        name: String,
        qualifier: Option<String>,
    },
    FunctionDeclaration {
        ty: String,
        name: String,
        params: Vec<Id>,
        body: Vec<Id>,
    },
    Ternary {
        cond: Option<Id>,
        left: Option<Id>,
        right: Option<Id>,
    },
    Operator {
        op: String,
        left: Option<Id>,
        right: Option<Id>,
    },
    Unary {
        op: String,
        expression: Option<Id>,
        after: bool,
    },
    Number {
        value: String,
        ty: String,
    },
    Str {
        value: String,
    },
    Conditional {
        cond: Option<Id>,
        body: Vec<Id>,
        else_conditional: Option<Id>,
    },
    FunctionCall {
        name: String,
        params: Vec<Id>,
    },
    Return {
        value: Option<Id>,
    },
    Discard,
    Continue,
    Break,
    Accessor {
        property: String,
    },
    StaticElement {
        value: Option<Id>,
    },
    DynamicElement {
        value: Option<Id>,
    },
    AccessorElements {
        object: Option<Id>,
        elements: Vec<Id>,
    },
    For {
        initialization: Option<Id>,
        condition: Option<Id>,
        afterthought: Option<Id>,
        body: Vec<Id>,
    },
    While {
        condition: Option<Id>,
        body: Vec<Id>,
        do_while: bool,
    },
    Switch {
        discriminant: Option<Id>,
        cases: Vec<Id>,
    },
    SwitchCase {
        body: Vec<Id>,
        conditions: Option<Vec<Id>>,
    },
    StructDefinition {
        name: String,
        members: Vec<StructMember>,
    },
}

impl Kind {
    /// The JS class name, as error messages print `constructor.name`.
    pub fn class_name(&self) -> &'static str {
        match self {
            Kind::Null => "null",
            Kind::Comment { .. } => "Comment",
            Kind::Program { .. } => "Program",
            Kind::VariableDeclaration { .. } => "VariableDeclaration",
            Kind::Uniform { .. } => "Uniform",
            Kind::Varying { .. } => "Varying",
            Kind::FunctionParameter { .. } => "FunctionParameter",
            Kind::FunctionDeclaration { .. } => "FunctionDeclaration",
            Kind::Ternary { .. } => "Ternary",
            Kind::Operator { .. } => "Operator",
            Kind::Unary { .. } => "Unary",
            Kind::Number { .. } => "Number",
            Kind::Str { .. } => "String",
            Kind::Conditional { .. } => "Conditional",
            Kind::FunctionCall { .. } => "FunctionCall",
            Kind::Return { .. } => "Return",
            Kind::Discard => "Discard",
            Kind::Continue => "Continue",
            Kind::Break => "Break",
            Kind::Accessor { .. } => "Accessor",
            Kind::StaticElement { .. } => "StaticElement",
            Kind::DynamicElement { .. } => "DynamicElement",
            Kind::AccessorElements { .. } => "AccessorElements",
            Kind::For { .. } => "For",
            Kind::While { .. } => "While",
            Kind::Switch { .. } => "Switch",
            Kind::SwitchCase { .. } => "SwitchCase",
            Kind::StructDefinition { .. } => "StructDefinition",
        }
    }
    /// The node's own children in the order `initialize()` visits its fields
    /// ( the constructor's assignment order ).
    fn children(&self) -> Vec<Id> {
        let one = |v: &Option<Id>| v.iter().copied().collect::<Vec<_>>();
        match self {
            Kind::Program { body } => body.clone(),
            Kind::VariableDeclaration { value, next, .. } => [one(value), one(next)].concat(),
            Kind::FunctionDeclaration { params, body, .. } => {
                [params.clone(), body.clone()].concat()
            }
            Kind::Ternary { cond, left, right } => [one(cond), one(left), one(right)].concat(),
            Kind::Operator { left, right, .. } => [one(left), one(right)].concat(),
            Kind::Unary { expression, .. } => one(expression),
            Kind::Conditional { cond, body, .. } => [one(cond), body.clone()].concat(),
            Kind::FunctionCall { params, .. } => params.clone(),
            Kind::Return { value } => one(value),
            Kind::StaticElement { value } | Kind::DynamicElement { value } => one(value),
            Kind::AccessorElements { object, elements } => [one(object), elements.clone()].concat(),
            Kind::For {
                initialization,
                condition,
                afterthought,
                body,
            } => [
                one(initialization),
                one(condition),
                one(afterthought),
                body.clone(),
            ]
            .concat(),
            Kind::While {
                condition, body, ..
            } => [one(condition), body.clone()].concat(),
            Kind::Switch {
                discriminant,
                cases,
            } => [one(discriminant), cases.clone()].concat(),
            Kind::SwitchCase { body, conditions } => {
                [body.clone(), conditions.clone().unwrap_or_default()].concat()
            }
            _ => vec![],
        }
    }
}

#[derive(Clone, Debug)]
pub struct Node {
    pub kind: Kind,
    pub parent: Option<Id>,
    pub linker: Linker,
}

#[derive(Default)]
pub struct Ast {
    pub nodes: Vec<Node>,
    /// The decoder's struct types, as `program.structTypes` holds them.
    pub struct_types: Vec<String>,
}

impl Ast {
    /// `new X( ... )`: the node, with `initialize()` making it its children's
    /// parent.
    pub fn add(&mut self, kind: Kind) -> Id {
        let id = self.nodes.len();
        let children = kind.children();
        self.nodes.push(Node {
            kind,
            parent: None,
            linker: Linker::default(),
        });
        for child in children {
            self.nodes[child].parent = Some(id);
        }
        id
    }
    pub fn kind(&self, id: Id) -> &Kind {
        &self.nodes[id].kind
    }
    pub fn kind_mut(&mut self, id: Id) -> &mut Kind {
        &mut self.nodes[id].kind
    }
    pub fn parent(&self, id: Id) -> Option<Id> {
        self.nodes[id].parent
    }
    /// `isAssignment`, defined on VariableDeclaration, Operator and Unary.
    pub fn is_assignment(&self, id: Id) -> bool {
        match self.kind(id) {
            Kind::VariableDeclaration { value, .. } => value.is_some(),
            Kind::Operator { op, .. } => matches!(
                op.as_str(),
                "=" | "+="
                    | "-="
                    | "*="
                    | "/="
                    | "%="
                    | "<<="
                    | ">>="
                    | ">>>="
                    | "&="
                    | "^="
                    | "|="
            ),
            Kind::Unary { op, .. } => op == "++" || op == "--",
            _ => false,
        }
    }
    /// `hasAssignment`: the node or one of its parents is an assignment.
    pub fn has_assignment(&self, id: Id) -> bool {
        let mut current = Some(id);
        while let Some(c) = current {
            if self.is_assignment(c) {
                return true;
            }
            current = self.parent(c);
        }
        false
    }
    /// `isNumericExpression`.
    pub fn is_numeric(&self, id: Option<Id>) -> bool {
        let Some(id) = id else { return false };
        match self.kind(id) {
            Kind::Number { .. } => true,
            Kind::Operator { left, right, .. } => self.is_numeric(*left) && self.is_numeric(*right),
            Kind::Unary { expression, .. } => {
                expression.is_some_and(|e| matches!(self.kind(e), Kind::Number { .. }))
            }
            _ => false,
        }
    }
    /// `getType()`: `null` as None.
    pub fn get_type(&self, id: Id) -> Option<String> {
        let own = |t: &str| (!t.is_empty()).then(|| t.to_string());
        match self.kind(id) {
            Kind::Operator { left, right, .. } => {
                let l = left.and_then(|l| self.get_type(l));
                let r = right.and_then(|r| self.get_type(r));
                if l == r {
                    l
                } else if to_float_type(l.as_deref()) == to_float_type(r.as_deref()) {
                    to_float_type(l.as_deref())
                } else {
                    None
                }
            }
            Kind::FunctionCall { name, .. } => {
                if is_builtin_type(name) {
                    return Some(name.clone());
                }
                self.nodes[id]
                    .linker
                    .reference
                    .and_then(|r| self.get_type(r))
            }
            Kind::Accessor { .. } => self.nodes[id]
                .linker
                .reference
                .and_then(|r| self.get_type(r)),
            Kind::VariableDeclaration { ty, .. }
            | Kind::Uniform { ty, .. }
            | Kind::Varying { ty, .. }
            | Kind::FunctionParameter { ty, .. }
            | Kind::FunctionDeclaration { ty, .. }
            | Kind::Number { ty, .. } => own(ty),
            // `this.type` holds the operator.
            Kind::Unary { op, .. } => own(op),
            _ => None,
        }
    }
    /// `getProgram()`: the root, if it is the program.
    pub fn program(&self, id: Id) -> Option<Id> {
        let mut current = id;
        while let Some(p) = self.parent(current) {
            current = p;
        }
        matches!(self.kind(current), Kind::Program { .. }).then_some(current)
    }
    pub fn is_function_declaration(&self, id: Id) -> bool {
        matches!(self.kind(id), Kind::FunctionDeclaration { .. })
    }
}

/// `TranspilerUtils.isExpression`.
pub fn is_expression(ast: &Ast, id: Id) -> bool {
    !matches!(
        ast.kind(id),
        Kind::FunctionDeclaration { .. }
            | Kind::For { .. }
            | Kind::While { .. }
            | Kind::Conditional { .. }
            | Kind::Switch { .. }
            | Kind::StructDefinition { .. }
    )
}

/// `TranspilerUtils.isPrimitive`: `/^(true|false|-?(\d|\.\d))/`.
pub fn is_primitive(value: &str) -> bool {
    if value.starts_with("true") || value.starts_with("false") {
        return true;
    }
    let s = value.strip_prefix('-').unwrap_or(value).as_bytes();
    match s {
        [d, ..] if d.is_ascii_digit() => true,
        [b'.', d, ..] if d.is_ascii_digit() => true,
        _ => false,
    }
}

/// `TranspilerUtils.isBuiltinType`:
/// `/^(void|bool|float|u?int|mat[234]|mat[234]x[234]|(u|i|b)?vec[234])$/`.
pub fn is_builtin_type(s: &str) -> bool {
    let n234 = |c: u8| matches!(c, b'2' | b'3' | b'4');
    match s {
        "void" | "bool" | "float" | "int" | "uint" => return true,
        _ => {}
    }
    let b = s.as_bytes();
    if let Some(rest) = s.strip_prefix("mat") {
        let r = rest.as_bytes();
        return (r.len() == 1 && n234(r[0]))
            || (r.len() == 3 && n234(r[0]) && r[1] == b'x' && n234(r[2]));
    }
    let v = match b.first() {
        Some(b'u' | b'i' | b'b') => &s[1..],
        _ => s,
    };
    v.len() == 4 && v.starts_with("vec") && n234(v.as_bytes()[3])
}

/// `TranspilerUtils.toFloatType`; null stays null.
pub fn to_float_type(ty: Option<&str>) -> Option<String> {
    let t = ty?;
    // `/^(i?int)$/`
    if t == "int" || t == "iint" {
        return Some("float".into());
    }
    // `/^(i|u)?vec([234])$/`
    let v = t
        .strip_prefix('i')
        .or_else(|| t.strip_prefix('u'))
        .unwrap_or(t);
    if v.len() == 4 && v.starts_with("vec") && matches!(v.as_bytes()[3], b'2' | b'3' | b'4') {
        return Some(format!("vec{}", &v[3..]));
    }
    Some(t.to_string())
}
