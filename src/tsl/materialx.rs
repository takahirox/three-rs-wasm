//! GPU MaterialX noise with the pinned r186 hash and gradient conventions.
use super::{Node, Type, WgslFn, float, uint, vec3, vec4};

/// Coordinate dimensionality. Two-dimensional noise has its own hash and
/// gradients; it is not a slice of three-dimensional noise.
#[derive(Clone, Copy, Debug)]
pub enum Dimension {
    D2,
    D3,
}
fn noise(position: Node, dimension: Dimension, kind: u32, options: Node) -> Node {
    let (position, dimensions) = match dimension {
        Dimension::D2 => (vec3(position.x(), position.y(), float(0.)), 2),
        Dimension::D3 => (position, 3),
    };
    WgslFn::new(
        "mxn_noise",
        include_str!("materialx.wgsl"),
        &[Type::Vec3, Type::Uint, Type::Uint, Type::Vec4],
        Type::Vec3,
    )
    .unwrap()
    .call(&[position, uint(dimensions), uint(kind), options])
}
fn zero() -> Node {
    vec4(vec3(float(0.), float(0.), float(0.)), float(0.))
}
/// Signed scalar Perlin noise (amplitude one, pivot zero).
pub fn perlin(position: Node, dimension: Dimension) -> Node {
    noise(position, dimension, 0, zero()).x()
}
/// Three decorrelated signed Perlin channels.
pub fn perlin_vec3(position: Node, dimension: Dimension) -> Node {
    noise(position, dimension, 1, zero())
}
/// Piecewise constant, deterministic cell value in [0, 1].
pub fn cell(position: Node, dimension: Dimension) -> Node {
    noise(position, dimension, 2, zero()).x()
}
/// Fractal Perlin sum. Octaves are truncated to an integer on the GPU.
/// The caller supplies a nonnegative, bounded octave count.
pub fn fractal(
    position: Node,
    dimension: Dimension,
    octaves: Node,
    lacunarity: Node,
    diminish: Node,
) -> Node {
    noise(
        position,
        dimension,
        3,
        vec4(vec3(octaves, lacunarity, diminish), float(0.)),
    )
    .x()
}
/// Fractal sum of three decorrelated Perlin channels.
pub fn fractal_vec3(
    position: Node,
    dimension: Dimension,
    octaves: Node,
    lacunarity: Node,
    diminish: Node,
) -> Node {
    noise(
        position,
        dimension,
        4,
        vec4(vec3(octaves, lacunarity, diminish), float(0.)),
    )
}
/// Worley distance (style 0) or nearest cell value (style 1).
pub fn worley(position: Node, dimension: Dimension, jitter: Node, style: Node) -> Node {
    noise(
        position,
        dimension,
        5,
        vec4(vec3(jitter, style, float(0.)), float(0.)),
    )
    .x()
}
/// Three nearest Worley distances: metric 0 is Euclidean, 1 squared
/// Euclidean (the r186 vector default), 2 Manhattan, 3 Chebyshev.
pub fn worley_vec3(position: Node, dimension: Dimension, jitter: Node, metric: Node) -> Node {
    noise(
        position,
        dimension,
        5,
        vec4(vec3(jitter, float(0.), metric), float(0.)),
    )
}
