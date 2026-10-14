//! A JavaScript object model: the values the ported classes compute with.
//!
//! `Val` is a JavaScript value. Objects carry a prototype, own properties in
//! JavaScript's order ( integer keys ascending, then insertion order ) with
//! data or accessor slots, and an optional internal kind ( array, function,
//! Map, Set, WeakMap ). Classes are constructor functions with a `prototype`
//! object, as in JavaScript, so `instanceof`, `super` calls, methods added to
//! a prototype at run time and `this.constructor.type` behave as written.
//! Exceptions are `Exc` values carried through `R`.
use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

mod builtins;
mod globals;
pub mod num;
pub mod ops;
#[cfg(target_arch = "wasm32")]
pub mod interop;

pub use builtins::*;
pub use globals::*;

/// A thrown value.
#[derive(Clone)]
pub struct Exc(pub Val);
pub type R<T> = Result<T, Exc>;

impl fmt::Debug for Exc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", error_message(&self.0))
    }
}

/// A JavaScript value.
#[derive(Clone, Default)]
pub enum Val {
    #[default]
    Undef,
    Null,
    Bool(bool),
    Num(f64),
    Str(Rc<str>),
    Obj(Obj),
    /// A value owned by the embedding JavaScript ( user objects and functions ).
    #[cfg(target_arch = "wasm32")]
    Js(wasm_bindgen::JsValue),
}

impl fmt::Debug for Val {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Val::Undef => write!(f, "undefined"),
            Val::Null => write!(f, "null"),
            Val::Bool(b) => write!(f, "{b}"),
            Val::Num(n) => write!(f, "{}", num::to_string(*n)),
            Val::Str(s) => write!(f, "{s:?}"),
            Val::Obj(o) => write!(f, "[object {}]", o.class_name()),
            #[cfg(target_arch = "wasm32")]
            Val::Js(_) => write!(f, "[js]"),
        }
    }
}

impl From<f64> for Val {
    fn from(n: f64) -> Self {
        Val::Num(n)
    }
}
impl From<i32> for Val {
    fn from(n: i32) -> Self {
        Val::Num(n as f64)
    }
}
impl From<usize> for Val {
    fn from(n: usize) -> Self {
        Val::Num(n as f64)
    }
}
impl From<u32> for Val {
    fn from(n: u32) -> Self {
        Val::Num(n as f64)
    }
}
impl From<bool> for Val {
    fn from(b: bool) -> Self {
        Val::Bool(b)
    }
}
impl From<&str> for Val {
    fn from(s: &str) -> Self {
        Val::Str(Rc::from(s))
    }
}
impl From<String> for Val {
    fn from(s: String) -> Self {
        Val::Str(Rc::from(s.as_str()))
    }
}
impl From<Rc<str>> for Val {
    fn from(s: Rc<str>) -> Self {
        Val::Str(s)
    }
}
impl From<&Val> for Val {
    fn from(v: &Val) -> Self {
        v.clone()
    }
}
impl From<Obj> for Val {
    fn from(o: Obj) -> Self {
        Val::Obj(o)
    }
}
impl<T: Into<Val>> From<Option<T>> for Val {
    fn from(o: Option<T>) -> Self {
        o.map_or(Val::Null, Into::into)
    }
}

/// A native function body: `this` and the arguments.
pub type NativeFn = Rc<dyn Fn(&Val, &[Val]) -> R<Val>>;

/// An object.
#[derive(Clone)]
pub struct Obj(pub Rc<ObjData>);

pub struct ObjData {
    pub proto: RefCell<Option<Obj>>,
    pub props: RefCell<PropMap>,
    pub kind: Kind,
}

/// An object's internal slots.
pub enum Kind {
    Plain,
    Array(RefCell<Vec<Val>>),
    Func(Func),
    Map(RefCell<OrderedMap>),
    Set(RefCell<OrderedMap>),
    WeakMap(RefCell<HashMap<usize, (Obj, Val)>>),
    Error,
    /// A typed array: its element type and its view of an ArrayBuffer's bytes.
    Typed(TypedKind, View),
    /// An ArrayBuffer's bytes, shared with its views.
    ArrayBuffer(Rc<RefCell<Vec<u8>>>),
    /// `new Proxy( target, handler )`.
    Proxy(Val, Val),
    /// A regular expression: source, flags, the compiled pattern and lastIndex.
    RegExp(Rc<str>, Rc<str>, regex::Regex, Cell<usize>),
    /// A promise: settled ( fulfilled or rejected, with its value ) or pending
    /// with its reactions.
    Promise(RefCell<PromiseState>),
}

pub enum PromiseState {
    Pending(Vec<(Val, Val, Val)>),
    Settled(bool, Val),
}

/// The typed array element types.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TypedKind {
    Int8,
    Uint8,
    Uint8Clamped,
    Int16,
    Uint16,
    Int32,
    Uint32,
    Float16,
    Float32,
    Float64,
}

impl TypedKind {
    pub const ALL: [TypedKind; 10] = [
        TypedKind::Int8,
        TypedKind::Uint8,
        TypedKind::Uint8Clamped,
        TypedKind::Int16,
        TypedKind::Uint16,
        TypedKind::Int32,
        TypedKind::Uint32,
        TypedKind::Float16,
        TypedKind::Float32,
        TypedKind::Float64,
    ];
    pub fn name(self) -> &'static str {
        match self {
            TypedKind::Int8 => "Int8Array",
            TypedKind::Uint8 => "Uint8Array",
            TypedKind::Uint8Clamped => "Uint8ClampedArray",
            TypedKind::Int16 => "Int16Array",
            TypedKind::Uint16 => "Uint16Array",
            TypedKind::Int32 => "Int32Array",
            TypedKind::Uint32 => "Uint32Array",
            TypedKind::Float16 => "Float16Array",
            TypedKind::Float32 => "Float32Array",
            TypedKind::Float64 => "Float64Array",
        }
    }
    pub fn bytes(self) -> usize {
        match self {
            TypedKind::Int8 | TypedKind::Uint8 | TypedKind::Uint8Clamped => 1,
            TypedKind::Int16 | TypedKind::Uint16 | TypedKind::Float16 => 2,
            TypedKind::Int32 | TypedKind::Uint32 | TypedKind::Float32 => 4,
            TypedKind::Float64 => 8,
        }
    }
    /// The stored value of `v` ( ToInt8 … ToFloat32 ).
    pub fn coerce(self, v: f64) -> f64 {
        let wrap = |v: f64, bits: u32, signed: bool| -> f64 {
            if !v.is_finite() {
                return 0.0;
            }
            let m = 2f64.powi(bits as i32);
            let mut x = v.trunc() % m;
            if x < 0.0 {
                x += m;
            }
            if signed && x >= m / 2.0 {
                x -= m;
            }
            x
        };
        match self {
            TypedKind::Int8 => wrap(v, 8, true),
            TypedKind::Uint8 => wrap(v, 8, false),
            TypedKind::Uint8Clamped => {
                if v.is_nan() {
                    0.0
                } else {
                    let c = v.clamp(0.0, 255.0);
                    // Round half to even.
                    let f = c.floor();
                    let d = c - f;
                    if d > 0.5 || (d == 0.5 && f % 2.0 != 0.0) { f + 1.0 } else { f }
                }
            }
            TypedKind::Int16 => wrap(v, 16, true),
            TypedKind::Uint16 => wrap(v, 16, false),
            TypedKind::Int32 => wrap(v, 32, true),
            TypedKind::Uint32 => wrap(v, 32, false),
            TypedKind::Float16 => f16_round(v),
            TypedKind::Float32 => v as f32 as f64,
            TypedKind::Float64 => v,
        }
    }
}

/// A typed array's window on a buffer ( little-endian, as on every platform the port runs on ).
pub struct View {
    pub bytes: Rc<RefCell<Vec<u8>>>,
    pub buffer: Obj,
    pub offset: usize,
    pub len: usize,
}

