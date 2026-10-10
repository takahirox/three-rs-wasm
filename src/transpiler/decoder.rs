//! `transpiler/GLSLDecoder.js`: the tokenizer and the parser that turns
//! GLSL into the AST.
use super::ast::*;
use super::js::*;
use std::rc::Rc;

#[derive(Clone, Copy, PartialEq, Debug)]
enum TokenType {
    Line,
    Comment,
    Number,
    String,
    Literal,
    Operator,
}

#[derive(Clone, Debug)]
struct Token {
    ty: TokenType,
    s: String,
    /// The position in the tokenized source, in characters.
    pos: usize,
    tags: Option<Vec<Token>>,
}

impl Token {
    fn is_tag(&self) -> bool {
        matches!(self.ty, TokenType::Line | TokenType::Comment)
    }
}

const UNARY: [&str; 6] = ["+", "-", "~", "!", "++", "--"];
const ARITHMETIC: [&str; 7] = ["*", "/", "%", "+", "-", "<<", ">>"];
const PRECEDENCE: [&[&str]; 14] = [
    &[","],
    &[
        "=", "+=", "-=", "*=", "/=", "%=", "^=", "&=", "|=", "<<=", ">>=",
    ],
    &["?"],
    &["||"],
    &["^^"],
    &["&&"],
    &["|"],
    &["^"],
    &["&"],
    &["==", "!="],
    &["<", ">", "<=", ">="],
    &["<<", ">>"],
    &["+", "-"],
    &["*", "/", "%"],
];
const RIGHT_TO_LEFT: [&str; 14] = [
    "=", "+=", "-=", "*=", "/=", "%=", "^=", "&=", "|=", "<<=", ">>=", ",", "?", ":",
];
const OPERATORS: [&str; 46] = [
    "<<=", ">>=", "++", "--", "<<", ">>", "+=", "-=", "*=", "/=", "%=", "&=", "^^", "^=", "|=",
    "<=", ">=", "==", "!=", "&&", "||", "(", ")", "[", "]", "{", "}", ".", ",", ";", "!", "=", "~",
    "*", "/", "%", "+", "-", "<", ">", "&", "^", "|", "?", ":", "#",
];
const SAMPLERS: [&str; 9] = [
    "sampler1D",
    "sampler2D",
    "sampler2DArray",
    "sampler2DShadow",
    "sampler2DArrayShadow",
    "isampler2D",
    "isampler2DArray",
    "usampler2D",
    "usampler2DArray",
];
const SAMPLERS_CUBE: [&str; 4] = [
    "samplerCube",
    "samplerCubeShadow",
    "usamplerCube",
    "isamplerCube",
];
const SAMPLERS_3D: [&str; 3] = ["sampler3D", "isampler3D", "usampler3D"];
const FRAG_COORD_POLYFILL: &str = "vec3 gl_FragCoord = vec3( screenCoordinate.x, screenCoordinate.y.oneMinus(), screenCoordinate.z );";

fn group_delta(s: &str) -> i32 {
    match s {
        "(" | "[" | "{" => 1,
        ")" | "]" | "}" => -1,
        _ => 0,
    }
}

/// `glslToTSL`.
fn function_name(s: &str) -> String {
    match s {
        "inversesqrt" => "inverseSqrt".into(),
        _ => s.into(),
    }
}

struct Tokenizer {
    source: Vec<char>,
    position: usize,
}

