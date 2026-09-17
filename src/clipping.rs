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
    if scene.clipping_planes.len() + local.len() > 16 {
        return Err(Error::Invalid("more than sixteen clipping planes"));
    }
    let mut result = Clipping {
        planes: [[0.0; 4]; 16],
        params: [
            scene.clipping_planes.len() as f32,
            local.len() as f32,
            f32::from(material.clip_intersection),
            0.0,
        ],
    };
    for (i, plane) in scene.clipping_planes.iter().chain(local).enumerate() {
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