/// binary16 bits of a value ( round to nearest even ).
pub fn f16_bits(v: f64) -> u16 {
    let r = f16_round(v);
    if r.is_nan() {
        return 0x7e00;
    }
    let sign = if r.is_sign_negative() { 0x8000u16 } else { 0 };
    let a = r.abs();
    if a == 0.0 {
        return sign;
    }
    if a.is_infinite() {
        return sign | 0x7c00;
    }
    let e = a.log2().floor() as i32;
    if e < -14 {
        // Subnormal: a = m × 2^-24.
        return sign | (a / 2f64.powi(-24)).round() as u16;
    }
    let m = (a / 2f64.powi(e) - 1.0) * 1024.0;
    sign | (((e + 15) as u16) << 10) | (m.round() as u16)
}

pub fn f16_value(bits: u16) -> f64 {
    let sign = if bits & 0x8000 != 0 { -1.0 } else { 1.0 };
    let e = ((bits >> 10) & 0x1f) as i32;
    let m = (bits & 0x3ff) as f64;
    sign * match e {
        0 => m * 2f64.powi(-24),
        31 => {
            if m == 0.0 {
                f64::INFINITY
            } else {
                f64::NAN
            }
        }
        _ => (1.0 + m / 1024.0) * 2f64.powi(e - 15),
    }
}

impl View {
    pub fn get(&self, kind: TypedKind, i: usize) -> f64 {
        let n = kind.bytes();
        let b = self.bytes.borrow();
        let at = self.offset + i * n;
        let s = &b[at..at + n];
        match kind {
            TypedKind::Int8 => s[0] as i8 as f64,
            TypedKind::Uint8 | TypedKind::Uint8Clamped => s[0] as f64,
            TypedKind::Int16 => i16::from_le_bytes([s[0], s[1]]) as f64,
            TypedKind::Uint16 => u16::from_le_bytes([s[0], s[1]]) as f64,
            TypedKind::Float16 => f16_value(u16::from_le_bytes([s[0], s[1]])),
            TypedKind::Int32 => i32::from_le_bytes([s[0], s[1], s[2], s[3]]) as f64,
            TypedKind::Uint32 => u32::from_le_bytes([s[0], s[1], s[2], s[3]]) as f64,
            TypedKind::Float32 => f32::from_le_bytes([s[0], s[1], s[2], s[3]]) as f64,
            TypedKind::Float64 => f64::from_le_bytes(s.try_into().unwrap()),
        }
    }
    pub fn set(&self, kind: TypedKind, i: usize, v: f64) {
        let n = kind.bytes();
        let v = kind.coerce(v);
        let bytes: Vec<u8> = match kind {
            TypedKind::Int8 => vec![v as i8 as u8],
            TypedKind::Uint8 | TypedKind::Uint8Clamped => vec![v as u8],
            TypedKind::Int16 => (v as i16).to_le_bytes().to_vec(),
            TypedKind::Uint16 => (v as u16).to_le_bytes().to_vec(),
            TypedKind::Float16 => f16_bits(v).to_le_bytes().to_vec(),
            TypedKind::Int32 => (v as i32).to_le_bytes().to_vec(),
            TypedKind::Uint32 => (v as u32).to_le_bytes().to_vec(),
            TypedKind::Float32 => (v as f32).to_le_bytes().to_vec(),
            TypedKind::Float64 => v.to_le_bytes().to_vec(),
        };
        let at = self.offset + i * n;
        self.bytes.borrow_mut()[at..at + n].copy_from_slice(&bytes);
    }
    pub fn values(&self, kind: TypedKind) -> Vec<f64> {
        (0..self.len).map(|i| self.get(kind, i)).collect()
    }
}

/// `new ArrayBuffer( n )`.
pub fn array_buffer(n: usize) -> (Obj, Rc<RefCell<Vec<u8>>>) {
    let bytes = Rc::new(RefCell::new(vec![0u8; n]));
    (Obj::with(Some(proto("ArrayBuffer")), Kind::ArrayBuffer(bytes.clone())), bytes)
}

/// Rounds to the nearest binary16 value ( Math.f16round ).
pub fn f16_round(v: f64) -> f64 {
    if !v.is_finite() || v == 0.0 {
        return v;
    }
    let a = v.abs();
    if a >= 65520.0 {
        return f64::INFINITY.copysign(v);
    }
    let e = a.log2().floor().max(-14.0);
    let ulp = 2f64.powf(e - 10.0);
    let q = a / ulp;
    let f = q.floor();
    let d = q - f;
    let r = if d > 0.5 || (d == 0.5 && f % 2.0 != 0.0) { f + 1.0 } else { f };
    (r * ulp).copysign(v)
}

pub struct Func {
    pub name: RefCell<Rc<str>>,
    pub length: Cell<u32>,
    pub call: Option<NativeFn>,
    /// A class constructor: the instance initializer `new` and `super()` run.
    pub init: Option<NativeFn>,
    /// A constructor that returns its own object ( a function used with `new` ).
    pub construct: Option<NativeFn>,
    /// Property reads and writes forwarded to another object ( TSL's Fn proxy ).
    pub forward: RefCell<Option<Obj>>,
}

thread_local! {
    static CONSTRUCTIBLE: RefCell<std::collections::HashSet<usize>> = RefCell::new(std::collections::HashSet::new());
    static SOURCES: RefCell<HashMap<usize, &'static str>> = RefCell::new(HashMap::new());
}

/// Attach a function's source text ( Function.prototype.toString ).
pub fn with_source(f: Val, source: &'static str) -> Val {
    if let Val::Obj(o) = &f {
        SOURCES.with(|s| s.borrow_mut().insert(o.ptr(), source));
    }
    f
}

/// A function's source text, if it was transpiled from JavaScript.
pub fn source_of(o: &Obj) -> Option<&'static str> {
    SOURCES.with(|s| s.borrow().get(&o.ptr()).copied())
}

/// A property slot.
#[derive(Clone)]
pub enum Slot {
    Data(Val),
    Accessor(Option<Val>, Option<Val>),
}

#[derive(Clone)]
pub struct Prop {
    pub slot: Slot,
    pub enumerable: bool,
}

/// Own properties in JavaScript's enumeration order.
#[derive(Default, Clone)]
pub struct PropMap {
    keys: Vec<Rc<str>>,
    map: HashMap<Rc<str>, Prop>,
}

/// An array index key ( a canonical integer below 2^32 - 1 ).
pub fn array_index(key: &str) -> Option<u32> {
    if key.is_empty() || key.len() > 10 || (key.len() > 1 && key.starts_with('0')) {
        return None;
    }
    if !key.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    let n: u64 = key.parse().ok()?;
    (n < u32::MAX as u64).then_some(n as u32)
}

impl PropMap {
    pub fn get(&self, key: &str) -> Option<&Prop> {
        self.map.get(key)
    }
    pub fn get_mut(&mut self, key: &str) -> Option<&mut Prop> {
        self.map.get_mut(key)
    }
    pub fn contains(&self, key: &str) -> bool {
        self.map.contains_key(key)
    }
    pub fn insert(&mut self, key: &str, prop: Prop) {
        if let Some(p) = self.map.get_mut(key) {
            *p = prop;
            return;
        }
        let key: Rc<str> = Rc::from(key);
        self.keys.push(key.clone());
        self.map.insert(key, prop);
    }
    /// Symbol keys, in insertion order ( Object.getOwnPropertySymbols ).
    pub fn symbol_keys(&self) -> Vec<Rc<str>> {
        self.keys.iter().filter(|k| k.starts_with("@@")).cloned().collect()
    }
    pub fn remove(&mut self, key: &str) -> bool {
        if self.map.remove(key).is_some() {
            self.keys.retain(|k| &**k != key);
            true
        } else {
            false
        }
    }
    /// Keys in enumeration order: array indices ascending, then the rest as
    /// inserted. Symbol keys ( `@@…` ) are not string keys and are left out.
    pub fn keys(&self) -> Vec<Rc<str>> {
        let mut index: Vec<(u32, Rc<str>)> = vec![];
        let mut rest = vec![];
        for k in &self.keys {
            if k.starts_with("@@") {
                continue;
            }
            match array_index(k) {
                Some(i) => index.push((i, k.clone())),
                None => rest.push(k.clone()),
            }
        }
        if index.is_empty() {
            return rest;
        }
        index.sort_by_key(|(i, _)| *i);
        index.into_iter().map(|(_, k)| k).chain(rest).collect()
    }
}