impl Tokenizer {
    /// `skip( spaceRegExp )`: `/^((\t| )\n*)+/`, repeated while it matches.
    fn skip_space(&mut self) {
        let c = &self.source;
        while self.position < c.len() && matches!(c[self.position], '\t' | ' ') {
            self.position += 1;
            while self.position < c.len() && c[self.position] == '\n' {
                self.position += 1;
            }
        }
    }
    /// The parsers in TokenParserList order: ( type, whole length, value ).
    fn match_at(&self, at: usize) -> Option<(TokenType, usize, String)> {
        let c = &self.source[at..];
        let text = |a: usize, b: usize| c[a..b].iter().collect::<String>();
        // /^\n+/
        let lines = c.iter().take_while(|&&ch| ch == '\n').count();
        if lines > 0 {
            return Some((TokenType::Line, lines, text(0, lines)));
        }
        // /^\/\*[\s\S]*?\*\//
        if c.starts_with(&['/', '*'])
            && let Some(k) =
                (2..c.len().saturating_sub(1)).find(|&k| c[k] == '*' && c[k + 1] == '/')
        {
            return Some((TokenType::Comment, k + 2, text(0, k + 2)));
        }
        // /^\/\/.*?(?=\n|$)/
        if c.starts_with(&['/', '/']) {
            let mut k = 2;
            loop {
                if k == c.len() || c[k] == '\n' {
                    return Some((TokenType::Comment, k, text(0, k)));
                }
                if is_line_terminator(c[k]) {
                    break;
                }
                k += 1;
            }
        }
        // /^((0x\w+)|(\.?\d+\.?\d*((e-?\d+)|\w)?))/
        if let Some(n) = number_length(c) {
            return Some((TokenType::Number, n, text(0, n)));
        }
        // /^(\"((?:[^"\\]|\\.)*)\")/ and its single-quoted form, group 2
        for quote in ['"', '\''] {
            if c.first() == Some(&quote) {
                let mut k = 1;
                let mut closed = false;
                while k < c.len() {
                    if c[k] == quote {
                        closed = true;
                        break;
                    }
                    if c[k] == '\\' {
                        if k + 1 < c.len() && !is_line_terminator(c[k + 1]) {
                            k += 2;
                            continue;
                        }
                        break;
                    }
                    k += 1;
                }
                if closed {
                    return Some((TokenType::String, k + 1, text(1, k)));
                }
            }
        }
        // /^[A-Za-z](\w|\.)*/
        if c.first().is_some_and(|ch| ch.is_ascii_alphabetic()) {
            let n = 1 + c[1..]
                .iter()
                .take_while(|&&ch| is_word(ch) || ch == '.')
                .count();
            return Some((TokenType::Literal, n, text(0, n)));
        }
        for op in OPERATORS {
            let o: Vec<char> = op.chars().collect();
            if c.starts_with(&o) {
                return Some((TokenType::Operator, o.len(), op.to_string()));
            }
        }
        None
    }
    fn next_token(&mut self) -> Option<Token> {
        self.skip_space();
        let (ty, length, s) = self.match_at(self.position)?;
        let token = Token {
            ty,
            s,
            pos: self.position,
            tags: None,
        };
        self.position += length;
        Some(token)
    }
    fn read_token(&mut self) -> Option<Token> {
        let mut token = self.next_token()?;
        if token.is_tag() {
            let mut tags = vec![];
            while token.is_tag() {
                tags.push(token);
                token = self.next_token()?;
            }
            token.tags = Some(tags);
        }
        Some(token)
    }
}

/// The number regular expression's match length.
fn number_length(c: &[char]) -> Option<usize> {
    if c.starts_with(&['0', 'x']) {
        let n = c[2..].iter().take_while(|&&ch| is_word(ch)).count();
        if n > 0 {
            return Some(2 + n);
        }
    }
    let mut k = 0;
    if c.first() == Some(&'.') {
        k = 1;
    }
    let digits = c[k..].iter().take_while(|ch| ch.is_ascii_digit()).count();
    if digits == 0 {
        return None;
    }
    k += digits;
    if c.get(k) == Some(&'.') {
        k += 1;
    }
    k += c[k..].iter().take_while(|ch| ch.is_ascii_digit()).count();
    // ((e-?\d+)|\w)?
    if c.get(k) == Some(&'e') {
        let mut j = k + 1;
        if c.get(j) == Some(&'-') {
            j += 1;
        }
        let d = c[j.min(c.len())..]
            .iter()
            .take_while(|ch| ch.is_ascii_digit())
            .count();
        if d > 0 {
            return Some(j + d);
        }
    }
    if c.get(k).is_some_and(|&ch| is_word(ch)) {
        k += 1;
    }
    Some(k)
}

/// `getTokensUntil( str, tokens, offset )`: up to and including the first
/// `str` at group depth 0.
fn tokens_until<'a>(s: &str, tokens: &'a [Token], offset: usize) -> &'a [Token] {
    let mut group = 0;
    for i in offset..tokens.len() {
        group += group_delta(&tokens[i].s);
        if group == 0 && tokens[i].s == s {
            return &tokens[offset..=i];
        }
    }
    &tokens[offset.min(tokens.len())..]
}

