//! The global objects the sources reach: Math, Number, Object, Array,
//! String, JSON, Symbol, Reflect, Proxy, Map, Set, WeakMap, WeakSet, the
//! Error classes, Promise, console, parseInt and friends, plus RegExp.
use super::ops::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;
use super::*;

/// `Symbol.iterator` as a property key.
pub const ITERATOR: &str = "@@iterator";

fn native_obj(entries: Vec<(&str, u32, NativeFn)>) -> Obj {
    let o = Obj::with(Some(proto("Object")), Kind::Plain);
    for (name, length, body) in entries {
        let f = Obj::with(
            Some(proto("Function")),
            Kind::Func(Func {
                name: RefCell::new(Rc::from(name)),
                length: Cell::new(length),
                call: Some(body),
                init: None,
                construct: None,
                forward: RefCell::new(None),
            }),
        );
        o.define(name, f, false);
    }
    o
}

macro_rules! nf {
    ($name:expr, $len:expr, |$args:ident| $body:expr) => {
        ($name, $len, Rc::new(move |_this: &Val, $args: &[Val]| -> R<Val> { $body }) as NativeFn)
    };
}

fn n(a: &[Val], i: usize) -> R<f64> {
    num(&arg(a, i))
}

thread_local! {
    static SEED: std::cell::Cell<u64> = const { std::cell::Cell::new(0x2545_F491_4F6C_DD1D) };
}

/// Math.random ( xorshift; the port never depends on its values ).
pub fn random() -> f64 {
    SEED.with(|s| {
        let mut x = s.get();
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        s.set(x);
        (x >> 11) as f64 / (1u64 << 53) as f64
    })
}

/// Math.round: half up.
pub fn js_round(x: f64) -> f64 {
    if !x.is_finite() || x.fract() == 0.0 {
        return x;
    }
    let f = x.floor();
    if x - f >= 0.5 { f + 1.0 } else { f }
}

fn install_math() {
    let math = native_obj(vec![
        nf!("floor", 1, |a| Ok(Val::Num(n(a, 0)?.floor()))),
        nf!("ceil", 1, |a| Ok(Val::Num(n(a, 0)?.ceil()))),
        nf!("round", 1, |a| Ok(Val::Num(js_round(n(a, 0)?)))),
        nf!("trunc", 1, |a| Ok(Val::Num(n(a, 0)?.trunc()))),
        nf!("abs", 1, |a| Ok(Val::Num(n(a, 0)?.abs()))),
        nf!("sign", 1, |a| {
            let x = n(a, 0)?;
            Ok(Val::Num(if x.is_nan() || x == 0.0 { x } else { x.signum() }))
        }),
        nf!("sqrt", 1, |a| Ok(Val::Num(n(a, 0)?.sqrt()))),
        nf!("cbrt", 1, |a| Ok(Val::Num(n(a, 0)?.cbrt()))),
        nf!("sin", 1, |a| Ok(Val::Num(n(a, 0)?.sin()))),
        nf!("cos", 1, |a| Ok(Val::Num(n(a, 0)?.cos()))),
        nf!("tan", 1, |a| Ok(Val::Num(n(a, 0)?.tan()))),
        nf!("asin", 1, |a| Ok(Val::Num(n(a, 0)?.asin()))),
        nf!("acos", 1, |a| Ok(Val::Num(n(a, 0)?.acos()))),
        nf!("atan", 1, |a| Ok(Val::Num(n(a, 0)?.atan()))),
        nf!("atan2", 2, |a| Ok(Val::Num(n(a, 0)?.atan2(n(a, 1)?)))),
        nf!("exp", 1, |a| Ok(Val::Num(n(a, 0)?.exp()))),
        nf!("log", 1, |a| Ok(Val::Num(n(a, 0)?.ln()))),
        nf!("log2", 1, |a| Ok(Val::Num(n(a, 0)?.log2()))),
        nf!("log10", 1, |a| Ok(Val::Num(n(a, 0)?.log10()))),
        nf!("pow", 2, |a| pow(&arg(a, 0), &arg(a, 1))),
        nf!("random", 0, |_a| Ok(Val::Num(random()))),
        nf!("hypot", 2, |a| {
            let mut s = 0.0;
            for v in a {
                let x = num(v)?;
                s += x * x;
            }
            Ok(Val::Num(s.sqrt()))
        }),
        nf!("imul", 2, |a| Ok(Val::Num(to_i32(n(a, 0)?).wrapping_mul(to_i32(n(a, 1)?)) as f64))),
        nf!("fround", 1, |a| Ok(Val::Num(n(a, 0)? as f32 as f64))),
        nf!("max", 2, |a| {
            let mut m = f64::NEG_INFINITY;
            for v in a {
                let x = num(v)?;
                if x.is_nan() {
                    return Ok(Val::Num(f64::NAN));
                }
                if x > m || (x == 0.0 && m == 0.0 && x.is_sign_positive()) {
                    m = x;
                }
            }
            Ok(Val::Num(m))
        }),
        nf!("min", 2, |a| {
            let mut m = f64::INFINITY;
            for v in a {
                let x = num(v)?;
                if x.is_nan() {
                    return Ok(Val::Num(f64::NAN));
                }
                if x < m || (x == 0.0 && m == 0.0 && x.is_sign_negative()) {
                    m = x;
                }
            }
            Ok(Val::Num(m))
        }),
    ]);
    for (k, v) in [
        ("PI", std::f64::consts::PI),
        ("E", std::f64::consts::E),
        ("LN2", std::f64::consts::LN_2),
        ("LN10", std::f64::consts::LN_10),
        ("LOG2E", std::f64::consts::LOG2_E),
        ("LOG10E", std::f64::consts::LOG10_E),
        ("SQRT2", std::f64::consts::SQRT_2),
        ("SQRT1_2", std::f64::consts::FRAC_1_SQRT_2),
    ] {
        math.define(k, v, false);
    }
    define_global("Math", Val::Obj(math));
}

/// parseInt( string, radix ).
pub fn parse_int(text: &str, radix: Option<u32>) -> f64 {
    let t = text.trim_start();
    let (sign, t) = match t.strip_prefix('-') {
        Some(rest) => (-1.0, rest),
        None => (1.0, t.strip_prefix('+').unwrap_or(t)),
    };
    let (radix, t) = match radix {
        Some(16) | None if t.starts_with("0x") || t.starts_with("0X") => (16, &t[2..]),
        Some(r) if r != 0 => (r, t),
        _ => (10, t),
    };
    let digits: String = t.chars().take_while(|c| c.is_digit(radix)).collect();
    if digits.is_empty() {
        return f64::NAN;
    }
    let mut v = 0.0;
    for c in digits.chars() {
        v = v * radix as f64 + c.to_digit(radix).unwrap() as f64;
    }
    sign * v
}

