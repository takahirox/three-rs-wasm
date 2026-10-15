//! The examples gallery's Rust ports of the Three.js r186 examples, built on
//! three-rs-wasm. The library's modules are re-exported so a port reads like
//! code inside it.
#![cfg_attr(target_arch = "wasm32", allow(clippy::arc_with_non_send_sync))]
pub use three_rs_wasm::*;

#[cfg(target_arch = "wasm32")]
pub mod browser;
