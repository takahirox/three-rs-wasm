//! A pipeline stage's own WGSL, as three's NodeBuilder writes each material's
//! shaders with only the code its configuration uses: the material shader's
//! override constants become `const` declarations with the pipeline's values,
//! the branches those constants decide are resolved, and only the declarations
//! the stage's entry point reaches are kept.
//!
//! The renderer's material shader ( src/shader.wgsl and the custom programs
//! built on it ) holds every material feature and one vertex entry point per
//! set of attributes; compiled whole, each pipeline costs the driver the
//! compilation of all of them.

use std::collections::{HashMap, HashSet};

/// `source` for one stage: the declarations `entries` reach, with `constants`
/// ( an override's name and value, as `PipelineCompilationOptions::constants`
/// takes them ) in place of the overrides and the other overrides at their
/// defaults.
pub(crate) fn specialize(source: &str, entries: &[&str], constants: &[(&str, f64)]) -> String {
    let source = without_comments(source);
    let values: HashMap<&str, f64> = constants.iter().copied().collect();
    let mut items: Vec<Item> = split(&source)
        .into_iter()
        .map(|text| Item::new(text, &values))
        .collect();
    let mut known: HashMap<String, Value> = items
        .iter()
        .filter_map(|item| Some((item.name.clone()?, item.constant?)))
        .collect();
    // Functions the constants reduce to returning one ( `material_kind()` with
    // MATERIAL_KIND set ) decide the branches that call them, in the next pass.
    loop {
        for item in &mut items {
            if item.function {
                item.text = prune(&item.text, &known);
            }
        }
        let decided: Vec<(String, Value)> = items
            .iter()
            .filter(|item| item.function)
            .filter_map(|item| {
                let call = format!("{}()", item.name.as_ref()?);
                (!known.contains_key(&call)).then_some(())?;
                Some((call, returned(&item.text, &known)?))
            })
            .collect();
        if decided.is_empty() {
            break;
        }
        known.extend(decided);
    }
    let mut index: HashMap<&str, Vec<usize>> = HashMap::new();
    for (i, item) in items.iter().enumerate() {
        if let Some(name) = &item.name {
            index.entry(name).or_default().push(i);
        }
    }
    let mut kept = vec![false; items.len()];
    let mut queue: Vec<usize> = Vec::new();
    for (i, item) in items.iter().enumerate() {
        if item.name.is_none() {
            kept[i] = true;
            queue.push(i);
        }
    }
    for entry in entries {
        for &i in index.get(entry).into_iter().flatten() {
            if !kept[i] {
                kept[i] = true;
                queue.push(i);
            }
        }
    }
    while let Some(i) = queue.pop() {
        let referenced: Vec<usize> = identifiers(&items[i].text)
            .filter_map(|name| index.get(name))
            .flatten()
            .copied()
            .collect();
        for j in referenced {
            if !kept[j] {
                kept[j] = true;
                queue.push(j);
            }
        }
    }
    let mut out = String::with_capacity(source.len() / 2);
    for (item, kept) in items.iter().zip(kept) {
        if kept {
            out.push_str(item.text.trim());
            out.push('\n');
        }
    }
    out
}

/// A module-scope declaration or directive.
struct Item {
    text: String,
    /// The name it declares; directives and text that is not a declaration
    /// ( which the GPU reports ) have none and are always kept.
    name: Option<String>,
    function: bool,
    /// A specialized `override`'s value. Only these decide branches: a
    /// function's own names may shadow other module-scope constants.
    constant: Option<Value>,
}

impl Item {
    fn new(text: &str, values: &HashMap<&str, f64>) -> Self {
        let mut rest = skip_attributes(text.trim_start());
        let keyword = leading_identifier(rest).unwrap_or("");
        rest = rest[keyword.len()..].trim_start();
        let named = match keyword {
            "fn" | "struct" | "const" | "override" | "alias" => true,
            "var" => {
                if rest.starts_with('<') {
                    rest = rest[rest.find('>').map_or(rest.len(), |e| e + 1)..].trim_start();
                }
                true
            }
            _ => false,
        };
        let name = named.then(|| leading_identifier(rest)).flatten();
        let mut item = Item {
            text: text.to_owned(),
            name: name.map(str::to_owned),
            function: keyword == "fn",
            constant: None,
        };
        if let (Some(name), "override") = (name, keyword) {
            // name : type = value ;
            let declaration = &rest[name.len()..];
            let ty = declaration
                .split_once(':')
                .and_then(|(_, t)| t.split(['=', ';']).next())
                .map(str::trim);
            let default = declaration
                .split_once('=')
                .map(|(_, v)| v.trim().trim_end_matches(';').trim());
            let value = values
                .get(name)
                .copied()
                .or_else(|| default.and_then(literal));
            if let (Some(ty), Some(value)) = (ty, value)
                && let Some(written) = write_literal(ty, value)
            {
                item.text = format!("const {name}:{ty}={written};");
                item.constant = Some(if ty == "bool" {
                    Value::Bool(value != 0.0)
                } else {
                    Value::Number(value)
                });
            }
        }
        item
    }
}

