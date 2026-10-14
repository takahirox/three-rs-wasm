//! JavaScript's operators, conversions and the statement helpers the
//! transpiled modules call ( tools/tsl/transpile.mjs ).
use super::*;

/// A captured variable: shared between a function and its closures.
pub type Var = Rc<RefCell<Val>>;

pub fn var(v: Val) -> Var {
    Rc::new(RefCell::new(v))
}

/// ToPrimitive ( hint default / number ): valueOf, then toString.
pub fn to_primitive(v: &Val, prefer_string: bool) -> R<Val> {
    match v {
        Val::Obj(_) => {
            let order: [&str; 2] = if prefer_string { ["toString", "valueOf"] } else { ["valueOf", "toString"] };
            for name in order {
                let f = get(v, name)?;
                if f.type_of() == "function" {
                    let r = call(&f, v, &[])?;
                    if !matches!(r, Val::Obj(_)) {
                        return Ok(r);
                    }
                }
            }
            Err(type_error("Cannot convert object to primitive value"))
        }
        #[cfg(target_arch = "wasm32")]
        Val::Js(j) => {
            if interop::type_of(j) == "object" || interop::type_of(j) == "function" {
                Ok(Val::from(interop::to_string(j)))
            } else {
                Ok(v.clone())
            }
        }
        v => Ok(v.clone()),
    }
}

/// ToNumber, with objects through ToPrimitive.
pub fn num(v: &Val) -> R<f64> {
    match v {
        Val::Obj(_) => Ok(to_number(&to_primitive(v, false)?)),
        v => Ok(to_number(v)),
    }
}

/// `a + b`.
pub fn add(a: &Val, b: &Val) -> R<Val> {
    if let (Val::Num(x), Val::Num(y)) = (a, b) {
        return Ok(Val::Num(x + y));
    }
    let (pa, pb) = (to_primitive(a, false)?, to_primitive(b, false)?);
    if matches!(pa, Val::Str(_)) || matches!(pb, Val::Str(_)) {
        let mut s = to_string(&pa)?.to_string();
        s.push_str(&to_string(&pb)?);
        return Ok(Val::from(s));
    }
    Ok(Val::Num(to_number(&pa) + to_number(&pb)))
}
pub fn sub(a: &Val, b: &Val) -> R<Val> {
    Ok(Val::Num(num(a)? - num(b)?))
}
pub fn mul(a: &Val, b: &Val) -> R<Val> {
    Ok(Val::Num(num(a)? * num(b)?))
}
pub fn div(a: &Val, b: &Val) -> R<Val> {
    Ok(Val::Num(num(a)? / num(b)?))
}
/// `a % b` ( the sign of the dividend ).
pub fn rem(a: &Val, b: &Val) -> R<Val> {
    Ok(Val::Num(num(a)? % num(b)?))
}
pub fn pow(a: &Val, b: &Val) -> R<Val> {
    let (x, y) = (num(a)?, num(b)?);
    Ok(Val::Num(if y.is_nan() || (x.abs() == 1.0 && y.is_infinite()) { f64::NAN } else { x.powf(y) }))
}

/// ToInt32.
pub fn to_i32(v: f64) -> i32 {
    if !v.is_finite() {
        return 0;
    }
    let m = 4294967296.0;
    let mut x = v.trunc() % m;
    if x < 0.0 {
        x += m;
    }
    if x >= 2147483648.0 {
        x -= m;
    }
    x as i32
}
pub fn to_u32(v: f64) -> u32 {
    to_i32(v) as u32
}
pub fn bit_and(a: &Val, b: &Val) -> R<Val> {
    Ok(Val::Num((to_i32(num(a)?) & to_i32(num(b)?)) as f64))
}
pub fn bit_or(a: &Val, b: &Val) -> R<Val> {
    Ok(Val::Num((to_i32(num(a)?) | to_i32(num(b)?)) as f64))
}
pub fn bit_xor(a: &Val, b: &Val) -> R<Val> {
    Ok(Val::Num((to_i32(num(a)?) ^ to_i32(num(b)?)) as f64))
}
pub fn shl(a: &Val, b: &Val) -> R<Val> {
    Ok(Val::Num(to_i32(num(a)?).wrapping_shl(to_u32(num(b)?) & 31) as f64))
}
pub fn shr(a: &Val, b: &Val) -> R<Val> {
    Ok(Val::Num((to_i32(num(a)?) >> (to_u32(num(b)?) & 31)) as f64))
}
pub fn ushr(a: &Val, b: &Val) -> R<Val> {
    Ok(Val::Num((to_u32(num(a)?) >> (to_u32(num(b)?) & 31)) as f64))
}
pub fn bit_not(a: &Val) -> R<Val> {
    Ok(Val::Num(!to_i32(num(a)?) as f64))
}
pub fn neg(a: &Val) -> R<Val> {
    Ok(Val::Num(-num(a)?))
}
pub fn plus(a: &Val) -> R<Val> {
    Ok(Val::Num(num(a)?))
}

