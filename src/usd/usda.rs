//! USDAParser ( examples/jsm/loaders/usd/USDAParser.js ): the text layer's
//! line-based tree, then its prims, attributes and relationships as
//! `specsByPath`. The original's regular expressions are matched by hand
//! with their backtracking, and values keep their JavaScript shapes.
use super::js::{Obj, Value, trim};
use super::{Spec, Specs};
use crate::fbx::js::{is_js_space, parse_float};
use crate::{Error, Result};

const ATTRIBUTE: f64 = 1.0;
const PRIM: f64 = 6.0;
const RELATIONSHIP: f64 = 8.0;

fn type_error(what: &str) -> Error {
    Error::Asset(format!("THREE.USDAParser: {what}"))
}
fn is_word(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_'
}
fn line_terminator(c: char) -> bool {
    matches!(c, '\n' | '\r' | '\u{2028}' | '\u{2029}')
}
/// The length in bytes of the run of characters matching `f` at `s`'s start.
fn run(s: &str, f: impl Fn(char) -> bool) -> usize {
    s.char_indices()
        .find(|&(_, c)| !f(c))
        .map_or(s.len(), |(i, _)| i)
}
/// The byte offsets after each of the first `n` characters of a run.
fn run_ends(s: &str, n: usize) -> Vec<usize> {
    s.char_indices()
        .take(n)
        .map(|(i, c)| i + c.len_utf8())
        .collect()
}
fn char_count(s: &str, f: impl Fn(char) -> bool) -> usize {
    s.chars().take_while(|&c| f(c)).count()
}

/// `"?([^"]+)"?$` at the start of `s`.
fn quoted_name(s: &str) -> Option<&str> {
    let body = |t: &str| -> Option<usize> {
        // [^"]+ then "? then the end.
        let n = run(t, |c| c != '"');
        if n == 0 {
            return None;
        }
        let rest = &t[n..];
        (rest.is_empty() || rest == "\"").then_some(n)
    };
    if let Some(t) = s.strip_prefix('"')
        && let Some(n) = body(t)
    {
        return Some(&t[..n]);
    }
    body(s).map(|n| &s[..n])
}

/// DEF_MATCH_REGEX `^def\s+(?:(\w+)\s+)?"?([^"]+)"?$`: the type name and
/// the prim name.
fn def_match(key: &str) -> Option<(Option<&str>, &str)> {
    let rest = key.strip_prefix("def")?;
    let spaces = char_count(rest, is_js_space);
    if spaces == 0 {
        return None;
    }
    let ends = run_ends(rest, spaces);
    for k in (0..spaces).rev() {
        let at = &rest[ends[k]..];
        if k == spaces - 1 {
            let w = run(at, is_word);
            if w > 0 {
                let after = &at[w..];
                let spaces2 = char_count(after, is_js_space);
                let ends2 = run_ends(after, spaces2);
                for k2 in (0..spaces2).rev() {
                    if let Some(name) = quoted_name(&after[ends2[k2]..]) {
                        return Some((Some(&at[..w]), name));
                    }
                }
            }
        }
        if let Some(name) = quoted_name(at) {
            return Some((None, name));
        }
    }
    None
}

/// VARIANT_STRING_REGEX `^string\s+(\w+)$`.
fn variant_string(key: &str) -> Option<&str> {
    let rest = key.strip_prefix("string")?;
    let spaces = run(rest, is_js_space);
    if spaces == 0 {
        return None;
    }
    let name = &rest[spaces..];
    (!name.is_empty() && name.chars().all(is_word)).then_some(name)
}

/// `(\w+(?:\[\])?)\s+(.+)$` at the start of `s`.
fn typed_name(s: &str) -> Option<(&str, &str)> {
    let w = run(s, is_word);
    if w == 0 {
        return None;
    }
    let try_at = |end: usize| -> Option<(&str, &str)> {
        let after = &s[end..];
        let spaces = char_count(after, is_js_space);
        if spaces == 0 {
            return None;
        }
        let ends = run_ends(after, spaces);
        for k in (0..spaces).rev() {
            let name = &after[ends[k]..];
            if !name.is_empty() && !name.chars().any(line_terminator) {
                return Some((&s[..end], name));
            }
        }
        None
    };
    if s[w..].starts_with("[]")
        && let Some(r) = try_at(w + 2)
    {
        return Some(r);
    }
    try_at(w)
}