fn write_literal(ty: &str, value: f64) -> Option<String> {
    Some(match ty {
        "bool" => (value != 0.0).to_string(),
        "f32" => format!("{value:?}"),
        "i32" => format!("{}i", value as i64),
        "u32" => format!("{}u", value as u64),
        _ => return None,
    })
}

/// A bool or scalar literal's value: true is 1.
fn literal(text: &str) -> Option<f64> {
    match text {
        "true" => Some(1.0),
        "false" => Some(0.0),
        _ => text
            .trim_end_matches(['u', 'i', 'f', 'h'])
            .parse::<f64>()
            .ok(),
    }
}

/// The source without its comments ( `//` lines and nested `/* */` blocks ).
fn without_comments(source: &str) -> String {
    let bytes = source.as_bytes();
    let mut out = String::with_capacity(source.len());
    let mut i = 0;
    let mut copied = 0;
    while i + 1 < bytes.len() {
        if bytes[i] == b'/' && bytes[i + 1] == b'/' {
            out.push_str(&source[copied..i]);
            i = source[i..].find('\n').map_or(bytes.len(), |e| i + e);
            copied = i;
        } else if bytes[i] == b'/' && bytes[i + 1] == b'*' {
            out.push_str(&source[copied..i]);
            let mut depth = 0;
            while i + 1 < bytes.len() {
                if bytes[i] == b'/' && bytes[i + 1] == b'*' {
                    depth += 1;
                    i += 2;
                } else if bytes[i] == b'*' && bytes[i + 1] == b'/' {
                    depth -= 1;
                    i += 2;
                    if depth == 0 {
                        break;
                    }
                } else {
                    i += 1;
                }
            }
            out.push(' ');
            copied = i.min(bytes.len());
        } else {
            i += 1;
        }
    }
    out.push_str(&source[copied.min(source.len())..]);
    out
}

/// The module-scope declarations: each ends at a `;` or at the `}` that
/// closes its body.
fn split(source: &str) -> Vec<&str> {
    let mut items = Vec::new();
    let mut depth = 0i32;
    let mut start = 0;
    for (i, c) in source.char_indices() {
        match c {
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                if depth == 0 {
                    items.push(&source[start..=i]);
                    start = i + 1;
                }
            }
            ';' if depth == 0 => {
                items.push(&source[start..=i]);
                start = i + 1;
            }
            _ => {}
        }
    }
    items.push(&source[start..]);
    items
        .into_iter()
        .filter(|item| !matches!(item.trim(), "" | ";"))
        .collect()
}

fn skip_attributes(mut text: &str) -> &str {
    while let Some(rest) = text.strip_prefix('@') {
        let name = leading_identifier(rest).unwrap_or("");
        let mut rest = rest[name.len()..].trim_start();
        if rest.starts_with('(') {
            rest = &rest[balanced(rest, '(', ')').unwrap_or(rest.len())..];
        }
        text = rest.trim_start();
    }
    text
}

fn is_identifier_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_'
}

fn leading_identifier(text: &str) -> Option<&str> {
    let end = text
        .find(|c: char| !is_identifier_char(c))
        .unwrap_or(text.len());
    (end > 0 && !text.as_bytes()[0].is_ascii_digit()).then(|| &text[..end])
}

fn identifiers(text: &str) -> impl Iterator<Item = &str> {
    text.split(|c: char| !is_identifier_char(c))
        .filter(|word| word.as_bytes().first().is_some_and(|b| !b.is_ascii_digit()))
}

