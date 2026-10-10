//! GLSLDecoder's preprocessor: `preprocess()`, `expandMacros()`,
//! `extractMacroArgs()` and `evaluateCondition()`, whose condition the
//! decoder evaluates as a strict-mode JS expression.
use super::js::*;

#[derive(Clone)]
pub struct Macro {
    /// `null` for an object-like macro.
    pub params: Option<Vec<String>>,
    pub body: String,
}

/// A `Map` keeps insertion order; `set` on an existing key keeps its place.
#[derive(Default)]
pub struct Macros(Vec<(String, Macro)>);

impl Macros {
    fn set(&mut self, name: String, m: Macro) {
        match self.0.iter_mut().find(|(n, _)| *n == name) {
            Some(entry) => entry.1 = m,
            None => self.0.push((name, m)),
        }
    }
    fn delete(&mut self, name: &str) {
        self.0.retain(|(n, _)| n != name);
    }
    fn has(&self, name: &str) -> bool {
        self.0.iter().any(|(n, _)| n == name)
    }
    fn get(&self, name: &str) -> Option<&Macro> {
        self.0.iter().find(|(n, _)| n == name).map(|(_, m)| m)
    }
}

/// `/^#\s*(\w+)(?:\s+(.*))?$/` and `/^(\w+)(?:\s+(.*))?$/` ( `hash` selects
/// the leading `#\s*` ).
fn name_and_rest(s: &str, hash: bool) -> Option<(String, Option<String>)> {
    let c: Vec<char> = s.chars().collect();
    let mut i = 0;
    if hash {
        if c.first() != Some(&'#') {
            return None;
        }
        i = 1;
        while i < c.len() && is_space(c[i]) {
            i += 1;
        }
    }
    let start = i;
    while i < c.len() && is_word(c[i]) {
        i += 1;
    }
    if i == start {
        return None;
    }
    let name: String = c[start..i].iter().collect();
    if i == c.len() {
        return Some((name, None));
    }
    if !is_space(c[i]) {
        return None;
    }
    while i < c.len() && is_space(c[i]) {
        i += 1;
    }
    let rest = &c[i..];
    if rest.iter().any(|&ch| is_line_terminator(ch)) {
        return None;
    }
    Some((name, Some(rest.iter().collect())))
}

/// `/^(\w+)\((.*?)\)\s*(.*)$/`.
fn function_macro(s: &str) -> Option<(String, String, String)> {
    let c: Vec<char> = s.chars().collect();
    let mut i = 0;
    while i < c.len() && is_word(c[i]) {
        i += 1;
    }
    if i == 0 || c.get(i) != Some(&'(') {
        return None;
    }
    let name: String = c[..i].iter().collect();
    let open = i + 1;
    let mut close = open;
    while close < c.len() {
        if is_line_terminator(c[close]) {
            return None;
        }
        if c[close] == ')' {
            let mut j = close + 1;
            while j < c.len() && is_space(c[j]) {
                j += 1;
            }
            if !c[j..].iter().any(|&ch| is_line_terminator(ch)) {
                return Some((
                    name,
                    c[open..close].iter().collect(),
                    c[j..].iter().collect(),
                ));
            }
        }
        close += 1;
    }
    None
}

/// `args.replace( /\/\/.*$/, '' ).replace( /\/\*.*?\*\//g, '' ).trim()`.
fn strip_comments(args: &str) -> String {
    let c: Vec<char> = args.chars().collect();
    let mut cut = c.len();
    for k in 0..c.len().saturating_sub(1) {
        if c[k] == '/' && c[k + 1] == '/' && !c[k + 2..].iter().any(|&ch| is_line_terminator(ch)) {
            cut = k;
            break;
        }
    }
    let c = &c[..cut];
    let mut out = String::new();
    let mut k = 0;
    while k < c.len() {
        if k + 1 < c.len() && c[k] == '/' && c[k + 1] == '*' {
            let mut j = k + 2;
            let mut end = None;
            while j < c.len() && !is_line_terminator(c[j]) {
                if j + 1 < c.len() && c[j] == '*' && c[j + 1] == '/' {
                    end = Some(j + 2);
                    break;
                }
                j += 1;
            }
            if let Some(e) = end {
                k = e;
                continue;
            }
        }
        out.push(c[k]);
        k += 1;
    }
    trim(&out).to_string()
}

