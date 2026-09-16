//! A Rust-first scene and rendering library, using Three.js r186 as its
//! behavioral reference. The compatibility manifest describes the MVP target;
//! see implementation evidence before treating any capability as complete.

pub mod attribute;
#[cfg(target_arch = "wasm32")]
pub mod browser;
pub mod camera;
pub mod environment;
mod environment_gpu;
pub mod event;
pub mod geometry;
pub mod identity;
pub mod material;
pub mod math;
pub mod raycast;
pub mod render_target;
pub mod renderer;
pub mod scene;
mod texture_gpu;
pub mod time;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("invalid {0}")]
    Invalid(&'static str),
    #[error("stale handle or handle belongs to another scene")]
    InvalidHandle,
    #[error("operation would introduce a scene graph cycle")]
    Cycle,
    #[error("GPU error: {0}")]
    Gpu(String),
    #[error("asset error: {0}")]
    Asset(String),
}
pub type Result<T> = std::result::Result<T, Error>;

pub mod gltf;

mod background;