/// A Map's or Set's entries in insertion order, keyed by SameValueZero.
#[derive(Default)]
pub struct OrderedMap {
    pub entries: Vec<Option<(Val, Val)>>,
    index: HashMap<MapKey, usize>,
    live: usize,
}

#[derive(Hash, PartialEq, Eq, Clone)]
enum MapKey {
    Undef,
    Null,
    Bool(bool),
    Num(u64),
    Str(Rc<str>),
    Obj(usize),
    #[cfg(target_arch = "wasm32")]
    Js(u64),
}

fn map_key(v: &Val) -> MapKey {
    match v {
        Val::Undef => MapKey::Undef,
        Val::Null => MapKey::Null,
        Val::Bool(b) => MapKey::Bool(*b),
        Val::Num(n) => {
            let n = if *n == 0.0 { 0.0 } else { *n };
            MapKey::Num(if n.is_nan() { f64::NAN.to_bits() } else { n.to_bits() })
        }
        Val::Str(s) => MapKey::Str(s.clone()),
        Val::Obj(o) => MapKey::Obj(o.ptr()),
        #[cfg(target_arch = "wasm32")]
        Val::Js(j) => MapKey::Js(interop::identity(j)),
    }
}

impl OrderedMap {
    pub fn get(&self, k: &Val) -> Option<&Val> {
        self.index
            .get(&map_key(k))
            .and_then(|&i| self.entries[i].as_ref().map(|(_, v)| v))
    }
    pub fn has(&self, k: &Val) -> bool {
        self.index.contains_key(&map_key(k))
    }
    pub fn set(&mut self, k: Val, v: Val) {
        let key = map_key(&k);
        if let Some(&i) = self.index.get(&key) {
            self.entries[i] = Some((k, v));
        } else {
            self.index.insert(key, self.entries.len());
            self.entries.push(Some((k, v)));
            self.live += 1;
        }
    }
    pub fn delete(&mut self, k: &Val) -> bool {
        if let Some(i) = self.index.remove(&map_key(k)) {
            self.entries[i] = None;
            self.live -= 1;
            true
        } else {
            false
        }
    }
    pub fn clear(&mut self) {
        self.entries.clear();
        self.index.clear();
        self.live = 0;
    }
    pub fn len(&self) -> usize {
        self.live
    }
    pub fn is_empty(&self) -> bool {
        self.live == 0
    }
    pub fn pairs(&self) -> Vec<(Val, Val)> {
        self.entries.iter().flatten().cloned().collect()
    }
}

impl Obj {
    pub fn with(proto: Option<Obj>, kind: Kind) -> Obj {
        Obj(Rc::new(ObjData {
            proto: RefCell::new(proto),
            props: RefCell::new(PropMap::default()),
            kind,
        }))
    }
    pub fn ptr(&self) -> usize {
        Rc::as_ptr(&self.0) as *const () as usize
    }
    pub fn ptr_eq(&self, other: &Obj) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
    pub fn proto(&self) -> Option<Obj> {
        self.0.proto.borrow().clone()
    }
    pub fn set_proto(&self, proto: Option<Obj>) {
        *self.0.proto.borrow_mut() = proto;
    }
    pub fn is_array(&self) -> bool {
        matches!(self.0.kind, Kind::Array(_))
    }
    pub fn is_func(&self) -> bool {
        matches!(self.0.kind, Kind::Func(_))
    }
    pub fn func(&self) -> Option<&Func> {
        match &self.0.kind {
            Kind::Func(f) => Some(f),
            _ => None,
        }
    }
    pub fn array(&self) -> Option<&RefCell<Vec<Val>>> {
        match &self.0.kind {
            Kind::Array(a) => Some(a),
            _ => None,
        }
    }
    /// The array's elements ( a copy ).
    pub fn items(&self) -> Vec<Val> {
        self.array().map(|a| a.borrow().clone()).unwrap_or_default()
    }
    /// The nearest constructor's name, for messages.
    pub fn class_name(&self) -> String {
        match &self.0.kind {
            Kind::Array(_) => return "Array".into(),
            Kind::Func(_) => return "Function".into(),
            _ => {}
        }
        let mut p = self.proto();
        while let Some(o) = p {
            if let Some(Prop {
                slot: Slot::Data(Val::Obj(c)),
                ..
            }) = o.0.props.borrow().get("constructor")
                && let Some(f) = c.func()
            {
                return f.name.borrow().to_string();
            }
            p = o.proto();
        }
        "Object".into()
    }
    /// An own data property's value.
    pub fn own(&self, key: &str) -> Option<Val> {
        if let Some(a) = self.array() {
            if key == "length" {
                return Some(Val::Num(a.borrow().len() as f64));
            }
            if let Some(i) = array_index(key) {
                return a.borrow().get(i as usize).cloned();
            }
        }
        match self.0.props.borrow().get(key) {
            Some(Prop {
                slot: Slot::Data(v),
                ..
            }) => Some(v.clone()),
            _ => None,
        }
    }
    /// Define an own data property ( enumerable unless stated ).
    pub fn define(&self, key: &str, v: impl Into<Val>, enumerable: bool) {
        self.0.props.borrow_mut().insert(
            key,
            Prop {
                slot: Slot::Data(v.into()),
                enumerable,
            },
        );
    }
    pub fn define_accessor(&self, key: &str, get: Option<Val>, set: Option<Val>, enumerable: bool) {
        self.0.props.borrow_mut().insert(
            key,
            Prop {
                slot: Slot::Accessor(get, set),
                enumerable,
            },
        );
    }
    /// Own property keys ( Object.getOwnPropertyNames ).
    pub fn own_keys(&self) -> Vec<Rc<str>> {
        let mut keys = vec![];
        if let Kind::Typed(_, view) = &self.0.kind {
            keys.extend((0..view.len).map(|i| Rc::from(i.to_string())));
        }
        if let Some(a) = self.array() {
            keys.extend((0..a.borrow().len()).map(|i| Rc::from(i.to_string())));
        }
        keys.extend(self.0.props.borrow().keys());
        if self.array().is_some() {
            keys.push(Rc::from("length"));
        }
        keys
    }
    /// Own enumerable keys ( Object.keys ).
    pub fn keys(&self) -> Vec<Rc<str>> {
        let mut keys = vec![];
        if let Kind::Typed(_, view) = &self.0.kind {
            keys.extend((0..view.len).map(|i| Rc::from(i.to_string())));
        }
        if let Some(a) = self.array() {
            keys.extend(
                a.borrow()
                    .iter()
                    .enumerate()
                    .map(|(i, _)| Rc::from(i.to_string())),
            );
        }
        let props = self.0.props.borrow();
        keys.extend(
            props
                .keys()
                .into_iter()
                .filter(|k| props.get(k).is_some_and(|p| p.enumerable)),
        );
        keys
    }
    pub fn has_own(&self, key: &str) -> bool {
        if let Some(a) = self.array() {
            if key == "length" {
                return true;
            }
            if let Some(i) = array_index(key) {
                return (i as usize) < a.borrow().len();
            }
        }
        self.0.props.borrow().contains(key)
    }
    pub fn delete(&self, key: &str) -> bool {
        self.0.props.borrow_mut().remove(key);
        true
    }
}

// ---------------------------------------------------------------- values

