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
