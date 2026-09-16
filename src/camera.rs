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
        }
    }
}
impl PerspectiveCamera {
    pub fn projection_matrix(&self) -> Result<Matrix4> {
        if !self.fov.is_finite()
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
        Ok(Matrix4::from_cols_array(&[
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
            -f / (f - n),
            -1.0,
            0.0,
            0.0,
            -f * n / (f - n),
            0.0,
        ]))
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
        if self.left == self.right
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
