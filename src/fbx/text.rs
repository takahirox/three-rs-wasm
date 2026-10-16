//! FBXLoader's TextParser ( FBX ASCII ), with its regular expressions
//! expanded into the equivalent string matching.
use super::js::{Obj, Value, is_js_space, number_key, parse_float, parse_int};
use crate::{Error, Result};
use std::cell::RefCell;
use std::rc::Rc;

fn bad(what: &str) -> Error {
    Error::Asset(format!("THREE.FBXLoader: {what}"))
}

/// `String.prototype.trim()`.
fn trim(s: &str) -> &str {
    s.trim_matches(is_js_space)
}
/// `.replace( /^"/, '' ).replace( /"$/, '' )`.
fn strip_quotes(s: &str) -> &str {
    let s = s.strip_prefix('"').unwrap_or(s);
    s.strip_suffix('"').unwrap_or(s)
}
fn is_word(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_'
}
/// `^\t{n}(\w+):` and the remainder after the colon.
fn word_prefix(line: &str, tabs: usize) -> Option<(&str, &str)> {
    let rest = line.get(..tabs).filter(|t| t.bytes().all(|b| b == b'\t'))?;
    let rest = &line[rest.len()..];
    let end = rest.find(|c: char| !is_word(c)).unwrap_or(rest.len());
    if end == 0 || !rest[end..].starts_with(':') {
        return None;
    }
    Some((&rest[..end], &rest[end + 1..]))
}

/// isFbxFormatASCII(): none of the characters it samples ( at the
/// triangular offsets ) is the corresponding one of `Kaydara\FBX\Binary\\`.
pub(crate) fn is_ascii(text: &str) -> bool {
    const CORRECT: &[u16] = &[
        b'K' as u16,
        b'a' as u16,
        b'y' as u16,
        b'd' as u16,
        b'a' as u16,
        b'r' as u16,
        b'a' as u16,
        b'\\' as u16,
        b'F' as u16,
        b'B' as u16,
        b'X' as u16,
        b'\\' as u16,
        b'B' as u16,
        b'i' as u16,
        b'n' as u16,
        b'a' as u16,
        b'r' as u16,
        b'y' as u16,
        b'\\' as u16,
        b'\\' as u16,
    ];
    let units: Vec<u16> = text.encode_utf16().take(256).collect();
    let mut position = 0;
    for (i, &c) in CORRECT.iter().enumerate() {
        if units.get(position) == Some(&c) {
            return false;
        }
        position += i + 1;
    }
    true
}

/// getFbxVersion(): the first `FBXVersion: <digits>`.
pub(crate) fn version(text: &str) -> Result<f64> {
    let mut from = 0;
    while let Some(i) = text[from..].find("FBXVersion: ") {
        let start = from + i + "FBXVersion: ".len();
        let digits = text[start..]
            .find(|c: char| !c.is_ascii_digit())
            .map_or(&text[start..], |e| &text[start..start + e]);
        if !digits.is_empty() {
            return Ok(parse_int(digits));
        }
        from = start;
    }
    Err(bad("Cannot find the version number for the file given."))
}

/// parseNumberArray(): `value.split( ',' ).map( parseFloat )`.
pub(crate) fn number_array(value: &str) -> Value {
    Value::numbers(value.split(',').map(parse_float).collect())
}

struct Parser {
    indent: usize,
    tree: Obj,
    stack: Vec<Obj>,
}