/// ATTR_MATCH_REGEX `^(?:uniform\s+)?(\w+(?:\[\])?)\s+(.+)$`.
fn attr_match(key: &str) -> Option<(&str, &str)> {
    if let Some(rest) = key.strip_prefix("uniform") {
        let spaces = run(rest, is_js_space);
        if spaces > 0
            && let Some(r) = typed_name(&rest[spaces..])
        {
            return Some(r);
        }
    }
    typed_name(key)
}

/// `/^[\d.]+$/`.
fn is_frame_key(key: &str) -> bool {
    !key.is_empty() && key.chars().all(|c| c.is_ascii_digit() || c == '.')
}

/// Find the first '=' that's not inside quotes.
fn find_assignment_operator(line: &str) -> Option<usize> {
    let mut in_string = false;
    let mut string_char = '\0';
    let mut escaped = false;
    for (i, ch) in line.char_indices() {
        if escaped {
            escaped = false;
            continue;
        }
        if ch == '\\' {
            escaped = true;
            continue;
        }
        if !in_string && (ch == '"' || ch == '\'') {
            in_string = true;
            string_char = ch;
        } else if in_string && ch == string_char {
            in_string = false;
        } else if !in_string && ch == '=' {
            return Some(i);
        }
    }
    None
}

fn strip_inline_comment(line: &str) -> &str {
    if trim(line).starts_with("#usda") {
        return line;
    }
    let mut in_string = false;
    let mut string_char = '\0';
    let mut escaped = false;
    for (i, ch) in line.char_indices() {
        if escaped {
            escaped = false;
            continue;
        }
        if ch == '\\' {
            escaped = true;
            continue;
        }
        if !in_string && (ch == '"' || ch == '\'') {
            in_string = true;
            string_char = ch;
        } else if in_string && ch == string_char {
            in_string = false;
        } else if !in_string && ch == '#' {
            return line[..i].trim_end_matches(is_js_space);
        }
    }
    line
}

fn strip_block_comments(text: &str) -> String {
    let b: Vec<char> = text.chars().collect();
    let mut out = String::with_capacity(text.len());
    let mut i = 0;
    while i < b.len() {
        if b[i] == '/' && i + 1 < b.len() && b[i + 1] == '*' {
            let mut j = i + 2;
            while j < b.len() {
                if b[j] == '*' && j + 1 < b.len() && b[j + 1] == '/' {
                    j += 2;
                    break;
                }
                j += 1;
            }
            i = j;
        } else {
            out.push(b[i]);
            i += 1;
        }
    }
    out
}

fn collapse_triple_quoted_strings(text: &str) -> String {
    let b: Vec<char> = text.chars().collect();
    let mut out = String::with_capacity(text.len());
    let triple = |i: usize| -> Option<char> {
        if i + 2 < b.len() && b[i] == b[i + 1] && b[i] == b[i + 2] && (b[i] == '\'' || b[i] == '"')
        {
            Some(b[i])
        } else {
            None
        }
    };
    let mut i = 0;
    while i < b.len() {
        if let Some(q) = triple(i) {
            out.extend([q; 3]);
            i += 3;
            while i < b.len() {
                if triple(i) == Some(q) {
                    out.extend([q; 3]);
                    i += 3;
                    break;
                }
                if b[i] == '\n' {
                    out.push_str("\\n");
                } else if b[i] != '\r' {
                    out.push(b[i]);
                }
                i += 1;
            }
            continue;
        }
        out.push(b[i]);
        i += 1;
    }
    out
}