/// The relational comparison `a < b` ( undefined → false ).
fn less(a: &Val, b: &Val) -> R<Option<bool>> {
    let (pa, pb) = (to_primitive(a, false)?, to_primitive(b, false)?);
    if let (Val::Str(x), Val::Str(y)) = (&pa, &pb) {
        let (x, y): (Vec<u16>, Vec<u16>) = (x.encode_utf16().collect(), y.encode_utf16().collect());
        return Ok(Some(x < y));
    }
    let (x, y) = (to_number(&pa), to_number(&pb));
    if x.is_nan() || y.is_nan() {
        return Ok(None);
    }
    Ok(Some(x < y))
}
pub fn lt(a: &Val, b: &Val) -> R<bool> {
    Ok(less(a, b)? == Some(true))
}
pub fn gt(a: &Val, b: &Val) -> R<bool> {
    Ok(less(b, a)? == Some(true))
}
pub fn le(a: &Val, b: &Val) -> R<bool> {
    Ok(less(b, a)? == Some(false))
}
pub fn ge(a: &Val, b: &Val) -> R<bool> {
    Ok(less(a, b)? == Some(false))
}

/// ToPropertyKey of a computed member.
pub fn key(v: &Val) -> R<Rc<str>> {
    match v {
        Val::Str(s) => Ok(s.clone()),
        Val::Num(n) => Ok(Rc::from(num::to_string(*n))),
        v => to_string(&to_primitive(v, true)?),
    }
}

/// `o[ k ]` for a computed key.
pub fn get_v(o: &Val, k: &Val) -> R<Val> {
    if let (Val::Obj(obj), Val::Num(n)) = (o, k)
        && *n >= 0.0
        && n.fract() == 0.0
    {
        if let Some(a) = obj.array() {
            return Ok(a.borrow().get(*n as usize).cloned().unwrap_or(Val::Undef));
        }
    }
    get(o, &key(k)?)
}

/// `o[ k ] = v` for a computed key.
pub fn set_v(o: &Val, k: &Val, v: Val) -> R<()> {
    set(o, &key(k)?, v)
}

/// `k in o`.
pub fn in_(k: &Val, o: &Val) -> R<bool> {
    has(o, &key(k)?)
}

/// `typeof v`.
pub fn type_of(v: &Val) -> Val {
    Val::from(v.type_of())
}

/// `delete o[ k ]`.
pub fn delete_v(o: &Val, k: &Val) -> R<bool> {
    delete(o, &key(k)?)?;
    Ok(true)
}

/// `[ …a, …b ]` and call spreads: append an iterable's values.
pub fn spread_into(out: &mut Vec<Val>, v: &Val, site: &str) -> R<()> {
    out.extend(iterate_site(v, site)?);
    Ok(())
}

/// `{ …source }` in an object literal.
pub fn spread_obj(target: &Val, source: &Val) -> R<()> {
    if source.is_nullish() {
        return Ok(());
    }
    if let Val::Obj(o) = source {
        // Own enumerable string keys, then symbols, as spread copies them.
        let symbols: Vec<Rc<str>> = o.0.props.borrow().symbol_keys();
        for k in o.keys().into_iter().chain(symbols) {
            let v = get(source, &k)?;
            if let Val::Obj(t) = target {
                t.define(&k, v, true);
            }
        }
        return Ok(());
    }
    assign(target, source)
}

/// A template literal's text.
pub fn template(parts: &[Val]) -> R<Val> {
    let mut out = String::new();
    for p in parts {
        out.push_str(&to_string(&to_primitive(p, true)?)?);
    }
    Ok(Val::from(out))
}

