//! `Math.random()` for the XR examples. The page may install
//! `galleryRandom` ( the comparison tests' seeded generator, which the
//! reference page's Math.random calls draw from too ).
use wasm_bindgen::{JsCast, JsValue};

pub fn random() -> f64 {
    js_sys::Reflect::get(&js_sys::global(), &JsValue::from_str("galleryRandom"))
        .ok()
        .and_then(|f| f.dyn_into::<js_sys::Function>().ok())
        .and_then(|f| f.call0(&JsValue::NULL).ok())
        .and_then(|v| v.as_f64())
        .unwrap_or_else(js_sys::Math::random)
}