pub fn undef() -> Val {
    Val::Undef
}
pub fn s(text: &str) -> Val {
    Val::Str(Rc::from(text))
}

impl Val {
    pub fn is_undef(&self) -> bool {
        matches!(self, Val::Undef)
    }
    pub fn is_null(&self) -> bool {
        matches!(self, Val::Null)
    }
    /// `== null`.
    pub fn is_nullish(&self) -> bool {
        matches!(self, Val::Undef | Val::Null)
    }
    pub fn obj(&self) -> Option<&Obj> {
        match self {
            Val::Obj(o) => Some(o),
            _ => None,
        }
    }
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Val::Str(s) => Some(s),
            _ => None,
        }
    }
    pub fn as_num(&self) -> Option<f64> {
        match self {
            Val::Num(n) => Some(*n),
            _ => None,
        }
    }
    /// ToBoolean.
    pub fn truthy(&self) -> bool {
        match self {
            Val::Undef | Val::Null => false,
            Val::Bool(b) => *b,
            Val::Num(n) => *n != 0.0 && !n.is_nan(),
            Val::Str(s) => !s.is_empty(),
            Val::Obj(_) => true,
            #[cfg(target_arch = "wasm32")]
            Val::Js(j) => j.is_truthy(),
        }
    }
    /// `typeof`.
    pub fn type_of(&self) -> &'static str {
        match self {
            Val::Undef => "undefined",
            Val::Null => "object",
            Val::Bool(_) => "boolean",
            Val::Num(_) => "number",
            Val::Str(_) => "string",
            Val::Obj(o) => {
                if let Kind::Proxy(target, _) = &o.0.kind {
                    return target.type_of();
                }
                if o.is_func() {
                    "function"
                } else {
                    "object"
                }
            }
            #[cfg(target_arch = "wasm32")]
            Val::Js(j) => interop::type_of(j),
        }
    }
    /// `===`.
    pub fn strict_eq(&self, other: &Val) -> bool {
        match (self, other) {
            (Val::Undef, Val::Undef) | (Val::Null, Val::Null) => true,
            (Val::Bool(a), Val::Bool(b)) => a == b,
            (Val::Num(a), Val::Num(b)) => a == b,
            (Val::Str(a), Val::Str(b)) => a == b,
            (Val::Obj(a), Val::Obj(b)) => a.ptr_eq(b),
            #[cfg(target_arch = "wasm32")]
            (Val::Js(a), Val::Js(b)) => a == b,
            _ => false,
        }
    }
    /// `==` ( only the cases the sources rely on: nullish, and number/string ).
    pub fn loose_eq(&self, other: &Val) -> bool {
        match (self, other) {
            (a, b) if a.is_nullish() && b.is_nullish() => true,
            (a, b) if a.is_nullish() || b.is_nullish() => false,
            (Val::Num(a), Val::Str(b)) | (Val::Str(b), Val::Num(a)) => to_number_str(b) == *a,
            (Val::Bool(a), b) => Val::Num(*a as u8 as f64).loose_eq(b),
            (a, Val::Bool(b)) => a.loose_eq(&Val::Num(*b as u8 as f64)),
            _ => self.strict_eq(other),
        }
    }
    pub fn num(&self) -> f64 {
        to_number(self)
    }
}

/// ToNumber.
pub fn to_number(v: &Val) -> f64 {
    match v {
        Val::Undef => f64::NAN,
        Val::Null => 0.0,
        Val::Bool(b) => *b as u8 as f64,
        Val::Num(n) => *n,
        Val::Str(s) => to_number_str(s),
        Val::Obj(o) => match o.array() {
            Some(a) => {
                let a = a.borrow();
                match a.len() {
                    0 => 0.0,
                    1 => to_number(&a[0]),
                    _ => f64::NAN,
                }
            }
            None => f64::NAN,
        },
        #[cfg(target_arch = "wasm32")]
        Val::Js(j) => interop::to_number(j),
    }
}

fn to_number_str(s: &str) -> f64 {
    let t = s.trim();
    if t.is_empty() {
        return 0.0;
    }
    if let Some(h) = t.strip_prefix("0x").or_else(|| t.strip_prefix("0X")) {
        return u64::from_str_radix(h, 16).map_or(f64::NAN, |v| v as f64);
    }
    match t {
        "Infinity" | "+Infinity" => f64::INFINITY,
        "-Infinity" => f64::NEG_INFINITY,
        _ => t.parse::<f64>().unwrap_or(f64::NAN),
    }
}

/// ToString ( String( v ) ).
pub fn to_string(v: &Val) -> R<Rc<str>> {
    Ok(match v {
        Val::Undef => Rc::from("undefined"),
        Val::Null => Rc::from("null"),
        Val::Bool(b) => Rc::from(if *b { "true" } else { "false" }),
        Val::Num(n) => Rc::from(num::to_string(*n)),
        Val::Str(s) => s.clone(),
        Val::Obj(_) => {
            let r = call_method(v, "toString", &[], "toString")?;
            match r {
                Val::Obj(_) => return Err(type_error("Cannot convert object to primitive value")),
                r => to_string(&r)?,
            }
        }
        #[cfg(target_arch = "wasm32")]
        Val::Js(j) => Rc::from(interop::to_string(j)),
    })
}

/// `String( v )` for display, never throwing.
pub fn display(v: &Val) -> String {
    to_string(v).map(|s| s.to_string()).unwrap_or_default()
}

// ---------------------------------------------------------------- errors

thread_local! {
    static PROTOS: RefCell<HashMap<&'static str, Obj>> = RefCell::new(HashMap::new());
}

/// A built-in prototype ( Object, Function, Array, Error, TypeError, Map, Set, WeakMap, String, Number, Boolean ).
pub fn proto(name: &'static str) -> Obj {
    if let Some(o) = PROTOS.with(|p| p.borrow().get(name).cloned()) {
        return o;
    }
    builtins::install();
    PROTOS.with(|p| p.borrow().get(name).cloned()).expect("built-in prototype")
}

pub(crate) fn register_proto(name: &'static str, o: Obj) {
    PROTOS.with(|p| p.borrow_mut().insert(name, o));
}

/// `new Error( message )` of the given kind ( "Error", "TypeError", "RangeError" ).
pub fn make_error(kind: &'static str, message: &str) -> Val {
    let o = Obj::with(Some(proto(kind)), Kind::Error);
    o.define("message", message, false);
    let mut stack = format!("{kind}: {message}");
    for name in call_stack().iter().rev() {
        stack.push_str("\n    at ");
        stack.push_str(if name.is_empty() { "<anonymous>" } else { name });
    }
    #[cfg(not(target_arch = "wasm32"))]
    if std::env::var("TSL_DEBUG_ERRORS").is_ok() {
        eprintln!("[error created] {stack}");
    }
    o.define("stack", stack, false);
    Val::Obj(o)
}
pub fn type_error(message: &str) -> Exc {
    Exc(make_error("TypeError", message))
}
pub fn error(message: &str) -> Exc {
    Exc(make_error("Error", message))
}
/// The message of a thrown value ( `e.message` ).
pub fn error_message(v: &Val) -> String {
    match get(v, "message") {
        Ok(Val::Undef) | Err(_) => display(v),
        Ok(m) => display(&m),
    }
}

// ---------------------------------------------------------------- properties