fn preprocess(text: &str) -> String {
    let text = collapse_triple_quoted_strings(&strip_block_comments(text));
    let mut processed: Vec<String> = vec![];
    let mut in_multiline = false;
    let mut bracket_depth = 0i64;
    let mut paren_depth = 0i64;
    let mut accumulated = String::new();
    for line in text.split('\n') {
        let line = strip_inline_comment(line);
        let trimmed = trim(line);
        if in_multiline {
            accumulated.push(' ');
            accumulated.push_str(trimmed);
            for ch in trimmed.chars() {
                match ch {
                    '[' => bracket_depth += 1,
                    ']' => bracket_depth -= 1,
                    '(' if bracket_depth > 0 => paren_depth += 1,
                    ')' if bracket_depth > 0 => paren_depth -= 1,
                    _ => {}
                }
            }
            if bracket_depth == 0 && paren_depth == 0 {
                processed.push(std::mem::take(&mut accumulated));
                in_multiline = false;
            }
        } else {
            if trimmed.contains('=')
                && let Some(eq) = find_assignment_operator(trimmed)
            {
                let rhs = trim(&trimmed[eq + 1..]);
                let open = rhs.chars().filter(|&c| c == '[').count() as i64;
                let close = rhs.chars().filter(|&c| c == ']').count() as i64;
                if open > close {
                    in_multiline = true;
                    bracket_depth = open - close;
                    paren_depth = 0;
                    accumulated = trimmed.to_string();
                    continue;
                }
            }
            processed.push(trimmed.to_string());
        }
    }
    processed.join("\n")
}

/// A node of the line tree: an object, or a string value.
fn node_obj(v: &Value) -> Result<Obj> {
    match v {
        Value::Object(o) => Ok(o.clone()),
        _ => Err(type_error("Cannot create property on a string")),
    }
}

/// parseText( text ).
fn parse_text(text: &str) -> Result<Obj> {
    let text = preprocess(text);
    let root = Obj::new();
    let mut string: Option<String> = None;
    let mut target: Option<Value> = Some(Value::Object(root.clone()));
    let mut stack: Vec<Value> = vec![Value::Object(root.clone())];
    let key = |s: &Option<String>| s.clone().unwrap_or_else(|| "null".into());
    for line in text.split('\n') {
        let target_obj = || -> Result<Obj> {
            match &target {
                Some(t) => node_obj(t),
                None => Err(type_error("Cannot set properties of undefined")),
            }
        };
        if line.contains('=') {
            let Some(eq) = find_assignment_operator(line) else {
                string = Some(trim(line).to_string());
                continue;
            };
            let lhs = trim(&line[..eq]);
            let rhs = trim(&line[eq + 1..]);
            let t = target_obj()?;
            if rhs.ends_with('{') {
                let group = Obj::new();
                stack.push(Value::Object(group.clone()));
                t.set(lhs, Value::Object(group.clone()));
                target = Some(Value::Object(group));
            } else if let Some(values) = rhs.strip_suffix('(') {
                let values = trim(values);
                t.set(lhs, Value::string(values));
                let meta = Obj::new();
                let all = match t.metadata() {
                    Some(m) => m,
                    None => {
                        let m = Obj::new();
                        t.set_metadata(m.clone());
                        m
                    }
                };
                all.set(lhs, Value::Object(meta.clone()));
                stack.push(Value::Object(meta.clone()));
                target = Some(Value::Object(meta));
            } else {
                t.set(lhs, Value::string(rhs));
            }
        } else if line.contains(':') {
            let colon = line.find(':').unwrap_or(0);
            let k = trim(&line[..colon]);
            let v = trim(&line[colon + 1..]);
            if is_frame_key(k) {
                target_obj()?.set(k, Value::string(v));
            }
        } else if let Some(s) = line.strip_suffix('{') {
            let s = trim(s);
            if !s.is_empty() {
                string = Some(s.to_string());
            }
            let t = target_obj()?;
            let existing = t.get(&key(&string));
            let group = if existing.truthy() {
                existing
            } else {
                Value::Object(Obj::new())
            };
            stack.push(group.clone());
            t.set(&key(&string), group.clone());
            target = Some(group);
        } else if line.ends_with('}') {
            stack.pop();
            if stack.is_empty() {
                continue;
            }
            target = stack.last().cloned();
        } else if line.ends_with('(') {
            let meta = Obj::new();
            stack.push(Value::Object(meta.clone()));
            let s = trim(line.split('(').next().unwrap_or(""));
            if !s.is_empty() {
                string = Some(s.to_string());
            }
            target_obj()?.set(&key(&string), Value::Object(meta.clone()));
            target = Some(Value::Object(meta));
        } else if line.ends_with(')') {
            stack.pop();
            target = stack.last().cloned();
        } else if !trim(line).is_empty() {
            string = Some(trim(line).to_string());
        }
    }
    Ok(root)
}

