//! The JavaScript values FBXLoader's parsers build its FBXTree from. Objects
//! are shared references with JavaScript's property order ( array-index keys
//! ascending, then the other keys in insertion order ), so the loader's
//! `for ( const nodeID in … )` loops and in-place mutations behave as in the
//! original.
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone, Debug, Default)]
pub(crate) enum Value {
    #[default]
    Undefined,
    Bool(bool),
    Number(f64),
    String(Rc<str>),
    /// An ArrayBuffer ( binary `R` properties ).
    Bytes(Rc<Vec<u8>>),
    /// A plain array of numbers, as the parsers' typed reads and
    /// `parseNumberArray` produce.
    Numbers(Rc<RefCell<Vec<f64>>>),
    Array(Rc<RefCell<Vec<Value>>>),
    Object(Obj),
}

#[derive(Clone, Debug, Default)]
pub(crate) struct Obj(Rc<RefCell<Object>>);

#[derive(Debug, Default)]
pub(crate) struct Object {
    order: Vec<Rc<str>>,
    map: HashMap<Rc<str>, Value>,
}

/// Whether a property key is an array index ( a canonical integer below 2³² − 1 ).
fn is_index(key: &str) -> bool {
    !key.is_empty()
        && key.len() <= 10
        && key.bytes().all(|b| b.is_ascii_digit())
        && (key == "0" || !key.starts_with('0'))
        && key.parse::<u64>().is_ok_and(|v| v < u32::MAX as u64)
}

/// `String( number )` for the integral values FBX ids and indices take.
pub(crate) fn number_key(v: f64) -> String {
    if v == 0.0 {
        "0".into()
    } else if v.is_nan() {
        "NaN".into()
    } else if v.is_infinite() {
        if v > 0.0 { "Infinity" } else { "-Infinity" }.into()
    } else if v.fract() == 0.0 && v.abs() < 1e21 {
        // Rust prints the shortest round-trip digits padded with zeros, as
        // Number.prototype.toString does below 10²¹.
        format!("{v}")
    } else {
        js_number_string(v)
    }
}

/// Number.prototype.toString() ( radix 10 ).
pub(crate) fn js_number_string(v: f64) -> String {
    if v.fract() == 0.0 && v.abs() < 1e21 {
        return number_key(v);
    }
    if v.is_nan() || v.is_infinite() || v == 0.0 {
        return number_key(v);
    }
    // The shortest round-trip digits and exponent ( where two such digit
    // strings are equally near, either can differ from the engine's ).
    let s = format!("{:e}", v.abs());
    let (mantissa, exponent) = s.split_once('e').unwrap_or((&s, "0"));
    let digits: String = mantissa.chars().filter(|c| *c != '.').collect();
    let e: i32 = exponent.parse().unwrap_or(0);
    let k = digits.len() as i32;
    let n = e + 1;
    let sign = if v < 0.0 { "-" } else { "" };
    let body = if k <= n && n <= 21 {
        format!("{digits}{}", "0".repeat((n - k) as usize))
    } else if 0 < n && n <= 21 {
        format!("{}.{}", &digits[..n as usize], &digits[n as usize..])
    } else if -6 < n && n <= 0 {
        format!("0.{}{digits}", "0".repeat((-n) as usize))
    } else {
        let exp = if n > 0 {
            format!("+{}", n - 1)
        } else {
            format!("{}", n - 1)
        };
        if k == 1 {
            format!("{digits}e{exp}")
        } else {
            format!("{}.{}e{exp}", &digits[..1], &digits[1..])
        }
    };
    format!("{sign}{body}")
}