fn token(tokens: &[Token], i: usize) -> Result<&Token> {
    tokens.get(i).ok_or_else(|| read_undefined("str"))
}

pub struct Decoder {
    index: usize,
    tokens: Rc<[Token]>,
    source: Vec<char>,
    pub ast: Ast,
}

impl Decoder {
    fn get_token(&self, offset: usize) -> Option<&Token> {
        self.tokens.get(self.index + offset)
    }
    fn read_token(&mut self) -> Option<Token> {
        let t = self.tokens.get(self.index).cloned();
        self.index += 1;
        t
    }
    /// `getTokenPosition( token )`: the line and the UTF-16 column.
    fn position(&self, token: &Token) -> String {
        let before = &self.source[..token.pos.min(self.source.len())];
        let line = before.iter().filter(|&&c| c == '\n').count() + 1;
        let last = before.iter().rposition(|&c| c == '\n').map_or(0, |p| p + 1);
        let column: usize = before[last..].iter().map(|c| c.len_utf16()).sum::<usize>() + 1;
        format!(" (line {line}, column {column})")
    }
    fn read_tokens_until(&mut self, s: &str) -> Rc<[Token]> {
        let tokens = self.tokens.clone();
        let out: Rc<[Token]> = tokens_until(s, &tokens, self.index.min(tokens.len())).into();
        self.index += out.len();
        out
    }