/// A small JSON.parse: ECMA-404 values as JavaScript arrays and objects.
struct Json<'a> {
    s: &'a [u8],
    i: usize,
}
impl Json<'_> {
    fn ws(&mut self) {
        while self.i < self.s.len() && matches!(self.s[self.i], b' ' | b'\t' | b'\n' | b'\r') {
            self.i += 1;
        }
    }
    fn value(&mut self) -> Option<Value> {
        self.ws();
        let v = match *self.s.get(self.i)? {
            b'[' => {
                self.i += 1;
                let mut items = vec![];
                self.ws();
                if self.s.get(self.i) == Some(&b']') {
                    self.i += 1;
                } else {
                    loop {
                        items.push(self.value()?);
                        self.ws();
                        match self.s.get(self.i)? {
                            b',' => self.i += 1,
                            b']' => {
                                self.i += 1;
                                break;
                            }
                            _ => return None,
                        }
                    }
                }
                Value::array(items)
            }
            b'{' => {
                self.i += 1;
                let o = Obj::new();
                self.ws();
                if self.s.get(self.i) == Some(&b'}') {
                    self.i += 1;
                } else {
                    loop {
                        self.ws();
                        let Value::String(k) = self.string()? else {
                            return None;
                        };
                        self.ws();
                        if self.s.get(self.i) != Some(&b':') {
                            return None;
                        }
                        self.i += 1;
                        let v = self.value()?;
                        o.set(&k, v);
                        self.ws();
                        match self.s.get(self.i)? {
                            b',' => self.i += 1,
                            b'}' => {
                                self.i += 1;
                                break;
                            }
                            _ => return None,
                        }
                    }
                }
                Value::Object(o)
            }
            b'"' => self.string()?,
            b't' if self.s[self.i..].starts_with(b"true") => {
                self.i += 4;
                Value::Bool(true)
            }
            b'f' if self.s[self.i..].starts_with(b"false") => {
                self.i += 5;
                Value::Bool(false)
            }
            b'n' if self.s[self.i..].starts_with(b"null") => {
                self.i += 4;
                Value::Null
            }
            _ => self.number()?,
        };
        Some(v)
    }
    fn string(&mut self) -> Option<Value> {
        if self.s.get(self.i) != Some(&b'"') {
            return None;
        }
        self.i += 1;
        let mut units: Vec<u16> = vec![];
        loop {
            let c = *self.s.get(self.i)?;
            match c {
                b'"' => {
                    self.i += 1;
                    break;
                }
                b'\\' => {
                    let e = *self.s.get(self.i + 1)?;
                    self.i += 2;
                    match e {
                        b'"' => units.push(0x22),
                        b'\\' => units.push(0x5c),
                        b'/' => units.push(0x2f),
                        b'b' => units.push(8),
                        b'f' => units.push(12),
                        b'n' => units.push(10),
                        b'r' => units.push(13),
                        b't' => units.push(9),
                        b'u' => {
                            let h = std::str::from_utf8(self.s.get(self.i..self.i + 4)?).ok()?;
                            if !h.bytes().all(|b| b.is_ascii_hexdigit()) {
                                return None;
                            }
                            units.push(u16::from_str_radix(h, 16).ok()?);
                            self.i += 4;
                        }
                        _ => return None,
                    }
                }
                0..=0x1f => return None,
                _ => {
                    // One UTF-8 character.
                    let len = match c {
                        0..=0x7f => 1,
                        0xc0..=0xdf => 2,
                        0xe0..=0xef => 3,
                        _ => 4,
                    };
                    let ch = std::str::from_utf8(self.s.get(self.i..self.i + len)?).ok()?;
                    units.extend(ch.encode_utf16());
                    self.i += len;
                }
            }
        }
        Some(Value::string(&String::from_utf16_lossy(&units)))
    }
    fn number(&mut self) -> Option<Value> {
        let start = self.i;
        let s = self.s;
        let mut i = self.i;
        if s.get(i) == Some(&b'-') {
            i += 1;
        }
        match s.get(i) {
            Some(b'0') => i += 1,
            Some(b'1'..=b'9') => {
                while s.get(i).is_some_and(u8::is_ascii_digit) {
                    i += 1;
                }
            }
            _ => return None,
        }
        if s.get(i) == Some(&b'.') {
            i += 1;
            if !s.get(i).is_some_and(u8::is_ascii_digit) {
                return None;
            }
            while s.get(i).is_some_and(u8::is_ascii_digit) {
                i += 1;
            }
        }
        if matches!(s.get(i), Some(b'e' | b'E')) {
            i += 1;
            if matches!(s.get(i), Some(b'+' | b'-')) {
                i += 1;
            }
            if !s.get(i).is_some_and(u8::is_ascii_digit) {
                return None;
            }
            while s.get(i).is_some_and(u8::is_ascii_digit) {
                i += 1;
            }
        }
        self.i = i;
        std::str::from_utf8(&s[start..i])
            .ok()?
            .parse::<f64>()
            .ok()
            .map(Value::Number)
    }
}
fn json_parse(s: &str) -> Option<Value> {
    let mut p = Json {
        s: s.as_bytes(),
        i: 0,
    };
    let v = p.value()?;
    p.ws();
    (p.i == p.s.len()).then_some(v)
}