/// `evaluateCondition( expr, macros )`.
fn evaluate_condition(expr: &str, macros: &Macros) -> bool {
    let c: Vec<char> = expr.chars().collect();
    // /defined\s*\(\s*(\w+)\s*\)/g
    let mut s = String::new();
    let mut i = 0;
    while i < c.len() {
        if let Some((end, name)) = defined_call(&c, i) {
            s.push(if macros.has(&name) { '1' } else { '0' });
            i = end;
        } else {
            s.push(c[i]);
            i += 1;
        }
    }
    // /defined\s+(\w+)/g
    let c: Vec<char> = s.chars().collect();
    let mut s = String::new();
    let mut i = 0;
    while i < c.len() {
        if let Some((end, name)) = defined_word(&c, i) {
            s.push(if macros.has(&name) { '1' } else { '0' });
            i = end;
        } else {
            s.push(c[i]);
            i += 1;
        }
    }
    // /\b([A-Za-z_]\w*)\b/g
    let c: Vec<char> = s.chars().collect();
    let mut s = String::new();
    let mut i = 0;
    while i < c.len() {
        if boundary(&c, i) && (c[i].is_ascii_alphabetic() || c[i] == '_') {
            let mut j = i + 1;
            while j < c.len() && is_word(c[j]) {
                j += 1;
            }
            let name: String = c[i..j].iter().collect();
            s.push_str(match name.as_str() {
                "true" => "1",
                "false" => "0",
                _ => match macros.get(&name) {
                    Some(m) if !m.body.is_empty() => m.body.as_str(),
                    Some(_) => "1",
                    None => "0",
                },
            });
            i = j;
        } else {
            s.push(c[i]);
            i += 1;
        }
    }
    // /^[\d\s+\-*/%&|^!=<>~()]+$/
    if s.is_empty()
        || !s
            .chars()
            .all(|ch| ch.is_ascii_digit() || is_space(ch) || "+-*/%&|^!=<>~()".contains(ch))
    {
        return false;
    }
    condition::evaluate(&s).is_some_and(|v| v.truthy())
}

/// `defined\s*\(\s*(\w+)\s*\)` at `i`.
fn defined_call(c: &[char], i: usize) -> Option<(usize, String)> {
    let word: Vec<char> = "defined".chars().collect();
    if c.len() < i + word.len() || c[i..i + word.len()] != word[..] {
        return None;
    }
    let mut j = i + word.len();
    while j < c.len() && is_space(c[j]) {
        j += 1;
    }
    if c.get(j) != Some(&'(') {
        return None;
    }
    j += 1;
    while j < c.len() && is_space(c[j]) {
        j += 1;
    }
    let start = j;
    while j < c.len() && is_word(c[j]) {
        j += 1;
    }
    if j == start {
        return None;
    }
    let name: String = c[start..j].iter().collect();
    while j < c.len() && is_space(c[j]) {
        j += 1;
    }
    (c.get(j) == Some(&')')).then(|| (j + 1, name))
}

/// `defined\s+(\w+)` at `i`.
fn defined_word(c: &[char], i: usize) -> Option<(usize, String)> {
    let word: Vec<char> = "defined".chars().collect();
    if c.len() < i + word.len() || c[i..i + word.len()] != word[..] {
        return None;
    }
    let mut j = i + word.len();
    let spaces = j;
    while j < c.len() && is_space(c[j]) {
        j += 1;
    }
    if j == spaces {
        return None;
    }
    let start = j;
    while j < c.len() && is_word(c[j]) {
        j += 1;
    }
    (j > start).then(|| (j, c[start..j].iter().collect()))
}

/// `extractMacroArgs( str )`.
fn extract_macro_args(s: &[char]) -> Vec<String> {
    let mut args = vec![];
    let mut current = String::new();
    let mut depth = 0i32;
    for &ch in s {
        match ch {
            '(' | '[' | '{' => {
                depth += 1;
                current.push(ch);
            }
            ')' | ']' | '}' => {
                depth -= 1;
                current.push(ch);
            }
            ',' if depth == 0 => {
                args.push(trim(&current).to_string());
                current.clear();
            }
            _ => current.push(ch),
        }
    }
    if !trim(&current).is_empty() || !args.is_empty() {
        args.push(trim(&current).to_string());
    }
    args
}