impl Parser {
    fn current(&self) -> Option<Obj> {
        self.indent
            .checked_sub(1)
            .and_then(|i| self.stack.get(i))
            .cloned()
    }
    fn prev(&self) -> Option<Obj> {
        self.indent
            .checked_sub(2)
            .and_then(|i| self.stack.get(i))
            .cloned()
    }
    fn node_begin(&mut self, name: &str, attrs: &str) -> Result<()> {
        let node_name = strip_quotes(trim(name)).to_string();
        let attrs: Vec<String> = attrs
            .split(',')
            .map(|a| strip_quotes(trim(a)).to_string())
            .collect();
        let node = Obj::new();
        node.set("name", Value::string(&node_name));
        // parseNodeAttr()
        let id = if attrs[0].is_empty() {
            Value::string("")
        } else {
            let v = parse_int(&attrs[0]);
            if v.is_nan() {
                Value::string(&attrs[0])
            } else {
                Value::Number(v)
            }
        };
        let (attr_name, attr_type) = if attrs.len() > 1 {
            let n = &attrs[1];
            let stripped = match n.find("::") {
                Some(i) if i > 0 && n[..i].chars().all(is_word) => &n[i + 2..],
                _ => n.as_str(),
            };
            (
                Value::string(stripped),
                attrs.get(2).map_or(Value::Undefined, |t| Value::string(t)),
            )
        } else {
            (Value::string(""), Value::string(""))
        };
        if self.indent == 0 {
            self.tree.set(&node_name, Value::Object(node.clone()));
        } else {
            let current = self.current().ok_or_else(|| bad("missing parent node"))?;
            if current.has(&node_name) {
                if node_name == "PoseNode" {
                    if let Value::Array(a) = current.get("PoseNode") {
                        a.borrow_mut().push(Value::Object(node.clone()));
                    }
                } else if !current.get(&node_name).get("id").is_undefined() {
                    // The original replaces the entry with an empty object
                    // holding itself under the key "undefined".
                    let container = Obj::new();
                    current.set(&node_name, Value::Object(container.clone()));
                    container.set("undefined", Value::Object(container.clone()));
                }
                if !id.strict_eq(&Value::string("")) {
                    let key = id.to_key();
                    match current.get(&node_name) {
                        Value::Object(o) => o.set(&key, Value::Object(node.clone())),
                        Value::Array(a) => {
                            if let Ok(i) = key.parse::<usize>()
                                && number_key(i as f64) == key
                            {
                                let mut a = a.borrow_mut();
                                if a.len() <= i {
                                    a.resize(i + 1, Value::Undefined);
                                }
                                a[i] = Value::Object(node.clone());
                            }
                        }
                        _ => {}
                    }
                }
            } else if let Value::Number(n) = id {
                let container = Obj::new();
                container.set(&number_key(n), Value::Object(node.clone()));
                current.set(&node_name, Value::Object(container));
            } else if node_name != "Properties70" {
                if node_name == "PoseNode" {
                    current.set(
                        &node_name,
                        Value::Array(Rc::new(RefCell::new(vec![Value::Object(node.clone())]))),
                    );
                } else {
                    current.set(&node_name, Value::Object(node.clone()));
                }
            }
        }
        if matches!(id, Value::Number(_)) {
            node.set("id", id);
        }
        if !attr_name.strict_eq(&Value::string("")) {
            node.set("attrName", attr_name);
        }
        if !attr_type.strict_eq(&Value::string("")) {
            node.set("attrType", attr_type);
        }
        self.stack.push(node);
        self.indent += 1;
        Ok(())
    }
    fn node_property(&mut self, name: &str, value: &str, content: Option<&str>) -> Result<()> {
        let mut prop_name = trim(strip_quotes(name)).to_string();
        let mut prop_value = trim(strip_quotes(value)).to_string();
        if prop_name == "Content" && prop_value == "," {
            let line = content.ok_or_else(|| bad("missing Content line"))?;
            let line = line.replace('"', "");
            prop_value = trim(line.strip_suffix(',').unwrap_or(&line)).to_string();
        }
        let current = self
            .current()
            .ok_or_else(|| bad("property outside a node"))?;
        if current.get("name").as_str() == Some("Properties70") {
            return self.special_property(&prop_value);
        }
        let mut value = Value::string(&prop_value);
        if prop_name == "C" {
            let parts: Vec<&str> = prop_value.split(',').collect();
            let from = parts.get(1).map_or(f64::NAN, |p| parse_int(p));
            let to = parts.get(2).map_or(f64::NAN, |p| parse_int(p));
            let mut array = vec![Value::Number(from), Value::Number(to)];
            for rest in parts.iter().skip(3) {
                let r = trim(rest);
                array.push(Value::string(r.strip_prefix('"').unwrap_or(r)));
            }
            prop_name = "connections".into();
            value = Value::Array(Rc::new(RefCell::new(array)));
            if current.get("connections").is_undefined() {
                current.set("connections", Value::Array(Rc::new(RefCell::new(vec![]))));
            }
        }
        if prop_name == "Node" {
            current.set("id", value.clone());
        }
        match current.get(&prop_name) {
            Value::Array(a) if current.has(&prop_name) => a.borrow_mut().push(value),
            Value::Numbers(a) if current.has(&prop_name) => {
                a.borrow_mut().push(value.to_number());
            }
            _ => current.set(&prop_name, value),
        }
        if prop_name == "a" && !prop_value.ends_with(',') {
            current.set("a", number_array(&prop_value));
        }
        Ok(())
    }
    fn property_continued(&mut self, line: &str) -> Result<()> {
        let current = self
            .current()
            .ok_or_else(|| bad("continuation outside a node"))?;
        let joined = format!("{}{line}", current.get("a").to_key());
        if line.ends_with(',') {
            current.set("a", Value::string(&joined));
        } else {
            current.set("a", number_array(&joined));
        }
        Ok(())
    }
    /// parseNodeSpecialProperty(): a `P:` entry, set on the node holding the
    /// Properties70.
    fn special_property(&mut self, value: &str) -> Result<()> {
        let props: Vec<String> = value
            .split("\",")
            .map(|p| {
                let p = trim(p);
                let p = p.strip_prefix('"').unwrap_or(p);
                match p.find(is_js_space) {
                    Some(i) => {
                        let c = p[i..].chars().next().map_or(1, char::len_utf8);
                        format!("{}_{}", &p[..i], &p[i + c..])
                    }
                    None => p.to_string(),
                }
            })
            .collect();
        let get = |i: usize| props.get(i).map_or(Value::Undefined, |p| Value::string(p));
        let type1 = props.get(1).cloned();
        let raw = props.get(4);
        let value = match type1.as_deref() {
            Some("int" | "enum" | "bool" | "ULongLong" | "double" | "Number" | "FieldOfView") => {
                Value::Number(raw.map_or(f64::NAN, |v| parse_float(v)))
            }
            Some(
                "Color" | "ColorRGB" | "Vector3D" | "Lcl_Translation" | "Lcl_Rotation"
                | "Lcl_Scaling",
            ) => number_array(raw.ok_or_else(|| bad("missing property value"))?),
            _ => get(4),
        };
        let p = Obj::new();
        p.set("type", get(1));
        p.set("type2", get(2));
        p.set("flag", get(3));
        p.set("value", value);
        let prev = self
            .prev()
            .ok_or_else(|| bad("Properties70 outside a node"))?;
        prev.set(&get(0).to_key(), Value::Object(p));
        Ok(())
    }
}

