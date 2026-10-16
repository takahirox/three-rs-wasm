//! `Math` functions as the JavaScript host computes them: the host's own in
//! the browser ( so ports round as the originals do ), the platform's natively
//! ( see `bounce::trig` ).
pub(crate) use crate::bounce::trig::{cos, sin};
#[cfg(target_arch = "wasm32")]
pub(crate) fn acos(x: f64) -> f64 {
    js_sys::Math::acos(x)
}
#[cfg(target_arch = "wasm32")]
pub(crate) fn asin(x: f64) -> f64 {
    js_sys::Math::asin(x)
}
#[cfg(target_arch = "wasm32")]
pub(crate) fn atan(x: f64) -> f64 {
    js_sys::Math::atan(x)
}
#[cfg(target_arch = "wasm32")]
pub(crate) fn atan2(y: f64, x: f64) -> f64 {
    js_sys::Math::atan2(y, x)
}
#[cfg(target_arch = "wasm32")]
pub(crate) fn pow(x: f64, y: f64) -> f64 {
    js_sys::Math::pow(x, y)
}
#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn acos(x: f64) -> f64 {
    x.acos()
}
#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn asin(x: f64) -> f64 {
    x.asin()
}
#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn atan(x: f64) -> f64 {
    x.atan()
}
#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn atan2(y: f64, x: f64) -> f64 {
    y.atan2(x)
}
#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn pow(x: f64, y: f64) -> f64 {
    x.powf(y)
}
#[cfg(target_arch = "wasm32")]
pub(crate) fn log10(x: f64) -> f64 {
    js_sys::Math::log10(x)
}
#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn log10(x: f64) -> f64 {
    x.log10()
}
#[cfg(target_arch = "wasm32")]
pub(crate) fn log(x: f64) -> f64 {
    js_sys::Math::log(x)
}
#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn log(x: f64) -> f64 {
    x.ln()
}
