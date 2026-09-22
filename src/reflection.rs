//! Planar reflection camera construction, following r186 ReflectorNode.
//! Only camera/plane transforms run on the CPU; geometry is rendered on the GPU.
use crate::{Error, Result, camera::PerspectiveCamera, math::*};

/// Replace the near plane of a zero-to-one depth projection. This clips geometry
/// during rasterization, rather than evaluating a fragment discard afterwards.
pub fn oblique_projection(mut projection: Matrix4, plane: Vector4) -> Result<Matrix4> {
    if !projection.is_finite()
        || !plane.is_finite()
        || plane.truncate().length_squared() == 0.
        || projection.determinant() == 0.
    {
        return Err(Error::Invalid("oblique projection"));
    }
    let sign = |x: f64| {
        if x > 0. {
            1.
        } else if x < 0. {
            -1.
        } else {
            0.
        }
    };
    let q = projection.inverse() * Vector4::new(sign(plane.x), sign(plane.y), 1., 1.);
    let denominator = plane.dot(q);
    if !denominator.is_finite() || denominator.abs() < 1e-12 {
        return Err(Error::Invalid("degenerate oblique plane"));
    }
    let p = plane / denominator;
    projection.x_axis.z = p.x;
    projection.y_axis.z = p.y;
    projection.z_axis.z = p.z;
    projection.w_axis.z = p.w;
    Ok(projection)
}

/// Reflected camera and world matrix for a world-space plane `(normal, offset)`.
/// Returns None when viewing the back of the reflector. The caller schedules a
/// render into a resident target and excludes the reflecting surface from it.
pub fn planar_camera(
    camera: &PerspectiveCamera,
    world: Matrix4,
    plane: Vector4,
) -> Result<Option<(PerspectiveCamera, Matrix4)>> {
    if !world.is_finite() || !plane.is_finite() || plane.truncate().length_squared() == 0. {
        return Err(Error::Invalid("reflection plane/camera"));
    }
    let length = plane.truncate().length();
    let plane = plane / length;
    let normal = plane.truncate();
    let position = world.w_axis.truncate();
    let distance = normal.dot(position) + plane.w;
    if distance < 0. {
        return Ok(None);
    }
    let reflect = |v: Vector3| v - 2. * normal.dot(v) * normal;
    let reflected_position = position - 2. * distance * normal;
    let forward = reflect(-world.z_axis.truncate().normalize());
    let up = reflect(world.y_axis.truncate().normalize());
    let reflected_world =
        Matrix4::look_at_rh(reflected_position, reflected_position + forward, up).inverse();
    if !reflected_world.is_finite() {
        return Err(Error::Invalid("reflection camera transform"));
    }
    let mut reflected = camera.clone();
    // ReflectorNode copies the incoming projection, including a previous bounce.
    // Its q construction assumes the perspective row and deliberately uses the
    // copied depth coefficients; inverting a previously clipped matrix differs.
    let clip = reflected_world.transpose() * plane;
    let mut p = camera.projection_matrix()?;
    let sign = |x: f64| if x == 0. { 0. } else { x.signum() };
    let q = Vector4::new(
        (sign(clip.x) + p.z_axis.x) / p.x_axis.x,
        (sign(clip.y) + p.z_axis.y) / p.y_axis.y,
        -1.,
        (1. + p.z_axis.z) / p.w_axis.z,
    );
    let denominator = clip.dot(q);
    if !denominator.is_finite() || denominator.abs() < 1e-12 {
        return Err(Error::Invalid("degenerate reflector projection"));
    }
    let clip = clip / denominator;
    p.x_axis.z = clip.x;
    p.y_axis.z = clip.y;
    p.z_axis.z = clip.z;
    p.w_axis.z = clip.w;
    reflected.oblique_clip_plane = None;
    reflected.projection_override = Some(p);
    reflected.projection_matrix()?;
    Ok(Some((reflected, reflected_world)))
}