/// `str.split( ',' ).map( s => parseFloat( s.trim() ) )`.
fn split_floats(s: &str) -> Value {
    Value::numbers(s.split(',').map(|p| parse_float(trim(p))).collect())
}

/// _parseString( str ).
pub(crate) fn parse_string(s: &str) -> String {
    let s = if s.len() >= 2
        && ((s.starts_with('"') && s.ends_with('"')) || (s.starts_with('\'') && s.ends_with('\'')))
    {
        &s[1..s.len() - 1]
    } else if s == "\"" || s == "'" {
        // A lone quote both starts and ends the string: slice( 1, -1 ) is empty.
        ""
    } else {
        s
    };
    let chars: Vec<char> = s.chars().collect();
    let mut out = String::new();
    let mut i = 0;
    while i < chars.len() {
        if chars[i] == '\\' && i + 1 < chars.len() {
            out.push(match chars[i + 1] {
                'n' => '\n',
                't' => '\t',
                'r' => '\r',
                c => c,
            });
            i += 2;
        } else {
            out.push(chars[i]);
            i += 1;
        }
    }
    out
}

/// _parseAttributeValue( valueType, rawValue ).
fn parse_attribute_value(value_type: &str, raw: &Value) -> Value {
    if raw.is_nullish() {
        return Value::Undefined;
    }
    let text = raw.to_js_string();
    let s = trim(&text);
    if value_type.ends_with("[]") {
        let mut cleaned = s.replace('(', "[").replace(')', "]");
        if cleaned.ends_with(',') {
            cleaned.pop();
        }
        let result = match json_parse(&cleaned) {
            Some(parsed) => {
                let nested = parsed.is_array() && parsed.at(0).is_array();
                if nested {
                    // flat(): one level.
                    let mut flat = vec![];
                    for v in parsed.items() {
                        if v.is_array() {
                            flat.extend(v.items());
                        } else {
                            flat.push(v);
                        }
                    }
                    Value::array(flat)
                } else {
                    parsed
                }
            }
            None => {
                let cleaned = s.replace(['[', ']'], "");
                Value::array(
                    cleaned
                        .split(',')
                        .map(|p| {
                            let t = trim(p);
                            let n = parse_float(t);
                            if n.is_nan() {
                                Value::string(&t.replace('"', ""))
                            } else {
                                Value::Number(n)
                            }
                        })
                        .collect(),
                )
            }
        };
        if value_type.starts_with("quat")
            && let Value::Array(a) = &result
        {
            // Reorder ( w, x, y, z ) to ( x, y, z, w ); stores past the end
            // extend the array.
            let mut a = a.borrow_mut();
            let get = |a: &Vec<Value>, i: usize| a.get(i).cloned().unwrap_or_default();
            let set = |a: &mut Vec<Value>, i: usize, v: Value| {
                if i >= a.len() {
                    a.resize(i + 1, Value::Undefined);
                }
                a[i] = v;
            };
            let mut i = 0;
            while i < a.len() {
                let w = get(&a, i);
                let v = get(&a, i + 1);
                set(&mut a, i, v);
                let v = get(&a, i + 2);
                set(&mut a, i + 1, v);
                let v = get(&a, i + 3);
                set(&mut a, i + 2, v);
                set(&mut a, i + 3, w);
                i += 4;
            }
        }
        return result;
    }
    if value_type.contains('3') || value_type.contains('2') || value_type.contains('4') {
        return split_floats(&s.replace(['(', ')'], ""));
    }
    if value_type.starts_with("quat") {
        let v = split_floats(&s.replace(['(', ')'], ""));
        return Value::array(vec![v.at(1), v.at(2), v.at(3), v.at(0)]);
    }
    if value_type.contains("matrix") {
        return split_floats(&s.replace(['(', ')'], ""));
    }
    if value_type == "float" || value_type == "double" || value_type == "int" {
        return Value::Number(parse_float(s));
    }
    if value_type == "string" || value_type == "token" {
        return Value::string(&parse_string(s));
    }
    if value_type == "asset" {
        return Value::string(&s.replace(['@', '"'], ""));
    }
    Value::string(&parse_string(s))
}