    fn expression(&mut self, tokens: &[Token]) -> Result<Option<Id>> {
        if tokens.is_empty() {
            return Ok(None);
        }
        let first = &tokens[0];
        let last = &tokens[tokens.len() - 1];
        // The group index carries across the precedence levels, as in the
        // original.
        let mut group = 0i32;
        for operators in PRECEDENCE {
            let right_assoc = operators.iter().any(|op| RIGHT_TO_LEFT.contains(op));
            let order: Vec<usize> = if right_assoc {
                (0..tokens.len()).collect()
            } else {
                (0..tokens.len()).rev().collect()
            };
            let inverse = !right_assoc;
            for i in order {
                let t = &tokens[i];
                group += group_delta(&t.s);
                if t.ty != TokenType::Operator || i == 0 || i == tokens.len() - 1 {
                    continue;
                }
                // a * -1, a * -( b )
                if inverse && ARITHMETIC.contains(&tokens[i - 1].s.as_str()) {
                    continue;
                }
                if group == 0 && operators.contains(&t.s.as_str()) {
                    let op = t.s.clone();
                    if op == "?" {
                        let condition = &tokens[..i];
                        let left = slice(tokens_until(":", tokens, i + 1), 0, Some(-1));
                        let right = slice(tokens, (i + left.len() + 2) as isize, None);
                        let cond = self.expression(condition)?;
                        let left = self.expression(left)?;
                        let right = self.expression(right)?;
                        return Ok(Some(self.ast.add(Kind::Ternary { cond, left, right })));
                    }
                    let left = self.expression(&tokens[..i])?;
                    let right = self.expression(&tokens[i + 1..])?;
                    return Ok(Some(self.ast.add(Kind::Operator { op, left, right })));
                }
                let result = if inverse {
                    if group > 0 {
                        self.expression(&tokens[i..])?
                    } else {
                        None
                    }
                } else if group < 0 {
                    self.expression(&tokens[..i])?
                } else {
                    None
                };
                if result.is_some() {
                    return Ok(result);
                }
            }
        }
        // unary operators ( before )
        if first.ty == TokenType::Operator && UNARY.contains(&first.s.as_str()) {
            let expression = self.expression(&tokens[1..])?;
            return Ok(Some(self.ast.add(Kind::Unary {
                op: first.s.clone(),
                expression,
                after: false,
            })));
        }
        // unary operators ( after )
        if last.ty == TokenType::Operator && UNARY.contains(&last.s.as_str()) {
            let expression = self.expression(&tokens[..tokens.len() - 1])?;
            return Ok(Some(self.ast.add(Kind::Unary {
                op: last.s.clone(),
                expression,
                after: true,
            })));
        }
        // groups
        if first.s == "(" {
            let left_tokens = tokens_until(")", tokens, 0);
            let left =
                self.expression(slice(left_tokens, 1, Some(left_tokens.len() as isize - 1)))?;
            if let Some(operator) = tokens.get(left_tokens.len()) {
                let right = self.expression(slice(tokens, left_tokens.len() as isize + 1, None))?;
                return Ok(Some(self.ast.add(Kind::Operator {
                    op: operator.s.clone(),
                    left,
                    right,
                })));
            }
            return Ok(left);
        } else if first.s == "{" {
            let internal = slice(tokens, 1, Some(tokens.len() as isize - 1));
            let params = self.parameters(internal)?;
            return Ok(Some(self.ast.add(Kind::FunctionCall {
                name: "array".into(),
                params,
            })));
        }
        // primitives and accessors
        match first.ty {
            TokenType::Number => {
                let s = &first.s;
                let hex = s.starts_with("0x");
                let ty = if hex {
                    "int"
                } else if s.ends_with('u') || s.ends_with('U') {
                    "uint"
                } else if s.contains('f') || s.contains('e') || s.contains('.') {
                    "float"
                } else {
                    "int"
                };
                // str.replace( /u|U|i$/, '' )
                let chars: Vec<char> = s.chars().collect();
                let cut = (0..chars.len()).find(|&k| {
                    chars[k] == 'u' || chars[k] == 'U' || (chars[k] == 'i' && k == chars.len() - 1)
                });
                let mut value: String = chars
                    .iter()
                    .enumerate()
                    .filter(|(k, _)| Some(*k) != cut)
                    .map(|(_, c)| c)
                    .collect();
                if !hex && value.ends_with('f') {
                    value.pop();
                }
                return Ok(Some(self.ast.add(Kind::Number {
                    value,
                    ty: ty.into(),
                })));
            }
            TokenType::String => {
                return Ok(Some(self.ast.add(Kind::Str {
                    value: first.s.clone(),
                })));
            }
            TokenType::Literal => {
                match first.s.as_str() {
                    "return" => {
                        let value = self.expression(&tokens[1..])?;
                        return Ok(Some(self.ast.add(Kind::Return { value })));
                    }
                    "discard" => return Ok(Some(self.ast.add(Kind::Discard))),
                    "continue" => return Ok(Some(self.ast.add(Kind::Continue))),
                    "break" => return Ok(Some(self.ast.add(Kind::Break))),
                    _ => {}
                }
                if let Some(second) = tokens.get(1) {
                    if second.s == "(" {
                        let internal = slice(tokens_until(")", tokens, 1), 1, Some(-1));
                        let n = internal.len();
                        let params = self.parameters(internal)?;
                        let call = self.ast.add(Kind::FunctionCall {
                            name: function_name(&first.s),
                            params,
                        });
                        let access = slice(tokens, 3 + n as isize, None);
                        if !access.is_empty() {
                            let elements = self.accessor_elements(access)?;
                            return Ok(Some(self.ast.add(Kind::AccessorElements {
                                object: Some(call),
                                elements,
                            })));
                        }
                        return Ok(Some(call));
                    } else if second.s == "[" {
                        let bracket = tokens_until("]", tokens, 1);
                        if tokens.get(1 + bracket.len()).is_some_and(|t| t.s == "(") {
                            // type[N]( args... ) or type[]( args... )
                            let internal =
                                slice(tokens_until(")", tokens, 1 + bracket.len()), 1, Some(-1));
                            let params = self.parameters(internal)?;
                            return Ok(Some(self.ast.add(Kind::FunctionCall {
                                name: "array".into(),
                                params,
                            })));
                        }
                        let elements = self.accessor_elements(&tokens[1..])?;
                        let object = self.ast.add(Kind::Accessor {
                            property: first.s.clone(),
                        });
                        return Ok(Some(self.ast.add(Kind::AccessorElements {
                            object: Some(object),
                            elements,
                        })));
                    }
                }
                return Ok(Some(self.ast.add(Kind::Accessor {
                    property: first.s.clone(),
                })));
            }
            _ => {}
        }
        error(format!(
            "THREE.GLSLDecoder: Unexpected token \"{}\"{}",
            first.s,
            self.position(first)
        ))
    }