/// `v[ key ]` with `receiver` as the getters' `this`.
fn get_from(o: &Obj, key: &str, receiver: &Val) -> R<Val> {
    if let Kind::Proxy(target, handler) = &o.0.kind {
        let trap = get(handler, "get")?;
        if trap.type_of() == "function" {
            return call(&trap, handler, &[target.clone(), Val::from(key), receiver.clone()]);
        }
        return get(target, key);
    }
    if let Some(f) = o.func() {
        let target = f.forward.borrow().clone();
        if let Some(t) = target
            && key != "prototype"
        {
            return get_from(&t, key, &Val::Obj(t.clone()));
        }
        match key {
            "length" if !o.0.props.borrow().contains("length") => {
                return Ok(Val::Num(f.length.get() as f64));
            }
            "name" if !o.0.props.borrow().contains("name") => {
                return Ok(Val::Str(f.name.borrow().clone()));
            }
            _ => {}
        }
    }
    if let Kind::Typed(kind, view) = &o.0.kind {
        match key {
            "length" => return Ok(Val::Num(view.len as f64)),
            "byteLength" => return Ok(Val::Num((view.len * kind.bytes()) as f64)),
            "byteOffset" => return Ok(Val::Num(view.offset as f64)),
            "BYTES_PER_ELEMENT" => return Ok(Val::Num(kind.bytes() as f64)),
            "buffer" => return Ok(Val::Obj(view.buffer.clone())),
            _ => {}
        }
        if let Some(i) = array_index(key) {
            let i = i as usize;
            return Ok(if i < view.len { Val::Num(view.get(*kind, i)) } else { Val::Undef });
        }
    }
    if let Kind::ArrayBuffer(bytes) = &o.0.kind
        && key == "byteLength"
    {
        return Ok(Val::Num(bytes.borrow().len() as f64));
    }
    let mut cur = Some(o.clone());
    while let Some(c) = cur {
        if let Some(a) = c.array() {
            if key == "length" {
                return Ok(Val::Num(a.borrow().len() as f64));
            }
            if let Some(i) = array_index(key) {
                if let Some(v) = a.borrow().get(i as usize) {
                    return Ok(v.clone());
                }
                cur = c.proto();
                continue;
            }
        }
        let prop = c.0.props.borrow().get(key).cloned();
        if let Some(p) = prop {
            return match p.slot {
                Slot::Data(v) => Ok(v),
                Slot::Accessor(Some(g), _) => call(&g, receiver, &[]),
                Slot::Accessor(None, _) => Ok(Val::Undef),
            };
        }
        cur = c.proto();
    }
    Ok(Val::Undef)
}

/// Reflect.get( target, key, receiver ): getters see `receiver`.
pub fn get_with_receiver(target: &Val, key: &str, receiver: &Val) -> R<Val> {
    match target {
        Val::Obj(o) => get_from(o, key, receiver),
        v => get(v, key),
    }
}

/// OrdinarySet ( Reflect.set( target, key, value, receiver ) ): a setter up
/// the chain runs on `receiver`; otherwise the value is defined on
/// `receiver` itself ( a Proxy receiver without traps defines on its target ).
pub fn set_with_receiver(target: &Val, key: &str, value: Val, receiver: &Val) -> R<bool> {
    let Val::Obj(t) = target else {
        return Ok(false);
    };
    if receiver.strict_eq(target) {
        set_on(t, key, value, receiver)?;
        return Ok(true);
    }
    // A setter on the target's chain runs with the receiver.
    let mut cur = Some(t.clone());
    while let Some(c) = cur {
        let prop = c.0.props.borrow().get(key).cloned();
        if let Some(p) = prop {
            match p.slot {
                Slot::Accessor(_, Some(setter)) => {
                    call(&setter, receiver, &[value])?;
                    return Ok(true);
                }
                Slot::Accessor(_, None) => return Ok(false),
                Slot::Data(_) => break,
            }
        }
        cur = c.proto();
    }
    define_on_receiver(receiver, key, value)
}

fn define_on_receiver(receiver: &Val, key: &str, value: Val) -> R<bool> {
    match receiver {
        Val::Obj(r) => {
            if let Kind::Proxy(target, handler) = &r.0.kind {
                let trap = get(handler, "defineProperty")?;
                if trap.type_of() == "function" {
                    let d = object(&[("value", value), ("writable", Val::Bool(true)), ("enumerable", Val::Bool(true)), ("configurable", Val::Bool(true))]);
                    return Ok(call(&trap, handler, &[target.clone(), Val::from(key), d])?.truthy());
                }
                return define_on_receiver(target, key, value);
            }
            if let Some(a) = r.array()
                && let Some(i) = array_index(key)
            {
                let mut a = a.borrow_mut();
                let i = i as usize;
                if i >= a.len() {
                    a.resize(i + 1, Val::Undef);
                }
                a[i] = value;
                return Ok(true);
            }
            let existing = r.0.props.borrow().get(key).cloned();
            match existing {
                Some(Prop { slot: Slot::Accessor(..), .. }) => Ok(false),
                Some(Prop { enumerable, .. }) => {
                    r.define(key, value, enumerable);
                    Ok(true)
                }
                None => {
                    r.define(key, value, true);
                    Ok(true)
                }
            }
        }
        #[cfg(target_arch = "wasm32")]
        Val::Js(j) => {
            interop::set(j, key, &value)?;
            Ok(true)
        }
        _ => Ok(false),
    }
}

/// `v[ key ]`.
pub fn get(v: &Val, key: &str) -> R<Val> {
    match v {
        Val::Undef | Val::Null => Err(type_error(&format!(
            "Cannot read properties of {} (reading '{key}')",
            if v.is_undef() { "undefined" } else { "null" }
        ))),
        Val::Obj(o) => get_from(o, key, v),
        Val::Str(text) => {
            if key == "length" {
                return Ok(Val::Num(text.encode_utf16().count() as f64));
            }
            if let Some(i) = array_index(key) {
                let units: Vec<u16> = text.encode_utf16().collect();
                return Ok(units
                    .get(i as usize)
                    .map_or(Val::Undef, |u| Val::from(String::from_utf16_lossy(&[*u]))));
            }
            get_from(&proto("String"), key, v)
        }
        Val::Num(_) => get_from(&proto("Number"), key, v),
        Val::Bool(_) => get_from(&proto("Boolean"), key, v),
        #[cfg(target_arch = "wasm32")]
        Val::Js(j) => interop::get(j, key),
    }
}

/// `v[ index ]`.
pub fn get_index(v: &Val, i: usize) -> R<Val> {
    if let Val::Obj(o) = v
        && let Some(a) = o.array()
    {
        return Ok(a.borrow().get(i).cloned().unwrap_or(Val::Undef));
    }
    get(v, &i.to_string())
}

/// `v[ key ] = value`.
pub fn set(v: &Val, key: &str, value: impl Into<Val>) -> R<()> {
    let value = value.into();
    match v {
        Val::Undef | Val::Null => Err(type_error(&format!(
            "Cannot set properties of {} (setting '{key}')",
            if v.is_undef() { "undefined" } else { "null" }
        ))),
        Val::Obj(o) => set_on(o, key, value, v),
        #[cfg(target_arch = "wasm32")]
        Val::Js(j) => interop::set(j, key, &value),
        _ => Ok(()),
    }
}

fn set_on(o: &Obj, key: &str, value: Val, receiver: &Val) -> R<()> {
    if let Kind::Proxy(target, handler) = &o.0.kind {
        let trap = get(handler, "set")?;
        if trap.type_of() == "function" {
            call(&trap, handler, &[target.clone(), Val::from(key), value, receiver.clone()])?;
            return Ok(());
        }
        return set(target, key, value);
    }
    if let Some(f) = o.func() {
        let target = f.forward.borrow().clone();
        if let Some(t) = target {
            return set_on(&t, key, value, &Val::Obj(t.clone()));
        }
    }
    if let Kind::Typed(kind, view) = &o.0.kind
        && let Some(i) = array_index(key)
    {
        if (i as usize) < view.len {
            view.set(*kind, i as usize, to_number(&value));
        }
        return Ok(());
    }
    if let Some(a) = o.array() {
        if key == "length" {
            let n = to_number(&value) as usize;
            a.borrow_mut().resize(n, Val::Undef);
            return Ok(());
        }
        if let Some(i) = array_index(key) {
            let i = i as usize;
            let mut a = a.borrow_mut();
            if i >= a.len() {
                a.resize(i + 1, Val::Undef);
            }
            a[i] = value;
            return Ok(());
        }
    }
    {
        let mut props = o.0.props.borrow_mut();
        if let Some(p) = props.get_mut(key) {
            match &p.slot {
                Slot::Data(_) => {
                    p.slot = Slot::Data(value);
                    return Ok(());
                }
                Slot::Accessor(_, Some(setter)) => {
                    let setter = setter.clone();
                    drop(props);
                    call(&setter, receiver, &[value])?;
                    return Ok(());
                }
                Slot::Accessor(_, None) => return Ok(()),
            }
        }
    }
    // An accessor up the prototype chain.
    let mut cur = o.proto();
    while let Some(c) = cur {
        let prop = c.0.props.borrow().get(key).cloned();
        if let Some(p) = prop {
            match p.slot {
                Slot::Accessor(_, Some(setter)) => {
                    call(&setter, receiver, &[value])?;
                    return Ok(());
                }
                Slot::Accessor(_, None) => return Ok(()),
                Slot::Data(_) => break,
            }
        }
        cur = c.proto();
    }
    o.define(key, value, true);
    Ok(())
}