/// parseFloat( string ).
pub fn parse_float(text: &str) -> f64 {
    let t = text.trim_start();
    let mut end = 0;
    let b = t.as_bytes();
    let mut seen_dot = false;
    let mut seen_e = false;
    let mut i = 0;
    if i < b.len() && (b[i] == b'+' || b[i] == b'-') {
        i += 1;
    }
    if t[i..].starts_with("Infinity") {
        return if t.starts_with('-') { f64::NEG_INFINITY } else { f64::INFINITY };
    }
    while i < b.len() {
        match b[i] {
            b'0'..=b'9' => end = i + 1,
            b'.' if !seen_dot && !seen_e => seen_dot = true,
            b'e' | b'E' if !seen_e && end > 0 => {
                seen_e = true;
                if i + 1 < b.len() && (b[i + 1] == b'+' || b[i + 1] == b'-') {
                    i += 1;
                }
            }
            _ => break,
        }
        i += 1;
    }
    t[..end].parse::<f64>().unwrap_or(f64::NAN)
}

fn install_number() {
    let number = func("Number", 1, |_t, a| Ok(Val::Num(if a.is_empty() { 0.0 } else { num(&a[0])? })));
    let o = number.obj().unwrap().clone();
    for (name, length, body) in vec![
        nf!("isInteger", 1, |a| Ok(Val::Bool(matches!(arg(a, 0), Val::Num(x) if x.is_finite() && x.fract() == 0.0)))),
        nf!("isNaN", 1, |a| Ok(Val::Bool(matches!(arg(a, 0), Val::Num(x) if x.is_nan())))),
        nf!("isFinite", 1, |a| Ok(Val::Bool(matches!(arg(a, 0), Val::Num(x) if x.is_finite())))),
        nf!("parseInt", 2, |a| {
            let r = arg(a, 1);
            Ok(Val::Num(parse_int(&to_string(&arg(a, 0))?, if r.is_undef() { None } else { Some(num(&r)? as u32) })))
        }),
        nf!("parseFloat", 1, |a| Ok(Val::Num(parse_float(&to_string(&arg(a, 0))?)))),
    ] {
        o.define(name, func(name, length, move |t, a| body(t, a)), false);
    }
    o.define("EPSILON", f64::EPSILON, false);
    o.define("MAX_SAFE_INTEGER", 9007199254740991.0, false);
    o.define("MIN_SAFE_INTEGER", -9007199254740991.0, false);
    o.define("MAX_VALUE", f64::MAX, false);
    o.define("MIN_VALUE", 5e-324, false);
    o.define("POSITIVE_INFINITY", f64::INFINITY, false);
    o.define("NEGATIVE_INFINITY", f64::NEG_INFINITY, false);
    o.define("NaN", f64::NAN, false);
    o.define("prototype", proto("Number"), false);
    proto("Number").define("constructor", o.clone(), false);
    define_global("Number", number);
    define_global(
        "parseInt",
        func("parseInt", 2, |_t, a| {
            let r = arg(a, 1);
            Ok(Val::Num(parse_int(&to_string(&arg(a, 0))?, if r.is_undef() { None } else { Some(num(&r)? as u32) })))
        }),
    );
    define_global("parseFloat", func("parseFloat", 1, |_t, a| Ok(Val::Num(parse_float(&to_string(&arg(a, 0))?)))));
    define_global("isNaN", func("isNaN", 1, |_t, a| Ok(Val::Bool(num(&arg(a, 0))?.is_nan()))));
    define_global("isFinite", func("isFinite", 1, |_t, a| Ok(Val::Bool(num(&arg(a, 0))?.is_finite()))));
    define_global("Infinity", f64::INFINITY);
    define_global("NaN", f64::NAN);
    define_global("undefined", Val::Undef);
    let boolean = func("Boolean", 1, |_t, a| Ok(Val::Bool(arg(a, 0).truthy())));
    boolean.obj().unwrap().define("prototype", proto("Boolean"), false);
    define_global("Boolean", boolean);
    let string = func("String", 1, |_t, a| {
        if a.is_empty() {
            return Ok(s(""));
        }
        Ok(Val::Str(to_string(&to_primitive(&a[0], true)?)?))
    });
    let so = string.obj().unwrap().clone();
    so.define(
        "fromCharCode",
        func("fromCharCode", 1, |_t, a| {
            let mut u = vec![];
            for v in a {
                u.push(to_u32(num(v)?) as u16);
            }
            Ok(Val::from(String::from_utf16_lossy(&u)))
        }),
        false,
    );
    so.define("prototype", proto("String"), false);
    proto("String").define("constructor", so, false);
    define_global("String", string);
}

/// ToObject's TypeError for undefined and null.
fn to_object_check(v: &Val) -> R<()> {
    if v.is_nullish() {
        return Err(type_error("Cannot convert undefined or null to object"));
    }
    Ok(())
}

/// A property descriptor object for Object.getOwnPropertyDescriptor.
fn descriptor(o: &Obj, k: &str) -> Val {
    if let Some(v) = o.own(k)
        && !o.0.props.borrow().contains(k)
    {
        return object(&[("value", v), ("writable", Val::Bool(true)), ("enumerable", Val::Bool(true)), ("configurable", Val::Bool(true))]);
    }
    match o.0.props.borrow().get(k) {
        Some(Prop { slot: Slot::Data(v), enumerable }) => object(&[
            ("value", v.clone()),
            ("writable", Val::Bool(true)),
            ("enumerable", Val::Bool(*enumerable)),
            ("configurable", Val::Bool(true)),
        ]),
        Some(Prop { slot: Slot::Accessor(g, s), enumerable }) => object(&[
            ("get", g.clone().unwrap_or(Val::Undef)),
            ("set", s.clone().unwrap_or(Val::Undef)),
            ("enumerable", Val::Bool(*enumerable)),
            ("configurable", Val::Bool(true)),
        ]),
        None => Val::Undef,
    }
}

