//! Module exports: each ported module registers its exports under its
//! source path ( relative to three's `src/` ) and reads its imports the same
//! way, so modules evaluate in the original's order and see the original's
//! bindings.
use crate::rt::*;
use std::cell::RefCell;
use std::collections::HashMap;

thread_local! {
    static EXPORTS: RefCell<HashMap<(&'static str, String), Val>> = RefCell::new(HashMap::new());
}

/// `export const name = value` from `module`.
pub fn export(module: &'static str, name: &str, value: impl Into<Val>) {
    EXPORTS.with(|e| e.borrow_mut().insert((module, name.to_string()), value.into()));
}

/// `import { name } from module` ( `undefined` when the module has no such export ).
pub fn imp(module: &'static str, name: &str) -> Val {
    EXPORTS
        .with(|e| e.borrow().get(&(module, name.to_string())).cloned())
        .unwrap_or(Val::Undef)
}

/// `export * from from` in `to`.
pub fn reexport_all(from: &'static str, to: &'static str) {
    let names: Vec<(String, Val)> = EXPORTS.with(|e| {
        e.borrow()
            .iter()
            .filter(|((m, n), _)| *m == from && n != "default")
            .map(|((_, n), v)| (n.clone(), v.clone()))
            .collect()
    });
    for (n, v) in names {
        export(to, &n, v);
    }
}

/// Every export name of a module.
pub fn export_names(module: &'static str) -> Vec<String> {
    EXPORTS.with(|e| {
        e.borrow()
            .keys()
            .filter(|(m, _)| *m == module)
            .map(|(_, n)| n.clone())
            .collect()
    })
}

/// A class from its constructor value.
pub fn cls(ctor: &Val) -> Class {
    let prototype = match get(ctor, "prototype") {
        Ok(Val::Obj(p)) => p,
        _ => panic!("not a class"),
    };
    Class {
        ctor: ctor.clone(),
        prototype,
    }
}

thread_local! {
    static NAMESPACES: RefCell<HashMap<&'static str, Val>> = RefCell::new(HashMap::new());
}

/// `import * as ns from module`: an object of the module's exports.
pub fn module_namespace(module: &'static str) -> Val {
    if let Some(v) = NAMESPACES.with(|n| n.borrow().get(module).cloned()) {
        return v;
    }
    let ns = object(&[]);
    let mut names = export_names(module);
    names.sort();
    for n in names {
        if let Val::Obj(o) = &ns {
            o.define(&n, imp(module, &n), true);
        }
    }
    NAMESPACES.with(|n| n.borrow_mut().insert(module, ns.clone()));
    ns
}

/// Evaluate a module, naming it in the error.
pub fn run_module(module: &str, init: fn() -> R<()>) -> R<()> {
    init().map_err(|e| {
        let stack = crate::rt::get(&e.0, "stack").map(|s| crate::rt::display(&s)).unwrap_or_default();
        let err = crate::rt::error(&format!("{module}: {}", crate::rt::error_message(&e.0)));
        let _ = crate::rt::set(&err.0, "stack", stack);
        err
    })
}