/// TextParser.parse(): the FBXTree.
pub(crate) fn parse(text: &str) -> Result<Obj> {
    let mut parser = Parser {
        indent: 0,
        tree: Obj::new(),
        stack: vec![],
    };
    let lines: Vec<&str> = text.split(['\r', '\n']).filter(|l| !l.is_empty()).collect();
    // split( /[\r\n]+/ ) keeps an empty first or last piece; neither matters.
    for (i, line) in lines.iter().enumerate() {
        let blank = trim_leading(line);
        if blank.starts_with(';') || blank.is_empty() {
            continue;
        }
        let n = parser.indent;
        if let Some((name, rest)) = word_prefix(line, n)
            && let Some(brace) = rest.rfind('{')
        {
            parser.node_begin(name, &rest[..brace])?;
        } else if let Some((name, rest)) = word_prefix(line, n)
            && rest.starts_with(is_js_space)
        {
            let c = rest.chars().next().map_or(1, char::len_utf8);
            parser.node_property(name, &rest[c..], lines.get(i + 1).copied())?;
        } else if is_end(line, n) {
            parser.stack.pop();
            parser.indent = parser.indent.saturating_sub(1);
        } else if line
            .chars()
            .next()
            .is_some_and(|c| !is_js_space(c) && c != '}')
        {
            parser.property_continued(line)?;
        }
    }
    Ok(parser.tree)
}

/// `^[\s\t]*`: the line after its leading whitespace.
fn trim_leading(line: &str) -> &str {
    line.trim_start_matches(is_js_space)
}

/// `^\t{n - 1}}`; with no open node the pattern is the literal `\t{-1}}`.
fn is_end(line: &str, indent: usize) -> bool {
    match indent.checked_sub(1) {
        None => line.starts_with("\t{-1}}"),
        Some(k) => {
            line.len() > k
                && line.as_bytes()[..k].iter().all(|&b| b == b'\t')
                && line[k..].starts_with('}')
        }
    }
}
