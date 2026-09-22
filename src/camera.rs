use crate::{Error, Result, math::*};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct ViewOffset {
    pub full_width: f64,
    pub full_height: f64,
    pub offset_x: f64,
    pub offset_y: f64,
    pub width: f64,
    pub height: f64,
}
impl ViewOffset {
    fn valid(self) -> bool {
        [
            self.full_width,
            self.full_height,
            self.offset_x,
            self.offset_y,
            self.width,
            self.height,
        ]
        .iter()
        .all(|v| v.is_finite())
            && self.full_width > 0.0
            && self.full_height > 0.0
            && self.width > 0.0
            && self.height > 0.0
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PerspectiveCamera {
    pub fov: f64,
    pub aspect: f64,
    pub near: f64,
    pub far: f64,
    pub zoom: f64,
    pub film_gauge: f64,
    pub film_offset: f64,
    pub view: Option<ViewOffset>,
    /// Optional near clipping plane in camera space, for planar reflections.
    /// Positive plane distances are retained. Uses WebGPU's zero-to-one depth.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub oblique_clip_plane: Option<Vector4>,
    /// Explicit projection for composed/recursive render cameras.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub projection_override: Option<Matrix4>,
}
impl Default for PerspectiveCamera {
    fn default() -> Self {
        Self {
            fov: 50.0,
            aspect: 1.0,
            near: 0.1,
            far: 2000.0,
            zoom: 1.0,
            film_gauge: 35.0,
            film_offset: 0.0,
            view: None,
            oblique_clip_plane: None,
            projection_override: None,
        }
    }
}
impl PerspectiveCamera {
    pub fn projection_matrix(&self) -> Result<Matrix4> {
        if let Some(matrix) = self.projection_override {
            if !matrix.is_finite() || matrix.determinant() == 0. {
                return Err(Error::Invalid("custom camera projection"));
            }
            return Ok(matrix);
        }
        if ![
            self.fov,
            self.aspect,
            self.near,
            self.zoom,
            self.film_gauge,
            self.film_offset,
        ]
        .iter()
        .all(|v| v.is_finite())
            || self.far.is_nan()
            || self.film_gauge <= 0.0
            || self.view.is_some_and(|v| !v.valid())
            || self.fov <= 0.0
            || self.fov >= 180.0
            || self.aspect <= 0.0
            || self.near <= 0.0
            || self.far <= self.near
            || self.zoom <= 0.0
        {
            return Err(Error::Invalid("perspective camera"));
        }
        let mut top = self.near * (0.5 * self.fov.to_radians()).tan() / self.zoom;
        let mut height = 2.0 * top;
        let mut width = self.aspect * height;
        let mut left = -0.5 * width;
        if let Some(v) = self.view {
            left += v.offset_x * width / v.full_width;
            top -= v.offset_y * height / v.full_height;
            width *= v.width / v.full_width;
            height *= v.height / v.full_height;
        }
        left += self.near * self.film_offset / (self.film_gauge * self.aspect.min(1.0));
        let right = left + width;
        let bottom = top - height;
        let n = self.near;
        let f = self.far;
        let projection = Matrix4::from_cols_array(&[
            2.0 * n / (right - left),
            0.0,
            0.0,
            0.0,
            0.0,
            2.0 * n / (top - bottom),
            0.0,
            0.0,
            (right + left) / (right - left),
            (top + bottom) / (top - bottom),
            if f.is_infinite() { -1.0 } else { -f / (f - n) },
            -1.0,
            0.0,
            0.0,
            if f.is_infinite() {
                -n
            } else {
                -f * n / (f - n)
            },
            0.0,
        ]);
        match self.oblique_clip_plane {
            Some(plane) => crate::reflection::oblique_projection(projection, plane),
            None => Ok(projection),
        }
    }
    pub fn set_focal_length(&mut self, focal_length: f64) {
        self.fov = (0.5 * self.film_gauge / self.aspect.max(1.0) / focal_length)
            .atan()
            .to_degrees()
            * 2.0;
    }
    pub fn focal_length(&self) -> f64 {
        0.5 * self.film_gauge / self.aspect.max(1.0) / (0.5 * self.fov.to_radians()).tan()
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OrthographicCamera {
    pub left: f64,
    pub right: f64,
    pub top: f64,
    pub bottom: f64,
    pub near: f64,
    pub far: f64,
    pub zoom: f64,
    pub view: Option<ViewOffset>,
}
impl Default for OrthographicCamera {
    fn default() -> Self {
        Self {
            left: -1.0,
            right: 1.0,
            top: 1.0,
            bottom: -1.0,
            near: 0.1,
            far: 2000.0,
            zoom: 1.0,
            view: None,
        }
    }
}
impl OrthographicCamera {
    pub fn projection_matrix(&self) -> Result<Matrix4> {
        if ![
            self.left,
            self.right,
            self.top,
            self.bottom,
            self.near,
            self.far,
            self.zoom,
        ]
        .iter()
        .all(|v| v.is_finite())
            || self.view.is_some_and(|v| !v.valid())
            || self.left == self.right
            || self.top == self.bottom
            || self.near < 0.0
            || self.far <= self.near
            || self.zoom <= 0.0
        {
            return Err(Error::Invalid("orthographic camera"));
        }
        let dx = (self.right - self.left) / (2.0 * self.zoom);
        let dy = (self.top - self.bottom) / (2.0 * self.zoom);
        let cx = (self.right + self.left) * 0.5;
        let cy = (self.top + self.bottom) * 0.5;
        let mut l = cx - dx;
        let mut r = cx + dx;
        let mut t = cy + dy;
        let mut b = cy - dy;
        if let Some(v) = self.view {
            let sx = (self.right - self.left) / v.full_width / self.zoom;
            let sy = (self.top - self.bottom) / v.full_height / self.zoom;
            l += sx * v.offset_x;
            r = l + sx * v.width;
            t -= sy * v.offset_y;
            b = t - sy * v.height;
        }
        Ok(Matrix4::orthographic_rh(l, r, b, t, self.near, self.far))
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Camera {
    Perspective(PerspectiveCamera),
    Orthographic(OrthographicCamera),
}
impl Camera {
    pub fn projection_matrix(&self) -> Result<Matrix4> {
        match self {
            Self::Perspective(c) => c.projection_matrix(),
            Self::Orthographic(c) => c.projection_matrix(),
        }
    }
    pub fn ray(&self, ndc: Vector2, world: Matrix4) -> Result<Ray> {
        let inv = self.projection_matrix()?.inverse();
        Ok(match self {
            Self::Perspective(_) => {
                let origin = world.transform_point3(Vector3::ZERO);
                let target =
                    world.transform_point3(inv.project_point3(Vector3::new(ndc.x, ndc.y, 0.5)));
                Ray {
                    origin,
                    direction: (target - origin).normalize(),
                }
            }
            Self::Orthographic(c) => {
                let origin = world.transform_point3(inv.project_point3(Vector3::new(
                    ndc.x,
                    ndc.y,
                    c.near / (c.near - c.far),
                )));
                Ray {
                    origin,
                    direction: world.transform_vector3(Vector3::NEG_Z).normalize(),
                }
            }
        })
    }
}
