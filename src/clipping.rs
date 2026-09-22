use crate::{Error, Result, material::MaterialProperties, scene::Scene};
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub(crate) struct Clipping {
    pub planes: [[f32; 4]; 16],
    pub params: [f32; 4],
}
pub(crate) fn prepare(
    scene: &Scene,
    material: &MaterialProperties,
    shadow: bool,
) -> Result<Clipping> {
    let local = if shadow && !material.clip_shadows {
        &[][..]
    } else {
        &material.clipping_planes
    };
    let global = if shadow && !scene.clipping_shadows {
        &[][..]
    } else {
        &scene.clipping_planes
    };
    if global.len() + local.len() > 16 {
        return Err(Error::Invalid("more than sixteen clipping planes"));
    }
    let mut result = Clipping {
        planes: [[0.0; 4]; 16],
        params: [
            global.len() as f32,
            local.len() as f32,
            f32::from(material.clip_intersection),
            f32::from(material.alpha_to_coverage && !shadow),
        ],
    };
    for (i, plane) in global.iter().chain(local).enumerate() {
        if !plane.normal.is_finite()
            || !plane.constant.is_finite()
            || plane.normal.length_squared() == 0.0
        {
            return Err(Error::Invalid("clipping plane"));
        }
        result.planes[i] = plane.normal.extend(plane.constant).as_vec4().to_array();
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::{Plane, Vector3};
    #[test]
    fn global_and_local_shadow_clipping_are_independent() {
        let mut scene = Scene::new();
        scene.clipping_planes.push(Plane {
            normal: Vector3::X,
            constant: 0.,
        });
        let mut material = MaterialProperties::default();
        material.clipping_planes.push(Plane {
            normal: Vector3::Y,
            constant: 0.,
        });
        material.alpha_to_coverage = true;
        assert_eq!(
            prepare(&scene, &material, false).unwrap().params,
            [1., 1., 0., 1.]
        );
        assert_eq!(
            prepare(&scene, &material, true).unwrap().params,
            [1., 0., 0., 0.]
        );
        scene.clipping_shadows = false;
        material.clip_shadows = true;
        assert_eq!(
            prepare(&scene, &material, true).unwrap().params,
            [0., 1., 0., 0.]
        );
        material.clip_shadows = false;
        assert_eq!(prepare(&scene, &material, true).unwrap().params, [0.; 4]);
    }
}