    fn accessor_elements(&mut self, tokens: &[Token]) -> Result<Vec<Id>> {
        let mut elements = vec![];
        let mut current = tokens;
        while let Some(t) = current.first() {
            if t.s == "[" {
                let accessor = tokens_until("]", current, 0);
                let element =
                    self.expression(slice(accessor, 1, Some(accessor.len() as isize - 1)))?;
                current = &current[accessor.len()..];
                elements.push(self.ast.add(Kind::DynamicElement { value: element }));
            } else if t.s == "." {
                let element = self.expression(slice(current, 1, Some(2)))?;
                current = slice(current, 2, None);
                elements.push(self.ast.add(Kind::StaticElement { value: element }));
            } else {
                return error(format!(
                    "THREE.GLSLDecoder: Unknown accessor expression token \"{}\"{}",
                    t.s,
                    self.position(t)
                ));
            }
        }
        Ok(elements)
    }

    /// `parseFunctionParametersFromTokens( tokens )`.
    fn parameters(&mut self, tokens: &[Token]) -> Result<Vec<Id>> {
        if tokens.is_empty() {
            return Ok(vec![]);
        }
        let Some(expression) = self.expression(tokens)? else {
            return error(format!(
                "THREE.GLSLDecoder: Invalid parameter expression{}",
                self.position(&tokens[0])
            ));
        };
        let mut params = vec![];
        let mut current = Some(expression);
        while let Some(c) = current {
            match self.ast.kind(c) {
                Kind::Operator { op, left, right } if op == "," => {
                    if let Some(l) = left {
                        params.push(*l);
                    }
                    current = *right;
                }
                _ => break,
            }
        }
        if let Some(c) = current {
            params.push(c);
        }
        Ok(params)
    }

    fn parse_expression(&mut self) -> Result<Option<Id>> {
        let tokens = self.read_tokens_until(";");
        self.expression(slice(&tokens, 0, Some(-1)))
    }

    fn function_params(&mut self, tokens: &[Token]) -> Result<Vec<Id>> {
        let mut params = vec![];
        let mut i = 0;
        while i < tokens.len() {
            let immutable = tokens[i].s == "const";
            if immutable {
                i += 1;
            }
            let mut qualifier = Some(token(tokens, i)?.s.clone());
            if matches!(qualifier.as_deref(), Some("in" | "out" | "inout")) {
                i += 1;
            } else {
                qualifier = None;
            }
            let ty = token(tokens, i)?.s.clone();
            i += 1;
            let name = token(tokens, i)?.s.clone();
            i += 1;
            params.push(self.ast.add(Kind::FunctionParameter {
                ty,
                name,
                qualifier,
            }));
            if tokens.get(i).is_some_and(|t| t.s != ",") {
                return error("THREE.GLSLDecoder: Expected \",\"");
            }
            i += 1;
        }
        Ok(params)
    }

    fn parse_function(&mut self) -> Result<Id> {
        let ty = self.read_token().ok_or_else(|| read_undefined("str"))?.s;
        let name = self.read_token().ok_or_else(|| read_undefined("str"))?.s;
        let params_tokens = self.read_tokens_until(")");
        let params = self.function_params(slice(
            &params_tokens,
            1,
            Some(params_tokens.len() as isize - 1),
        ))?;
        let body = self.parse_block()?;
        Ok(self.ast.add(Kind::FunctionDeclaration {
            ty,
            name,
            params,
            body,
        }))
    }