/// The length of `text` up to and including the bracket that closes its first one.
fn balanced(text: &str, open: char, close: char) -> Option<usize> {
    let mut depth = 0;
    for (i, c) in text.char_indices() {
        if c == open {
            depth += 1;
        } else if c == close {
            depth -= 1;
            if depth == 0 {
                return Some(i + 1);
            }
        }
    }
    None
}

/// `text` with each `if` whose condition the constants decide replaced by
/// the statements it runs.
fn prune(text: &str, known: &HashMap<String, Value>) -> String {
    let mut out = String::with_capacity(text.len());
    let mut i = 0;
    while let Some(found) = find_keyword(text, i, "if") {
        out.push_str(&text[i..found]);
        let after = found + 2;
        let Some(open) = text[after..].find('{').map(|o| after + o) else {
            out.push_str(&text[found..]);
            return out;
        };
        let Some(length) = balanced(&text[open..], '{', '}') else {
            out.push_str(&text[found..]);
            return out;
        };
        let condition = &text[after..open];
        let body = &text[open..open + length];
        let mut next = open + length;
        match evaluate(condition, known) {
            Value::Bool(true) => {
                out.push_str(&prune(body, known));
                next = skip_else_chain(text, next);
            }
            Value::Bool(false) => {
                // The else branch, or the next `if` of the chain, takes its place;
                // without one, an empty statement ( after an `else` too ).
                match after_else(text, next) {
                    Some(rest) => next = rest,
                    None => out.push_str("{}"),
                }
            }
            _ => {
                out.push_str("if");
                out.push_str(condition);
                out.push_str(&prune(body, known));
            }
        }
        i = next;
    }
    out.push_str(&text[i..]);
    out
}

/// The constant a function without parameters returns, when its body is
/// only that `return` ( and the empty statements of resolved branches ).
fn returned(text: &str, known: &HashMap<String, Value>) -> Option<Value> {
    let rest = skip_attributes(text.trim_start())
        .strip_prefix("fn")?
        .trim_start();
    let name = leading_identifier(rest)?;
    let rest = rest[name.len()..]
        .trim_start()
        .strip_prefix('(')?
        .trim_start()
        .strip_prefix(')')?;
    let body = &rest[rest.find('{')?..];
    let statements: String = body[1..body.len() - 1]
        .split_whitespace()
        .collect::<String>()
        .replace("{}", "");
    let expression = statements.strip_prefix("return")?.strip_suffix(';')?;
    match evaluate(expression, known) {
        value @ (Value::Bool(_) | Value::Number(_)) => Some(value),
        _ => None,
    }
}

/// The position after `else` when an else follows `at`.
fn after_else(text: &str, at: usize) -> Option<usize> {
    let rest = text[at..].trim_start();
    let skipped = text.len() - at - rest.len();
    (rest.starts_with("else") && !rest[4..].starts_with(is_identifier_char))
        .then_some(at + skipped + 4)
}

/// The position after the else branches that follow `at`.
fn skip_else_chain(text: &str, mut at: usize) -> usize {
    while let Some(next) = after_else(text, at) {
        let Some(open) = text[next..].find('{').map(|o| next + o) else {
            return at;
        };
        let Some(length) = balanced(&text[open..], '{', '}') else {
            return at;
        };
        at = open + length;
    }
    at
}

fn find_keyword(text: &str, from: usize, keyword: &str) -> Option<usize> {
    let mut at = from;
    while let Some(offset) = text[at..].find(keyword) {
        let i = at + offset;
        let before = text[..i].chars().next_back();
        let after = text[i + keyword.len()..].chars().next();
        if !before.is_some_and(is_identifier_char) && !after.is_some_and(is_identifier_char) {
            return Some(i);
        }
        at = i + keyword.len();
    }
    None
}

/// A condition's value: decided by the constants, or decided at run time.
/// `pure` tells whether evaluating it calls a function.
#[derive(Clone, Copy, Debug, PartialEq)]
enum Value {
    Bool(bool),
    Number(f64),
    Runtime { pure: bool },
}

impl Value {
    fn pure(self) -> bool {
        !matches!(self, Value::Runtime { pure: false })
    }
    fn runtime(a: Value, b: Value) -> Value {
        Value::Runtime {
            pure: a.pure() && b.pure(),
        }
    }
}

