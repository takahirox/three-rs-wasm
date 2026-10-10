//! A Rust port of the parts of bounce 1.8.1 ( @perplexdotgg/bounce, MIT )
//! that webgpu_postprocessing_ssgi_ballpool runs: sphere and box bodies in a
//! `World` stepped at a fixed rate. The arithmetic, pools and iteration
//! orders follow the JS build, so the bodies follow the original's
//! trajectories.
mod epa;
mod gjk;
pub mod math;
mod pool;
pub mod trig;
pub mod world;

pub use math::{Quat, Vec3};
pub use world::{BodyType, Options, World};