/// Object.defineProperty( o, key, descriptor ).
pub fn define_property(o: &Val, k: &str, d: &Val) -> R<()> {
    let Val::Obj(obj) = o else {
        #[cfg(target_arch = "wasm32")]
        if let Val::Js(_) = o {
            return set(o, k, get(d, "value")?);
        }
        return Err(type_error("Object.defineProperty called on non-object"));
    };
    let enumerable = get(d, "enumerable")?.truthy();
    let getter = get(d, "get")?;
    let setter = get(d, "set")?;
    if !getter.is_undef() || !setter.is_undef() {
        obj.define_accessor(
            k,
            (!getter.is_undef()).then_some(getter),
            (!setter.is_undef()).then_some(setter),
            enumerable,
        );
    } else {
        obj.define(k, get(d, "value")?, enumerable);
    }
    Ok(())
}

fn install_object() {
    let object_ctor = global("Object");
    let o = object_ctor.obj().unwrap().clone();
    for (name, length, body) in vec![
        nf!("keys", 1, |a| {
            to_object_check(&arg(a, 0))?;
            Ok(array(keys(&arg(a, 0)).into_iter().map(Val::Str).collect()))
        }),
        nf!("values", 1, |a| {
            let v = arg(a, 0);
            to_object_check(&v)?;
            let mut out = vec![];
            for k in keys(&v) {
                out.push(get(&v, &k)?);
            }
            Ok(array(out))
        }),
        nf!("entries", 1, |a| {
            let v = arg(a, 0);
            to_object_check(&v)?;
            let mut out = vec![];
            for k in keys(&v) {
                out.push(array(vec![Val::Str(k.clone()), get(&v, &k)?]));
            }
            Ok(array(out))
        }),
        nf!("fromEntries", 1, |a| {
            let out = object(&[]);
            for entry in iterate(&arg(a, 0))? {
                let key = crate::rt::ops::key(&get_index(&entry, 0)?)?;
                set(&out, &key, get_index(&entry, 1)?)?;
            }
            Ok(out)
        }),
        nf!("assign", 2, |a| {
            let target = arg(a, 0);
            to_object_check(&target)?;
            for src in a.iter().skip(1) {
                assign(&target, src)?;
            }
            Ok(target)
        }),
        nf!("getPrototypeOf", 1, |a| Ok(match arg(a, 0) {
            Val::Obj(o) => o.proto().map_or(Val::Null, Val::Obj),
            Val::Str(_) => Val::Obj(proto("String")),
            Val::Num(_) => Val::Obj(proto("Number")),
            Val::Bool(_) => Val::Obj(proto("Boolean")),
            #[cfg(target_arch = "wasm32")]
            Val::Js(j) => interop::get_prototype_of(&j),
            _ => return Err(type_error("Cannot convert undefined or null to object")),
        })),
        nf!("setPrototypeOf", 2, |a| {
            if let (Val::Obj(o), p) = (arg(a, 0), arg(a, 1)) {
                o.set_proto(p.obj().cloned());
            }
            Ok(arg(a, 0))
        }),
        nf!("getOwnPropertyNames", 1, |a| Ok(match arg(a, 0) {
            Val::Obj(o) => array(o.own_keys().into_iter().map(Val::Str).collect()),
            v => array(keys(&v).into_iter().map(Val::Str).collect()),
        })),
        nf!("getOwnPropertyDescriptor", 2, |a| Ok(match arg(a, 0) {
            Val::Obj(o) => descriptor(&o, &key(&arg(a, 1))?),
            _ => Val::Undef,
        })),
        nf!("getOwnPropertyDescriptors", 1, |a| {
            let out = object(&[]);
            if let Val::Obj(o) = arg(a, 0) {
                for k in o.own_keys() {
                    set(&out, &k, descriptor(&o, &k))?;
                }
            }
            Ok(out)
        }),
        nf!("defineProperty", 3, |a| {
            define_property(&arg(a, 0), &key(&arg(a, 1))?, &arg(a, 2))?;
            Ok(arg(a, 0))
        }),
        nf!("defineProperties", 2, |a| {
            let target = arg(a, 0);
            let props = arg(a, 1);
            for k in keys(&props) {
                define_property(&target, &k, &get(&props, &k)?)?;
            }
            Ok(target)
        }),
        nf!("freeze", 1, |a| Ok(arg(a, 0))),
        nf!("create", 1, |a| Ok(Val::Obj(Obj::with(arg(a, 0).obj().cloned(), Kind::Plain)))),
    ] {
        o.define(name, func(name, length, move |t, a| body(t, a)), false);
    }
    let array_ctor = global("Array");
    let ao = array_ctor.obj().unwrap().clone();
    ao.define("isArray", func("isArray", 1, |_t, a| Ok(Val::Bool(is_array(&arg(a, 0))))), false);
    ao.define(
        "from",
        func("from", 1, |_t, a| {
            let src = arg(a, 0);
            let mapper = arg(a, 1);
            let items = if src.is_nullish() {
                return Err(type_error("undefined is not iterable"));
            } else if is_array(&src) || matches!(&src, Val::Obj(o) if matches!(o.0.kind, Kind::Typed(..) | Kind::Set(_) | Kind::Map(_))) || has(&src, ITERATOR).unwrap_or(false) {
                iterate(&src)?
            } else {
                let len = num(&get(&src, "length")?)?;
                let len = if len.is_nan() { 0 } else { len as usize };
                (0..len).map(|i| get(&src, &i.to_string())).collect::<R<Vec<_>>>()?
            };
            if mapper.is_undef() {
                return Ok(array(items));
            }
            let mut out = vec![];
            for (i, v) in items.into_iter().enumerate() {
                out.push(call(&mapper, &Val::Undef, &[v, Val::Num(i as f64)])?);
            }
            Ok(array(out))
        }),
        false,
    );
    ao.define("of", func("of", 0, |_t, a| Ok(array(a.to_vec()))), false);
}