fn evaluate(condition: &str, known: &HashMap<String, Value>) -> Value {
    let tokens = tokenize(condition);
    let mut parser = Parser {
        tokens: &tokens,
        at: 0,
        known,
    };
    let value = parser.binary(0);
    if parser.at == tokens.len() {
        value
    } else {
        Value::Runtime { pure: false }
    }
}

fn tokenize(text: &str) -> Vec<&str> {
    const OPERATORS: [&str; 10] = ["&&", "||", "==", "!=", "<=", ">=", "<<", ">>", "->", "::"];
    let mut tokens = Vec::new();
    let mut i = 0;
    let bytes = text.as_bytes();
    while i < bytes.len() {
        let c = bytes[i] as char;
        if c.is_whitespace() {
            i += 1;
        } else if is_identifier_char(c)
            || (c == '.' && bytes.get(i + 1).is_some_and(u8::is_ascii_digit))
        {
            let number = c.is_ascii_digit() || c == '.';
            let start = i;
            while i < bytes.len() {
                let d = bytes[i] as char;
                if is_identifier_char(d)
                    || (number
                        && (d == '.'
                            || ((d == '-' || d == '+') && matches!(bytes[i - 1], b'e' | b'E'))))
                {
                    i += 1;
                } else {
                    break;
                }
            }
            tokens.push(&text[start..i]);
        } else if let Some(op) = OPERATORS.iter().find(|op| text[i..].starts_with(*op)) {
            tokens.push(op);
            i += op.len();
        } else {
            tokens.push(&text[i..i + c.len_utf8()]);
            i += c.len_utf8();
        }
    }
    tokens
}

struct Parser<'a> {
    tokens: &'a [&'a str],
    at: usize,
    known: &'a HashMap<String, Value>,
}

/// WGSL's binary operators, loosest first.
const LEVELS: [&[&str]; 9] = [
    &["||"],
    &["&&"],
    &["|"],
    &["^"],
    &["&"],
    &["==", "!=", "<", ">", "<=", ">="],
    &["<<", ">>"],
    &["+", "-"],
    &["*", "/", "%"],
];

impl<'a> Parser<'a> {
    fn peek(&self) -> Option<&'a str> {
        self.tokens.get(self.at).copied()
    }
    fn binary(&mut self, level: usize) -> Value {
        if level == LEVELS.len() {
            return self.unary();
        }
        let mut left = self.binary(level + 1);
        while let Some(op) = self.peek().filter(|t| LEVELS[level].contains(t)) {
            self.at += 1;
            let right = self.binary(level + 1);
            left = apply(op, left, right);
        }
        left
    }
    fn unary(&mut self) -> Value {
        match self.peek() {
            Some("!") => {
                self.at += 1;
                match self.unary() {
                    Value::Bool(b) => Value::Bool(!b),
                    other => Value::runtime(other, other),
                }
            }
            Some("-") => {
                self.at += 1;
                match self.unary() {
                    Value::Number(n) => Value::Number(-n),
                    other => Value::runtime(other, other),
                }
            }
            _ => self.postfix(),
        }
    }
    fn postfix(&mut self) -> Value {
        let Some(token) = self.peek() else {
            return Value::Runtime { pure: false };
        };
        self.at += 1;
        let mut value = if token == "(" {
            let inner = self.binary(0);
            if self.peek() == Some(")") {
                self.at += 1;
            }
            inner
        } else if token == "true" || token == "false" {
            Value::Bool(token == "true")
        } else if token.starts_with(|c: char| c.is_ascii_digit() || c == '.') {
            literal(token).map_or(Value::Runtime { pure: true }, Value::Number)
        } else if leading_identifier(token).is_some() {
            if self.peek() == Some("(")
                && let Some(value) = self.call(token)
            {
                return value;
            }
            self.known
                .get(token)
                .copied()
                .unwrap_or(Value::Runtime { pure: true })
        } else {
            return Value::Runtime { pure: false };
        };
        // Calls, indexing and members are decided at run time.
        loop {
            match self.peek() {
                Some("(") => {
                    self.skip_group("(", ")");
                    value = Value::Runtime { pure: false };
                }
                Some("[") => {
                    self.skip_group("[", "]");
                    value = Value::runtime(value, value);
                }
                Some(".") => {
                    self.at += 2;
                    value = Value::runtime(value, value);
                }
                Some("<") if matches!(value, Value::Runtime { .. }) && self.is_template() => {
                    self.skip_group("<", ">");
                }
                _ => return value,
            }
        }
    }
    /// A call the constants decide: a function without parameters that
    /// returns one ( `name()` in `known` ), or a scalar conversion of one.
    fn call(&mut self, name: &str) -> Option<Value> {
        let start = self.at;
        if self.tokens.get(self.at + 1) == Some(&")") {
            let value = self.known.get(&format!("{name}()")).copied()?;
            self.at += 2;
            return Some(value);
        }
        if !matches!(name, "f32" | "u32" | "i32" | "bool") {
            return None;
        }
        self.at += 1;
        let inner = self.binary(0);
        let value = match (self.peek(), name, inner) {
            (Some(")"), "bool", Value::Number(n)) => Some(Value::Bool(n != 0.0)),
            (Some(")"), "bool", Value::Bool(b)) => Some(Value::Bool(b)),
            (Some(")"), "f32", Value::Number(n)) => Some(Value::Number(n)),
            (Some(")"), _, Value::Number(n)) => Some(Value::Number(n.trunc())),
            (Some(")"), _, Value::Bool(b)) => Some(Value::Number(f64::from(u8::from(b)))),
            _ => None,
        };
        if value.is_some() {
            self.at += 1;
        } else {
            self.at = start;
        }
        value
    }
    /// A type's template list ( `vec3<f32>(…)` ) rather than a comparison.
    fn is_template(&self) -> bool {
        let mut depth = 0;
        for (i, t) in self.tokens[self.at..].iter().enumerate() {
            match *t {
                "<" => depth += 1,
                ">" => {
                    depth -= 1;
                    if depth == 0 {
                        return self.tokens.get(self.at + i + 1) == Some(&"(");
                    }
                }
                ">>" | "&&" | "||" | ";" => return false,
                _ => {}
            }
        }
        false
    }
    fn skip_group(&mut self, open: &str, close: &str) {
        let mut depth = 0;
        while let Some(t) = self.peek() {
            self.at += 1;
            if t == open {
                depth += 1;
            } else if t == close {
                depth -= 1;
                if depth == 0 {
                    return;
                }
            } else if close == ">" && t == ">>" {
                depth -= 2;
                if depth <= 0 {
                    return;
                }
            }
        }
    }
}