/// The value a destructuring pattern reads: V8's message for nullish sources.
pub fn destructure(v: &Val, key: &str, site: &str) -> R<Val> {
    if v.is_nullish() {
        let what = if v.is_undef() { "undefined" } else { "null" };
        // A parameter has no source expression: V8 names the value.
        let site = if site.is_empty() { what } else { site };
        return Err(type_error(&format!("Cannot destructure property '{key}' of '{site}' as it is {what}.")));
    }
    get(v, key)
}

/// `new C( …args )` with `site` the constructor's source text.
pub fn new_site(c: &Val, args: &[Val], site: &str) -> R<Val> {
    let constructible = match c {
        Val::Obj(o) => o.func().is_some_and(|f| f.init.is_some() || f.construct.is_some()) || matches!(o.0.kind, Kind::Proxy(..)) || is_constructible(o),
        #[cfg(target_arch = "wasm32")]
        Val::Js(_) => c.type_of() == "function",
        _ => false,
    };
    if !constructible {
        return Err(type_error(&format!("{site} is not a constructor")));
    }
    construct(c, args)
}

/// The function a call reaches through `recv[ key ]`, or V8's message.
pub fn method(recv: &Val, key: &str, site: &str) -> R<Val> {
    let f = get(recv, key)?;
    if f.type_of() != "function" {
        return Err(type_error(&format!("{site} is not a function")));
    }
    Ok(f)
}

/// `recv[ key ]( …args )`.
pub fn call_m(recv: &Val, key: &str, args: &[Val], site: &str) -> R<Val> {
    let f = method(recv, key, site)?;
    call(&f, recv, args)
}

/// `recv[ k ]( …args )` for a computed key.
pub fn call_mv(recv: &Val, k: &Val, args: &[Val], site: &str) -> R<Val> {
    let k = key(k)?;
    call_m(recv, &k, args, site)
}

/// `f( …args )` with `site` the callee's source text.
pub fn call_f(f: &Val, args: &[Val], site: &str) -> R<Val> {
    if f.type_of() != "function" {
        return Err(type_error(&format!("{site} is not a function")));
    }
    call(f, &Val::Undef, args)
}

/// A function's completion inside a try block.
pub enum Ctl {
    Normal,
    Return(Val),
    Break(Option<&'static str>),
    Continue(Option<&'static str>),
}

/// The message a `catch` sees: our exceptions are values already.
pub fn caught(e: Exc) -> Val {
    e.0
}

/// `throw v`.
pub fn throw<T>(v: Val) -> R<T> {
    Err(Exc(v))
}

/// Call `f` found as `recv[ key ]`: V8 checks callability after the arguments.
pub fn call_checked(f: &Val, recv: &Val, args: &[Val], site: &str) -> R<Val> {
    if f.type_of() != "function" {
        return Err(type_error(&format!("{site} is not a function")));
    }
    call(f, recv, args)
}

/// The other half of an accessor being defined ( a getter's setter or a setter's getter ).
pub fn accessor_part(o: &Obj, key: &str, setter: bool) -> Option<Val> {
    match o.0.props.borrow().get(key) {
        Some(Prop { slot: Slot::Accessor(g, s), .. }) => {
            if setter {
                g.clone()
            } else {
                s.clone()
            }
        }
        _ => None,
    }
}
pub fn get_setter(o: &Val, key: &str) -> Option<Val> {
    o.obj().and_then(|o| accessor_part(o, key, false))
}
pub fn get_getter(o: &Val, key: &str) -> Option<Val> {
    o.obj().and_then(|o| accessor_part(o, key, true))
}

/// `{ a, ...rest } = v`: the remaining own enumerable properties.
pub fn object_rest(v: &Val, used: &[&str]) -> R<Val> {
    let out = object(&[]);
    for k in keys(v) {
        if !used.contains(&&*k) {
            let x = get(v, &k)?;
            set(&out, &k, x)?;
        }
    }
    Ok(out)
}

/// A call with V8's whole message given ( "x is not a function …" ).
pub fn call_checked_msg(f: &Val, recv: &Val, args: &[Val], message: &str) -> R<Val> {
    if f.type_of() != "function" {
        return Err(type_error(message));
    }
    call(f, recv, args)
}
pub fn call_f_msg(f: &Val, args: &[Val], message: &str) -> R<Val> {
    if f.type_of() != "function" {
        return Err(type_error(message));
    }
    call(f, &Val::Undef, args)
}