fn install_collection_ctors() {
    for (name, make) in [
        ("Map", new_map as fn() -> Val),
        ("Set", new_set as fn() -> Val),
        ("WeakMap", new_weak_map as fn() -> Val),
        ("WeakSet", new_weak_map as fn() -> Val),
    ] {
        let c = Obj::with(
            Some(proto("Function")),
            Kind::Func(Func {
                name: RefCell::new(Rc::from(name)),
                length: Cell::new(0),
                call: None,
                init: None,
                construct: Some(Rc::new(move |_t, a| {
                    let m = make();
                    let init = arg(a, 0);
                    if !init.is_nullish() {
                        for item in iterate(&init)? {
                            match name {
                                "Map" | "WeakMap" => {
                                    super::m(&m, "set", &[get_index(&item, 0)?, get_index(&item, 1)?])?;
                                }
                                "Set" => {
                                    super::m(&m, "add", &[item])?;
                                }
                                _ => {
                                    super::m(&m, "set", &[item, Val::Bool(true)])?;
                                }
                            }
                        }
                    }
                    Ok(m)
                })),
                forward: RefCell::new(None),
            }),
        );
        let p = match name {
            "WeakSet" => proto("WeakMap"),
            n => proto(match n {
                "Map" => "Map",
                "Set" => "Set",
                _ => "WeakMap",
            }),
        };
        c.define("prototype", p.clone(), false);
        if name != "WeakSet" {
            p.define("constructor", c.clone(), false);
        }
        define_global(name, Val::Obj(c));
    }
    // WeakSet's add is WeakMap's set.
    proto("WeakMap").define(
        "add",
        func("add", 1, |this, a| {
            super::m(this, "set", &[arg(a, 0), Val::Bool(true)])?;
            Ok(this.clone())
        }),
        false,
    );
}

fn install_errors() {
    for kind in ["Error", "TypeError", "RangeError", "SyntaxError"] {
        let p = proto(kind);
        let c = Obj::with(
            Some(if kind == "Error" { proto("Function") } else { global("Error").obj().unwrap().clone() }),
            Kind::Func(Func {
                name: RefCell::new(Rc::from(kind)),
                length: Cell::new(1),
                call: Some(Rc::new(move |_t, a| Ok(make_error(kind, &display(&arg(a, 0)))))),
                init: Some(Rc::new(move |this, a| {
                    let message = arg(a, 0);
                    if let Val::Obj(o) = this {
                        if !message.is_undef() {
                            o.define("message", Val::Str(to_string(&message)?), false);
                        }
                        let name = display(&get(this, "name")?);
                        o.define("stack", format!("{name}: {}", display(&get(this, "message")?)), false);
                    }
                    Ok(Val::Undef)
                })),
                construct: None,
                forward: RefCell::new(None),
            }),
        );
        c.define("prototype", p.clone(), false);
        p.define("constructor", c.clone(), false);
        define_global(kind, Val::Obj(c));
    }
}

/// JavaScript regular expression syntax as `regex` syntax: identity escapes
/// of non-meta characters become the characters, `\uXXXX` becomes `\u{XXXX}`.
pub fn js_regex_syntax(source: &str, flags: &str) -> String {
    let mut pattern = String::new();
    for f in flags.chars() {
        match f {
            'i' => pattern.push_str("(?i)"),
            'm' => pattern.push_str("(?m)"),
            's' => pattern.push_str("(?s)"),
            _ => {}
        }
    }
    let chars: Vec<char> = source.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        if c == '\\' && i + 1 < chars.len() {
            let e = chars[i + 1];
            if e == 'u' && i + 5 < chars.len() && chars[i + 2..i + 6].iter().all(|h| h.is_ascii_hexdigit()) {
                pattern.push_str("\\u{");
                pattern.extend(&chars[i + 2..i + 6]);
                pattern.push('}');
                i += 6;
                continue;
            }
            if e.is_ascii_alphanumeric() {
                pattern.push('\\');
                pattern.push(e);
            } else if "\\.+*?()|[]{}^$#&-~".contains(e) {
                pattern.push('\\');
                pattern.push(e);
            } else {
                pattern.push(e);
            }
            i += 2;
            continue;
        }
        pattern.push(c);
        i += 1;
    }
    pattern
}

thread_local! {
    /// Compiled patterns by source and flags: a literal evaluates to a new
    /// RegExp each time, its pattern compiled once ( as engines cache it ).
    static COMPILED: RefCell<HashMap<(Rc<str>, Rc<str>), Result<Rc<regex::Regex>, String>>> = RefCell::new(HashMap::new());
}

/// Shared, not cloned: a cloned Regex starts with an empty search cache.
fn compile(source: &str, flags: &str) -> Result<Rc<regex::Regex>, String> {
    COMPILED.with(|c| {
        c.borrow_mut()
            .entry((Rc::from(source), Rc::from(flags)))
            .or_insert_with(|| regex::Regex::new(&js_regex_syntax(source, flags)).map(Rc::new).map_err(|e| e.to_string()))
            .clone()
    })
}

/// A RegExp from a JavaScript literal's source and flags.
pub fn regexp(source: &str, flags: &str) -> Val {
    let re = compile(source, flags).unwrap_or_else(|e| panic!("regular expression /{source}/{flags}: {e}"));
    Val::Obj(Obj::with(
        Some(proto("RegExp")),
        Kind::RegExp(Rc::from(source), Rc::from(flags), re, Cell::new(0)),
    ))
}

/// The regex of a RegExp value.
pub fn regex_of(v: &Val) -> Option<(Rc<regex::Regex>, bool, &Cell<usize>)> {
    match v {
        Val::Obj(o) => match &o.0.kind {
            Kind::RegExp(_, flags, re, last) => Some((re.clone(), flags.contains('g'), last)),
            _ => None,
        },
        _ => None,
    }
}

/// RegExp.prototype.exec's result array: groups, `index` and `input`.
fn exec_result(text: &str, caps: &regex::Captures) -> Val {
    let items: Vec<Val> = (0..caps.len())
        .map(|i| caps.get(i).map_or(Val::Undef, |m| Val::from(m.as_str())))
        .collect();
    let a = array(items);
    let start = caps.get(0).unwrap().start();
    if let Val::Obj(o) = &a {
        o.define("index", Val::Num(text[..start].encode_utf16().count() as f64), true);
        o.define("input", text, true);
    }
    a
}

/// `re.exec( text )`.
pub fn regexp_exec(re_val: &Val, text: &str) -> R<Val> {
    let (re, global, last) = regex_of(re_val).ok_or_else(|| type_error("not a RegExp"))?;
    let start = if global { last.get() } else { 0 };
    if start > text.len() {
        last.set(0);
        return Ok(Val::Null);
    }
    match re.captures_at(text, start) {
        Some(caps) => {
            if global {
                last.set(caps.get(0).unwrap().end());
            }
            Ok(exec_result(text, &caps))
        }
        None => {
            if global {
                last.set(0);
            }
            Ok(Val::Null)
        }
    }
}

