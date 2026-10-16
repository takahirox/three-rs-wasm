//! TeapotGeometry( size, segments, bottom, lid, body, fitLid, blinn ).
use crate::{Result, geometry::*};

pub(super) fn teapot(
    size: f64,
    segments: u32,
    bottom: bool,
    lid: bool,
    body: bool,
    fit_lid: bool,
    blinn: bool,
) -> Result<BufferGeometry> {
    TeapotGeometry::build(size, segments, bottom, lid, body, fit_lid, blinn)
}
