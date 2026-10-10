//! `transpiler/Linker.js`: resolves accessors to their declarations and
//! records each declaration's accesses and assignments.
use super::ast::*;
use super::js::*;
use std::collections::HashMap;

struct Block {
    node: Id,
    properties: HashMap<String, Id>,
}

pub struct Linker {
    blocks: Vec<Block>,
}

impl Linker {
    fn add_block(&mut self, node: Id) {
        self.blocks.push(Block {
            node,
            properties: HashMap::new(),
        });
    }
    fn remove_block(&mut self, node: Id) -> Result<()> {
        if self.blocks.last().is_none_or(|b| b.node != node) {
            return error("THREE.Linker: No block to remove or block mismatch.");
        }
        self.blocks.pop();
        Ok(())
    }
    fn set_property(&mut self, name: &str, node: Id) {
        if let Some(b) = self.blocks.last_mut() {
            b.properties.insert(name.to_string(), node);
        }
    }
    fn get_property(&self, name: &str) -> Option<Id> {
        self.blocks
            .iter()
            .rev()
            .find_map(|b| b.properties.get(name).copied())
    }
    /// `evalProperty( node )`: an accessor's property, else ''.
    fn eval_property(ast: &Ast, node: Option<Id>) -> String {
        match node.map(|n| ast.kind(n)) {
            Some(Kind::Accessor { property }) => property.clone(),
            _ => String::new(),
        }
    }

    fn expression(&mut self, ast: &mut Ast, node: Option<Id>) -> Result<()> {
        let Some(id) = node else {
            return Err(read_null("isAccessor"));
        };
        match ast.kind(id).clone() {
            Kind::Null => return Err(read_null("isAccessor")),
            Kind::Accessor { property } => {
                if let Some(p) = self.get_property(&property) {
                    ast.nodes[id].linker.reference = Some(p);
                    ast.nodes[p].linker.accesses.push(id);
                }
            }
            Kind::Number { .. } | Kind::Str { .. } => {}
            Kind::Operator { left, right, .. } => {
                self.expression(ast, left)?;
                self.expression(ast, right)?;
                if ast.is_assignment(id)
                    && let Some(p) = self.get_property(&Self::eval_property(ast, left))
                {
                    ast.nodes[p].linker.assignments.push(id);
                }
            }
            Kind::FunctionCall { params, .. } => {
                for p in params {
                    self.expression(ast, Some(p))?;
                }
            }
            Kind::Return { value } => {
                if value.is_some() {
                    self.expression(ast, value)?;
                }
            }
            Kind::Discard | Kind::Break | Kind::Continue => {}
            Kind::AccessorElements { object, elements } => {
                self.expression(ast, object)?;
                for e in elements {
                    let value = match ast.kind(e) {
                        Kind::StaticElement { value } | Kind::DynamicElement { value } => *value,
                        _ => None,
                    };
                    self.expression(ast, value)?;
                }
            }
            Kind::StaticElement { value } | Kind::DynamicElement { value } => {
                self.expression(ast, value)?
            }
            Kind::For {
                initialization,
                condition,
                afterthought,
                body,
            } => {
                if initialization.is_some() {
                    self.expression(ast, initialization)?;
                }
                if condition.is_some() {
                    self.expression(ast, condition)?;
                }
                if afterthought.is_some() {
                    self.expression(ast, afterthought)?;
                }
                self.body(ast, &body)?;
            }
            Kind::While {
                condition, body, ..
            } => {
                if condition.is_some() {
                    self.expression(ast, condition)?;
                }
                self.body(ast, &body)?;
            }
            Kind::Switch {
                discriminant,
                cases,
            } => {
                self.expression(ast, discriminant)?;
                for c in cases {
                    if let Kind::SwitchCase { body, conditions } = ast.kind(c).clone() {
                        for condition in conditions.unwrap_or_default() {
                            self.expression(ast, Some(condition))?;
                        }
                        self.body(ast, &body)?;
                    }
                }
            }
            Kind::VariableDeclaration { name, value, .. } => {
                self.set_property(&name, id);
                if value.is_some() {
                    self.expression(ast, value)?;
                }
            }
            Kind::Uniform { name, .. } | Kind::Varying { name, .. } => self.set_property(&name, id),
            Kind::Ternary { cond, left, right } => {
                self.expression(ast, cond)?;
                self.expression(ast, left)?;
                self.expression(ast, right)?;
            }
            Kind::Conditional {
                cond,
                body,
                else_conditional,
            } => {
                self.expression(ast, cond)?;
                self.body(ast, &body)?;
                let mut current = else_conditional;
                while let Some(c) = current {
                    let Kind::Conditional {
                        cond,
                        body,
                        else_conditional,
                    } = ast.kind(c).clone()
                    else {
                        break;
                    };
                    if cond.is_some() {
                        self.expression(ast, cond)?;
                    }
                    self.body(ast, &body)?;
                    current = else_conditional;
                }
            }
            Kind::Unary { expression, .. } => {
                self.expression(ast, expression)?;
                if ast.is_assignment(id) {
                    let parent = ast.parent(id).ok_or_else(|| read_null("hasAssignment"))?;
                    // Optimize a statement increment / decrement: no new
                    // variable.
                    if !ast.has_assignment(parent)
                        && let Kind::Unary { after, .. } = ast.kind_mut(id)
                    {
                        *after = false;
                    }
                    if let Some(p) = self.get_property(&Self::eval_property(ast, expression)) {
                        ast.nodes[p].linker.assignments.push(id);
                    }
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn body(&mut self, ast: &mut Ast, body: &[Id]) -> Result<()> {
        for &s in body {
            self.expression(ast, Some(s))?;
        }
        Ok(())
    }

    fn function(&mut self, ast: &mut Ast, id: Id) -> Result<()> {
        self.add_block(id);
        if let Kind::FunctionDeclaration { params, body, .. } = ast.kind(id).clone() {
            for p in params {
                if let Kind::FunctionParameter { name, .. } = ast.kind(p) {
                    let name = name.clone();
                    self.set_property(&name, p);
                }
            }
            self.body(ast, &body)?;
        }
        self.remove_block(id)
    }
}

/// `new Linker().process( ast )`.
pub fn process(ast: &mut Ast, program: Id) -> Result<()> {
    let mut linker = Linker { blocks: vec![] };
    linker.add_block(program);
    let Kind::Program { body } = ast.kind(program).clone() else {
        return Ok(());
    };
    for s in body {
        if ast.is_function_declaration(s) {
            linker.function(ast, s)?;
        } else {
            linker.expression(ast, Some(s))?;
        }
    }
    linker.remove_block(program)
}