/// `delete v[ key ]`.
pub fn delete(v: &Val, key: &str) -> R<()> {
    match v {
        Val::Obj(o) => {
            o.delete(key);
            Ok(())
        }
        #[cfg(target_arch = "wasm32")]
        Val::Js(j) => interop::delete(j, key),
        _ => Ok(()),
    }
}

/// `key in v`.
pub fn has(v: &Val, key: &str) -> R<bool> {
    match v {
        Val::Obj(o) => {
            if let Some(f) = o.func()
                && let Some(t) = f.forward.borrow().clone()
            {
                return has(&Val::Obj(t), key);
            }
            let mut cur = Some(o.clone());
            while let Some(c) = cur {
                if c.has_own(key) {
                    return Ok(true);
                }
                cur = c.proto();
            }
            Ok(false)
        }
        #[cfg(target_arch = "wasm32")]
        Val::Js(j) => Ok(interop::has(j, key)),
        _ => Err(type_error(&format!(
            "Cannot use 'in' operator to search for '{key}' in {}",
            display(v)
        ))),
    }
}

/// `Object.keys( v )`.
pub fn keys(v: &Val) -> Vec<Rc<str>> {
    match v {
        Val::Obj(o) => o.keys(),
        #[cfg(target_arch = "wasm32")]
        Val::Js(j) => interop::keys(j),
        _ => vec![],
    }
}

/// `for ( const key in v )`: own and inherited enumerable keys.
pub fn for_in_keys(v: &Val) -> Vec<Rc<str>> {
    match v {
        Val::Obj(o) => {
            let mut out: Vec<Rc<str>> = vec![];
            let mut seen = std::collections::HashSet::new();
            let mut cur = Some(o.clone());
            while let Some(c) = cur {
                for k in c.own_keys() {
                    if seen.insert(k.clone()) {
                        let enumerable = c.array().is_some_and(|_| array_index(&k).is_some())
                            || c.0.props.borrow().get(&k).is_some_and(|p| p.enumerable);
                        if enumerable {
                            out.push(k);
                        }
                    }
                }
                cur = c.proto();
            }
            out
        }
        #[cfg(target_arch = "wasm32")]
        Val::Js(j) => interop::for_in_keys(j),
        _ => vec![],
    }
}

// ---------------------------------------------------------------- calls

thread_local! {
    static CALLS: RefCell<Vec<Rc<str>>> = const { RefCell::new(Vec::new()) };
}

/// The names of the functions being called, innermost last ( for error stacks ).
pub fn call_stack() -> Vec<Rc<str>> {
    CALLS.with(|c| c.borrow().clone())
}

/// Call a function value with `this`.
pub fn call(f: &Val, this: &Val, args: &[Val]) -> R<Val> {
    let name = match f {
        Val::Obj(o) => o.func().map(|f| f.name.borrow().clone()),
        _ => None,
    };
    if let Some(n) = &name {
        CALLS.with(|c| c.borrow_mut().push(n.clone()));
    }
    let r = call_inner(f, this, args);
    if name.is_some() {
        CALLS.with(|c| c.borrow_mut().pop());
    }
    r
}

fn call_inner(f: &Val, this: &Val, args: &[Val]) -> R<Val> {
    if let Val::Obj(o) = f
        && let Kind::Proxy(target, handler) = &o.0.kind
    {
        let trap = get(handler, "apply")?;
        if trap.type_of() == "function" {
            return call(&trap, handler, &[target.clone(), this.clone(), array(args.to_vec())]);
        }
        return call(target, this, args);
    }
    match f {
        Val::Obj(o) => match o.func() {
            Some(func) => match &func.call {
                Some(body) => body(this, args),
                None => Err(type_error(&format!(
                    "Class constructor {} cannot be invoked without 'new'",
                    func.name.borrow()
                ))),
            },
            None => Err(type_error(&format!("{} is not a function", display(f)))),
        },
        #[cfg(target_arch = "wasm32")]
        Val::Js(j) => interop::call(j, this, args),
        _ => Err(type_error(&format!("{} is not a function", display(f)))),
    }
}

/// `recv.method( ...args )`; `site` is the source text of the callee for
/// V8's "… is not a function".
pub fn call_method(recv: &Val, method: &str, args: &[Val], site: &str) -> R<Val> {
    let f = get(recv, method)?;
    if f.type_of() != "function" {
        return Err(type_error(&format!("{site} is not a function")));
    }
    call(&f, recv, args)
}

/// The JavaScript source text of a Rust receiver expression, for V8's
/// messages: `g!(a, "b")` reads `a.b` and snake_case reads camelCase.
pub fn js_label(rust: &str) -> String {
    let mut s = rust.replace(' ', "");
    // g!(x,"k") → x.k, innermost first.
    loop {
        let Some(start) = s.rfind("g!(") else { break };
        let Some(end) = s[start..].find(')') else { break };
        let inner = &s[start + 3..start + end];
        let rendered = match inner.split_once(",\"") {
            Some((recv, key)) => format!("{recv}.{}", key.trim_end_matches('"')),
            None => inner.to_string(),
        };
        s = format!("{}{}{}", &s[..start], rendered, &s[start + end + 1..]);
    }
    let mut out = String::new();
    let mut upper = false;
    for ch in s.chars() {
        if ch == '_' {
            upper = !out.is_empty() && !out.ends_with('.');
            continue;
        }
        if upper {
            out.extend(ch.to_uppercase());
            upper = false;
        } else {
            out.push(ch);
        }
    }
    out.replace('&', "")
}

/// `recv.method( ...args )`, with `site` the receiver's source text.
pub fn m_site(recv: &Val, method: &str, args: &[Val], site: &str) -> R<Val> {
    let f = get(recv, method)?;
    if f.type_of() != "function" {
        return Err(type_error(&format!("{}.{method} is not a function", js_label(site))));
    }
    call(&f, recv, args)
}

/// `f( ...args )` for a value, with `site` its source text.
pub fn call_site(f: &Val, args: &[Val], site: &str) -> R<Val> {
    if f.type_of() != "function" {
        return Err(type_error(&format!("{} is not a function", js_label(site))));
    }
    call(f, &Val::Undef, args)
}

/// `for ( const x of v )`, with `site` the iterable's source text.
pub fn iterate_site(v: &Val, site: &str) -> R<Vec<Val>> {
    match iterate(v) {
        Err(e) if error_message(&e.0).ends_with("is not iterable") => {
            Err(type_error(&format!("{} is not iterable", js_label(site))))
        }
        r => r,
    }
}

/// `recv.method( ...args )` reported as `<recv>.<method>`.
pub fn m(recv: &Val, method: &str, args: &[Val]) -> R<Val> {
    let f = get(recv, method)?;
    if f.type_of() != "function" {
        return Err(type_error(&format!("{}.{method} is not a function", display_site(recv))));
    }
    call(&f, recv, args)
}

