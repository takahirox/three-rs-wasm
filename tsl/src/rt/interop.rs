//! The embedding JavaScript: Rust objects cross as Proxies over a handle,
//! JavaScript values cross as `Val::Js` and are read through Reflect, so
//! both sides keep each other's identities. Exceptions cross as values.
use super::*;
use std::cell::RefCell;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(inline_js = r#"
const handles = new WeakMap();
const proxies = new Map();
let traps = null;
const ids = new WeakMap();
let nextId = 1;
export function rtInstall( t ) {
	traps = t;
}
// A Proxy for Rust object `h`: kind 0 plain, 1 array, 2 function.
export function rtWrap( h, kind ) {
	let p = proxies.get( h );
	if ( p !== undefined ) return p;
	const target = kind === 2 ? function () { 'use strict'; } : kind === 1 ? [] : {};
	const handler = {
		get: ( t, key, receiver ) => {
			if ( typeof key === 'symbol' ) {
				if ( key === Symbol.iterator ) return traps.iterator( h );
				if ( key === Symbol.toPrimitive ) return undefined;
				if ( key === Symbol.toStringTag ) return undefined;
				return undefined;
			}
			return traps.get( h, key );
		},
		set: ( t, key, value ) => {
			if ( typeof key === 'symbol' ) return true;
			traps.set( h, key, value );
			return true;
		},
		has: ( t, key ) => typeof key === 'symbol' ? key === Symbol.iterator && traps.iterable( h ) : traps.has( h, key ),
		deleteProperty: ( t, key ) => {
			if ( typeof key !== 'symbol' ) traps.delete( h, key );
			return true;
		},
		ownKeys: ( t ) => {
			const keys = traps.ownKeys( h );
			for ( const k of Reflect.ownKeys( t ) ) {
				const d = Reflect.getOwnPropertyDescriptor( t, k );
				if ( d && d.configurable === false && ! keys.includes( k ) ) keys.push( k );
			}
			return keys;
		},
		getOwnPropertyDescriptor: ( t, key ) => {
			const own = Reflect.getOwnPropertyDescriptor( t, key );
			if ( own && own.configurable === false ) return own;
			if ( typeof key === 'symbol' ) return undefined;
			const d = traps.descriptor( h, key );
			if ( d === undefined ) return undefined;
			return { value: d[ 0 ], writable: true, enumerable: d[ 1 ], configurable: true };
		},
		getPrototypeOf: ( t ) => traps.proto( h ),
		apply: ( t, thisArg, args ) => traps.apply( h, thisArg, args ),
		construct: ( t, args ) => traps.construct( h, args ),
		defineProperty: ( t, key, d ) => {
			if ( typeof key !== 'symbol' ) traps.set( h, key, d.value );
			return true;
		}
	};
	p = new Proxy( target, handler );
	handles.set( p, h );
	proxies.set( h, p );
	return p;
}
export function rtHandle( v ) {
	return ( v !== null && ( typeof v === 'object' || typeof v === 'function' ) ) ? handles.get( v ) : undefined;
}
export function rtIdentity( v ) {
	let id = ids.get( v );
	if ( id === undefined ) {
		id = nextId ++;
		ids.set( v, id );
	}
	return id;
}
export function rtTypeOf( v ) {
	return typeof v;
}
export function rtGet( o, k ) {
	return o[ k ];
}
export function rtSet( o, k, v ) {
	o[ k ] = v;
}
export function rtDelete( o, k ) {
	delete o[ k ];
}
export function rtHas( o, k ) {
	return ( o !== null && ( typeof o === 'object' || typeof o === 'function' ) ) && k in o;
}
export function rtKeys( o ) {
	return Object.keys( o );
}
export function rtForInKeys( o ) {
	const keys = [];
	for ( const k in o ) keys.push( k );
	return keys;
}
export function rtCall( f, thisArg, args ) {
	return Reflect.apply( f, thisArg, args );
}
export function rtConstruct( f, args ) {
	return Reflect.construct( f, args );
}
export function rtInstanceOf( v, c ) {
	try {
		return v instanceof c;
	} catch ( e ) {
		return false;
	}
}
export function rtIsPlainObject( v ) {
	return v !== null && typeof v === 'object' && Object.getPrototypeOf( v ) === Object.prototype;
}
export function rtIterate( v ) {
	return [ ...v ];
}
export function rtToString( v ) {
	return String( v );
}
export function rtToNumber( v ) {
	return Number( v );
}
export function rtConsole( level, args ) {
	console[ level ]( ...args );
}
export function rtError( kind, message ) {
	return kind === 'TypeError' ? new TypeError( message ) : kind === 'RangeError' ? new RangeError( message ) : new Error( message );
}
export function rtGlobal( name ) {
	return globalThis[ name ];
}
export function rtGetPrototypeOf( v ) {
	return Object.getPrototypeOf( v );
}
export function rtJson( v ) {
	return JSON.stringify( v );
}
export function rtIterator( values ) {
	return function* () {
		yield* values;
	};
}
"#)]
extern "C" {
    #[wasm_bindgen(js_name = rtInstall)]
    fn rt_install(traps: &js_sys::Object);
    #[wasm_bindgen(js_name = rtWrap)]
    fn rt_wrap(h: u32, kind: u32) -> JsValue;
    #[wasm_bindgen(js_name = rtHandle)]
    fn rt_handle(v: &JsValue) -> Option<u32>;
    #[wasm_bindgen(js_name = rtIdentity)]
    fn rt_identity(v: &JsValue) -> f64;
    #[wasm_bindgen(js_name = rtTypeOf)]
    fn rt_type_of(v: &JsValue) -> String;
    #[wasm_bindgen(js_name = rtGet, catch)]
    fn rt_get(o: &JsValue, k: &str) -> Result<JsValue, JsValue>;
    #[wasm_bindgen(js_name = rtSet, catch)]
    fn rt_set(o: &JsValue, k: &str, v: &JsValue) -> Result<(), JsValue>;
    #[wasm_bindgen(js_name = rtDelete, catch)]
    fn rt_delete(o: &JsValue, k: &str) -> Result<(), JsValue>;
    #[wasm_bindgen(js_name = rtHas)]
    fn rt_has(o: &JsValue, k: &str) -> bool;
    #[wasm_bindgen(js_name = rtKeys)]
    fn rt_keys(o: &JsValue) -> Vec<String>;
    #[wasm_bindgen(js_name = rtForInKeys)]
    fn rt_for_in_keys(o: &JsValue) -> Vec<String>;
    #[wasm_bindgen(js_name = rtCall, catch)]
    fn rt_call(f: &JsValue, this: &JsValue, args: &js_sys::Array) -> Result<JsValue, JsValue>;
    #[wasm_bindgen(js_name = rtConstruct, catch)]
    fn rt_construct(f: &JsValue, args: &js_sys::Array) -> Result<JsValue, JsValue>;
    #[wasm_bindgen(js_name = rtInstanceOf)]
    fn rt_instance_of(v: &JsValue, c: &JsValue) -> bool;
    #[wasm_bindgen(js_name = rtIsPlainObject)]
    fn rt_is_plain_object(v: &JsValue) -> bool;
    #[wasm_bindgen(js_name = rtIterate, catch)]
    fn rt_iterate(v: &JsValue) -> Result<js_sys::Array, JsValue>;
    #[wasm_bindgen(js_name = rtToString)]
    fn rt_to_string(v: &JsValue) -> String;
    #[wasm_bindgen(js_name = rtToNumber)]
    fn rt_to_number(v: &JsValue) -> f64;
    #[wasm_bindgen(js_name = rtConsole)]
    fn rt_console(level: &str, args: &js_sys::Array);
    #[wasm_bindgen(js_name = rtError)]
    fn rt_error(kind: &str, message: &str) -> JsValue;
    #[wasm_bindgen(js_name = rtIterator)]
    fn rt_iterator(values: &js_sys::Array) -> JsValue;
    #[wasm_bindgen(js_name = rtGlobal)]
    fn rt_global(name: &str) -> JsValue;
    #[wasm_bindgen(js_name = rtGetPrototypeOf)]
    fn rt_get_prototype_of(v: &JsValue) -> JsValue;
    #[wasm_bindgen(js_name = rtJson)]
    fn rt_json(v: &JsValue) -> String;
}

/// `globalThis[ name ]` of the embedding JavaScript.
pub fn global(name: &str) -> Val {
    from_js(rt_global(name))
}
pub fn get_prototype_of(j: &JsValue) -> Val {
    from_js(rt_get_prototype_of(j))
}
pub fn json_stringify(j: &JsValue) -> String {
    rt_json(j)
}

thread_local! {
    static HANDLES: RefCell<Vec<Obj>> = const { RefCell::new(Vec::new()) };
    static BY_PTR: RefCell<HashMap<usize, u32>> = RefCell::new(HashMap::new());
    static INSTALLED: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
    static CLOSURES: RefCell<Vec<JsValue>> = const { RefCell::new(Vec::new()) };
}

fn obj_of(h: u32) -> Obj {
    HANDLES.with(|hs| hs.borrow()[h as usize].clone())
}

fn handle_of(o: &Obj) -> u32 {
    BY_PTR.with(|m| {
        if let Some(&h) = m.borrow().get(&o.ptr()) {
            return h;
        }
        let h = HANDLES.with(|hs| {
            let mut hs = hs.borrow_mut();
            hs.push(o.clone());
            (hs.len() - 1) as u32
        });
        m.borrow_mut().insert(o.ptr(), h);
        h
    })
}

/// A Rust exception as a JavaScript one: errors become real Error objects.
pub fn exc_to_js(e: &Exc) -> JsValue {
    match &e.0 {
        Val::Obj(o) if matches!(o.0.kind, Kind::Error) => {
            let kind = display(&super::get(&e.0, "name").unwrap_or(Val::Undef));
            let kind = match kind.as_str() {
                "TypeError" | "RangeError" => kind,
                _ => "Error".to_string(),
            };
            rt_error(&kind, &error_message(&e.0))
        }
        v => to_js(v),
    }
}

fn js_exc(e: JsValue) -> Exc {
    Exc(from_js(e))
}

/// A Rust value as a JavaScript one.
pub fn to_js(v: &Val) -> JsValue {
    install();
    match v {
        Val::Undef => JsValue::UNDEFINED,
        Val::Null => JsValue::NULL,
        Val::Bool(b) => JsValue::from_bool(*b),
        Val::Num(n) => JsValue::from_f64(*n),
        Val::Str(s) => JsValue::from_str(s),
        Val::Js(j) => j.clone(),
        Val::Obj(o) => {
            let callable = o.is_func() || matches!(&o.0.kind, Kind::Proxy(target, _) if target.type_of() == "function");
            let kind = if callable {
                2
            } else if o.is_array() {
                1
            } else {
                0
            };
            rt_wrap(handle_of(o), kind)
        }
    }
}

/// A JavaScript value as a Rust one: our proxies unwrap to their objects.
pub fn from_js(v: JsValue) -> Val {
    if v.is_undefined() {
        return Val::Undef;
    }
    if v.is_null() {
        return Val::Null;
    }
    if let Some(b) = v.as_bool() {
        return Val::Bool(b);
    }
    if let Some(n) = v.as_f64() {
        return Val::Num(n);
    }
    if let Some(s) = v.as_string() {
        return Val::from(s);
    }
    if let Some(h) = rt_handle(&v) {
        return Val::Obj(obj_of(h));
    }
    Val::Js(v)
}

pub fn identity(j: &JsValue) -> u64 {
    rt_identity(j) as u64
}
pub fn type_of(j: &JsValue) -> &'static str {
    match rt_type_of(j).as_str() {
        "undefined" => "undefined",
        "boolean" => "boolean",
        "number" => "number",
        "string" => "string",
        "function" => "function",
        "bigint" => "bigint",
        "symbol" => "symbol",
        _ => "object",
    }
}
pub fn to_number(j: &JsValue) -> f64 {
    rt_to_number(j)
}
pub fn to_string(j: &JsValue) -> String {
    rt_to_string(j)
}
pub fn get(j: &JsValue, key: &str) -> R<Val> {
    rt_get(j, key).map(from_js).map_err(js_exc)
}
pub fn set(j: &JsValue, key: &str, v: &Val) -> R<()> {
    rt_set(j, key, &to_js(v)).map_err(js_exc)
}
pub fn delete(j: &JsValue, key: &str) -> R<()> {
    rt_delete(j, key).map_err(js_exc)
}
pub fn has(j: &JsValue, key: &str) -> bool {
    rt_has(j, key)
}
pub fn keys(j: &JsValue) -> Vec<Rc<str>> {
    rt_keys(j).into_iter().map(|k| Rc::from(k.as_str())).collect()
}
pub fn for_in_keys(j: &JsValue) -> Vec<Rc<str>> {
    rt_for_in_keys(j).into_iter().map(|k| Rc::from(k.as_str())).collect()
}
fn js_args(args: &[Val]) -> js_sys::Array {
    args.iter().map(to_js).collect()
}
pub fn call(f: &JsValue, this: &Val, args: &[Val]) -> R<Val> {
    rt_call(f, &to_js(this), &js_args(args)).map(from_js).map_err(js_exc)
}
pub fn construct(f: &JsValue, args: &[Val]) -> R<Val> {
    rt_construct(f, &js_args(args)).map(from_js).map_err(js_exc)
}
pub fn instance_of(v: &JsValue, c: &Val) -> bool {
    rt_instance_of(v, &to_js(c))
}
pub fn is_plain_object(j: &JsValue) -> bool {
    rt_is_plain_object(j)
}
pub fn iterate(j: &JsValue) -> R<Vec<Val>> {
    rt_iterate(j)
        .map(|a| a.iter().map(from_js).collect())
        .map_err(js_exc)
}
pub fn console(level: &str, args: &[Val]) {
    rt_console(level, &js_args(args));
}