/// `text.replace( re, replacement )`.
pub fn regexp_replace(text: &str, re_val: &Val, replacement: &Val) -> R<String> {
    let (re, global, last) = regex_of(re_val).ok_or_else(|| type_error("not a RegExp"))?;
    let mut out = String::new();
    let mut prev = 0;
    for caps in re.captures_iter(text) {
        let m0 = caps.get(0).unwrap();
        out.push_str(&text[prev..m0.start()]);
        if replacement.type_of() == "function" {
            let mut args: Vec<Val> = (0..caps.len())
                .map(|i| caps.get(i).map_or(Val::Undef, |m| Val::from(m.as_str())))
                .collect();
            args.push(Val::Num(m0.start() as f64));
            args.push(Val::from(text));
            out.push_str(&to_string(&call(replacement, &Val::Undef, &args)?)?);
        } else {
            let rep = to_string(replacement)?;
            let groups: Vec<Option<&str>> = (1..caps.len()).map(|i| caps.get(i).map(|g| g.as_str())).collect();
            out.push_str(&super::builtins::expand_replacement(&rep, m0.as_str(), &groups, text, m0.start()));
        }
        prev = m0.end();
        if !global {
            break;
        }
    }
    out.push_str(&text[prev..]);
    if global {
        last.set(0);
    }
    Ok(out)
}

/// A RegExp from a dynamic pattern ( `new RegExp( source, flags )` ), or SyntaxError.
pub fn regexp_checked(source: &str, flags: &str) -> R<Val> {
    match compile(source, flags) {
        Ok(_) => Ok(regexp(source, flags)),
        Err(_) => Err(Exc(make_error("SyntaxError", &format!("Invalid regular expression: /{source}/{flags}")))),
    }
}

fn install_regexp() {
    let p = Obj::with(Some(proto("Object")), Kind::Plain);
    register_proto("RegExp", p.clone());
    let ctor = Obj::with(
        Some(proto("Function")),
        Kind::Func(Func {
            name: RefCell::new(Rc::from("RegExp")),
            length: Cell::new(2),
            call: None,
            init: None,
            construct: Some(Rc::new(|_t, a| {
                let src = arg(a, 0);
                if let Some((_, _, _)) = regex_of(&src) {
                    if let Val::Obj(o) = &src
                        && let Kind::RegExp(source, flags, ..) = &o.0.kind
                    {
                        let f = arg(a, 1);
                        let flags = if f.is_undef() { flags.to_string() } else { to_string(&f)?.to_string() };
                        return regexp_checked(source, &flags);
                    }
                }
                let source = if src.is_undef() { "(?:)".to_string() } else { to_string(&src)?.to_string() };
                let flags = match arg(a, 1) {
                    Val::Undef => String::new(),
                    f => to_string(&f)?.to_string(),
                };
                regexp_checked(&source, &flags)
            })),
            forward: RefCell::new(None),
        }),
    );
    ctor.define("prototype", p.clone(), false);
    p.define("constructor", ctor.clone(), false);
    define_global("RegExp", Val::Obj(ctor));
    p.define(
        "test",
        func("test", 1, |this, a| Ok(Val::Bool(!regexp_exec(this, &to_string(&arg(a, 0))?)?.is_null()))),
        false,
    );
    p.define("exec", func("exec", 1, |this, a| regexp_exec(this, &to_string(&arg(a, 0))?)), false);
    p.define_accessor(
        "source",
        Some(func("source", 0, |this, _a| Ok(match this {
            Val::Obj(o) => match &o.0.kind {
                Kind::RegExp(src, ..) => Val::Str(src.clone()),
                _ => Val::Undef,
            },
            _ => Val::Undef,
        }))),
        None,
        false,
    );
    p.define_accessor(
        "lastIndex",
        Some(func("lastIndex", 0, |this, _a| Ok(regex_of(this).map_or(Val::Undef, |(_, _, l)| Val::Num(l.get() as f64))))),
        Some(func("lastIndex", 1, |this, a| {
            if let Some((_, _, l)) = regex_of(this) {
                l.set(num(&arg(a, 0))? as usize);
            }
            Ok(Val::Undef)
        })),
        false,
    );
}

/// A settled promise.
pub fn promise(fulfilled: bool, v: Val) -> Val {
    Val::Obj(Obj::with(Some(proto("Promise")), Kind::Promise(RefCell::new(PromiseState::Settled(fulfilled, v)))))
}

fn pending_promise() -> Val {
    Val::Obj(Obj::with(Some(proto("Promise")), Kind::Promise(RefCell::new(PromiseState::Pending(vec![])))))
}

/// Settle a pending promise and run its reactions.
fn settle(p: &Val, fulfilled: bool, v: Val) -> R<()> {
    // Resolving with a promise adopts its state.
    let (fulfilled, v) = if fulfilled {
        match &v {
            Val::Obj(o) if matches!(o.0.kind, Kind::Promise(_)) => match await_val(v.clone()) {
                Ok(x) => (true, x),
                Err(e) => (false, e.0),
            },
            _ => (true, v),
        }
    } else {
        (false, v)
    };
    let Val::Obj(o) = p else { return Ok(()) };
    let Kind::Promise(state) = &o.0.kind else { return Ok(()) };
    let reactions = match &*state.borrow() {
        PromiseState::Pending(r) => r.clone(),
        PromiseState::Settled(..) => return Ok(()),
    };
    *state.borrow_mut() = PromiseState::Settled(fulfilled, v.clone());
    for (on_ok, on_err, derived) in reactions {
        react(&on_ok, &on_err, &derived, fulfilled, &v)?;
    }
    Ok(())
}

fn react(on_ok: &Val, on_err: &Val, derived: &Val, fulfilled: bool, v: &Val) -> R<()> {
    let handler = if fulfilled { on_ok } else { on_err };
    if handler.type_of() != "function" {
        return settle(derived, fulfilled, v.clone());
    }
    match call(handler, &Val::Undef, &[v.clone()]) {
        Ok(r) => settle(derived, true, r),
        Err(e) => settle(derived, false, e.0),
    }
}

