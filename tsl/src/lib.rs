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
pub mod dom;
pub mod materialx_page;
#[macro_use]
pub mod js;
pub mod reg;
pub mod three;

/// Evaluate three's modules ( once ).
pub fn init_three() -> rt::R<()> {
    thread_local! {
        static DONE: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
    }
    if DONE.with(|d| d.replace(true)) {
        return Ok(());
    }
    three::order::init_all()
}

/// Evaluate the ported addons ( after `three::order::init_all` ) with the
/// DOMParser and TextDecoder they read.
pub fn init_addons() -> rt::R<()> {
    dom::install();
    dom::install_text_decoder();
    three::order::init_addons()
}

/// The entry points the editor and the corpus check use.
#[cfg(target_arch = "wasm32")]
pub mod api {
    use wasm_bindgen::prelude::*;

    /// Evaluate every module ( once ).
    #[wasm_bindgen]
    pub fn tsl_init() -> Result<(), JsValue> {
        crate::init_three().map_err(|e| crate::rt::interop::exc_to_js(&e))
    }

    /// webgpu_loader_materialx's shaders ( `materialx_page` ) for the
    /// documents `texts`, each loaded from `paths[ i ]`, on a `width` ×
    /// `height` canvas: `{ grid, materials }`, each shader set
    /// `{ vertexShader, fragmentShader, transparent, textures }` or
    /// `{ error }`, each texture binding `{ binding, source, uri, wrapS,
    /// wrapT }` ( source 'image', 'dfg', 'pmrem' or 'transmission' ).
    #[wasm_bindgen]
    pub fn materialx_shaders(texts: Vec<String>, paths: Vec<String>, width: f64, height: f64) -> Result<JsValue, JsValue> {
        use crate::materialx_page::{Document, Shaders, Source, page_shaders};
        use crate::rt::{Val, array, object, s};
        let documents: Vec<Document> =
            texts.into_iter().zip(paths).map(|(text, path)| Document { text, path }).collect();
        let page = page_shaders(&documents, width, height).map_err(|e| crate::rt::interop::exc_to_js(&e))?;
        let shaders = |sh: &Shaders| {
            let textures = sh
                .textures
                .iter()
                .map(|b| {
                    let (source, uri) = match &b.source {
                        Source::Image(uri) => ("image", s(uri)),
                        Source::Dfg => ("dfg", Val::Null),
                        Source::Pmrem => ("pmrem", Val::Null),
                        Source::Transmission => ("transmission", Val::Null),
                    };
                    object(&[
                        ("binding", Val::Num(b.binding as f64)),
                        ("source", s(source)),
                        ("uri", uri),
                        ("wrapS", Val::Num(b.wrap.0 as f64)),
                        ("wrapT", Val::Num(b.wrap.1 as f64)),
                    ])
                })
                .collect();
            object(&[
                ("vertexShader", s(&sh.vertex)),
                ("fragmentShader", s(&sh.fragment)),
                ("transparent", Val::Bool(sh.transparent)),
                ("textures", array(textures)),
            ])
        };
        let materials = page
            .materials
            .iter()
            .map(|m| match m {
                Ok(sh) => shaders(sh),
                Err(e) => object(&[("error", s(e))]),
            })
            .collect();
        let result = object(&[("grid", shaders(&page.grid)), ("materials", array(materials))]);
        Ok(crate::rt::interop::to_js(&result))
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
