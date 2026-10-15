use super::*;
use crate::{attribute::BufferAttribute, compute::ComputeKernel};
pub(super) struct Indirect {
    init: ComputeKernel,
    update: ComputeKernel,
    uniform: GpuBuffer,
}
impl Demo {
    pub(super) async fn indirect(
        &mut self,
        s: &mut Scene,
        c: Object3D,
        r: &Renderer,
    ) -> Result<()> {
        s.background = Color::from_hex(0x00001f);
        self.viewer = OrbitViewer::from_camera(Vector3::ZERO, 3.0_f64.sqrt());
        self.viewer.fixture(
            std::f64::consts::FRAC_PI_4,
            (1.0 / 3.0_f64.sqrt()).asin(),
            1.8,
        );
        self.viewer.update(s, c)?;
        let mut seed = 186;
        let mut offsets = Vec::new();
        let mut colors = Vec::new();
        let mut starts = Vec::new();
        let mut ends = Vec::new();
        for _ in 0..100000 {
            offsets.push([
                (random(&mut seed) - 0.5) as f32,
                (random(&mut seed) - 0.5) as f32,
                (random(&mut seed) - 0.5) as f32,
                0.0,
            ]);
            colors.push(std::array::from_fn::<_, 4, _>(|_| random(&mut seed) as f32));
            for rotations in [&mut starts, &mut ends] {
                let v = Vector4::new(
                    random(&mut seed) * 2.0 - 1.0,
                    random(&mut seed) * 2.0 - 1.0,
                    random(&mut seed) * 2.0 - 1.0,
                    random(&mut seed) * 2.0 - 1.0,
                )
                .normalize();
                rotations.push(v.as_vec4().to_array());
            }
        }
        let buffers = [offsets, colors, starts, ends]
            .iter()
            .map(|a| GpuBuffer::new(r, bytemuck::cast_slice(a), BufferAccess::Read))
            .collect::<Result<Vec<_>>>()?;
        let half = (uniform(0, Type::Float) * float(0.5)).sin();
        let point = instanced_attribute(0).rgb()
            * (half.clone() * float(2.0) + float(1.0))
                .abs()
                .max(float(0.5))
            + position_geometry();
        let q = mix(instanced_attribute(2), instanced_attribute(3), half).normalize();
        let cross = q.rgb().cross(point.clone());
        let position =
            cross.clone() * q.swizzle("w") * float(2.0) + q.rgb().cross(cross) * float(2.0) + point;
        let base = instanced_attribute(1);
        let color = vec4(
            vec3(
                base.x()
                    + (position_local().x() * float(10.0) + uniform(0, Type::Float)).sin()
                        * float(0.5),
                base.y(),
                base.swizzle("z"),
            ),
            base.swizzle("w"),
        );
        let mut material = NodeMaterial {
            position: Some(position),
            color,
        }
        .build_with_storage(
            r,
            &buffers.iter().map(|b| (b, Type::Vec4)).collect::<Vec<_>>(),
            &[],
        )
        .await?;
        material.properties.side = Side::Double;
        material.properties.transparent = true;
        material.properties.force_single_pass = true;
        let commands = GpuBuffer::zeroed(r, 20, BufferAccess::ReadWrite)?;
        let mut geometry = BufferGeometry::default();
        geometry.set_attribute(
            "position",
            Attribute::F32(BufferAttribute::new(
                vec![0.025, -0.025, 0.0, -0.025, 0.025, 0.0, 0.0, 0.0, 0.025],
                3,
                false,
            )?),
        );
        geometry.instance_count = Some(100000);
        geometry.gpu_indirect = Some(commands.buffer.clone());
        let h = mesh(s, Arc::new(geometry), Material::Shader(material));
        s.get_mut(h)?.frustum_culled = false;
        self.objects.push(h);
        let uniform = GpuBuffer::zeroed(r, 16, BufferAccess::Uniform)?;
        let declarations = "struct DrawBuffer{vertexCount:u32,instanceCount:atomic<u32>,firstVertex:u32,firstInstance:u32,offset:u32}; @group(0) @binding(0) var<storage,read_write> draw:DrawBuffer; @group(0) @binding(1) var<uniform> params:vec4<f32>;";
        let init=ComputeKernel::new(r,&format!("{declarations}\n@compute @workgroup_size(64) fn main(@builtin(global_invocation_id) id:vec3<u32>){{if(id.x>=1u){{return;}}draw.vertexCount=3u;atomicStore(&draw.instanceCount,0u);draw.firstVertex=0u;draw.firstInstance=0u;draw.offset=0u;}}"),&[&commands,&uniform]).await?;
        let update=ComputeKernel::new(r,&format!("{declarations}\n@compute @workgroup_size(64) fn main(@builtin(global_invocation_id) id:vec3<u32>){{if(id.x>=100000u){{return;}}let halfTime=sin(params.x*0.5);let count=max(pow(halfTime+1.0,4.0)*100000.0,100.0);atomicStore(&draw.instanceCount,u32(count));}}"),&[&commands,&uniform]).await?;
        self.indirect = Some(Indirect {
            init,
            update,
            uniform,
        });
        Ok(())
    }
}
impl Indirect {
    pub(super) fn render(
        &self,
        r: &Renderer,
        s: &mut Scene,
        c: Object3D,
        target: &RenderTarget,
        time: f64,
    ) -> Result<()> {
        r.render(s, c, target)?;
        self.uniform
            .write(r, 0, bytemuck::cast_slice(&[time as f32, 0.0, 0.0, 0.0]))?;
        self.init.dispatch(r, [1, 1, 1])?;
        self.update.dispatch(r, [100000_u32.div_ceil(64), 1, 1])
    }
}
