//! Three.js r186's node system ( `src/nodes`, the node builders of the WebGPU
//! and WebGL fallback backends and `materials/nodes` ) ported to Rust.
//!
//! The JavaScript classes are dynamic: a node's children are its own
//! properties in creation order, `generate.length` selects how it is built,
//! and TSL hands nodes to user JavaScript as chainable proxies. `rt` is a
//! JavaScript object model in Rust ( values, prototype chains, accessors,
//! property order, function arity and V8's error messages ) on which every
//! class is ported method for method, so a program builds the same graph and
//! the same shader text as the original.
pub mod rt;
#[macro_use]
pub mod js;
pub mod reg;
pub mod three;

/// The entry points the editor and the corpus check use.
#[cfg(target_arch = "wasm32")]
pub mod api {
    use wasm_bindgen::prelude::*;

    /// Evaluate every module ( once ).
    #[wasm_bindgen]
    pub fn tsl_init() -> Result<(), JsValue> {
        thread_local! {
            static DONE: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
        }
        if DONE.with(|d| d.replace(true)) {
            return Ok(());
        }
        crate::three::order::init_all().map_err(|e| crate::rt::interop::exc_to_js(&e))
    }

    /// `import * as THREE from 'three/webgpu'`.
    #[wasm_bindgen]
    pub fn three_namespace() -> JsValue {
        crate::rt::interop::to_js(&crate::reg::module_namespace("Three.WebGPU.js"))
    }

    /// `await import( 'three/tsl' )`.
    #[wasm_bindgen]
    pub fn tsl_namespace() -> JsValue {
        crate::rt::interop::to_js(&crate::reg::module_namespace("Three.TSL.js"))
    }
}