/// Install the proxy traps once.
pub fn install() {
    if INSTALLED.with(|i| i.replace(true)) {
        return;
    }
    let traps = js_sys::Object::new();
    let add = |name: &str, f: JsValue| {
        js_sys::Reflect::set(&traps, &JsValue::from_str(name), &f).unwrap();
        CLOSURES.with(|c| c.borrow_mut().push(f));
    };
    let throw = |e: Exc| -> JsValue { exc_to_js(&e) };
    add(
        "get",
        Closure::<dyn Fn(u32, String) -> Result<JsValue, JsValue>>::new(move |h, k: String| {
            super::get(&Val::Obj(obj_of(h)), &k).map(|v| to_js(&v)).map_err(throw)
        })
        .into_js_value(),
    );
    add(
        "set",
        Closure::<dyn Fn(u32, String, JsValue) -> Result<(), JsValue>>::new(move |h, k: String, v| {
            super::set(&Val::Obj(obj_of(h)), &k, from_js(v)).map_err(throw)
        })
        .into_js_value(),
    );
    add(
        "has",
        Closure::<dyn Fn(u32, String) -> bool>::new(|h, k: String| {
            super::has(&Val::Obj(obj_of(h)), &k).unwrap_or(false)
        })
        .into_js_value(),
    );
    add(
        "delete",
        Closure::<dyn Fn(u32, String)>::new(|h, k: String| {
            obj_of(h).delete(&k);
        })
        .into_js_value(),
    );
    add(
        "ownKeys",
        Closure::<dyn Fn(u32) -> js_sys::Array>::new(|h| {
            let o = obj_of(h);
            let target = o.func().and_then(|f| f.forward.borrow().clone()).unwrap_or(o);
            target.own_keys().iter().map(|k| JsValue::from_str(k)).collect()
        })
        .into_js_value(),
    );
    add(
        "descriptor",
        Closure::<dyn Fn(u32, String) -> JsValue>::new(|h, k: String| {
            let o = obj_of(h);
            let target = o.func().and_then(|f| f.forward.borrow().clone()).unwrap_or(o);
            if !target.has_own(&k) {
                return JsValue::UNDEFINED;
            }
            let enumerable = target.array().is_some_and(|_| array_index(&k).is_some())
                || target.0.props.borrow().get(&k).is_some_and(|p| p.enumerable);
            let value = super::get(&Val::Obj(target.clone()), &k).unwrap_or(Val::Undef);
            let pair = js_sys::Array::new();
            pair.push(&to_js(&value));
            pair.push(&JsValue::from_bool(enumerable));
            pair.into()
        })
        .into_js_value(),
    );
    add(
        "proto",
        Closure::<dyn Fn(u32) -> JsValue>::new(|h| {
            let o = obj_of(h);
            match o.proto() {
                Some(p) => to_js(&Val::Obj(p)),
                None => JsValue::NULL,
            }
        })
        .into_js_value(),
    );
    add(
        "apply",
        Closure::<dyn Fn(u32, JsValue, js_sys::Array) -> Result<JsValue, JsValue>>::new(
            move |h, this, args: js_sys::Array| {
                let args: Vec<Val> = args.iter().map(from_js).collect();
                super::call(&Val::Obj(obj_of(h)), &from_js(this), &args)
                    .map(|v| to_js(&v))
                    .map_err(throw)
            },
        )
        .into_js_value(),
    );
    add(
        "construct",
        Closure::<dyn Fn(u32, js_sys::Array) -> Result<JsValue, JsValue>>::new(
            move |h, args: js_sys::Array| {
                let args: Vec<Val> = args.iter().map(from_js).collect();
                super::construct(&Val::Obj(obj_of(h)), &args)
                    .map(|v| to_js(&v))
                    .map_err(throw)
            },
        )
        .into_js_value(),
    );
    // Iterable: a built-in collection, or an @@iterator read as JavaScript reads it ( through a Proxy's get trap ).
    fn iterable(o: &Obj) -> bool {
        matches!(o.0.kind, Kind::Array(_) | Kind::Map(_) | Kind::Set(_) | Kind::Typed(..))
            || super::get(&Val::Obj(o.clone()), "@@iterator").is_ok_and(|f| f.type_of() == "function")
    }
    add(
        "iterable",
        Closure::<dyn Fn(u32) -> bool>::new(|h| iterable(&obj_of(h))).into_js_value(),
    );
    add(
        "iterator",
        Closure::<dyn Fn(u32) -> JsValue>::new(|h| {
            let o = obj_of(h);
            if !iterable(&o) {
                return JsValue::UNDEFINED;
            }
            match super::iterate(&Val::Obj(o)) {
                Ok(values) => rt_iterator(&js_args(&values)),
                Err(_) => JsValue::UNDEFINED,
            }
        })
        .into_js_value(),
    );
    rt_install(&traps);
    let _ = JsValue::UNDEFINED.dyn_ref::<js_sys::Object>();
}