/// `await v`: a settled promise's value or its rejection; other values as they are.
pub fn await_val(v: Val) -> R<Val> {
    #[cfg(target_arch = "wasm32")]
    if let Val::Js(j) = &v
        && let Some(value) = interop::settled(j)
    {
        return Ok(value);
    }
    if let Val::Obj(o) = &v
        && let Kind::Promise(state) = &o.0.kind
    {
        return match &*state.borrow() {
            PromiseState::Settled(true, value) => Ok(value.clone()),
            PromiseState::Settled(false, value) => Err(Exc(value.clone())),
            PromiseState::Pending(_) => Err(error("THREE.TSL port: awaited a pending promise")),
        };
    }
    Ok(v)
}

/// The result of an async function: its completion as a settled promise.
pub fn async_result(r: R<Val>) -> R<Val> {
    #[cfg(not(target_arch = "wasm32"))]
    if let Err(e) = &r
        && std::env::var("TSL_DEBUG").is_ok()
    {
        eprintln!("async rejection: {}\n{}", error_message(&e.0), display(&get(&e.0, "stack").unwrap_or(Val::Undef)));
    }
    Ok(match r {
        Ok(v) => match await_val(v) {
            Ok(v) => promise(true, v),
            Err(e) => promise(false, e.0),
        },
        Err(e) => promise(false, e.0),
    })
}

fn install_promise() {
    let p = Obj::with(Some(proto("Object")), Kind::Plain);
    register_proto("Promise", p.clone());
    p.define(
        "then",
        func("then", 2, |this, a| {
            let Val::Obj(o) = this else { return Err(type_error("not a promise")) };
            let Kind::Promise(state) = &o.0.kind else { return Err(type_error("not a promise")) };
            let derived = pending_promise();
            let settled = match &*state.borrow() {
                PromiseState::Settled(ok, v) => Some((*ok, v.clone())),
                PromiseState::Pending(_) => None,
            };
            match settled {
                Some((ok, v)) => react(&arg(a, 0), &arg(a, 1), &derived, ok, &v)?,
                None => {
                    if let PromiseState::Pending(r) = &mut *state.borrow_mut() {
                        r.push((arg(a, 0), arg(a, 1), derived.clone()));
                    }
                }
            }
            Ok(derived)
        }),
        false,
    );
    p.define("catch", func("catch", 1, |this, a| super::m(this, "then", &[Val::Undef, arg(a, 0)])), false);
    p.define(
        "finally",
        func("finally", 1, |this, a| {
            call(&arg(a, 0), &Val::Undef, &[])?;
            Ok(this.clone())
        }),
        false,
    );
    let ctor = Obj::with(
        Some(proto("Function")),
        Kind::Func(Func {
            name: RefCell::new(Rc::from("Promise")),
            length: Cell::new(1),
            call: None,
            init: None,
            construct: Some(Rc::new(|_t, a| {
                let p = pending_promise();
                let (p1, p2) = (p.clone(), p.clone());
                let resolve = func("resolve", 1, move |_t, a| settle(&p1, true, arg(a, 0)).map(|_| Val::Undef));
                let reject = func("reject", 1, move |_t, a| settle(&p2, false, arg(a, 0)).map(|_| Val::Undef));
                if let Err(e) = call(&arg(a, 0), &Val::Undef, &[resolve, reject]) {
                    settle(&p, false, e.0)?;
                }
                Ok(p)
            })),
            forward: RefCell::new(None),
        }),
    );
    for (name, length, body) in vec![
        nf!("resolve", 1, |a| async_result(Ok(arg(a, 0)))),
        nf!("reject", 1, |a| Ok(promise(false, arg(a, 0)))),
        nf!("all", 1, |a| {
            let mut out = vec![];
            for v in iterate(&arg(a, 0))? {
                match await_val(v) {
                    Ok(v) => out.push(v),
                    Err(e) => return Ok(promise(false, e.0)),
                }
            }
            Ok(promise(true, array(out)))
        }),
    ] {
        ctor.define(name, func(name, length, move |t, a| body(t, a)), false);
    }
    ctor.define("prototype", p.clone(), false);
    p.define("constructor", ctor.clone(), false);
    define_global("Promise", Val::Obj(ctor));
}

thread_local! {
    static FRAMES: RefCell<Vec<(u32, Val)>> = const { RefCell::new(Vec::new()) };
    static NEXT_FRAME: std::cell::Cell<u32> = const { std::cell::Cell::new(1) };
}

/// Run the callbacks queued with requestAnimationFrame / setTimeout ( the
/// port drives frames itself; nothing runs on its own ).
pub fn flush_frames() -> R<()> {
    let queued = FRAMES.with(|f| std::mem::take(&mut *f.borrow_mut()));
    for (_, cb) in queued {
        call(&cb, &Val::Undef, &[Val::Num(now())])?;
    }
    Ok(())
}

fn install_timers(target: &Val) {
    let raf = func("requestAnimationFrame", 1, |_t, a| {
        // The editor's preview animates on the browser's frames.
        #[cfg(target_arch = "wasm32")]
        if interop::real_gpu() {
            return Ok(Val::Num(interop::request_frame(&arg(a, 0))));
        }
        let id = NEXT_FRAME.with(|n| {
            let id = n.get();
            n.set(id + 1);
            id
        });
        FRAMES.with(|f| f.borrow_mut().push((id, arg(a, 0))));
        Ok(Val::Num(id as f64))
    });
    let cancel = func("cancelAnimationFrame", 1, |_t, a| {
        #[cfg(target_arch = "wasm32")]
        if interop::real_gpu() {
            interop::cancel_frame(to_number(&arg(a, 0)));
            return Ok(Val::Undef);
        }
        let id = to_number(&arg(a, 0)) as u32;
        FRAMES.with(|f| f.borrow_mut().retain(|(i, _)| *i != id));
        Ok(Val::Undef)
    });
    let _ = set(target, "requestAnimationFrame", raf.clone());
    let _ = set(target, "cancelAnimationFrame", cancel.clone());
    let _ = set(target, "setTimeout", raf.clone());
    let _ = set(target, "clearTimeout", cancel.clone());
    // scheduler.yield(): yieldToMain resolves at once.
    let _ = set(target, "scheduler", object(&[("yield", func("yield", 0, |_t, _a| Ok(promise(true, Val::Undef))))]));
    let _ = set(target, "devicePixelRatio", Val::Num(1.0));
}