/// `value.replace( … )` needs a string.
fn str_of(v: &Value) -> Result<String> {
    v.as_str()
        .map(str::to_string)
        .ok_or_else(|| type_error("replace is not a function"))
}

/// _extractPrimData( data, path, primFields, specsByPath ).
fn extract_prim_data(data: &Value, path: &str, prim_fields: &Obj, specs: &mut Specs) -> Result<()> {
    let Value::Object(data) = data else {
        return Ok(());
    };
    for (key, value) in data.entries() {
        if key.starts_with("def ") {
            continue;
        }
        if &*key == "prepend references" {
            prim_fields.set("references", Value::array(vec![value]));
            continue;
        }
        if &*key == "payload" {
            prim_fields.set("payload", value);
            continue;
        }
        if &*key == "variants" {
            let selection = Obj::new();
            if let Value::Object(variants) = &value {
                for (v_key, v_value) in variants.entries() {
                    if let Some(set_name) = variant_string(&v_key) {
                        selection.set(set_name, Value::string(&str_of(&v_value)?.replace('"', "")));
                    }
                }
            }
            if selection.len() > 0 {
                prim_fields.set("variantSelection", Value::Object(selection));
            }
            continue;
        }
        if let Some(rel_name) = key.strip_prefix("rel ") {
            let rel_path = format!("{path}.{rel_name}");
            let target = str_of(&value)?.replace(['<', '>'], "");
            let fields = Obj::new();
            fields.set("targetPaths", Value::array(vec![Value::string(&target)]));
            if let Some(meta) = data.metadata()
                && let Value::Object(m) = meta.get(&key)
            {
                let bind = m.get("bindMaterialAs");
                if !bind.is_undefined() {
                    fields.set(
                        "bindMaterialAs",
                        Value::string(&parse_string(trim(&bind.to_js_string()))),
                    );
                }
            }
            specs.set(&rel_path, Spec::new(RELATIONSHIP, fields));
            continue;
        }
        if key.contains("xformOpOrder") {
            let ops: Vec<Value> = str_of(&value)?
                .replace(['[', ']'], "")
                .split(',')
                .map(|s| Value::string(&trim(s).replace('"', "")))
                .collect();
            prim_fields.set("xformOpOrder", Value::array(ops));
            continue;
        }
        let Some((value_type, attr_name)) = attr_match(&key) else {
            continue;
        };
        if let Some(base) = attr_name.strip_suffix(".connect") {
            let attr_path = format!("{path}.{base}");
            let text = value.to_js_string();
            let mut conn = trim(&text);
            if let Some(c) = conn.strip_prefix('<') {
                conn = c;
            }
            if let Some(c) = conn.strip_suffix('>') {
                conn = c;
            }
            if specs.get(&attr_path).is_none() {
                let fields = Obj::new();
                fields.set("typeName", Value::string(value_type));
                specs.set(&attr_path, Spec::new(ATTRIBUTE, fields));
            }
            if let Some(spec) = specs.get(&attr_path) {
                spec.fields
                    .set("connectionPaths", Value::array(vec![Value::string(conn)]));
            }
            continue;
        }
        if let (Some(base), Value::Object(samples)) =
            (attr_name.strip_suffix(".timeSamples"), &value)
        {
            let attr_path = format!("{path}.{base}");
            let mut sorted: Vec<(f64, Value)> = vec![];
            for (frame_key, raw) in samples.entries() {
                let frame = parse_float(&frame_key);
                if frame.is_nan() {
                    continue;
                }
                sorted.push((frame, parse_attribute_value(value_type, &raw)));
            }
            // A stable sort by time.
            sorted.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
            let time_samples = Obj::new();
            time_samples.set(
                "times",
                Value::numbers(sorted.iter().map(|s| s.0).collect()),
            );
            time_samples.set(
                "values",
                Value::array(sorted.into_iter().map(|s| s.1).collect()),
            );
            let fields = Obj::new();
            fields.set("timeSamples", Value::Object(time_samples));
            fields.set("typeName", Value::string(value_type));
            specs.set(&attr_path, Spec::new(ATTRIBUTE, fields));
        } else {
            let parsed = parse_attribute_value(value_type, &value);
            let attr_path = format!("{path}.{attr_name}");
            if let Some(spec) = specs.get(&attr_path) {
                spec.fields.set("default", parsed);
                spec.fields.set("typeName", Value::string(value_type));
            } else {
                let fields = Obj::new();
                fields.set("default", parsed);
                fields.set("typeName", Value::string(value_type));
                specs.set(&attr_path, Spec::new(ATTRIBUTE, fields));
            }
        }
    }
    Ok(())
}

