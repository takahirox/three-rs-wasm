//! Resident compute skinning positions and per-dispatch displacement.
//! Static vertex influences are uploaded once; updates upload only bone matrices.
use crate::{
    Error, Result,
    compute::{BufferAccess, ComputeKernel, GpuBuffer},
    renderer::Renderer,
    scene::{Object3D, Scene},
};

pub struct ComputeSkinning {
    pub positions: GpuBuffer,
    pub displacement: GpuBuffer,
    palette: GpuBuffer,
    kernel: ComputeKernel,
    object: Object3D,
    count: u32,
    joints: usize,
}
impl ComputeSkinning {
    /// World-space output; currently requires a skin without morph targets.
    pub async fn new(r: &Renderer, scene: &Scene, object: Object3D) -> Result<Self> {
        let node = scene.get(object)?;
        let skin = node
            .skin
            .as_ref()
            .ok_or(Error::Invalid("compute skinning requires skin"))?;
        let geometry = node
            .geometry()
            .ok_or(Error::Invalid("compute skinning geometry"))?;
        if !geometry.morph_attributes.is_empty()
            || skin.joints.is_empty()
            || skin.joints.len() != skin.inverse_bind_matrices.len()
        {
            return Err(Error::Invalid("compute skinning skin/morph configuration"));
        }
        let attribute = |name| {
            geometry
                .attributes
                .get(name)
                .ok_or(Error::Invalid("compute skinning attribute"))
        };
        let p = attribute("position")?;
        let j = attribute("skinIndex")?;
        let w = attribute("skinWeight")?;
        let count = p.count();
        if count == 0
            || count > u32::MAX as usize
            || p.item_size() != 3
            || j.item_size() != 4
            || w.item_size() != 4
            || j.count() != count
            || w.count() != count
        {
            return Err(Error::Invalid("compute skinning dimensions"));
        }
        let mut input = Vec::<[f32; 4]>::with_capacity(count * 3);
        for i in 0..count {
            let pos = p.vector3(i)?;
            if !pos.is_finite() {
                return Err(Error::Invalid("compute skinning position"));
            }
            input.push([pos.x as f32, pos.y as f32, pos.z as f32, 1.]);
            let mut indices = [0.; 4];
            let mut weights = [0.; 4];
            for k in 0..4 {
                let index = j.get_component(i, k)?;
                let weight = w.get_component(i, k)?;
                if !index.is_finite()
                    || index < 0.
                    || index.fract() != 0.
                    || index >= skin.joints.len() as f64
                    || !weight.is_finite()
                    || weight < 0.
                {
                    return Err(Error::Invalid("compute skinning influence"));
                }
                indices[k] = index as f32;
                weights[k] = weight as f32;
            }
            input.push(indices);
            input.push(weights);
        }
        let input = GpuBuffer::new(r, bytemuck::cast_slice(&input), BufferAccess::Read)?;
        let palette = GpuBuffer::zeroed(r, (skin.joints.len() * 64) as u64, BufferAccess::Read)?;
        let positions = GpuBuffer::zeroed(r, count as u64 * 16, BufferAccess::ReadWrite)?;
        let displacement = GpuBuffer::zeroed(r, count as u64 * 16, BufferAccess::ReadWrite)?;
        let kernel = ComputeKernel::new(
            r,
            include_str!("shaders/compute_skinning.wgsl"),
            &[&input, &palette, &positions, &displacement],
        )
        .await?;
        Ok(Self {
            positions,
            displacement,
            palette,
            kernel,
            object,
            count: count as u32,
            joints: skin.joints.len(),
        })
    }
    pub fn count(&self) -> u32 {
        self.count
    }
    /// Caller updates world matrices first. No vertex data is read back to the CPU.
    pub fn update(&self, r: &Renderer, scene: &Scene) -> Result<()> {
        let skin = scene
            .get(self.object)?
            .skin
            .as_ref()
            .ok_or(Error::Invalid("compute skinning removed skin"))?;
        if skin.joints.len() != self.joints || skin.inverse_bind_matrices.len() != self.joints {
            return Err(Error::Invalid("compute skinning changed skeleton"));
        }
        let mut matrices = Vec::with_capacity(self.joints);
        for (&joint, bind) in skin.joints.iter().zip(&skin.inverse_bind_matrices) {
            let m = scene.get(joint)?.matrix_world * *bind;
            if !m.is_finite() {
                return Err(Error::Invalid("compute skinning matrix"));
            }
            matrices.push(m.as_mat4().to_cols_array());
        }
        self.palette.write(r, 0, bytemuck::cast_slice(&matrices))?;
        self.kernel.dispatch(r, [self.count.div_ceil(64), 1, 1])
    }
}