/// `expandMacros( line, macros )`.
fn expand_macros(line: &str, macros: &Macros) -> String {
    if macros.0.is_empty() {
        return line.to_string();
    }
    let mut result = line.to_string();
    for _ in 0..10 {
        let mut changed = false;
        for (name, m) in &macros.0 {
            if let Some(params) = &m.params {
                // /\bNAME\s*\(/g
                let r: Vec<char> = result.chars().collect();
                let n: Vec<char> = name.chars().collect();
                let mut k = 0;
                while k + n.len() <= r.len() {
                    if r[k..k + n.len()] == n[..] && boundary(&r, k) {
                        let mut j = k + n.len();
                        while j < r.len() && is_space(r[j]) {
                            j += 1;
                        }
                        if r.get(j) == Some(&'(') {
                            let paren_start = j;
                            let mut depth = 1;
                            let mut paren_end = None;
                            for (q, &ch) in r.iter().enumerate().skip(paren_start + 1) {
                                if ch == '(' {
                                    depth += 1;
                                } else if ch == ')' {
                                    depth -= 1;
                                }
                                if depth == 0 {
                                    paren_end = Some(q);
                                    break;
                                }
                            }
                            if let Some(end) = paren_end {
                                let args = extract_macro_args(&r[paren_start + 1..end]);
                                let mut body = m.body.clone();
                                for (p, param) in params.iter().enumerate() {
                                    let arg = args.get(p).map_or("", |a| a.as_str());
                                    body = replace_word(&body, param, arg);
                                }
                                result = r[..k].iter().collect::<String>()
                                    + &body
                                    + &r[end + 1..].iter().collect::<String>();
                                changed = true;
                                break;
                            }
                            // The search goes on after the match ( lastIndex ).
                            k = j + 1;
                            continue;
                        }
                    }
                    k += 1;
                }
            } else {
                if m.body.is_empty() {
                    continue;
                }
                if has_word(&result, name) {
                    result = replace_word(&result, name, &m.body);
                    changed = true;
                }
            }
        }
        if !changed {
            break;
        }
    }
    result
}

struct Frame {
    active: bool,
    any_branch_executed: bool,
}

/// `preprocess( source )`.
pub fn preprocess(source: &str) -> String {
    let mut macros = Macros::default();
    let mut stack: Vec<Frame> = vec![];
    let lines: Vec<&str> = source.split('\n').collect();
    let mut out: Vec<String> = vec![];
    let mut in_block_comment = false;
    let mut i = 0;
    while i < lines.len() {
        let mut line = lines[i].to_string();
        while line.ends_with('\\') && i + 1 < lines.len() {
            line.pop();
            line = line + " " + lines[i + 1];
            i += 1;
            out.push(String::new());
        }
        let executing = |stack: &[Frame]| stack.iter().all(|f| f.active);
        let mut trimmed = trim(&line).to_string();
        if in_block_comment {
            match trimmed.find("*/") {
                Some(end) => {
                    in_block_comment = false;
                    trimmed = trim(&trimmed[end + 2..]).to_string();
                }
                None => {
                    out.push(line);
                    i += 1;
                    continue;
                }
            }
        }
        if trimmed.starts_with("/*") && !trimmed[2..].contains("*/") {
            in_block_comment = true;
            out.push(line);
            i += 1;
            continue;
        }
        if let Some((directive, args)) = name_and_rest(&trimmed, true) {
            let args = strip_comments(&args.unwrap_or_default());
            match directive.as_str() {
                "define" => {
                    if executing(&stack) {
                        if let Some((name, params, body)) = function_macro(&args) {
                            let params = params
                                .split(',')
                                .map(|p| trim(p).to_string())
                                .filter(|p| !p.is_empty())
                                .collect();
                            macros.set(
                                name,
                                Macro {
                                    params: Some(params),
                                    body: trim(&body).to_string(),
                                },
                            );
                        } else if let Some((name, value)) = name_and_rest(&args, false) {
                            macros.set(
                                name,
                                Macro {
                                    params: None,
                                    body: value.map_or(String::new(), |v| trim(&v).to_string()),
                                },
                            );
                        }
                    }
                }
                "undef" => {
                    if executing(&stack) {
                        macros.delete(trim(&args));
                    }
                }
                "ifdef" | "ifndef" => {
                    let name = trim(&args);
                    let parent = executing(&stack);
                    let defined = macros.has(name);
                    let condition = parent && (defined == (directive == "ifdef"));
                    stack.push(Frame {
                        active: condition,
                        any_branch_executed: condition,
                    });
                }
                "if" => {
                    let parent = executing(&stack);
                    let condition = parent && evaluate_condition(&args, &macros);
                    stack.push(Frame {
                        active: condition,
                        any_branch_executed: condition,
                    });
                }
                "elif" | "else" => {
                    if !stack.is_empty() {
                        let parent = executing(&stack[..stack.len() - 1]);
                        let frame_executed = stack.last().is_some_and(|f| f.any_branch_executed);
                        let condition = if !parent || frame_executed {
                            None
                        } else if directive == "elif" {
                            Some(evaluate_condition(&args, &macros))
                        } else {
                            Some(true)
                        };
                        let frame = stack.last_mut().expect("a frame");
                        match condition {
                            None => frame.active = false,
                            Some(c) => {
                                frame.active = c;
                                if c {
                                    frame.any_branch_executed = true;
                                }
                            }
                        }
                    }
                }
                "endif" => {
                    stack.pop();
                }
                _ => {}
            }
            out.push(String::new());
        } else if executing(&stack) {
            out.push(expand_macros(&line, &macros));
        } else {
            out.push(String::new());
        }
        i += 1;
    }
    out.join("\n")
}