    /// `parseVariablesFromToken( tokens, type )`.
    fn variables_from_tokens(&mut self, tokens: &[Token], ty: Option<String>) -> Result<Id> {
        let mut index = 0;
        let immutable = token(tokens, 0)?.s == "const";
        if immutable {
            index += 1;
        }
        let ty = match ty.filter(|t| !t.is_empty()) {
            Some(t) => t,
            None => {
                let t = token(tokens, index)?.s.clone();
                index += 1;
                t
            }
        };
        let name = token(tokens, index)?.s.clone();
        index += 1;
        let mut current = tokens.get(index);
        let mut init = None;
        let mut next = None;
        if current.is_some_and(|t| t.s == "[") {
            let bracket = tokens_until("]", tokens, index);
            index += bracket.len();
            current = tokens.get(index);
        }
        if current.is_some() {
            let init_tokens = tokens_until(",", tokens, index);
            if init_tokens[0].s == "=" {
                let mut expression = &init_tokens[1..];
                // `expressionTokens[ expressionTokens.length - 1 ].str`
                if expression.last().ok_or_else(|| read_undefined("str"))?.s == "," {
                    expression = &expression[..expression.len() - 1];
                }
                init = self.expression(expression)?;
            }
            let next_tokens = slice(tokens, (init_tokens.len() + index) as isize - 1, None);
            if next_tokens.first().is_some_and(|t| t.s == ",") {
                next = Some(self.variables_from_tokens(&next_tokens[1..], Some(ty.clone()))?);
            }
        }
        Ok(self.ast.add(Kind::VariableDeclaration {
            ty,
            name,
            value: init,
            next,
            needs_to_var: false,
        }))
    }

    fn parse_variables(&mut self) -> Result<Id> {
        let tokens = self.read_tokens_until(";");
        self.variables_from_tokens(slice(&tokens, 0, Some(tokens.len() as isize - 1)), None)
    }

    fn parse_uniform(&mut self) -> Result<Id> {
        let tokens = self.read_tokens_until(";");
        let mut ty = token(&tokens, 1)?.s.clone();
        let name = token(&tokens, 2)?.s.clone();
        if SAMPLERS.contains(&ty.as_str()) {
            ty = "texture".into();
        } else if SAMPLERS_CUBE.contains(&ty.as_str()) {
            ty = "cubeTexture".into();
        } else if SAMPLERS_3D.contains(&ty.as_str()) {
            ty = "texture3D".into();
        }
        Ok(self.ast.add(Kind::Uniform { ty, name }))
    }

    fn parse_varying(&mut self) -> Result<Id> {
        let tokens = self.read_tokens_until(";");
        let ty = token(&tokens, 1)?.s.clone();
        let name = token(&tokens, 2)?.s.clone();
        Ok(self.ast.add(Kind::Varying { ty, name }))
    }

    fn parse_struct(&mut self) -> Result<Id> {
        let tokens = self.read_tokens_until(";");
        let name = token(&tokens, 1)?.s.clone();
        if token(&tokens, 2)?.s != "{" {
            return error("THREE.GLSLDecoder: Expected '{' after struct name ");
        }
        let mut members = vec![];
        let mut i = 3;
        while i + 2 < tokens.len() {
            let ty = token(&tokens, i)?;
            let n = token(&tokens, i + 1)?;
            if ty.ty != TokenType::Literal || n.ty != TokenType::Literal {
                return error("THREE.GLSLDecoder: Invalid struct declaration");
            }
            if token(&tokens, i + 2)?.s != ";" {
                return error("THREE.GLSLDecoder: Missing ';' after struct member name");
            }
            members.push(StructMember {
                ty: ty.s.clone(),
                name: n.s.clone(),
            });
            i += 3;
        }
        if tokens.len() < 2 || token(&tokens, tokens.len() - 2)?.s != "}" {
            return error(format!(
                "THREE.GLSLDecoder: Missing closing '}}' for struct {name}"
            ));
        }
        let id = self.ast.add(Kind::StructDefinition {
            name: name.clone(),
            members,
        });
        if !self.ast.struct_types.contains(&name) {
            self.ast.struct_types.push(name);
        }
        Ok(id)
    }

    fn parse_return(&mut self) -> Result<Id> {
        self.read_token();
        let value = self.parse_expression()?;
        Ok(self.ast.add(Kind::Return { value }))
    }

    /// A `{ ... }` block or `[ this.parseExpression() ]`, which keeps a null
    /// statement ( a `Null` node ).
    fn block_or_statement(&mut self) -> Result<Vec<Id>> {
        if self.get_token(0).ok_or_else(|| read_undefined("str"))?.s == "{" {
            self.parse_block()
        } else {
            let statement = match self.parse_expression()? {
                Some(s) => s,
                None => self.ast.add(Kind::Null),
            };
            Ok(vec![statement])
        }
    }

    fn parse_while(&mut self) -> Result<Id> {
        self.read_token();
        let tokens = self.read_tokens_until(")");
        let condition = self.expression(slice(&tokens, 1, Some(-1)))?;
        let body = self.block_or_statement()?;
        Ok(self.ast.add(Kind::While {
            condition,
            body,
            do_while: false,
        }))
    }

