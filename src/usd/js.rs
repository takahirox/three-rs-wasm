//! The JavaScript values USDLoader's parsers build their `specsByPath`
//! from: plain arrays, typed arrays ( `Array.isArray` is false for them, and
//! a store rounds to their element type ), and objects with JavaScript's
//! property order ( array-index keys ascending, then insertion order ), so
//! the composer's `for … in` loops and type checks behave as in the original.
use crate::fbx::js::{is_js_space, js_number_string, number_key, string_to_number};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Typed {
    F32,
    F64,
    I32,
    U32,
    U16,
}
impl Typed {
    /// The value a store of `v` keeps.
    pub(crate) fn store(self, v: f64) -> f64 {
        let int = |m: f64| {
            if !v.is_finite() {
                0.0
            } else {
                v.trunc().rem_euclid(m)
            }
        };
        match self {
            Typed::F32 => f64::from(v as f32),
            Typed::F64 => v,
            Typed::I32 => {
                let u = int(4294967296.0);
                if u >= 2147483648.0 {
                    u - 4294967296.0
                } else {
                    u
                }
            }
            Typed::U32 => int(4294967296.0),
            Typed::U16 => int(65536.0),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub(crate) enum Value {
    #[default]
    Undefined,
    Null,
    Bool(bool),
    Number(f64),
    String(Rc<str>),
    Array(Rc<RefCell<Vec<Value>>>),
    TypedArray(Typed, Rc<RefCell<Vec<f64>>>),
    Object(Obj),
}

#[derive(Clone, Debug, Default)]
pub(crate) struct Obj(Rc<RefCell<Object>>);

#[derive(Debug, Default)]
pub(crate) struct Object {
    order: Vec<Rc<str>>,
    map: HashMap<Rc<str>, Value>,
    /// USDAParser's `VALUE_METADATA` symbol property ( not enumerated ).
    metadata: Option<Obj>,
}

/// Whether a property key is an array index ( a canonical integer below 2³² − 1 ).
fn is_index(key: &str) -> bool {
    !key.is_empty()
        && key.len() <= 10
        && key.bytes().all(|b| b.is_ascii_digit())
        && (key == "0" || !key.starts_with('0'))
        && key.parse::<u64>().is_ok_and(|v| v < u32::MAX as u64)
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
    pub(crate) fn metadata(&self) -> Option<Obj> {
        self.0.borrow().metadata.clone()
    }
    pub(crate) fn set_metadata(&self, m: Obj) {
        self.0.borrow_mut().metadata = Some(m);
    }
    pub(crate) fn len(&self) -> usize {
        self.0.borrow().order.len()
    }
    /// `{ ...a }`: a shallow copy with the same key order.
    pub(crate) fn spread(&self, into: &Obj) {
        for (k, v) in self.entries() {
            into.set(&k, v);
        }
    }
}

impl Value {
    pub(crate) fn string(s: &str) -> Self {
        Value::String(s.into())
    }
    pub(crate) fn array(v: Vec<Value>) -> Self {
        Value::Array(Rc::new(RefCell::new(v)))
    }
    pub(crate) fn numbers(v: Vec<f64>) -> Self {
        Value::array(v.into_iter().map(Value::Number).collect())
    }
    pub(crate) fn typed(kind: Typed, v: Vec<f64>) -> Self {
        Value::TypedArray(kind, Rc::new(RefCell::new(v)))
    }
    pub(crate) fn is_undefined(&self) -> bool {
        matches!(self, Value::Undefined)
    }
    pub(crate) fn is_nullish(&self) -> bool {
        matches!(self, Value::Undefined | Value::Null)
    }
    pub(crate) fn as_str(&self) -> Option<&str> {
        match self {
            Value::String(s) => Some(s),
            _ => None,
        }
    }
    /// `Array.isArray( v )`.
    pub(crate) fn is_array(&self) -> bool {
        matches!(self, Value::Array(_))
    }
    /// An array or typed array.
    pub(crate) fn is_list(&self) -> bool {
        matches!(self, Value::Array(_) | Value::TypedArray(..))
    }
    pub(crate) fn truthy(&self) -> bool {
        match self {
            Value::Undefined | Value::Null => false,
            Value::Bool(b) => *b,
            Value::Number(n) => *n != 0.0 && !n.is_nan(),
            Value::String(s) => !s.is_empty(),
            _ => true,
        }
    }
    /// `v.length` ( undefined for values without one ).
    pub(crate) fn length(&self) -> Option<usize> {
        match self {
            Value::Array(a) => Some(a.borrow().len()),
            Value::TypedArray(_, a) => Some(a.borrow().len()),
            Value::String(s) => Some(s.encode_utf16().count()),
            _ => None,
        }
    }
    /// `v.length` as a number ( NaN when undefined, for comparisons ).
    pub(crate) fn len_number(&self) -> f64 {
        self.length().map_or(f64::NAN, |l| l as f64)
    }
    /// `v[ i ]`.
    pub(crate) fn at(&self, i: usize) -> Value {
        match self {
            Value::Array(a) => a.borrow().get(i).cloned().unwrap_or_default(),
            Value::TypedArray(_, a) => a
                .borrow()
                .get(i)
                .map_or(Value::Undefined, |&v| Value::Number(v)),
            Value::String(s) => s.encode_utf16().nth(i).map_or(Value::Undefined, |u| {
                Value::String(String::from_utf16_lossy(&[u]).into())
            }),
            _ => Value::Undefined,
        }
    }
    /// `v[ i ]` with a numeric index ( a non-integer or negative index reads
    /// a missing property ).
    pub(crate) fn at_number(&self, i: f64) -> Value {
        if i >= 0.0 && i.fract() == 0.0 && i < 4294967295.0 {
            self.at(i as usize)
        } else {
            Value::Undefined
        }
    }
    /// `v[ key ]` for objects.
    pub(crate) fn get(&self, key: &str) -> Value {
        match self {
            Value::Object(o) => o.get(key),
            Value::Array(_) | Value::TypedArray(..) | Value::String(_) => {
                if key == "length" {
                    return self
                        .length()
                        .map_or(Value::Undefined, |l| Value::Number(l as f64));
                }
                match key.parse::<usize>() {
                    Ok(i) if is_index(key) => self.at(i),
                    _ => Value::Undefined,
                }
            }
            _ => Value::Undefined,
        }
    }
    /// The elements of an array or typed array.
    pub(crate) fn items(&self) -> Vec<Value> {
        match self {
            Value::Array(a) => a.borrow().clone(),
            Value::TypedArray(_, a) => a.borrow().iter().map(|&v| Value::Number(v)).collect(),
            _ => vec![],
        }
    }
    /// `Number( v )`.
    pub(crate) fn to_number(&self) -> f64 {
        match self {
            Value::Undefined => f64::NAN,
            Value::Null => 0.0,
            Value::Bool(b) => f64::from(u8::from(*b)),
            Value::Number(n) => *n,
            Value::String(s) => string_to_number(s),
            Value::Array(_) | Value::TypedArray(..) => {
                let items = self.items();
                match items.len() {
                    0 => 0.0,
                    1 => match &items[0] {
                        Value::Undefined | Value::Null => 0.0,
                        v => string_to_number(&v.to_js_string()),
                    },
                    _ => f64::NAN,
                }
            }
            Value::Object(_) => f64::NAN,
        }
    }
    /// `String( v )`.
    pub(crate) fn to_js_string(&self) -> String {
        match self {
            Value::Undefined => "undefined".into(),
            Value::Null => "null".into(),
            Value::Bool(b) => b.to_string(),
            Value::Number(n) => js_number_string(*n),
            Value::String(s) => s.to_string(),
            Value::Array(_) | Value::TypedArray(..) => self.join(","),
            Value::Object(_) => "[object Object]".into(),
        }
    }
    /// `a.join( sep )`: undefined and null elements are empty.
    pub(crate) fn join(&self, sep: &str) -> String {
        self.items()
            .iter()
            .map(|v| match v {
                Value::Undefined | Value::Null => String::new(),
                v => v.to_js_string(),
            })
            .collect::<Vec<_>>()
            .join(sep)
    }
    /// The property key a value converts to ( `obj[ v ]` ).
    pub(crate) fn to_key(&self) -> String {
        match self {
            Value::Number(n) => number_key(*n),
            v => v.to_js_string(),
        }
    }
    /// `a === b`.
    pub(crate) fn strict_eq(&self, other: &Value) -> bool {
        match (self, other) {
            (Value::Undefined, Value::Undefined) | (Value::Null, Value::Null) => true,
            (Value::Bool(a), Value::Bool(b)) => a == b,
            (Value::Number(a), Value::Number(b)) => a == b,
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Array(a), Value::Array(b)) => Rc::ptr_eq(a, b),
            (Value::TypedArray(_, a), Value::TypedArray(_, b)) => Rc::ptr_eq(a, b),
            (Value::Object(a), Value::Object(b)) => Rc::ptr_eq(&a.0, &b.0),
            _ => false,
        }
    }
    /// SameValueZero ( Map keys, `includes`, `indexOf` uses strict equality ).
    pub(crate) fn same_value_zero(&self, other: &Value) -> bool {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => a == b || (a.is_nan() && b.is_nan()),
            _ => self.strict_eq(other),
        }
    }
    /// `typeof v === 'number'`.
    pub(crate) fn is_number(&self) -> bool {
        matches!(self, Value::Number(_))
    }
    /// `a < b` ( the abstract relational comparison ).
    pub(crate) fn lt(&self, other: &Value) -> bool {
        if let (Value::String(a), Value::String(b)) = (self, other) {
            return a.encode_utf16().lt(b.encode_utf16());
        }
        self.to_primitive_number() < other.to_primitive_number()
    }
    fn to_primitive_number(&self) -> f64 {
        self.to_number()
    }
    /// `a > b`.
    pub(crate) fn gt(&self, other: &Value) -> bool {
        other.lt(self)
    }
}

/// `String.prototype.trim()`.
pub(crate) fn trim(s: &str) -> &str {
    s.trim_matches(is_js_space)
}
