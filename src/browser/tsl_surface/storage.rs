use super::*;
use crate::{
    compute::{BufferAccess, GpuBuffer},
    tsl::{compute::*, *},
};
pub(super) struct Storage {
    init: BufferCompute,
    invert: BufferCompute,
    initialized: bool,
    last: Option<i64>,
}
impl Storage {
    pub fn update(&mut self, r: &Renderer, time: f64) -> Result<()> {
        if !self.initialized {
            self.init.dispatch(r)?;
            self.initialized = true;
        }
        let second = time.floor() as i64;
        if self.last != Some(second) {
            self.invert.dispatch(r)?;
            self.last = Some(second);
        }
        Ok(())
    }
}
impl Demo {
    pub(super) async fn storage(scene: &mut Scene, cam: Object3D, r: &Renderer) -> Result<Self> {
        scene.background = Color::from_hex(0x313131);
        scene.get_mut(cam)?.kind = NodeKind::Camera(Camera::Orthographic(OrthographicCamera {
            left: -1.0,
            right: 1.0,
            top: 1.0,
            bottom: -1.0,
            near: 0.0,
            far: 2.0,
            ..Default::default()
        }));
        scene.get_mut(cam)?.position = Vector3::Z;
        scene.look_at(cam, Vector3::ZERO)?;
        let types = [Type::Float, Type::Vec2, Type::Vec3, Type::Vec4];
        let mut buffers = Vec::new();
        for stride in [4, 8, 16, 16] {
            buffers.push(GpuBuffer::zeroed(r, 32 * stride, BufferAccess::ReadWrite)?);
        }
        let bindings: Vec<_> = buffers.iter().zip(types).collect();
        let initial: Vec<_> = types
            .iter()
            .enumerate()
            .map(|(i, ty)| BufferStore {
                binding: i,
                index: instance_index(),
                value: if *ty == Type::Float {
                    instance_index().to_float()
                } else {
                    splat(instance_index().to_float(), *ty)
                },
            })
            .collect();
        let reverse: Vec<_> = (0..4)
            .map(|i| BufferStore {
                binding: i,
                index: instance_index(),
                value: storage_element(i, uint(31) - instance_index()),
            })
            .collect();
        let init = BufferCompute::new(r, 32, &bindings, &initial).await?;
        let invert = BufferCompute::new_workgroup_snapshot(r, 32, &bindings, &reverse).await?;
        let index = WgslFn::new(
            "bar_index",
            "fn bar_index(x:f32)->u32{return u32(min(floor(x*32.0),31.0));}",
            &[Type::Float],
            Type::Uint,
        )?
        .call(&[uv().x()]);
        let values: Vec<_> = (0..4)
            .map(|i| {
                let value = storage_element(i, index.clone());
                let value = if i == 0 { value } else { value.x() };
                (value / float(32.0) * float(32.0)).floor() / float(32.0)
            })
            .collect();
        let mut color = vec3(values[0].clone(), float(0.0), float(0.0));
        for (edge, c) in [
            (0.25, vec3(float(0.0), values[1].clone(), float(0.0))),
            (0.5, vec3(float(0.0), float(0.0), values[2].clone())),
            (0.75, splat(values[3].clone(), Type::Vec3)),
        ] {
            color = uv().y().greater_than(float(edge)).select(c, color);
        }
        let material = NodeMaterial::new(color)
            .build_with_storage(r, &bindings, &[])
            .await?;
        let object = scene.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(PlaneGeometry::build(1.0, 1.0, 1, 1)?),
            Arc::new(Material::Shader(material)),
        )));
        let viewer = OrbitViewer::from_camera(Vector3::ZERO, 1.0);
        Ok(Self {
            example: 74,
            time: 0.0,
            lights: vec![],
            objects: vec![object],
            params: [[0.0; 4]; 16],
            viewer,
            orbit: Vector2::ZERO,
            pan: Vector3::ZERO,
            mixer: None,
            depth: None,
            mrt_sampler: None,
            bloom: None,
            jelly: None,
            mask: None,
            storage: Some(Storage {
                init,
                invert,
                initialized: false,
                last: None,
            }),
        })
    }
}
