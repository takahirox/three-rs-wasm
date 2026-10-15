//! The examples gallery's Rust ports of the Three.js r186 examples, built on
//! three-rs-wasm. The library's modules are re-exported so a port reads like
//! code inside it.
#![cfg_attr(target_arch = "wasm32", allow(clippy::arc_with_non_send_sync))]
// A page's package builds only its example's modules; the dispatch leaves
// the others' parameters unused.
#![cfg_attr(not(feature = "all"), allow(unused, dead_code))]
pub use three_rs_wasm::*;

#[cfg(target_arch = "wasm32")]
pub mod browser;