    fn parse_do_while(&mut self) -> Result<Id> {
        self.read_token();
        let body = self.block_or_statement()?;
        if self.get_token(0).is_some_and(|t| t.s == "while") {
            self.read_token();
        } else {
            let position = self
                .get_token(0)
                .map_or(String::new(), |t| self.position(t));
            return error(format!(
                "THREE.GLSLDecoder: Expected 'while' after 'do' block{position}"
            ));
        }
        let tokens = self.read_tokens_until(")");
        let condition = self.expression(slice(&tokens, 1, Some(-1)))?;
        if self.get_token(0).is_some_and(|t| t.s == ";") {
            self.read_token();
        }
        Ok(self.ast.add(Kind::While {
            condition,
            body,
            do_while: true,
        }))
    }

    fn parse_for(&mut self) -> Result<Id> {
        self.read_token();
        let for_tokens = self.read_tokens_until(")");
        let for_tokens = slice(&for_tokens, 1, Some(-1));
        let initialization_tokens = slice(tokens_until(";", for_tokens, 0), 0, Some(-1));
        let condition_tokens = slice(
            tokens_until(
                ";",
                for_tokens,
                (initialization_tokens.len() + 1).min(for_tokens.len()),
            ),
            0,
            Some(-1),
        );
        let afterthought_tokens = slice(
            for_tokens,
            (initialization_tokens.len() + condition_tokens.len() + 2) as isize,
            None,
        );
        let initialization = match initialization_tokens.first() {
            Some(t) if is_builtin_type(&t.s) || self.ast.struct_types.contains(&t.s) => {
                Some(self.variables_from_tokens(initialization_tokens, None)?)
            }
            _ => self.expression(initialization_tokens)?,
        };
        let condition = self.expression(condition_tokens)?;
        let afterthought = self.expression(afterthought_tokens)?;
        let body = self.block_or_statement()?;
        Ok(self.ast.add(Kind::For {
            initialization,
            condition,
            afterthought,
            body,
        }))
    }

    fn parse_switch(&mut self) -> Result<Id> {
        self.read_token();
        let tokens = self.read_tokens_until(")");
        let discriminant = self.expression(slice(&tokens, 1, Some(-1)))?;
        if self.get_token(0).ok_or_else(|| read_undefined("str"))?.s != "{" {
            return error("THREE.GLSLDecoder: Expected '{' after switch(...) ");
        }
        self.read_token();
        let cases = self.parse_switch_cases()?;
        Ok(self.ast.add(Kind::Switch {
            discriminant,
            cases,
        }))
    }

    fn parse_switch_cases(&mut self) -> Result<Vec<Id>> {
        let mut cases = vec![];
        let mut conditions: Option<Vec<Id>> = None;
        let is_case = |t: Option<&Token>| -> Result<bool> {
            let t = t.ok_or_else(|| read_undefined("str"))?;
            Ok(t.s == "case" || t.s == "default")
        };
        let mut current = self.get_token(0).cloned();
        while is_case(current.as_ref())? {
            let t = current.clone().expect("a case token");
            self.read_token();
            if t.s == "case" {
                let case_tokens = self.read_tokens_until(":");
                let c = match self.expression(slice(&case_tokens, 0, Some(-1)))? {
                    Some(c) => c,
                    None => self.ast.add(Kind::Null),
                };
                conditions.get_or_insert_with(Vec::new).push(c);
            } else {
                self.read_tokens_until(":");
                conditions = None;
            }
            current = self.get_token(0).cloned();
            if is_case(current.as_ref())? {
                continue;
            }
            let body = self.parse_block()?;
            cases.push(self.ast.add(Kind::SwitchCase {
                body,
                conditions: conditions.take(),
            }));
            current = self.get_token(0).cloned();
        }
        Ok(cases)
    }