fn display_site(v: &Val) -> String {
    match v {
        Val::Obj(o) => o.class_name(),
        v => display(v),
    }
}

/// `new C( ...args )`.
pub fn construct(c: &Val, args: &[Val]) -> R<Val> {
    if let Val::Obj(o) = c
        && let Kind::Proxy(target, handler) = &o.0.kind
    {
        let trap = get(handler, "construct")?;
        if trap.type_of() == "function" {
            return call(&trap, handler, &[target.clone(), array(args.to_vec())]);
        }
        return construct(target, args);
    }
    match c {
        Val::Obj(o) => match o.func() {
            Some(f) => {
                if let Some(body) = &f.construct {
                    return body(&Val::Undef, args);
                }
                let Some(init) = &f.init else {
                    // A plain function used with `new`: the result is its returned object or `this`.
                    if CONSTRUCTIBLE.with(|c| c.borrow().contains(&o.ptr()))
                        && let Some(body) = &f.call
                    {
                        let prototype = match get(c, "prototype")? {
                            Val::Obj(p) => p,
                            _ => proto("Object"),
                        };
                        let this = Val::Obj(Obj::with(Some(prototype), Kind::Plain));
                        let r = body(&this, args)?;
                        return Ok(if matches!(r, Val::Obj(_)) { r } else { this });
                    }
                    return Err(type_error(&format!("{} is not a constructor", f.name.borrow())));
                };
                let prototype = match get(c, "prototype")? {
                    Val::Obj(p) => Some(p),
                    _ => Some(proto("Object")),
                };
                // Subclasses of Error make error objects.
                let error = proto("Error");
                let mut is_error = false;
                let mut cur = prototype.clone();
                while let Some(p) = cur {
                    if p.ptr_eq(&error) {
                        is_error = true;
                        break;
                    }
                    cur = p.proto();
                }
                let this = Val::Obj(Obj::with(prototype, if is_error { Kind::Error } else { Kind::Plain }));
                init(&this, args)?;
                Ok(this)
            }
            None => Err(type_error(&format!("{} is not a constructor", display(c)))),
        },
        #[cfg(target_arch = "wasm32")]
        Val::Js(j) => interop::construct(j, args),
        _ => Err(type_error(&format!("{} is not a constructor", display(c)))),
    }
}

/// `super( ...args )` from a subclass initializer.
pub fn super_init(parent: &Val, this: &Val, args: &[Val]) -> R<()> {
    let f = parent.obj().and_then(Obj::func).and_then(|f| f.init.clone());
    match f {
        Some(init) => init(this, args).map(|_| ()),
        None => Ok(()),
    }
}

/// `super.method( ...args )`: the parent prototype's method on `this`.
pub fn super_call(parent: &Val, this: &Val, method: &str, args: &[Val]) -> R<Val> {
    let prototype = get(parent, "prototype")?;
    let Val::Obj(p) = prototype else {
        return Err(type_error("super has no prototype"));
    };
    let f = get_from(&p, method, this)?;
    call(&f, this, args)
}

/// `super.prop` ( a getter of the parent prototype ).
pub fn super_get(parent: &Val, this: &Val, key: &str) -> R<Val> {
    let Val::Obj(p) = get(parent, "prototype")? else {
        return Ok(Val::Undef);
    };
    get_from(&p, key, this)
}

/// `v instanceof C`.
pub fn instance_of(v: &Val, c: &Val) -> R<bool> {
    if c.type_of() != "function" {
        return Err(type_error("Right-hand side of 'instanceof' is not callable"));
    }
    let Val::Obj(o) = v else {
        #[cfg(target_arch = "wasm32")]
        if let Val::Js(j) = v {
            return Ok(interop::instance_of(j, c));
        }
        return Ok(false);
    };
    let Val::Obj(target) = get(c, "prototype")? else {
        return Ok(false);
    };
    let mut cur = o.proto();
    while let Some(p) = cur {
        if p.ptr_eq(&target) {
            return Ok(true);
        }
        cur = p.proto();
    }
    Ok(false)
}

// ---------------------------------------------------------------- functions and classes

/// A native function.
pub fn func(name: &str, length: u32, body: impl Fn(&Val, &[Val]) -> R<Val> + 'static) -> Val {
    Val::Obj(Obj::with(
        Some(proto("Function")),
        Kind::Func(Func {
            name: RefCell::new(Rc::from(name)),
            length: Cell::new(length),
            call: Some(Rc::new(body)),
            init: None,
            construct: None,
            forward: RefCell::new(None),
        }),
    ))
}

pub fn is_constructible(o: &Obj) -> bool {
    CONSTRUCTIBLE.with(|c| c.borrow().contains(&o.ptr()))
}

/// A `function` ( not an arrow or a method ): constructible, with a `prototype`.
pub fn func_ctor(name: &str, length: u32, body: impl Fn(&Val, &[Val]) -> R<Val> + 'static) -> Val {
    let f = func(name, length, body);
    if let Val::Obj(o) = &f {
        let prototype = Obj::with(Some(proto("Object")), Kind::Plain);
        prototype.define("constructor", o.clone(), false);
        o.define("prototype", prototype, false);
        CONSTRUCTIBLE.with(|c| c.borrow_mut().insert(o.ptr()));
    }
    f
}

/// An argument, or `undefined`.
pub fn arg(args: &[Val], i: usize) -> Val {
    args.get(i).cloned().unwrap_or(Val::Undef)
}

/// An argument with a default for `undefined` ( `param = default` ).
pub fn arg_or(args: &[Val], i: usize, default: impl Into<Val>) -> Val {
    match args.get(i) {
        None | Some(Val::Undef) => default.into(),
        Some(v) => v.clone(),
    }
}

/// A class: its constructor function and prototype.
#[derive(Clone)]
pub struct Class {
    pub ctor: Val,
    pub prototype: Obj,
}

impl Class {
    /// `class Name extends Parent { constructor( … ) { … } }`; `init` is the
    /// constructor body ( it calls `super_init` itself ), `length` its arity.
    pub fn new(
        name: &str,
        parent: Option<&Class>,
        length: u32,
        init: impl Fn(&Val, &[Val]) -> R<Val> + 'static,
    ) -> Class {
        let prototype = Obj::with(
            Some(parent.map_or_else(|| proto("Object"), |p| p.prototype.clone())),
            Kind::Plain,
        );
        let ctor = Obj::with(
            Some(parent.map_or_else(
                || proto("Function"),
                |p| p.ctor.obj().expect("constructor").clone(),
            )),
            Kind::Func(Func {
                name: RefCell::new(Rc::from(name)),
                length: Cell::new(length),
                call: None,
                init: Some(Rc::new(init)),
                construct: None,
                forward: RefCell::new(None),
            }),
        );
        ctor.define("prototype", prototype.clone(), false);
        prototype.define("constructor", ctor.clone(), false);
        Class {
            ctor: Val::Obj(ctor),
            prototype,
        }
    }
    /// A prototype method ( non-enumerable ), with its declared arity.
    pub fn method(
        &self,
        name: &str,
        length: u32,
        body: impl Fn(&Val, &[Val]) -> R<Val> + 'static,
    ) -> &Self {
        self.prototype.define(name, func(name, length, body), false);
        self
    }
    pub fn getter(&self, name: &str, body: impl Fn(&Val) -> R<Val> + 'static) -> &Self {
        let existing = match self.prototype.0.props.borrow().get(name) {
            Some(Prop {
                slot: Slot::Accessor(_, s),
                ..
            }) => s.clone(),
            _ => None,
        };
        self.prototype.define_accessor(
            name,
            Some(func(name, 0, move |this, _| body(this))),
            existing,
            false,
        );
        self
    }
    pub fn setter(&self, name: &str, body: impl Fn(&Val, Val) -> R<()> + 'static) -> &Self {
        let existing = match self.prototype.0.props.borrow().get(name) {
            Some(Prop {
                slot: Slot::Accessor(g, _),
                ..
            }) => g.clone(),
            _ => None,
        };
        self.prototype.define_accessor(
            name,
            existing,
            Some(func(name, 1, move |this, args| {
                body(this, arg(args, 0)).map(|_| Val::Undef)
            })),
            false,
        );
        self
    }
    /// A static property ( `static get type() { … }` is a static getter ).
    pub fn static_getter(&self, name: &str, body: impl Fn(&Val) -> R<Val> + 'static) -> &Self {
        if let Val::Obj(c) = &self.ctor {
            c.define_accessor(name, Some(func(name, 0, move |this, _| body(this))), None, false);
        }
        self
    }
    pub fn static_value(&self, name: &str, v: impl Into<Val>) -> &Self {
        if let Val::Obj(c) = &self.ctor {
            c.define(name, v, true);
        }
        self
    }
    /// `new Class( ...args )`.
    pub fn new_instance(&self, args: &[Val]) -> R<Val> {
        construct(&self.ctor, args)
    }
}