/// The strict-mode JS expression `evaluateCondition()` evaluates, over the
/// characters its test admits ( digits, whitespace and operators ). A syntax
/// error makes the condition false, as the decoder's `catch` does.
mod condition {
    use super::is_space;

    #[derive(Clone, Copy)]
    pub enum Value {
        Number(f64),
        Bool(bool),
    }
    impl Value {
        fn number(self) -> f64 {
            match self {
                Value::Number(n) => n,
                Value::Bool(b) => f64::from(u8::from(b)),
            }
        }
        pub fn truthy(self) -> bool {
            match self {
                Value::Number(n) => n != 0. && !n.is_nan(),
                Value::Bool(b) => b,
            }
        }
        fn int32(self) -> i32 {
            let n = self.number();
            if !n.is_finite() {
                return 0;
            }
            let m = n.trunc().rem_euclid(4294967296.);
            (if m >= 2147483648. { m - 4294967296. } else { m }) as i32
        }
        fn uint32(self) -> u32 {
            self.int32() as u32
        }
    }

    #[derive(PartialEq, Clone)]
    enum Token {
        Number(f64),
        Op(&'static str),
    }

    /// Longest first, as the JS tokenizer reads punctuators.
    const OPS: [&str; 43] = [
        ">>>=", "===", "!==", "**=", "<<=", ">>=", ">>>", "&&=", "||=", "==", "!=", "<=", ">=",
        "<<", ">>", "&&", "||", "**", "++", "--", "+=", "-=", "*=", "/=", "%=", "&=", "|=", "^=",
        "(", ")", "+", "-", "*", "/", "%", "&", "|", "^", "!", "~", "<", ">", "=",
    ];

    fn tokenize(s: &str) -> Option<Vec<Token>> {
        let c: Vec<char> = s.chars().collect();
        let mut out = vec![];
        let mut i = 0;
        'outer: while i < c.len() {
            if is_space(c[i]) {
                i += 1;
                continue;
            }
            if c[i].is_ascii_digit() {
                let start = i;
                while i < c.len() && c[i].is_ascii_digit() {
                    i += 1;
                }
                // Strict mode rejects legacy octal and leading-zero decimals.
                if c[start] == '0' && i - start > 1 {
                    return None;
                }
                let text: String = c[start..i].iter().collect();
                out.push(Token::Number(text.parse().ok()?));
                continue;
            }
            for op in OPS.iter() {
                let o: Vec<char> = op.chars().collect();
                if c.len() >= i + o.len() && c[i..i + o.len()] == o[..] {
                    // `/` would start a regular expression literal in operand
                    // position; such conditions are not supported here.
                    out.push(Token::Op(op));
                    i += o.len();
                    continue 'outer;
                }
            }
            return None;
        }
        Some(out)
    }

    struct Parser {
        tokens: Vec<Token>,
        at: usize,
    }