impl Obj {
    pub(crate) fn new() -> Self {
        Self::default()
    }
    pub(crate) fn get(&self, key: &str) -> Value {
        self.0.borrow().map.get(key).cloned().unwrap_or_default()
    }
    pub(crate) fn has(&self, key: &str) -> bool {
        self.0.borrow().map.contains_key(key)
    }
    pub(crate) fn set(&self, key: &str, value: Value) {
        let mut o = self.0.borrow_mut();
        if let Some(v) = o.map.get_mut(key) {
            *v = value;
        } else {
            let key: Rc<str> = key.into();
            o.order.push(key.clone());
            o.map.insert(key, value);
        }
    }
    /// `Object.keys( o )`: array indices ascending, then insertion order.
    pub(crate) fn keys(&self) -> Vec<Rc<str>> {
        let o = self.0.borrow();
        let mut indices: Vec<(u64, Rc<str>)> = o
            .order
            .iter()
            .filter(|k| is_index(k))
            .map(|k| (k.parse().unwrap_or(0), k.clone()))
            .collect();
        indices.sort_by_key(|(i, _)| *i);
        let mut keys: Vec<Rc<str>> = indices.into_iter().map(|(_, k)| k).collect();
        keys.extend(o.order.iter().filter(|k| !is_index(k)).cloned());
        keys
    }
    pub(crate) fn entries(&self) -> Vec<(Rc<str>, Value)> {
        self.keys()
            .into_iter()
            .map(|k| {
                let v = self.get(&k);
                (k, v)
            })
            .collect()
    }
    pub(crate) fn ptr_eq(&self, other: &Obj) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl Value {
    pub(crate) fn string(s: &str) -> Self {
        Value::String(s.into())
    }
    pub(crate) fn numbers(v: Vec<f64>) -> Self {
        Value::Numbers(Rc::new(RefCell::new(v)))
    }
    pub(crate) fn is_undefined(&self) -> bool {
        matches!(self, Value::Undefined)
    }
    pub(crate) fn as_obj(&self) -> Option<&Obj> {
        match self {
            Value::Object(o) => Some(o),
            _ => None,
        }
    }
    /// `value[ key ]` on an object ( undefined otherwise; arrays by index ).
    pub(crate) fn get(&self, key: &str) -> Value {
        match self {
            Value::Object(o) => o.get(key),
            Value::Numbers(a) => key
                .parse::<usize>()
                .ok()
                .filter(|_| is_index(key))
                .and_then(|i| a.borrow().get(i).copied())
                .map_or(Value::Undefined, Value::Number),
            Value::Array(a) => key
                .parse::<usize>()
                .ok()
                .filter(|_| is_index(key))
                .and_then(|i| a.borrow().get(i).cloned())
                .unwrap_or_default(),
            Value::String(s) if key == "length" => Value::Number(s.encode_utf16().count() as f64),
            _ => Value::Undefined,
        }
    }
    pub(crate) fn has(&self, key: &str) -> bool {
        match self {
            Value::Object(o) => o.has(key),
            _ => false,
        }
    }
    /// JavaScript truthiness.
    pub(crate) fn truthy(&self) -> bool {
        match self {
            Value::Undefined => false,
            Value::Bool(b) => *b,
            Value::Number(n) => *n != 0.0 && !n.is_nan(),
            Value::String(s) => !s.is_empty(),
            _ => true,
        }
    }
    pub(crate) fn is_array(&self) -> bool {
        matches!(self, Value::Numbers(_) | Value::Array(_))
    }
    pub(crate) fn as_str(&self) -> Option<&str> {
        match self {
            Value::String(s) => Some(s),
            _ => None,
        }
    }
    /// `ToNumber( value )`.
    pub(crate) fn to_number(&self) -> f64 {
        match self {
            Value::Undefined => f64::NAN,
            Value::Bool(b) => f64::from(u8::from(*b)),
            Value::Number(n) => *n,
            Value::String(s) => string_to_number(s),
            Value::Numbers(a) => {
                let a = a.borrow();
                match a.len() {
                    0 => 0.0,
                    1 => a[0],
                    _ => f64::NAN,
                }
            }
            Value::Array(a) => {
                let a = a.borrow();
                match a.len() {
                    0 => 0.0,
                    1 => a[0].to_number(),
                    _ => f64::NAN,
                }
            }
            _ => f64::NAN,
        }
    }
    /// `String( value )` for the values property keys come from.
    pub(crate) fn to_key(&self) -> String {
        match self {
            Value::Undefined => "undefined".into(),
            Value::Bool(b) => b.to_string(),
            Value::Number(n) => number_key(*n),
            Value::String(s) => s.to_string(),
            Value::Numbers(a) => a
                .borrow()
                .iter()
                .map(|v| js_number_string(*v))
                .collect::<Vec<_>>()
                .join(","),
            Value::Array(a) => a
                .borrow()
                .iter()
                .map(|v| match v {
                    Value::Undefined => String::new(),
                    v => v.to_key(),
                })
                .collect::<Vec<_>>()
                .join(","),
            Value::Bytes(_) => "[object ArrayBuffer]".into(),
            Value::Object(_) => "[object Object]".into(),
        }
    }
    /// The elements of an array as numbers ( `Number` of each ).
    pub(crate) fn to_f64s(&self) -> Option<Vec<f64>> {
        match self {
            Value::Numbers(a) => Some(a.borrow().clone()),
            Value::Array(a) => Some(a.borrow().iter().map(Value::to_number).collect()),
            _ => None,
        }
    }
    /// Strict equality ( `===` ).
    pub(crate) fn strict_eq(&self, other: &Value) -> bool {
        match (self, other) {
            (Value::Undefined, Value::Undefined) => true,
            (Value::Bool(a), Value::Bool(b)) => a == b,
            (Value::Number(a), Value::Number(b)) => a == b,
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Bytes(a), Value::Bytes(b)) => Rc::ptr_eq(a, b),
            (Value::Numbers(a), Value::Numbers(b)) => Rc::ptr_eq(a, b),
            (Value::Array(a), Value::Array(b)) => Rc::ptr_eq(a, b),
            (Value::Object(a), Value::Object(b)) => a.ptr_eq(b),
            _ => false,
        }
    }
}