/// `{ … }`: a plain object with the given entries in order.
pub fn object(entries: &[(&str, Val)]) -> Val {
    let o = Obj::with(Some(proto("Object")), Kind::Plain);
    for (k, v) in entries {
        o.define(k, v.clone(), true);
    }
    Val::Obj(o)
}

/// `[ … ]`.
pub fn array(items: Vec<Val>) -> Val {
    Val::Obj(Obj::with(Some(proto("Array")), Kind::Array(RefCell::new(items))))
}

/// `new Map()`.
pub fn new_map() -> Val {
    Val::Obj(Obj::with(Some(proto("Map")), Kind::Map(RefCell::new(OrderedMap::default()))))
}
/// `new Set()`.
pub fn new_set() -> Val {
    Val::Obj(Obj::with(Some(proto("Set")), Kind::Set(RefCell::new(OrderedMap::default()))))
}
/// `new WeakMap()`.
pub fn new_weak_map() -> Val {
    Val::Obj(Obj::with(Some(proto("WeakMap")), Kind::WeakMap(RefCell::new(HashMap::new()))))
}

/// `Object.getPrototypeOf( v ) === Object.prototype`.
pub fn is_plain_object(v: &Val) -> bool {
    match v {
        Val::Obj(o) => {
            matches!(o.0.kind, Kind::Plain)
                && o.proto().is_some_and(|p| p.ptr_eq(&proto("Object")))
        }
        #[cfg(target_arch = "wasm32")]
        Val::Js(j) => interop::is_plain_object(j),
        _ => false,
    }
}

/// `Array.isArray( v )`.
pub fn is_array(v: &Val) -> bool {
    match v {
        Val::Obj(o) => o.is_array(),
        #[cfg(target_arch = "wasm32")]
        Val::Js(j) => js_sys::Array::is_array(j),
        _ => false,
    }
}

/// The values `for ( const x of v )` visits.
pub fn iterate(v: &Val) -> R<Vec<Val>> {
    match v {
        Val::Obj(o) => match &o.0.kind {
            Kind::Array(a) => Ok(a.borrow().clone()),
            Kind::Typed(kind, view) => Ok(view.values(*kind).into_iter().map(Val::Num).collect()),
            Kind::Set(s) => Ok(s.borrow().pairs().into_iter().map(|(k, _)| k).collect()),
            Kind::Map(m) => Ok(m
                .borrow()
                .pairs()
                .into_iter()
                .map(|(k, v)| array(vec![k, v]))
                .collect()),
            _ => {
                let it = get(v, "@@iterator")?;
                if it.type_of() != "function" {
                    return Err(type_error(&format!("{} is not iterable", display_site(v))));
                }
                let result = call(&it, v, &[])?;
                match &result {
                    Val::Obj(r) if r.is_array() => Ok(r.items()),
                    _ => {
                        // An iterator object: call next() until done.
                        let mut out = vec![];
                        loop {
                            let step = m(&result, "next", &[])?;
                            if get(&step, "done")?.truthy() {
                                break;
                            }
                            out.push(get(&step, "value")?);
                        }
                        Ok(out)
                    }
                }
            }
        },
        Val::Str(text) => Ok(text.chars().map(|c| Val::from(c.to_string())).collect()),
        #[cfg(target_arch = "wasm32")]
        Val::Js(j) => interop::iterate(j),
        _ => Err(type_error(&format!("{} is not iterable", display(v)))),
    }
}

/// An object's identity, for WeakMap keys.
pub fn identity(v: &Val) -> Option<usize> {
    match v {
        Val::Obj(o) => Some(o.ptr()),
        #[cfg(target_arch = "wasm32")]
        Val::Js(j) => Some(interop::identity(j) as usize | (1usize << (usize::BITS - 1))),
        _ => None,
    }
}

/// `new Float32Array( n )` and its kin, over a new buffer.
pub fn typed(kind: TypedKind, values: Vec<f64>) -> Val {
    let (buffer, bytes) = array_buffer(values.len() * kind.bytes());
    let view = View { bytes, buffer, offset: 0, len: values.len() };
    for (i, v) in values.into_iter().enumerate() {
        view.set(kind, i, v);
    }
    Val::Obj(Obj::with(Some(proto(kind.name())), Kind::Typed(kind, view)))
}

/// `new Float32Array( buffer, byteOffset, length )`: a view sharing the buffer's bytes.
pub fn typed_view(kind: TypedKind, buffer: &Obj, offset: usize, len: Option<usize>) -> R<Val> {
    let Kind::ArrayBuffer(bytes) = &buffer.0.kind else {
        return Err(type_error("not an ArrayBuffer"));
    };
    let total = bytes.borrow().len();
    if offset % kind.bytes() != 0 {
        return Err(Exc(make_error("RangeError", &format!("start offset of {} should be a multiple of {}", kind.name(), kind.bytes()))));
    }
    let len = len.unwrap_or((total.saturating_sub(offset)) / kind.bytes());
    let view = View { bytes: bytes.clone(), buffer: buffer.clone(), offset, len };
    Ok(Val::Obj(Obj::with(Some(proto(kind.name())), Kind::Typed(kind, view))))
}

/// A typed array's element type.
pub fn typed_kind(v: &Val) -> Option<TypedKind> {
    match v {
        Val::Obj(o) => match &o.0.kind {
            Kind::Typed(k, _) => Some(*k),
            _ => None,
        },
        _ => None,
    }
}

/// A typed array's or array's values as numbers.
pub fn numbers(v: &Val) -> R<Vec<f64>> {
    match v {
        Val::Obj(o) => match &o.0.kind {
            Kind::Typed(kind, view) => Ok(view.values(*kind)),
            _ => Ok(iterate(v)?.iter().map(to_number).collect()),
        },
        v => Ok(iterate(v)?.iter().map(to_number).collect()),
    }
}

/// `{ ...a, ...b }` and Object.assign's copy of enumerable own data.
pub fn assign(target: &Val, source: &Val) -> R<()> {
    if source.is_nullish() {
        return Ok(());
    }
    // Object.assign copies own enumerable string keys, then symbols.
    let symbols: Vec<Rc<str>> = match source {
        Val::Obj(o) => o.0.props.borrow().symbol_keys(),
        _ => vec![],
    };
    for k in keys(source).into_iter().chain(symbols) {
        let v = get(source, &k)?;
        set(target, &k, v)?;
    }
    Ok(())
}

/// `{ ...v }`.
pub fn spread(v: &Val) -> R<Val> {
    let o = object(&[]);
    assign(&o, v)?;
    Ok(o)
}

/// `console.warn` / `console.error` / `console.log` with three's arguments.
pub fn console(level: &str, args: &[Val]) {
    #[cfg(target_arch = "wasm32")]
    interop::console(level, args);
    #[cfg(not(target_arch = "wasm32"))]
    {
        let text: Vec<String> = args.iter().map(display).collect();
        eprintln!("[{level}] {}", text.join(" "));
    }
}