fn apply(op: &str, a: Value, b: Value) -> Value {
    use Value::*;
    match (op, a, b) {
        ("&&", Bool(false), _) => Bool(false),
        ("&&", other, Bool(false)) if other.pure() => Bool(false),
        ("&&", Bool(true), other) | ("&&", other, Bool(true)) => other,
        ("||", Bool(true), _) => Bool(true),
        ("||", other, Bool(true)) if other.pure() => Bool(true),
        ("||", Bool(false), other) | ("||", other, Bool(false)) => other,
        (_, Number(x), Number(y)) => match op {
            "==" => Bool(x == y),
            "!=" => Bool(x != y),
            "<" => Bool(x < y),
            ">" => Bool(x > y),
            "<=" => Bool(x <= y),
            ">=" => Bool(x >= y),
            "+" => Number(x + y),
            "-" => Number(x - y),
            "*" => Number(x * y),
            "/" if y != 0.0 => Number(x / y),
            "&" => Number(((x as i64) & (y as i64)) as f64),
            "|" => Number(((x as i64) | (y as i64)) as f64),
            "^" => Number(((x as i64) ^ (y as i64)) as f64),
            "<<" => Number(((x as i64) << (y as i64)) as f64),
            ">>" => Number(((x as i64) >> (y as i64)) as f64),
            _ => Value::runtime(a, b),
        },
        (_, Bool(x), Bool(y)) => match op {
            "==" => Bool(x == y),
            "!=" => Bool(x != y),
            "&" => Bool(x & y),
            "|" => Bool(x | y),
            _ => Value::runtime(a, b),
        },
        _ => Value::runtime(a, b),
    }
}