fn install_misc() {
    let json = native_obj(vec![
        nf!("stringify", 1, |a| Ok(Val::from(json_stringify(&arg(a, 0))?))),
        nf!("parse", 1, |a| {
            let text = to_string(&arg(a, 0))?;
            let mut p = JsonParser { s: text.as_bytes(), i: 0 };
            let v = p.value()?;
            p.ws();
            if p.i != p.s.len() {
                return Err(p.fail());
            }
            Ok(v)
        }),
    ]);
    define_global("JSON", Val::Obj(json));
    let symbol = native_obj(vec![]);
    symbol.define("iterator", ITERATOR, false);
    define_global("Symbol", Val::Obj(symbol));
    let reflect = native_obj(vec![
        nf!("get", 2, |a| {
            let target = arg(a, 0);
            if !matches!(target.type_of(), "object" | "function") || target.is_null() {
                return Err(type_error("Reflect.get called on non-object"));
            }
            let receiver = if a.len() > 2 { arg(a, 2) } else { target.clone() };
            get_with_receiver(&target, &key(&arg(a, 1))?, &receiver)
        }),
        nf!("set", 3, |a| {
            let target = arg(a, 0);
            if !matches!(target.type_of(), "object" | "function") || target.is_null() {
                return Err(type_error("Reflect.set called on non-object"));
            }
            let receiver = if a.len() > 3 { arg(a, 3) } else { target.clone() };
            Ok(Val::Bool(set_with_receiver(&target, &key(&arg(a, 1))?, arg(a, 2), &receiver)?))
        }),
        nf!("has", 2, |a| Ok(Val::Bool(has(&arg(a, 0), &key(&arg(a, 1))?)?))),
        nf!("apply", 3, |a| call(&arg(a, 0), &arg(a, 1), &iterate(&arg(a, 2))?)),
        nf!("construct", 2, |a| construct(&arg(a, 0), &iterate(&arg(a, 1))?)),
        nf!("ownKeys", 1, |a| Ok(match arg(a, 0) {
            Val::Obj(o) => array(o.own_keys().into_iter().map(Val::Str).collect()),
            _ => array(vec![]),
        })),
    ]);
    define_global("Reflect", Val::Obj(reflect));
    let proxy = Obj::with(
        Some(proto("Function")),
        Kind::Func(Func {
            name: RefCell::new(Rc::from("Proxy")),
            length: Cell::new(2),
            call: None,
            init: None,
            construct: Some(Rc::new(|_t, a| {
                let is_object = |v: &Val| matches!(v.type_of(), "object" | "function") && !v.is_null();
                if !is_object(&arg(a, 0)) || !is_object(&arg(a, 1)) {
                    return Err(type_error("Cannot create proxy with a non-object as target or handler"));
                }
                Ok(Val::Obj(Obj::with(None, Kind::Proxy(arg(a, 0), arg(a, 1)))))
            })),
            forward: RefCell::new(None),
        }),
    );
    define_global("Proxy", Val::Obj(proxy));
    let console = native_obj(vec![
        nf!("log", 0, |a| {
            super::console("log", a);
            Ok(Val::Undef)
        }),
        nf!("warn", 0, |a| {
            super::console("warn", a);
            Ok(Val::Undef)
        }),
        nf!("error", 0, |a| {
            super::console("error", a);
            Ok(Val::Undef)
        }),
        nf!("trace", 0, |_a| Ok(Val::Undef)),
    ]);
    define_global("console", Val::Obj(console));
    let performance = native_obj(vec![nf!("now", 0, |_a| Ok(Val::Num(now())))]);
    define_global("performance", Val::Obj(performance));
    // The reference browser's navigator ( tests/tsl/device.json ): the preferred
    // canvas format and the user agent the sources test.
    define_global(
        "navigator",
        object(&[
            ("userAgent", Val::from("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) HeadlessChrome/155.0.0.0 Safari/537.36")),
            (
                "gpu",
                object(&[
                    ("getPreferredCanvasFormat", func("getPreferredCanvasFormat", 0, |_t, _a| Ok(Val::from("bgra8unorm")))),
                    ("wgslLanguageFeatures", {
                        let set = new_set();
                        for f in WGSL_LANGUAGE_FEATURES {
                            let _ = super::m(&set, "add", &[Val::from(*f)]);
                        }
                        set
                    }),
                ]),
            ),
        ]),
    );
    define_global("structuredClone", func("structuredClone", 1, |_t, a| Ok(arg(a, 0))));
}

/// navigator.gpu.wgslLanguageFeatures of the reference browser.
const WGSL_LANGUAGE_FEATURES: &[&str] = &[
    "buffer_view",
    "immediate_address_space",
    "linear_indexing",
    "packed_4x8_integer_dot_product",
    "pointer_composite_access",
    "readonly_and_readwrite_storage_textures",
    "subgroup_id",
    "subgroup_uniformity",
    "swizzle_assignment",
    "texture_and_sampler_let",
    "texture_formats_tier1",
    "uniform_buffer_standard_layout",
    "unrestricted_pointer_parameters",
];

/// performance.now() in milliseconds.
pub fn now() -> f64 {
    #[cfg(target_arch = "wasm32")]
    {
        // The page's performance.now(), so a page's clock reaches the port.
        let p = interop::js_global("performance");
        js_sys::Reflect::get(&p, &wasm_bindgen::JsValue::from_str("now"))
            .ok()
            .and_then(|f| f.dyn_into::<js_sys::Function>().ok())
            .and_then(|f| f.call0(&p).ok())
            .and_then(|v| v.as_f64())
            .unwrap_or_else(js_sys::Date::now)
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_or(0.0, |d| d.as_secs_f64() * 1000.0)
    }
}

/// JSON.stringify for plain data.
pub fn json_stringify(v: &Val) -> R<String> {
    Ok(match v {
        Val::Undef => "undefined".into(),
        Val::Null => "null".into(),
        Val::Bool(b) => b.to_string(),
        Val::Num(n) => {
            if n.is_finite() {
                num::to_string(*n)
            } else {
                "null".into()
            }
        }
        Val::Str(text) => {
            let mut out = String::from("\"");
            for c in text.chars() {
                match c {
                    '"' => out.push_str("\\\""),
                    '\\' => out.push_str("\\\\"),
                    '\n' => out.push_str("\\n"),
                    '\r' => out.push_str("\\r"),
                    '\t' => out.push_str("\\t"),
                    c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
                    c => out.push(c),
                }
            }
            out.push('"');
            out
        }
        Val::Obj(o) if o.is_array() || matches!(o.0.kind, Kind::Typed(..)) => {
            let mut parts = vec![];
            for item in iterate(v)? {
                parts.push(if item.is_undef() || item.type_of() == "function" { "null".to_string() } else { json_stringify(&item)? });
            }
            format!("[{}]", parts.join(","))
        }
        Val::Obj(_) => {
            let to_json = get(v, "toJSON")?;
            if to_json.type_of() == "function" {
                return json_stringify(&call(&to_json, v, &[])?);
            }
            let mut parts = vec![];
            for k in keys(v) {
                let item = get(v, &k)?;
                if item.is_undef() || item.type_of() == "function" {
                    continue;
                }
                parts.push(format!("{}:{}", json_stringify(&Val::Str(k.clone()))?, json_stringify(&item)?));
            }
            format!("{{{}}}", parts.join(","))
        }
        #[cfg(target_arch = "wasm32")]
        Val::Js(j) => interop::json_stringify(j),
    })
}

