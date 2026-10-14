//! The built-in prototypes the sources and their callers use: Object,
//! Function, Array, String, Number, Boolean, Error, Map, Set and WeakMap,
//! plus helpers for the regular expressions the sources write as literals.
use super::*;

fn this_obj(this: &Val) -> R<Obj> {
    match this {
        Val::Obj(o) => Ok(o.clone()),
        _ => Err(type_error("Method called on incompatible receiver")),
    }
}

fn this_array(this: &Val) -> R<Obj> {
    let o = this_obj(this)?;
    if o.array().is_none() {
        return Err(type_error("Array.prototype method called on non-array"));
    }
    Ok(o)
}

fn this_str(this: &Val) -> R<Rc<str>> {
    match this {
        Val::Str(s) => Ok(s.clone()),
        v => to_string(v),
    }
}

/// A relative index ( slice, splice, at ): negative counts from the end.
fn rel_index(v: &Val, len: usize, default: usize) -> usize {
    if v.is_undef() {
        return default;
    }
    let n = to_number(v);
    let n = if n.is_nan() { 0.0 } else { n.trunc() };
    if n < 0.0 {
        (len as f64 + n).max(0.0) as usize
    } else {
        (n as usize).min(len)
    }
}

/// The comparison `sort` uses: the comparator's sign, or string order.
fn sort_compare(cmp: &Val, a: &Val, b: &Val) -> R<std::cmp::Ordering> {
    use std::cmp::Ordering;
    match (a.is_undef(), b.is_undef()) {
        (true, true) => return Ok(Ordering::Equal),
        (true, false) => return Ok(Ordering::Greater),
        (false, true) => return Ok(Ordering::Less),
        _ => {}
    }
    if cmp.is_undef() {
        let (x, y) = (to_string(a)?, to_string(b)?);
        let (x, y): (Vec<u16>, Vec<u16>) = (x.encode_utf16().collect(), y.encode_utf16().collect());
        return Ok(x.cmp(&y));
    }
    let r = to_number(&call(cmp, &Val::Undef, &[a.clone(), b.clone()])?);
    Ok(if r < 0.0 {
        Ordering::Less
    } else if r > 0.0 {
        Ordering::Greater
    } else {
        Ordering::Equal
    })
}

/// A stable merge sort that propagates the comparator's exceptions ( V8's TimSort is stable ).
fn merge_sort(items: Vec<Val>, cmp: &Val) -> R<Vec<Val>> {
    if items.len() <= 1 {
        return Ok(items);
    }
    let mid = items.len() / 2;
    let mut right = items;
    let left = right.drain(..mid).collect::<Vec<_>>();
    let (left, right) = (merge_sort(left, cmp)?, merge_sort(right, cmp)?);
    let mut out = Vec::with_capacity(left.len() + right.len());
    let (mut i, mut j) = (0, 0);
    while i < left.len() && j < right.len() {
        if sort_compare(cmp, &right[j], &left[i])? == std::cmp::Ordering::Less {
            out.push(right[j].clone());
            j += 1;
        } else {
            out.push(left[i].clone());
            i += 1;
        }
    }
    out.extend_from_slice(&left[i..]);
    out.extend_from_slice(&right[j..]);
    Ok(out)
}

/// `Array.prototype.join`.
pub fn join(items: &[Val], sep: &str) -> R<String> {
    let mut out = String::new();
    for (i, v) in items.iter().enumerate() {
        if i > 0 {
            out.push_str(sep);
        }
        if !v.is_nullish() {
            out.push_str(&to_string(v)?);
        }
    }
    Ok(out)
}