    impl Parser {
        fn peek(&self) -> Option<&Token> {
            self.tokens.get(self.at)
        }
        fn op(&self) -> Option<&'static str> {
            match self.peek() {
                Some(Token::Op(o)) => Some(o),
                _ => None,
            }
        }
        fn binary(&mut self, level: usize) -> Option<Value> {
            const LEVELS: [&[&str]; 9] = [
                &["||"],
                &["&&"],
                &["|"],
                &["^"],
                &["&"],
                &["==", "!=", "===", "!=="],
                &["<", ">", "<=", ">="],
                &["<<", ">>", ">>>"],
                &["+", "-"],
            ];
            if level == LEVELS.len() {
                return self.multiplicative();
            }
            let mut left = self.binary(level + 1)?;
            while let Some(op) = self.op().filter(|o| LEVELS[level].contains(o)) {
                self.at += 1;
                let right = self.binary(level + 1)?;
                left = apply(op, left, right);
            }
            Some(left)
        }
        fn multiplicative(&mut self) -> Option<Value> {
            let mut left = self.exponent()?;
            while let Some(op) = self.op().filter(|o| ["*", "/", "%"].contains(o)) {
                self.at += 1;
                let right = self.exponent()?;
                left = apply(op, left, right);
            }
            Some(left)
        }
        fn exponent(&mut self) -> Option<Value> {
            let unary = matches!(self.op(), Some("!" | "~" | "+" | "-"));
            let base = self.unary()?;
            if self.op() == Some("**") {
                // `-2 ** 2` is a SyntaxError.
                if unary {
                    return None;
                }
                self.at += 1;
                let exponent = self.exponent()?;
                return Some(Value::Number(base.number().powf(exponent.number())));
            }
            Some(base)
        }
        fn unary(&mut self) -> Option<Value> {
            match self.op() {
                Some("!") => {
                    self.at += 1;
                    Some(Value::Bool(!self.unary()?.truthy()))
                }
                Some("~") => {
                    self.at += 1;
                    Some(Value::Number(f64::from(!self.unary()?.int32())))
                }
                Some("+") => {
                    self.at += 1;
                    Some(Value::Number(self.unary()?.number()))
                }
                Some("-") => {
                    self.at += 1;
                    Some(Value::Number(-self.unary()?.number()))
                }
                _ => self.primary(),
            }
        }
        fn primary(&mut self) -> Option<Value> {
            match self.peek()?.clone() {
                Token::Number(n) => {
                    self.at += 1;
                    // A postfix `++` / `--` on a literal is a SyntaxError.
                    if matches!(self.op(), Some("++" | "--")) {
                        return None;
                    }
                    Some(Value::Number(n))
                }
                Token::Op("(") => {
                    self.at += 1;
                    let v = self.binary(0)?;
                    if self.op() != Some(")") {
                        return None;
                    }
                    self.at += 1;
                    if matches!(self.op(), Some("++" | "--")) {
                        return None;
                    }
                    Some(v)
                }
                _ => None,
            }
        }
    }

    fn apply(op: &str, a: Value, b: Value) -> Value {
        let n = |v: Value| v.number();
        match op {
            "||" => {
                if a.truthy() {
                    a
                } else {
                    b
                }
            }
            "&&" => {
                if a.truthy() {
                    b
                } else {
                    a
                }
            }
            "|" => Value::Number(f64::from(a.int32() | b.int32())),
            "^" => Value::Number(f64::from(a.int32() ^ b.int32())),
            "&" => Value::Number(f64::from(a.int32() & b.int32())),
            "==" | "!=" => {
                let eq = match (a, b) {
                    (Value::Bool(x), Value::Bool(y)) => x == y,
                    _ => n(a) == n(b),
                };
                Value::Bool(eq == (op == "=="))
            }
            "===" | "!==" => {
                let eq = match (a, b) {
                    (Value::Bool(x), Value::Bool(y)) => x == y,
                    (Value::Number(x), Value::Number(y)) => x == y,
                    _ => false,
                };
                Value::Bool(eq == (op == "==="))
            }
            "<" => Value::Bool(n(a) < n(b)),
            ">" => Value::Bool(n(a) > n(b)),
            "<=" => Value::Bool(n(a) <= n(b)),
            ">=" => Value::Bool(n(a) >= n(b)),
            "<<" => Value::Number(f64::from(a.int32().wrapping_shl(b.uint32() & 31))),
            ">>" => Value::Number(f64::from(a.int32().wrapping_shr(b.uint32() & 31))),
            ">>>" => Value::Number(f64::from(a.uint32().wrapping_shr(b.uint32() & 31))),
            "+" => Value::Number(n(a) + n(b)),
            "-" => Value::Number(n(a) - n(b)),
            "*" => Value::Number(n(a) * n(b)),
            "/" => Value::Number(n(a) / n(b)),
            // JS `%` keeps the dividend's sign, as Rust's does.
            _ => Value::Number(n(a) % n(b)),
        }
    }

    /// `Function( '"use strict"; return (' + s + ');' )()`: None for a
    /// SyntaxError.
    pub fn evaluate(s: &str) -> Option<Value> {
        let tokens = tokenize(s)?;
        let mut p = Parser { tokens, at: 0 };
        let v = p.binary(0)?;
        (p.at == p.tokens.len()).then_some(v)
    }
}