    fn parse_if(&mut self) -> Result<Id> {
        let cond = self.if_expression()?;
        let body = self.block_or_statement()?;
        let conditional = self.ast.add(Kind::Conditional {
            cond,
            body,
            else_conditional: None,
        });
        let mut current = conditional;
        while self.get_token(0).is_some_and(|t| t.s == "else") {
            self.read_token();
            let previous = current;
            let mut expression = None;
            if self.get_token(0).ok_or_else(|| read_undefined("str"))?.s == "if" {
                expression = self.if_expression()?;
            }
            let body = self.block_or_statement()?;
            current = self.ast.add(Kind::Conditional {
                cond: expression,
                body,
                else_conditional: None,
            });
            self.ast.nodes[current].parent = Some(previous);
            if let Kind::Conditional {
                else_conditional, ..
            } = self.ast.kind_mut(previous)
            {
                *else_conditional = Some(current);
            }
        }
        Ok(conditional)
    }

    fn if_expression(&mut self) -> Result<Option<Id>> {
        self.read_token();
        let tokens = self.read_tokens_until(")");
        self.expression(slice(&tokens, 1, Some(tokens.len() as isize - 1)))
    }

    fn parse_block(&mut self) -> Result<Vec<Id>> {
        let mut body = vec![];
        if self.get_token(0).ok_or_else(|| read_undefined("str"))?.s == "{" {
            self.read_token();
        }
        let mut group = 0;
        while self.index < self.tokens.len() {
            let t = self.tokens[self.index].clone();
            group += group_delta(&t.s);
            if group == 0 && (t.s == "case" || t.s == "default") {
                return Ok(body);
            } else if group < 0 {
                self.read_token();
                return Ok(body);
            }
            if let Some(tags) = &t.tags {
                let mut last: Option<Id> = None;
                for tag in tags {
                    if tag.ty == TokenType::Comment {
                        let s = tag.s.replace('\t', "");
                        match last {
                            Some(id) => {
                                if let Kind::Comment { comment } = self.ast.kind_mut(id) {
                                    comment.push('\n');
                                    comment.push_str(&s);
                                }
                            }
                            None => {
                                let id = self.ast.add(Kind::Comment { comment: s });
                                body.push(id);
                                last = Some(id);
                            }
                        }
                    }
                }
            }
            let mut statement = None;
            if matches!(t.ty, TokenType::Literal | TokenType::Operator) {
                statement = match t.s.as_str() {
                    "const" => Some(self.parse_variables()?),
                    "uniform" => Some(self.parse_uniform()?),
                    "varying" => Some(self.parse_varying()?),
                    "struct" => Some(self.parse_struct()?),
                    s if is_builtin_type(s) || self.ast.struct_types.iter().any(|n| n == s) => {
                        if self.get_token(2).ok_or_else(|| read_undefined("str"))?.s == "(" {
                            Some(self.parse_function()?)
                        } else {
                            Some(self.parse_variables()?)
                        }
                    }
                    "return" => Some(self.parse_return()?),
                    "if" => Some(self.parse_if()?),
                    "for" => Some(self.parse_for()?),
                    "while" => Some(self.parse_while()?),
                    "do" => Some(self.parse_do_while()?),
                    "switch" => Some(self.parse_switch()?),
                    _ => self.parse_expression()?,
                };
            }
            match statement {
                Some(s) => body.push(s),
                None => self.index += 1,
            }
        }
        Ok(body)
    }
}

/// `new GLSLDecoder().parse( source )`.
pub fn parse(source: &str) -> Result<(Ast, Id)> {
    let source = super::preprocess::preprocess(source);
    let mut polyfill = String::new();
    if has_word(&source, "gl_FragCoord") {
        polyfill.push_str(FRAG_COORD_POLYFILL);
        polyfill.push('\n');
    }
    if !polyfill.is_empty() {
        polyfill = format!("// Polyfills\n\n{polyfill}\n");
    }
    let mut tokenizer = Tokenizer {
        source: (polyfill + &source).chars().collect(),
        position: 0,
    };
    let mut tokens = vec![];
    while let Some(t) = tokenizer.read_token() {
        tokens.push(t);
    }
    let mut decoder = Decoder {
        index: 0,
        tokens: tokens.into(),
        source: tokenizer.source,
        ast: Ast::default(),
    };
    let body = if decoder.tokens.is_empty() {
        return error(read_undefined("str").0);
    } else {
        decoder.parse_block()?
    };
    let program = decoder.ast.add(Kind::Program { body });
    Ok((decoder.ast, program))
}
