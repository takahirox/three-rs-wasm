//! Two stabilized shadow cascades, matching Three.js r186 SunLightShadow.
use crate::{Error, Result, camera::*, math::*, shadow::Shadow};
#[derive(Clone, Debug)]
pub struct SunCascade {
    pub projection_view: Matrix4,
    /// View depth begin, end, and start of the fade into the following cascade.
    pub range: Vector3,
    /// Inset viewport in normalized coordinates within one cascade tile.
    pub viewport: Vector4,
}
pub fn cascades(
    camera: &Camera,
    camera_world: Matrix4,
    light_position: Vector3,
    shadow: Shadow,
    size: u32,
) -> Result<[SunCascade; 2]> {
    if size == 0
        || !camera_world.is_finite()
        || !light_position.is_finite()
        || light_position.length_squared() == 0.
        || !shadow.radius.is_finite()
        || shadow.radius < 0.
        || !shadow.far.is_finite()
        || !shadow.near.is_finite()
        || shadow.near <= 0.
        || shadow.far <= shadow.near
    {
        return Err(Error::Invalid("sun shadow configuration"));
    }
    let (near, far, perspective) = match camera {
        Camera::Perspective(p) => (p.near, p.far, true),
        Camera::Orthographic(p) => (p.near, p.far, false),
    };
    let far = shadow.far.min(far).max(near + 1e-6);
    let inverse = camera.projection_matrix()?.inverse();
    let inset = ((shadow.radius.ceil() + 1.) / size as f64).min(0.25);
    let resolution = size as f64 * (1. - 2. * inset);
    let split = ((near + far) * 0.5
        + if near > 0. {
            near * (far / near).sqrt()
        } else {
            (near + far) * 0.5
        })
        * 0.5;
    let splits = [near, split, far];
    let direction = -light_position.normalize();
    let up = if direction.y.abs() > 0.99 {
        Vector3::Z
    } else {
        Vector3::Y
    };
    let orientation = Matrix4::look_at_rh(Vector3::ZERO, direction, up).inverse();
    let view_to_light = orientation.transpose() * camera_world;
    let mut near_corners = [Vector3::ZERO; 4];
    let mut far_corners = [Vector3::ZERO; 4];
    let mut ceiling = f64::NEG_INFINITY;
    for i in 0..4 {
        let p = inverse.project_point3(Vector3::new(
            if i < 2 { 1. } else { -1. },
            if i == 0 || i == 3 { 1. } else { -1. },
            0.,
        ));
        let q = if perspective {
            p * (far / near)
        } else {
            Vector3::new(p.x, p.y, -far)
        };
        near_corners[i] = view_to_light.transform_point3(p);
        far_corners[i] = view_to_light.transform_point3(q);
        ceiling = ceiling.max(near_corners[i].z).max(far_corners[i].z);
    }
    ceiling += far;
    let mut result = Vec::with_capacity(2);
    let mut previous_fade = near;
    for i in 0..2 {
        let begin = if i == 0 { near } else { previous_fade };
        let end = splits[i + 1];
        let fade = end - 0.1 * (end - splits[i]);
        previous_fade = fade;
        let mut corners = [Vector3::ZERO; 8];
        let mut center = Vector3::ZERO;
        for j in 0..4 {
            corners[j * 2] = near_corners[j].lerp(far_corners[j], (begin - near) / (far - near));
            corners[j * 2 + 1] = near_corners[j].lerp(far_corners[j], (end - near) / (far - near));
            center += corners[j * 2] + corners[j * 2 + 1];
        }
        center /= 8.;
        let mut radius = corners
            .iter()
            .map(|p| p.distance_squared(center))
            .fold(0., f64::max)
            .sqrt();
        let min_z = corners.iter().map(|p| p.z).fold(f64::INFINITY, f64::min);
        if resolution > 1. {
            radius /= 1. - 1. / resolution;
            let texel = 2. * radius / resolution;
            center.x = (center.x / texel + 0.5).floor() * texel;
            center.y = (center.y / texel + 0.5).floor() * texel;
        }
        center.z = ceiling + shadow.near;
        center = orientation.transform_point3(center);
        let mut world = orientation;
        world.w_axis = center.extend(1.);
        let projection = OrthographicCamera {
            left: -radius,
            right: radius,
            top: radius,
            bottom: -radius,
            near: shadow.near,
            far: ceiling - min_z + 2. * shadow.near,
            ..Default::default()
        }
        .projection_matrix()?;
        result.push(SunCascade {
            projection_view: projection * world.inverse(),
            range: Vector3::new(if i == 0 { -1e10 } else { begin }, end, fade),
            viewport: Vector4::new(inset, inset, 1. - 2. * inset, 1. - 2. * inset),
        });
    }
    Ok(result.try_into().unwrap())
}