/// A global by name: the port's, then the embedding JavaScript's.
pub fn glob(name: &str) -> Val {
    // With a real GPU the browser's navigator answers ( features, preferred format ).
    #[cfg(target_arch = "wasm32")]
    if name == "navigator" && interop::real_gpu() {
        return interop::global(name);
    }
    let v = global(name);
    if !v.is_undef() || name == "undefined" {
        return v;
    }
    // Properties the port's modules put on globalThis.
    if let Val::Obj(g) = global("globalThis")
        && g.has_own(name)
    {
        return get(&Val::Obj(g), name).unwrap_or(Val::Undef);
    }
    #[cfg(target_arch = "wasm32")]
    {
        interop::global(name)
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        Val::Undef
    }
}

pub(crate) fn install_globals() {
    install_math();
    install_number();
    install_object();
    install_collection_ctors();
    install_errors();
    install_regexp();
    install_promise();
    install_misc();
    let global_this = object(&[]);
    install_timers(&global_this);
    for name in ["requestAnimationFrame", "cancelAnimationFrame", "setTimeout", "clearTimeout"] {
        define_global(name, get(&global_this, name).unwrap_or(Val::Undef));
    }
    define_global("globalThis", global_this.clone());
    define_global("self", global_this.clone());
    define_global("window", global_this);
}

struct JsonParser<'a> {
    s: &'a [u8],
    i: usize,
}

impl JsonParser<'_> {
    fn fail(&self) -> Exc {
        Exc(make_error("SyntaxError", &format!("Unexpected token in JSON at position {}", self.i)))
    }
    fn ws(&mut self) {
        while self.i < self.s.len() && matches!(self.s[self.i], b' ' | b'\t' | b'\n' | b'\r') {
            self.i += 1;
        }
    }
    fn lit(&mut self, word: &str, v: Val) -> R<Val> {
        if self.s[self.i..].starts_with(word.as_bytes()) {
            self.i += word.len();
            Ok(v)
        } else {
            Err(self.fail())
        }
    }
    fn value(&mut self) -> R<Val> {
        self.ws();
        match self.s.get(self.i) {
            Some(b'{') => {
                self.i += 1;
                let o = object(&[]);
                self.ws();
                if self.s.get(self.i) == Some(&b'}') {
                    self.i += 1;
                    return Ok(o);
                }
                loop {
                    self.ws();
                    let k = self.string()?;
                    self.ws();
                    if self.s.get(self.i) != Some(&b':') {
                        return Err(self.fail());
                    }
                    self.i += 1;
                    let v = self.value()?;
                    set(&o, &k, v)?;
                    self.ws();
                    match self.s.get(self.i) {
                        Some(b',') => self.i += 1,
                        Some(b'}') => {
                            self.i += 1;
                            return Ok(o);
                        }
                        _ => return Err(self.fail()),
                    }
                }
            }
            Some(b'[') => {
                self.i += 1;
                let mut items = vec![];
                self.ws();
                if self.s.get(self.i) == Some(&b']') {
                    self.i += 1;
                    return Ok(array(items));
                }
                loop {
                    items.push(self.value()?);
                    self.ws();
                    match self.s.get(self.i) {
                        Some(b',') => self.i += 1,
                        Some(b']') => {
                            self.i += 1;
                            return Ok(array(items));
                        }
                        _ => return Err(self.fail()),
                    }
                }
            }
            Some(b'"') => Ok(Val::from(self.string()?)),
            Some(b't') => self.lit("true", Val::Bool(true)),
            Some(b'f') => self.lit("false", Val::Bool(false)),
            Some(b'n') => self.lit("null", Val::Null),
            Some(c) if *c == b'-' || c.is_ascii_digit() => {
                let start = self.i;
                while self.i < self.s.len() && matches!(self.s[self.i], b'-' | b'+' | b'.' | b'e' | b'E' | b'0'..=b'9') {
                    self.i += 1;
                }
                let t = std::str::from_utf8(&self.s[start..self.i]).unwrap();
                t.parse::<f64>().map(Val::Num).map_err(|_| self.fail())
            }
            _ => Err(self.fail()),
        }
    }
    fn string(&mut self) -> R<String> {
        if self.s.get(self.i) != Some(&b'"') {
            return Err(self.fail());
        }
        self.i += 1;
        let mut out: Vec<u16> = vec![];
        loop {
            let Some(&c) = self.s.get(self.i) else { return Err(self.fail()) };
            match c {
                b'"' => {
                    self.i += 1;
                    return Ok(String::from_utf16_lossy(&out));
                }
                b'\\' => {
                    let e = *self.s.get(self.i + 1).ok_or_else(|| self.fail())?;
                    self.i += 2;
                    match e {
                        b'n' => out.push(10),
                        b't' => out.push(9),
                        b'r' => out.push(13),
                        b'b' => out.push(8),
                        b'f' => out.push(12),
                        b'u' => {
                            let h = std::str::from_utf8(&self.s[self.i..self.i + 4]).map_err(|_| self.fail())?;
                            out.push(u16::from_str_radix(h, 16).map_err(|_| self.fail())?);
                            self.i += 4;
                        }
                        c => out.push(c as u16),
                    }
                }
                _ => {
                    // A UTF-8 sequence.
                    let len = match c {
                        0..=0x7f => 1,
                        0xc0..=0xdf => 2,
                        0xe0..=0xef => 3,
                        _ => 4,
                    };
                    let ch = std::str::from_utf8(&self.s[self.i..self.i + len]).map_err(|_| self.fail())?;
                    out.extend(ch.encode_utf16());
                    self.i += len;
                }
            }
        }
    }
}
