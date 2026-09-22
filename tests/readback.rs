use std::sync::Arc;
use three_rs_wasm::{
    camera::*,
    geometry::*,
    material::*,
    readback::RgbaReadback,
    renderer::*,
    scene::*,
    tsl::{self, *},
};
#[test]
fn asynchronous_readback_selects_mrt_attachment_and_strips_row_padding() {
    pollster::block_on(async {
        let r = Renderer::new().await.unwrap();
        let mut s = Scene::new();
        let c = s.insert(NodeKind::Camera(Camera::Orthographic(OrthographicCamera {
            left: -1.,
            right: 1.,
            top: 1.,
            bottom: -1.,
            near: 0.,
            far: 4.,
            ..Default::default()
        })));
        s.get_mut(c).unwrap().position.z = 2.;
        let program = tsl::surface::SurfaceNodes::default()
            .build_mrt(
                &r,
                &[
                    vec4(vec3(float(1.), float(0.), float(0.)), float(1.)),
                    vec4(vec3(float(0.), float(1.), float(0.)), float(1.)),
                ],
                &[],
                &[],
            )
            .await
            .unwrap();
        let mut m = MeshBasicMaterial::default();
        m.properties.vertex_program = Some(Arc::new(program));
        s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(PlaneGeometry::build(2., 2., 1, 1).unwrap()),
            Arc::new(Material::Basic(m)),
        )));
        let target = RenderTarget::with_options(
            &r.device,
            13,
            7,
            RenderTargetOptions {
                count: 2,
                ..Default::default()
            },
        )
        .unwrap();
        r.render(&mut s, c, &target).unwrap();
        let reader = RgbaReadback::new(&r, 13, 7).unwrap();
        assert!(reader.take().is_none());
        assert!(reader.begin(&r, &target, 2).is_err());
        for attachment in [0, 1, 0] {
            reader.begin(&r, &target, attachment).unwrap();
            assert!(reader.begin(&r, &target, attachment).is_err());
            r.device.poll(wgpu::PollType::Wait).unwrap();
            let bytes = reader.take().unwrap().unwrap();
            assert_eq!(bytes.len(), 13 * 7 * 4);
            let color = if attachment == 0 {
                [255, 0, 0, 255]
            } else {
                [0, 255, 0, 255]
            };
            assert!(bytes.as_chunks::<4>().0.iter().all(|p| *p == color));
            assert!(reader.is_idle());
        }
    });
}

#[test]
fn buffer_readback_reuses_staging_and_reads_gpu_converted_subranges() {
    use three_rs_wasm::{
        compute::{BufferAccess, GpuBuffer},
        readback::BufferReadback,
        tsl::compute::{BufferCompute, BufferStore},
    };
    pollster::block_on(async {
        let r = Renderer::new().await.unwrap();
        let input = GpuBuffer::new(
            &r,
            bytemuck::cast_slice(&[-2.5f32, 0., 0.3, 1.2, 8.8]),
            BufferAccess::Read,
        )
        .unwrap();
        let output = GpuBuffer::zeroed(&r, 20, BufferAccess::ReadWrite).unwrap();
        let kernel = BufferCompute::new(
            &r,
            5,
            &[(&input, Type::Float), (&output, Type::Uint)],
            &[BufferStore {
                binding: 1,
                index: instance_index(),
                value: storage_element(0, instance_index()).to_uint(),
            }],
        )
        .await
        .unwrap();
        let reader = BufferReadback::new(&r, 12).unwrap();
        assert!(BufferReadback::new(&r, 3).is_err());
        assert!(reader.begin(&r, &output.buffer, 2).is_err());
        assert!(reader.begin(&r, &output.buffer, 12).is_err());
        for (offset, expected) in [(0, [0u32, 0, 0]), (4, [0, 0, 1]), (8, [0, 1, 8])] {
            kernel.dispatch(&r).unwrap();
            reader.begin(&r, &output.buffer, offset).unwrap();
            assert!(reader.begin(&r, &output.buffer, offset).is_err());
            r.device.poll(wgpu::PollType::Wait).unwrap();
            let bytes = reader.take().unwrap().unwrap();
            let actual = bytes
                .as_chunks::<4>()
                .0
                .iter()
                .map(|v| u32::from_le_bytes(*v))
                .collect::<Vec<_>>();
            assert_eq!(actual, expected);
            assert!(reader.is_idle());
        }
    });
}