fn walk_tree(data: &Value, parent: &str, specs: &mut Specs) -> Result<()> {
    let Value::Object(data) = data else {
        return Ok(());
    };
    let mut children = vec![];
    for (key, prim_data) in data.entries() {
        if &*key == "#usda 1.0" || &*key == "variants" {
            continue;
        }
        let Some((type_name, name)) = def_match(&key) else {
            continue;
        };
        let path = if parent == "/" {
            format!("/{name}")
        } else {
            format!("{parent}/{name}")
        };
        children.push(Value::string(name));
        let fields = Obj::new();
        fields.set("typeName", Value::string(type_name.unwrap_or("")));
        extract_prim_data(&prim_data, &path, &fields, specs)?;
        specs.set(&path, Spec::new(PRIM, fields));
        walk_tree(&prim_data, &path, specs)?;
    }
    if !children.is_empty()
        && let Some(spec) = specs.get(parent)
    {
        spec.fields.set("primChildren", Value::array(children));
    }
    Ok(())
}

fn infer_element_size(spec: Option<std::rc::Rc<Spec>>, vertices: f64) {
    let Some(spec) = spec else {
        return;
    };
    let default = spec.fields.get("default");
    if !spec.fields.get("elementSize").is_undefined() || !default.truthy() {
        return;
    }
    let len = default.len_number();
    if len > 0.0 && len % vertices == 0.0 {
        spec.fields
            .set("elementSize", Value::Number(len / vertices));
    }
}

/// parseData( text ).
pub(crate) fn parse(text: &str) -> Result<Specs> {
    let root = parse_text(text)?;
    let mut specs = Specs::default();
    let root_fields = Obj::new();
    if root.has("#usda 1.0") {
        let header = root.get("#usda 1.0");
        let up = header.get("upAxis");
        if up.truthy() {
            root_fields.set("upAxis", Value::string(&str_of(&up)?.replace('"', "")));
        }
        let prim = header.get("defaultPrim");
        if prim.truthy() {
            root_fields.set(
                "defaultPrim",
                Value::string(&str_of(&prim)?.replace('"', "")),
            );
        }
        for key in ["metersPerUnit", "framesPerSecond", "timeCodesPerSecond"] {
            let v = header.get(key);
            if !v.is_undefined() {
                root_fields.set(key, Value::Number(parse_float(&v.to_js_string())));
            }
        }
    }
    specs.set("/", Spec::new(PRIM, root_fields));
    walk_tree(&Value::Object(root), "/", &mut specs)?;
    // _inferSkelElementSize
    for path in specs.paths() {
        let Some(spec) = specs.get(&path) else {
            continue;
        };
        if spec.spec_type != PRIM
            || !spec
                .fields
                .get("typeName")
                .strict_eq(&Value::string("Mesh"))
        {
            continue;
        }
        let Some(points) = specs.get(&format!("{path}.points")) else {
            continue;
        };
        let default = points.fields.get("default");
        if !default.truthy() {
            continue;
        }
        let vertices = default.len_number() / 3.0;
        if vertices == 0.0 {
            continue;
        }
        infer_element_size(
            specs.get(&format!("{path}.primvars:skel:jointIndices")),
            vertices,
        );
        infer_element_size(
            specs.get(&format!("{path}.primvars:skel:jointWeights")),
            vertices,
        );
    }
    Ok(specs)
}