/// The names `source` declares at module scope, for tests and diagnostics.
#[allow(dead_code)]
pub(crate) fn declarations(source: &str) -> HashSet<String> {
    split(&without_comments(source))
        .into_iter()
        .filter_map(|text| Item::new(text, &HashMap::new()).name)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SOURCE: &str = "
override FEATURE:bool=true;
override COUNT:i32=-1;
struct Out { @location(0) color: vec4<f32> };
@group(0) @binding(0) var<uniform> u: vec4<f32>;
fn used()->f32 { return 1.0; }
fn feature()->f32 { return u.x; }
fn unused()->f32 { return 2.0; } // never called
/* block /* nested */ comment */
fn count()->u32 { if COUNT<0 {return u32(u.y);} return u32(COUNT); }
@fragment fn fs_main()->Out {
    var c=vec4(used());
    if FEATURE && u.z>0.0 { c.x=feature(); } else if COUNT==2 { c.y=2.0; } else { c.z=3.0; }
    c.w=f32(count());
    return Out(c);
}
@vertex fn vs_main()->@builtin(position) vec4<f32> { return vec4(0.0); }
";

    #[test]
    fn keeps_what_the_entry_point_reaches() {
        let out = specialize(SOURCE, &["fs_main"], &[]);
        let names = declarations(&out);
        for name in [
            "FEATURE", "COUNT", "Out", "u", "used", "feature", "count", "fs_main",
        ] {
            assert!(names.contains(name), "{name} in {out}");
        }
        assert!(
            !names.contains("unused") && !names.contains("vs_main"),
            "{out}"
        );
        assert!(!out.contains("never called") && !out.contains("nested"));
    }

    #[test]
    fn resolves_the_branches_constants_decide() {
        let out = specialize(SOURCE, &["fs_main"], &[("FEATURE", 0.0), ("COUNT", 2.0)]);
        // FEATURE false: the else-if chain remains, decided by COUNT == 2; FEATURE,
        // read only by the resolved branch, goes with it.
        assert!(
            out.contains("const COUNT:i32=2i;") && !out.contains("FEATURE"),
            "{out}"
        );
        assert!(!declarations(&out).contains("feature"), "{out}");
        assert!(
            out.contains("c.y=2.0;") && !out.contains("c.z=3.0;"),
            "{out}"
        );
        assert!(!out.contains("u32(u.y)"), "{out}");
        let out = specialize(SOURCE, &["fs_main"], &[("FEATURE", 1.0), ("COUNT", 3.0)]);
        // FEATURE true: decided at run time by u.z; COUNT 3 drops the else-if branch.
        assert!(
            out.contains("c.x=feature();") && out.contains("c.z=3.0;") && !out.contains("c.y=2.0;"),
            "{out}"
        );
        assert!(evaluate(" 1+2*3==7 ", &HashMap::new()) == Value::Bool(true));
    }

    #[test]
    fn keeps_runtime_conditions_and_unknown_text() {
        let out = specialize("invalid WGSL\nfn a(){}", &["b"], &[]);
        assert!(out.contains("invalid WGSL"));
        assert_eq!(
            evaluate(" f() && false ", &HashMap::new()),
            Value::Runtime { pure: false }
        );
        assert_eq!(
            evaluate(" x.y && false ", &HashMap::new()),
            Value::Bool(false)
        );
        let known = HashMap::from([("MASK".to_owned(), Value::Number(5.0))]);
        assert_eq!(evaluate(" (MASK & 4u) != 0u ", &known), Value::Bool(true));
        assert_eq!(evaluate(" (MASK>>1u)&1u ", &known), Value::Number(0.0));
        assert_eq!(
            evaluate(" vec3<f32>(1.0).x > 0.0 ", &known),
            Value::Runtime { pure: false }
        );
        assert_eq!(evaluate(" u32(MASK)>4u ", &known), Value::Bool(true));
        assert_eq!(
            evaluate(" f(MASK)>4u ", &known),
            Value::Runtime { pure: false }
        );
    }

    #[test]
    fn decides_calls_of_functions_that_return_constants() {
        let source = "override KIND:f32=-1.0;
@group(0) @binding(0) var<uniform> u: vec4<f32>;
fn kind()->f32 {if KIND<0.0 {return u.x;}return KIND;}
fn a()->f32 {return 1.0;}
@fragment fn fs_main()->@location(0) vec4<f32> { if kind()==6.0 { return vec4(a()); } return vec4(kind()); }";
        let out = specialize(source, &["fs_main"], &[("KIND", 2.0)]);
        assert!(
            !out.contains("vec4(a())") && !declarations(&out).contains("a"),
            "{out}"
        );
        let out = specialize(source, &["fs_main"], &[]);
        assert!(out.contains("if kind()==6.0"), "{out}");
    }
}