/// `StringToNumber`: whitespace-trimmed decimal, hexadecimal, Infinity or empty.
pub(crate) fn string_to_number(s: &str) -> f64 {
    let t = s.trim_matches(is_js_space);
    if t.is_empty() {
        return 0.0;
    }
    if let Some(h) = t.strip_prefix("0x").or_else(|| t.strip_prefix("0X")) {
        return u128::from_str_radix(h, 16).map_or(f64::NAN, |v| v as f64);
    }
    match t {
        "Infinity" | "+Infinity" => return f64::INFINITY,
        "-Infinity" => return f64::NEG_INFINITY,
        _ => {}
    }
    if t.bytes()
        .all(|b| b.is_ascii_digit() || matches!(b, b'.' | b'e' | b'E' | b'+' | b'-'))
        && !t.contains("inf")
    {
        t.parse::<f64>().unwrap_or(f64::NAN)
    } else {
        f64::NAN
    }
}

/// JavaScript's `\s` ( WhiteSpace and LineTerminator ).
pub(crate) fn is_js_space(c: char) -> bool {
    matches!(
        c,
        '\t' | '\n' | '\u{b}' | '\u{c}' | '\r' | ' ' | '\u{a0}' | '\u{1680}' | '\u{2000}'
            ..='\u{200a}'
                | '\u{2028}'
                | '\u{2029}'
                | '\u{202f}'
                | '\u{205f}'
                | '\u{3000}'
                | '\u{feff}'
    )
}

/// `parseFloat( string )`: the longest StrDecimalLiteral prefix after leading
/// whitespace.
pub(crate) fn parse_float(s: &str) -> f64 {
    let t = s.trim_start_matches(is_js_space);
    let b = t.as_bytes();
    let mut i = 0;
    if i < b.len() && (b[i] == b'+' || b[i] == b'-') {
        i += 1;
    }
    if t[i..].starts_with("Infinity") {
        return if b.first() == Some(&b'-') {
            f64::NEG_INFINITY
        } else {
            f64::INFINITY
        };
    }
    let start_digits = i;
    while i < b.len() && b[i].is_ascii_digit() {
        i += 1;
    }
    let mut digits = i > start_digits;
    if i < b.len() && b[i] == b'.' {
        let j = i + 1;
        let mut k = j;
        while k < b.len() && b[k].is_ascii_digit() {
            k += 1;
        }
        if k > j || digits {
            digits = digits || k > j;
            i = k;
        }
    }
    if !digits {
        return f64::NAN;
    }
    if i < b.len() && (b[i] == b'e' || b[i] == b'E') {
        let mut k = i + 1;
        if k < b.len() && (b[k] == b'+' || b[k] == b'-') {
            k += 1;
        }
        let ds = k;
        while k < b.len() && b[k].is_ascii_digit() {
            k += 1;
        }
        if k > ds {
            i = k;
        }
    }
    t[..i].parse::<f64>().unwrap_or(f64::NAN)
}

/// `parseFloat( value )` of any value: numbers convert through their string.
pub(crate) fn parse_float_value(v: &Value) -> f64 {
    match v {
        Value::Number(n) => *n,
        v => parse_float(&v.to_key()),
    }
}

/// `parseInt( string )` ( radix 10, or 16 with a 0x prefix ).
pub(crate) fn parse_int(s: &str) -> f64 {
    let t = s.trim_start_matches(is_js_space);
    let (negative, t) = match t.as_bytes().first() {
        Some(b'-') => (true, &t[1..]),
        Some(b'+') => (false, &t[1..]),
        _ => (false, t),
    };
    let (radix, t) = if t.starts_with("0x") || t.starts_with("0X") {
        (16, &t[2..])
    } else {
        (10, t)
    };
    let end = t
        .char_indices()
        .find(|(_, c)| !c.is_digit(radix))
        .map_or(t.len(), |(i, _)| i);
    if end == 0 {
        return f64::NAN;
    }
    let digits = &t[..end];
    let v = if radix == 10 {
        digits.parse::<f64>().unwrap_or(f64::NAN)
    } else {
        digits.chars().fold(0.0, |a, c| {
            a * 16.0 + f64::from(c.to_digit(16).unwrap_or(0))
        })
    };
    if negative { -v } else { v }
}

/// `parseInt( value )` of any value.
pub(crate) fn parse_int_value(v: &Value) -> f64 {
    parse_int(&v.to_key())
}