fn define_methods(o: &Obj, methods: Vec<(&str, u32, NativeFn)>) {
    for (name, length, body) in methods {
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
}

macro_rules! native {
    ($name:expr, $len:expr, |$this:ident, $args:ident| $body:expr) => {
        (
            $name,
            $len,
            Rc::new(move |$this: &Val, $args: &[Val]| -> R<Val> { $body }) as NativeFn,
        )
    };
}

fn callback_args(v: &Val, i: usize, this: &Val) -> [Val; 3] {
    [v.clone(), Val::Num(i as f64), this.clone()]
}

pub(crate) fn install() {
    let object = Obj::with(None, Kind::Plain);
    register_proto("Object", object.clone());
    let function = Obj::with(Some(object.clone()), Kind::Plain);
    register_proto("Function", function.clone());
    for (name, parent) in [
        ("Array", "Object"),
        ("String", "Object"),
        ("Number", "Object"),
        ("Boolean", "Object"),
        ("Error", "Object"),
        ("Map", "Object"),
        ("Set", "Object"),
        ("WeakMap", "Object"),
    ] {
        register_proto(name, Obj::with(Some(proto(parent)), Kind::Plain));
    }
    for kind in ["TypeError", "RangeError", "SyntaxError"] {
        let p = Obj::with(Some(proto("Error")), Kind::Plain);
        p.define("name", kind, false);
        register_proto(kind, p);
    }
    define_methods(
        &object,
        vec![
            native!("toString", 0, |this, _a| {
                Ok(Val::from(match this {
                    Val::Obj(o) if o.is_array() => return Ok(Val::from(join(&o.items(), ",")?)),
                    Val::Obj(o) if o.is_func() => {
                        format!("function {}() {{ [native code] }}", o.func().unwrap().name.borrow())
                    }
                    Val::Obj(o) if matches!(o.0.kind, Kind::Error) => {
                        let name = display(&get(this, "name")?);
                        let message = display(&get(this, "message")?);
                        if message.is_empty() { name } else { format!("{name}: {message}") }
                    }
                    _ => "[object Object]".to_string(),
                }))
            }),
            native!("hasOwnProperty", 1, |this, a| {
                let key = to_string(&arg(a, 0))?;
                Ok(Val::Bool(this_obj(this)?.has_own(&key)))
            }),
            native!("valueOf", 0, |this, _a| Ok(this.clone())),
        ],
    );
    let error = proto("Error");
    error.define("name", "Error", false);
    error.define("message", "", false);
    define_methods(
        &proto("Function"),
        vec![
            native!("call", 1, |this, a| {
                call(this, &arg(a, 0), if a.is_empty() { &[] } else { &a[1..] })
            }),
            native!("apply", 2, |this, a| {
                let args = match arg(a, 1) {
                    Val::Undef | Val::Null => vec![],
                    v => iterate(&v)?,
                };
                call(this, &arg(a, 0), &args)
            }),
            native!("bind", 1, |this, a| {
                let target = this.clone();
                let bound_this = arg(a, 0);
                let bound: Vec<Val> = a.iter().skip(1).cloned().collect();
                let length = to_number(&get(this, "length")?) as u32;
                let name = display(&get(this, "name")?);
                Ok(func(
                    &format!("bound {name}"),
                    length.saturating_sub(bound.len() as u32),
                    move |_this, args| {
                        let mut all = bound.clone();
                        all.extend_from_slice(args);
                        call(&target, &bound_this, &all)
                    },
                ))
            }),
        ],
    );
    install_array();
    install_string();
    define_methods(
        &proto("Number"),
        vec![
            native!("toString", 1, |this, a| {
                let n = to_number(this);
                let radix = arg(a, 0);
                if radix.is_undef() || to_number(&radix) == 10.0 {
                    return Ok(Val::from(num::to_string(n)));
                }
                let radix = to_number(&radix) as u32;
                Ok(Val::from(int_radix(n as i64, radix)))
            }),
            native!("toFixed", 1, |this, a| {
                let digits = to_number(&arg(a, 0)) as usize;
                Ok(Val::from(format!("{:.*}", digits, to_number(this))))
            }),
            native!("valueOf", 0, |this, _a| Ok(this.clone())),
        ],
    );
    define_methods(
        &proto("Boolean"),
        vec![native!("toString", 0, |this, _a| Ok(Val::from(display(this))))],
    );
    install_collections();
    install_typed();
    super::globals::install_globals();
}

thread_local! {
    static GLOBALS: RefCell<HashMap<&'static str, Val>> = RefCell::new(HashMap::new());
}

pub(crate) fn define_global(name: &'static str, v: impl Into<Val>) {
    GLOBALS.with(|g| g.borrow_mut().insert(name, v.into()));
}

/// A JavaScript global constructor the ported code compares or constructs ( `Float32Array`, `Array` ).
pub fn global(name: &str) -> Val {
    proto("Object");
    GLOBALS.with(|g| g.borrow().get(name).cloned()).unwrap_or(Val::Undef)
}

fn this_typed(this: &Val) -> R<(TypedKind, Obj)> {
    match this {
        Val::Obj(o) => match &o.0.kind {
            Kind::Typed(k, _) => Ok((*k, o.clone())),
            _ => Err(type_error("this is not a typed array.")),
        },
        _ => Err(type_error("this is not a typed array.")),
    }
}

fn typed_values(o: &Obj) -> Vec<f64> {
    match &o.0.kind {
        Kind::Typed(k, v) => v.values(*k),
        _ => unreachable!(),
    }
}

fn install_typed() {
    for kind in TypedKind::ALL {
        let p = Obj::with(Some(proto("Object")), Kind::Plain);
        register_proto(kind.name(), p.clone());
        define_methods(
            &p,
            vec![
                native!("set", 2, |this, a| {
                    let (k, o) = this_typed(this)?;
                    let src = numbers(&arg(a, 0))?;
                    let offset = to_number(&arg(a, 1)) as usize;
                    let Kind::Typed(_, view) = &o.0.kind else { unreachable!() };
                    if offset + src.len() > view.len {
                        return Err(Exc(make_error("RangeError", "offset is out of bounds")));
                    }
                    for (i, v) in src.into_iter().enumerate() {
                        view.set(k, offset + i, v);
                    }
                    Ok(Val::Undef)
                }),
                native!("subarray", 2, |this, a| {
                    let (k, o) = this_typed(this)?;
                    let Kind::Typed(_, view) = &o.0.kind else { unreachable!() };
                    let start = rel_index(&arg(a, 0), view.len, 0);
                    let end = rel_index(&arg(a, 1), view.len, view.len);
                    typed_view(k, &view.buffer, view.offset + start * k.bytes(), Some(end.saturating_sub(start)))
                }),
                native!("slice", 2, |this, a| {
                    let (k, o) = this_typed(this)?;
                    let values = typed_values(&o);
                    let start = rel_index(&arg(a, 0), values.len(), 0);
                    let end = rel_index(&arg(a, 1), values.len(), values.len());
                    Ok(typed(k, if start < end { values[start..end].to_vec() } else { vec![] }))
                }),
                native!("fill", 1, |this, a| {
                    let (k, o) = this_typed(this)?;
                    let v = to_number(&arg(a, 0));
                    let Kind::Typed(_, view) = &o.0.kind else { unreachable!() };
                    let len = view.len;
                    let start = rel_index(&arg(a, 1), len, 0);
                    let end = rel_index(&arg(a, 2), len, len);
                    for i in start.min(end)..end {
                        view.set(k, i, v);
                    }
                    Ok(this.clone())
                }),
                native!("indexOf", 1, |this, a| {
                    let (_, o) = this_typed(this)?;
                    let x = to_number(&arg(a, 0));
                    Ok(Val::Num(typed_values(&o).iter().position(|v| *v == x).map_or(-1.0, |i| i as f64)))
                }),
                native!("includes", 1, |this, a| {
                    let (_, o) = this_typed(this)?;
                    let x = to_number(&arg(a, 0));
                    Ok(Val::Bool(typed_values(&o).iter().any(|v| *v == x || (v.is_nan() && x.is_nan()))))
                }),
                native!("join", 1, |this, a| {
                    let (_, o) = this_typed(this)?;
                    let sep = match arg(a, 0) {
                        Val::Undef => ",".to_string(),
                        v => to_string(&v)?.to_string(),
                    };
                    let items: Vec<Val> = typed_values(&o).iter().map(|v| Val::Num(*v)).collect();
                    Ok(Val::from(join(&items, &sep)?))
                }),
                native!("toString", 0, |this, _a| {
                    let (_, o) = this_typed(this)?;
                    let items: Vec<Val> = typed_values(&o).iter().map(|v| Val::Num(*v)).collect();
                    Ok(Val::from(join(&items, ",")?))
                }),
                native!("forEach", 1, |this, a| {
                    let (_, o) = this_typed(this)?;
                    let values = typed_values(&o);
                    for (i, v) in values.into_iter().enumerate() {
                        call(&arg(a, 0), &arg(a, 1), &[Val::Num(v), Val::Num(i as f64), this.clone()])?;
                    }
                    Ok(Val::Undef)
                }),
                native!("map", 1, |this, a| {
                    let (k, o) = this_typed(this)?;
                    let values = typed_values(&o);
                    let mut out = vec![];
                    for (i, v) in values.into_iter().enumerate() {
                        out.push(to_number(&call(&arg(a, 0), &arg(a, 1), &[Val::Num(v), Val::Num(i as f64), this.clone()])?));
                    }
                    Ok(typed(k, out))
                }),
            ],
        );
        let ctor = Obj::with(
            Some(proto("Function")),
            Kind::Func(Func {
                name: RefCell::new(Rc::from(kind.name())),
                length: Cell::new(3),
                call: None,
                init: None,
                construct: Some(Rc::new(move |_this, a| {
                    let first = arg(a, 0);
                    if let Val::Obj(b) = &first
                        && matches!(b.0.kind, Kind::ArrayBuffer(_))
                    {
                        let offset = arg(a, 1);
                        let len = arg(a, 2);
                        return typed_view(
                            kind,
                            b,
                            if offset.is_undef() { 0 } else { to_number(&offset) as usize },
                            if len.is_undef() { None } else { Some(to_number(&len) as usize) },
                        );
                    }
                    let values = match &first {
                        Val::Undef => vec![],
                        Val::Num(n) => vec![0.0; *n as usize],
                        v => numbers(v)?,
                    };
                    Ok(typed(kind, values))
                })),
                forward: RefCell::new(None),
            }),
        );
        ctor.define("prototype", p.clone(), false);
        ctor.define("BYTES_PER_ELEMENT", Val::Num(kind.bytes() as f64), false);
        p.define("constructor", ctor.clone(), false);
        GLOBALS.with(|g| g.borrow_mut().insert(kind.name(), Val::Obj(ctor)));
    }
    let buffer_proto = Obj::with(Some(proto("Object")), Kind::Plain);
    register_proto("ArrayBuffer", buffer_proto.clone());
    define_methods(
        &buffer_proto,
        vec![native!("slice", 2, |this, a| {
            let o = this_obj(this)?;
            let Kind::ArrayBuffer(bytes) = &o.0.kind else { return Err(type_error("not an ArrayBuffer")) };
            let b = bytes.borrow();
            let start = rel_index(&arg(a, 0), b.len(), 0);
            let end = rel_index(&arg(a, 1), b.len(), b.len());
            let (n, nb) = array_buffer(end.saturating_sub(start));
            nb.borrow_mut().copy_from_slice(&b[start..end.max(start)]);
            Ok(Val::Obj(n))
        })],
    );
    let buffer_ctor = Obj::with(
        Some(proto("Function")),
        Kind::Func(Func {
            name: RefCell::new(Rc::from("ArrayBuffer")),
            length: Cell::new(1),
            call: None,
            init: None,
            construct: Some(Rc::new(|_this, a| Ok(Val::Obj(array_buffer(to_number(&arg(a, 0)) as usize).0)))),
            forward: RefCell::new(None),
        }),
    );
    buffer_ctor.define("prototype", buffer_proto.clone(), false);
    buffer_ctor.define(
        "isView",
        func("isView", 1, |_t, a| Ok(Val::Bool(matches!(arg(a, 0), Val::Obj(o) if matches!(o.0.kind, Kind::Typed(..)))))),
        false,
    );
    buffer_proto.define("constructor", buffer_ctor.clone(), false);
    GLOBALS.with(|g| g.borrow_mut().insert("ArrayBuffer", Val::Obj(buffer_ctor)));
    let array_ctor = Obj::with(
        Some(proto("Function")),
        Kind::Func(Func {
            name: RefCell::new(Rc::from("Array")),
            length: Cell::new(1),
            call: None,
            init: None,
            construct: Some(Rc::new(|_this, a| {
                Ok(match a {
                    [Val::Num(n)] => array(vec![Val::Undef; *n as usize]),
                    items => array(items.to_vec()),
                })
            })),
            forward: RefCell::new(None),
        }),
    );
    array_ctor.define("prototype", proto("Array"), false);
    proto("Array").define("constructor", array_ctor.clone(), false);
    GLOBALS.with(|g| g.borrow_mut().insert("Array", Val::Obj(array_ctor)));
    let object_ctor = func("Object", 1, |_this, a| Ok(arg(a, 0)));
    if let Val::Obj(o) = &object_ctor {
        o.define("prototype", proto("Object"), false);
        proto("Object").define("constructor", o.clone(), false);
    }
    GLOBALS.with(|g| g.borrow_mut().insert("Object", object_ctor));
}

fn int_radix(mut n: i64, radix: u32) -> String {
    if n == 0 {
        return "0".into();
    }
    let neg = n < 0;
    n = n.abs();
    let mut out = vec![];
    while n > 0 {
        out.push(std::char::from_digit((n % radix as i64) as u32, radix).unwrap());
        n /= radix as i64;
    }
    if neg {
        out.push('-');
    }
    out.iter().rev().collect()
}

fn install_array() {
    define_methods(
        &proto("Array"),
        vec![
            native!("push", 1, |this, a| {
                let o = this_array(this)?;
                let mut v = o.array().unwrap().borrow_mut();
                v.extend_from_slice(a);
                Ok(Val::Num(v.len() as f64))
            }),
            native!("pop", 0, |this, _a| {
                let o = this_array(this)?;
                Ok(o.array().unwrap().borrow_mut().pop().unwrap_or(Val::Undef))
            }),
            native!("shift", 0, |this, _a| {
                let o = this_array(this)?;
                let mut v = o.array().unwrap().borrow_mut();
                Ok(if v.is_empty() { Val::Undef } else { v.remove(0) })
            }),
            native!("unshift", 1, |this, a| {
                let o = this_array(this)?;
                let mut v = o.array().unwrap().borrow_mut();
                for (i, x) in a.iter().enumerate() {
                    v.insert(i, x.clone());
                }
                Ok(Val::Num(v.len() as f64))
            }),
            native!("slice", 2, |this, a| {
                let items = this_array(this)?.items();
                let start = rel_index(&arg(a, 0), items.len(), 0);
                let end = rel_index(&arg(a, 1), items.len(), items.len());
                Ok(array(if start < end { items[start..end].to_vec() } else { vec![] }))
            }),
            native!("splice", 2, |this, a| {
                let o = this_array(this)?;
                let mut v = o.array().unwrap().borrow_mut();
                let len = v.len();
                let start = rel_index(&arg(a, 0), len, 0);
                let count = if a.len() < 2 {
                    len - start
                } else {
                    (to_number(&a[1]).max(0.0) as usize).min(len - start)
                };
                let removed: Vec<Val> = v.splice(start..start + count, a.iter().skip(2).cloned()).collect();
                Ok(array(removed))
            }),
            native!("concat", 1, |this, a| {
                let mut items = this_array(this)?.items();
                for x in a {
                    if is_array(x) {
                        items.extend(iterate(x)?);
                    } else {
                        items.push(x.clone());
                    }
                }
                Ok(array(items))
            }),
            native!("join", 1, |this, a| {
                let sep = match arg(a, 0) {
                    Val::Undef => ",".to_string(),
                    v => to_string(&v)?.to_string(),
                };
                Ok(Val::from(join(&this_array(this)?.items(), &sep)?))
            }),
            native!("indexOf", 1, |this, a| {
                let items = this_array(this)?.items();
                let x = arg(a, 0);
                Ok(Val::Num(items.iter().position(|v| v.strict_eq(&x)).map_or(-1.0, |i| i as f64)))
            }),
            native!("lastIndexOf", 1, |this, a| {
                let items = this_array(this)?.items();
                let x = arg(a, 0);
                Ok(Val::Num(items.iter().rposition(|v| v.strict_eq(&x)).map_or(-1.0, |i| i as f64)))
            }),
            native!("includes", 1, |this, a| {
                let items = this_array(this)?.items();
                let x = arg(a, 0);
                let nan = matches!(x, Val::Num(n) if n.is_nan());
                Ok(Val::Bool(items.iter().any(|v| {
                    v.strict_eq(&x) || (nan && matches!(v, Val::Num(n) if n.is_nan()))
                })))
            }),
            native!("map", 1, |this, a| {
                let f = arg(a, 0);
                let items = this_array(this)?.items();
                let mut out = Vec::with_capacity(items.len());
                for (i, v) in items.iter().enumerate() {
                    out.push(call(&f, &arg(a, 1), &callback_args(v, i, this))?);
                }
                Ok(array(out))
            }),
            native!("filter", 1, |this, a| {
                let f = arg(a, 0);
                let items = this_array(this)?.items();
                let mut out = vec![];
                for (i, v) in items.iter().enumerate() {
                    if call(&f, &arg(a, 1), &callback_args(v, i, this))?.truthy() {
                        out.push(v.clone());
                    }
                }
                Ok(array(out))
            }),
            native!("forEach", 1, |this, a| {
                let f = arg(a, 0);
                for (i, v) in this_array(this)?.items().iter().enumerate() {
                    call(&f, &arg(a, 1), &callback_args(v, i, this))?;
                }
                Ok(Val::Undef)
            }),
            native!("some", 1, |this, a| {
                let f = arg(a, 0);
                for (i, v) in this_array(this)?.items().iter().enumerate() {
                    if call(&f, &arg(a, 1), &callback_args(v, i, this))?.truthy() {
                        return Ok(Val::Bool(true));
                    }
                }
                Ok(Val::Bool(false))
            }),
            native!("every", 1, |this, a| {
                let f = arg(a, 0);
                for (i, v) in this_array(this)?.items().iter().enumerate() {
                    if !call(&f, &arg(a, 1), &callback_args(v, i, this))?.truthy() {
                        return Ok(Val::Bool(false));
                    }
                }
                Ok(Val::Bool(true))
            }),
            native!("find", 1, |this, a| {
                let f = arg(a, 0);
                for (i, v) in this_array(this)?.items().iter().enumerate() {
                    if call(&f, &arg(a, 1), &callback_args(v, i, this))?.truthy() {
                        return Ok(v.clone());
                    }
                }
                Ok(Val::Undef)
            }),
            native!("findIndex", 1, |this, a| {
                let f = arg(a, 0);
                for (i, v) in this_array(this)?.items().iter().enumerate() {
                    if call(&f, &arg(a, 1), &callback_args(v, i, this))?.truthy() {
                        return Ok(Val::Num(i as f64));
                    }
                }
                Ok(Val::Num(-1.0))
            }),
            native!("reduce", 1, |this, a| {
                let f = arg(a, 0);
                let items = this_array(this)?.items();
                let mut iter = items.iter().enumerate();
                let mut acc = if a.len() > 1 {
                    a[1].clone()
                } else {
                    match iter.next() {
                        Some((_, v)) => v.clone(),
                        None => return Err(type_error("Reduce of empty array with no initial value")),
                    }
                };
                for (i, v) in iter {
                    acc = call(&f, &Val::Undef, &[acc, v.clone(), Val::Num(i as f64), this.clone()])?;
                }
                Ok(acc)
            }),
            native!("sort", 1, |this, a| {
                let o = this_array(this)?;
                let sorted = merge_sort(o.items(), &arg(a, 0))?;
                *o.array().unwrap().borrow_mut() = sorted;
                Ok(this.clone())
            }),
            native!("reverse", 0, |this, _a| {
                let o = this_array(this)?;
                o.array().unwrap().borrow_mut().reverse();
                Ok(this.clone())
            }),
            native!("fill", 1, |this, a| {
                let o = this_array(this)?;
                let len = o.array().unwrap().borrow().len();
                let start = rel_index(&arg(a, 1), len, 0);
                let end = rel_index(&arg(a, 2), len, len);
                for x in o.array().unwrap().borrow_mut()[start.min(end)..end].iter_mut() {
                    *x = arg(a, 0);
                }
                Ok(this.clone())
            }),
            native!("at", 1, |this, a| {
                let items = this_array(this)?.items();
                let n = to_number(&arg(a, 0)).trunc();
                let i = if n < 0.0 { items.len() as f64 + n } else { n };
                Ok(if i < 0.0 { Val::Undef } else { items.get(i as usize).cloned().unwrap_or(Val::Undef) })
            }),
            native!("flat", 0, |this, _a| {
                let mut out = vec![];
                for v in this_array(this)?.items() {
                    if is_array(&v) {
                        out.extend(iterate(&v)?);
                    } else {
                        out.push(v);
                    }
                }
                Ok(array(out))
            }),
            native!("toString", 0, |this, _a| Ok(Val::from(join(&this_array(this)?.items(), ",")?))),
        ],
    );
}

/// UTF-16 code units of a string, for JavaScript's indices.
fn units(s: &str) -> Vec<u16> {
    s.encode_utf16().collect()
}
fn from_units(u: &[u16]) -> Val {
    Val::from(String::from_utf16_lossy(u))
}

fn install_string() {
    define_methods(
        &proto("String"),
        vec![
            native!("toString", 0, |this, _a| Ok(Val::Str(this_str(this)?))),
            native!("valueOf", 0, |this, _a| Ok(Val::Str(this_str(this)?))),
            native!("slice", 2, |this, a| {
                let u = units(&this_str(this)?);
                let start = rel_index(&arg(a, 0), u.len(), 0);
                let end = rel_index(&arg(a, 1), u.len(), u.len());
                Ok(if start < end { from_units(&u[start..end]) } else { s("") })
            }),
            native!("substring", 2, |this, a| {
                let u = units(&this_str(this)?);
                let clamp = |v: &Val, d: usize| -> usize {
                    if v.is_undef() {
                        return d;
                    }
                    let n = to_number(v);
                    if n.is_nan() || n < 0.0 { 0 } else { (n as usize).min(u.len()) }
                };
                let (x, y) = (clamp(&arg(a, 0), 0), clamp(&arg(a, 1), u.len()));
                let (x, y) = if x > y { (y, x) } else { (x, y) };
                Ok(from_units(&u[x..y]))
            }),
            native!("charAt", 1, |this, a| {
                let u = units(&this_str(this)?);
                let i = to_number(&arg(a, 0));
                let i = if i.is_nan() { 0 } else { i as i64 };
                Ok(if i < 0 || i as usize >= u.len() { s("") } else { from_units(&u[i as usize..i as usize + 1]) })
            }),
            native!("charCodeAt", 1, |this, a| {
                let u = units(&this_str(this)?);
                let i = to_number(&arg(a, 0));
                let i = if i.is_nan() { 0 } else { i as i64 };
                Ok(if i < 0 || i as usize >= u.len() { Val::Num(f64::NAN) } else { Val::Num(u[i as usize] as f64) })
            }),
            native!("startsWith", 1, |this, a| {
                Ok(Val::Bool(this_str(this)?.starts_with(&*to_string(&arg(a, 0))?)))
            }),
            native!("endsWith", 1, |this, a| {
                Ok(Val::Bool(this_str(this)?.ends_with(&*to_string(&arg(a, 0))?)))
            }),
            native!("includes", 1, |this, a| {
                Ok(Val::Bool(this_str(this)?.contains(&*to_string(&arg(a, 0))?)))
            }),
            native!("indexOf", 1, |this, a| {
                let text = this_str(this)?;
                let needle = to_string(&arg(a, 0))?;
                Ok(Val::Num(text.find(&*needle).map_or(-1.0, |b| text[..b].encode_utf16().count() as f64)))
            }),
            native!("toUpperCase", 0, |this, _a| Ok(Val::from(this_str(this)?.to_uppercase()))),
            native!("toLowerCase", 0, |this, _a| Ok(Val::from(this_str(this)?.to_lowercase()))),
            native!("trim", 0, |this, _a| {
                Ok(Val::from(this_str(this)?.trim_matches(|c: char| c.is_whitespace() || c == '\u{FEFF}').to_string()))
            }),
            native!("repeat", 1, |this, a| {
                let n = to_number(&arg(a, 0));
                if n < 0.0 || n.is_infinite() {
                    return Err(Exc(make_error("RangeError", &format!("Invalid count value: {}", num::to_string(n)))));
                }
                Ok(Val::from(this_str(this)?.repeat(n as usize)))
            }),
            native!("padStart", 2, |this, a| {
                let text = this_str(this)?;
                let width = to_number(&arg(a, 0)) as usize;
                let fill = match arg(a, 1) {
                    Val::Undef => " ".to_string(),
                    v => to_string(&v)?.to_string(),
                };
                let len = text.encode_utf16().count();
                if len >= width || fill.is_empty() {
                    return Ok(Val::Str(text));
                }
                let pad: String = fill.chars().cycle().take(width - len).collect();
                Ok(Val::from(format!("{pad}{text}")))
            }),
            native!("concat", 1, |this, a| {
                let mut text = this_str(this)?.to_string();
                for x in a {
                    text.push_str(&to_string(x)?);
                }
                Ok(Val::from(text))
            }),
            native!("split", 2, |this, a| {
                let text = this_str(this)?;
                let sep = arg(a, 0);
                if sep.is_undef() {
                    return Ok(array(vec![Val::Str(text)]));
                }
                if let Some((r, _, _)) = regex_of(&sep) {
                    return Ok(array(r.split(&text).map(Val::from).collect()));
                }
                let sep = to_string(&sep)?;
                let parts: Vec<Val> = if sep.is_empty() {
                    text.encode_utf16().map(|u| from_units(&[u])).collect()
                } else {
                    text.split(&*sep).map(Val::from).collect()
                };
                Ok(array(parts))
            }),
            native!("replace", 2, |this, a| {
                let text = this_str(this)?;
                if regex_of(&arg(a, 0)).is_some() {
                    return Ok(Val::from(regexp_replace(&text, &arg(a, 0), &arg(a, 1))?));
                }
                let pattern = to_string(&arg(a, 0))?;
                let Some(at) = text.find(&*pattern) else {
                    return Ok(Val::Str(text));
                };
                let replacement = match arg(a, 1) {
                    f if f.type_of() == "function" => to_string(&call(&f, &Val::Undef, &[Val::Str(pattern.clone()), Val::Num(at as f64), Val::Str(text.clone())])?)?.to_string(),
                    v => expand_replacement(&to_string(&v)?, &pattern, &[], &text, at),
                };
                Ok(Val::from(format!("{}{replacement}{}", &text[..at], &text[at + pattern.len()..])))
            }),
            native!("match", 1, |this, a| {
                let text = this_str(this)?;
                let re = arg(a, 0);
                let Some((r, global, _)) = regex_of(&re) else {
                    return regexp_exec(&regexp(&regex::escape(&to_string(&re)?), ""), &text);
                };
                if !global {
                    return regexp_exec(&re, &text);
                }
                let all: Vec<Val> = r.find_iter(&text).map(|m| Val::from(m.as_str())).collect();
                Ok(if all.is_empty() { Val::Null } else { array(all) })
            }),
            native!("search", 1, |this, a| {
                let text = this_str(this)?;
                let Some((r, _, _)) = regex_of(&arg(a, 0)) else { return Ok(Val::Num(-1.0)) };
                Ok(Val::Num(r.find(&text).map_or(-1.0, |m| text[..m.start()].encode_utf16().count() as f64)))
            }),
            native!("replaceAll", 2, |this, a| {
                let text = this_str(this)?;
                if regex_of(&arg(a, 0)).is_some() {
                    return Ok(Val::from(regexp_replace(&text, &arg(a, 0), &arg(a, 1))?));
                }
                let pattern = to_string(&arg(a, 0))?;
                let replacement = to_string(&arg(a, 1))?;
                Ok(Val::from(text.replace(&*pattern, &replacement)))
            }),
        ],
    );
}

/// `$&`, `$1`… and `$$` in a replacement string.
pub(crate) fn expand_replacement(rep: &str, matched: &str, groups: &[Option<&str>], text: &str, at: usize) -> String {
    let mut out = String::new();
    let mut chars = rep.chars().peekable();
    while let Some(c) = chars.next() {
        if c != '$' {
            out.push(c);
            continue;
        }
        match chars.peek().copied() {
            Some('$') => {
                chars.next();
                out.push('$');
            }
            Some('&') => {
                chars.next();
                out.push_str(matched);
            }
            Some('`') => {
                chars.next();
                out.push_str(&text[..at]);
            }
            Some('\'') => {
                chars.next();
                out.push_str(&text[at + matched.len()..]);
            }
            Some(d) if d.is_ascii_digit() && d != '0' => {
                chars.next();
                let i = d.to_digit(10).unwrap() as usize;
                if let Some(g) = groups.get(i - 1) {
                    out.push_str(g.unwrap_or(""));
                } else {
                    out.push('$');
                    out.push(d);
                }
            }
            _ => out.push('$'),
        }
    }
    out
}

/// A regular expression literal ( `regex` syntax, which covers the sources' patterns ).
pub fn re(pattern: &str) -> regex::Regex {
    thread_local! {
        static CACHE: RefCell<HashMap<String, regex::Regex>> = RefCell::new(HashMap::new());
    }
    CACHE.with(|c| {
        c.borrow_mut()
            .entry(pattern.to_string())
            .or_insert_with(|| regex::Regex::new(pattern).expect("regular expression"))
            .clone()
    })
}

/// `/pattern/.test( text )`.
pub fn re_test(pattern: &str, text: &str) -> bool {
    re(pattern).is_match(text)
}

/// `text.replace( /pattern/flags, replacement )`; `global` is the `g` flag.
pub fn re_replace(text: &str, pattern: &str, global: bool, replacement: &str) -> String {
    let r = re(pattern);
    let mut out = String::new();
    let mut last = 0;
    for caps in r.captures_iter(text) {
        let m = caps.get(0).unwrap();
        out.push_str(&text[last..m.start()]);
        let groups: Vec<Option<&str>> = (1..caps.len()).map(|i| caps.get(i).map(|g| g.as_str())).collect();
        out.push_str(&expand_replacement(replacement, m.as_str(), &groups, text, m.start()));
        last = m.end();
        if !global {
            break;
        }
    }
    out.push_str(&text[last..]);
    out
}

/// `/pattern/.exec( text )`: the groups, `None` for no match.
pub fn re_exec(pattern: &str, text: &str) -> Option<Vec<Option<String>>> {
    re(pattern)
        .captures(text)
        .map(|c| (0..c.len()).map(|i| c.get(i).map(|g| g.as_str().to_string())).collect())
}

fn this_map(this: &Val) -> R<Obj> {
    let o = this_obj(this)?;
    match &o.0.kind {
        Kind::Map(_) | Kind::Set(_) | Kind::WeakMap(_) => Ok(o),
        _ => Err(type_error("Method called on incompatible receiver")),
    }
}

fn weak_key(v: &Val) -> R<usize> {
    match identity(v) {
        Some(k) if v.type_of() == "object" || v.type_of() == "function" => Ok(k),
        _ => Err(type_error("Invalid value used as weak map key")),
    }
}

fn install_collections() {
    let size = func("size", 0, |this, _a| {
        let o = this_map(this)?;
        Ok(Val::Num(match &o.0.kind {
            Kind::Map(m) | Kind::Set(m) => m.borrow().len() as f64,
            _ => 0.0,
        }))
    });
    proto("Map").define_accessor("size", Some(size.clone()), None, false);
    proto("Set").define_accessor("size", Some(size), None, false);
    define_methods(
        &proto("Map"),
        vec![
            native!("get", 1, |this, a| {
                let o = this_map(this)?;
                let Kind::Map(m) = &o.0.kind else { unreachable!() };
                Ok(m.borrow().get(&arg(a, 0)).cloned().unwrap_or(Val::Undef))
            }),
            native!("set", 2, |this, a| {
                let o = this_map(this)?;
                let Kind::Map(m) = &o.0.kind else { unreachable!() };
                m.borrow_mut().set(arg(a, 0), arg(a, 1));
                Ok(this.clone())
            }),
            native!("has", 1, |this, a| {
                let o = this_map(this)?;
                let Kind::Map(m) = &o.0.kind else { unreachable!() };
                Ok(Val::Bool(m.borrow().has(&arg(a, 0))))
            }),
            native!("delete", 1, |this, a| {
                let o = this_map(this)?;
                let Kind::Map(m) = &o.0.kind else { unreachable!() };
                Ok(Val::Bool(m.borrow_mut().delete(&arg(a, 0))))
            }),
            native!("clear", 0, |this, _a| {
                let o = this_map(this)?;
                let Kind::Map(m) = &o.0.kind else { unreachable!() };
                m.borrow_mut().clear();
                Ok(Val::Undef)
            }),
            native!("keys", 0, |this, _a| {
                let o = this_map(this)?;
                let Kind::Map(m) = &o.0.kind else { unreachable!() };
                Ok(array(m.borrow().pairs().into_iter().map(|(k, _)| k).collect()))
            }),
            native!("values", 0, |this, _a| {
                let o = this_map(this)?;
                let Kind::Map(m) = &o.0.kind else { unreachable!() };
                Ok(array(m.borrow().pairs().into_iter().map(|(_, v)| v).collect()))
            }),
            native!("entries", 0, |this, _a| {
                let o = this_map(this)?;
                let Kind::Map(m) = &o.0.kind else { unreachable!() };
                Ok(array(m.borrow().pairs().into_iter().map(|(k, v)| array(vec![k, v])).collect()))
            }),
            native!("forEach", 1, |this, a| {
                let o = this_map(this)?;
                let Kind::Map(m) = &o.0.kind else { unreachable!() };
                let pairs = m.borrow().pairs();
                for (k, v) in pairs {
                    call(&arg(a, 0), &arg(a, 1), &[v, k, this.clone()])?;
                }
                Ok(Val::Undef)
            }),
        ],
    );
    define_methods(
        &proto("Set"),
        vec![
            native!("add", 1, |this, a| {
                let o = this_map(this)?;
                let Kind::Set(m) = &o.0.kind else { unreachable!() };
                let k = arg(a, 0);
                if !m.borrow().has(&k) {
                    m.borrow_mut().set(k, Val::Undef);
                }
                Ok(this.clone())
            }),
            native!("has", 1, |this, a| {
                let o = this_map(this)?;
                let Kind::Set(m) = &o.0.kind else { unreachable!() };
                Ok(Val::Bool(m.borrow().has(&arg(a, 0))))
            }),
            native!("delete", 1, |this, a| {
                let o = this_map(this)?;
                let Kind::Set(m) = &o.0.kind else { unreachable!() };
                Ok(Val::Bool(m.borrow_mut().delete(&arg(a, 0))))
            }),
            native!("clear", 0, |this, _a| {
                let o = this_map(this)?;
                let Kind::Set(m) = &o.0.kind else { unreachable!() };
                m.borrow_mut().clear();
                Ok(Val::Undef)
            }),
            native!("values", 0, |this, _a| {
                let o = this_map(this)?;
                let Kind::Set(m) = &o.0.kind else { unreachable!() };
                Ok(array(m.borrow().pairs().into_iter().map(|(k, _)| k).collect()))
            }),
            native!("forEach", 1, |this, a| {
                let o = this_map(this)?;
                let Kind::Set(m) = &o.0.kind else { unreachable!() };
                let pairs = m.borrow().pairs();
                for (k, _) in pairs {
                    call(&arg(a, 0), &arg(a, 1), &[k.clone(), k, this.clone()])?;
                }
                Ok(Val::Undef)
            }),
        ],
    );
    define_methods(
        &proto("WeakMap"),
        vec![
            native!("get", 1, |this, a| {
                let o = this_map(this)?;
                let Kind::WeakMap(m) = &o.0.kind else { unreachable!() };
                let Some(k) = identity(&arg(a, 0)) else { return Ok(Val::Undef) };
                Ok(m.borrow().get(&k).map_or(Val::Undef, |(_, v)| v.clone()))
            }),
            native!("set", 2, |this, a| {
                let o = this_map(this)?;
                let Kind::WeakMap(m) = &o.0.kind else { unreachable!() };
                let key = arg(a, 0);
                let k = weak_key(&key)?;
                let keep = match &key {
                    Val::Obj(ko) => ko.clone(),
                    _ => Obj::with(None, Kind::Plain),
                };
                // A Js key stays alive through the stored value pair.
                #[cfg(target_arch = "wasm32")]
                if let Val::Js(_) = &key {
                    keep.define("key", key.clone(), false);
                }
                m.borrow_mut().insert(k, (keep, arg(a, 1)));
                Ok(this.clone())
            }),
            native!("has", 1, |this, a| {
                let o = this_map(this)?;
                let Kind::WeakMap(m) = &o.0.kind else { unreachable!() };
                Ok(Val::Bool(identity(&arg(a, 0)).is_some_and(|k| m.borrow().contains_key(&k))))
            }),
            native!("delete", 1, |this, a| {
                let o = this_map(this)?;
                let Kind::WeakMap(m) = &o.0.kind else { unreachable!() };
                Ok(Val::Bool(identity(&arg(a, 0)).is_some_and(|k| m.borrow_mut().remove(&k).is_some())))
            }),
        ],
    );
}

/// `Map.prototype.get` etc. directly, for the ported code's own maps.
pub fn map_get(map: &Val, key: &Val) -> R<Val> {
    m(map, "get", &[key.clone()])
}
pub fn map_set(map: &Val, key: &Val, value: &Val) -> R<()> {
    m(map, "set", &[key.clone(), value.clone()]).map(|_| ())
}
pub fn map_has(map: &Val, key: &Val) -> R<bool> {
    Ok(m(map, "has", &[key.clone()])?.truthy())
}
/// `array.push( value )`.
pub fn push(array: &Val, value: impl Into<Val>) -> R<()> {
    match array {
        Val::Obj(o) if o.array().is_some() => {
            o.array().unwrap().borrow_mut().push(value.into());
            Ok(())
        }
        v => m(v, "push", &[value.into()]).map(|_| ()),
    }
}
/// `array.length`.
pub fn length(v: &Val) -> R<usize> {
    Ok(to_number(&get(v, "length")?) as usize)
}
