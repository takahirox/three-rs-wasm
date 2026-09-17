//! A Rust-first scene and rendering library, using Three.js r186 as its
//! behavioral reference. The compatibility manifest describes the MVP target;
//! see implementation evidence before treating any capability as complete.

pub mod animation;
mod area_light;
pub mod attribute;
pub mod batching;
#[cfg(target_arch = "wasm32")]
pub mod browser;
pub mod camera;
mod clipping;
pub mod compression;
pub mod compute;
pub mod curve;
pub mod deformation;
mod deformation_gpu;
mod draw_gpu;
pub mod environment;
mod environment_gpu;
pub mod event;
pub mod geometry;
mod geometry_gpu;
pub mod identity;
pub mod material;
pub mod math;
mod physical_maps;
pub mod postprocessing;
pub mod raycast;
pub mod render_target;
pub mod renderer;
pub mod scene;
pub mod shader;
pub mod shadow;
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
