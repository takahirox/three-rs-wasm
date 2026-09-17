use std::sync::Arc;
use three_rs_wasm::{camera::*, geometry::*, material::*, math::*, renderer::*, scene::*};
#[test]
fn clipping_stencil_and_custom_blend_control_real_pixels() {
    let renderer = pollster::block_on(Renderer::new()).unwrap();
    let mut scene = Scene::new();
    let camera = scene.insert(NodeKind::Camera(Camera::Orthographic(OrthographicCamera {
        left: -1.0,
        right: 1.0,
        top: 1.0,
        bottom: -1.0,
        near: 0.1,
        far: 10.0,
        ..Default::default()
    })));
    scene.get_mut(camera).unwrap().position.z = 2.0;
    let geometry = Arc::new(PlaneGeometry::build(2.0, 2.0, 1, 1).unwrap());
    let mut mask = Material::default();
    let p = mask.properties_mut();
    p.color_write = false;
    p.depth_write = false;
    p.clipping_planes = vec![Plane {
        normal: Vector3::X,
        constant: 0.0,
    }];
    p.stencil_reference = 1;
    p.stencil = Some(wgpu::StencilState {
        front: wgpu::StencilFaceState {
            compare: wgpu::CompareFunction::Always,
            pass_op: wgpu::StencilOperation::Replace,
            ..Default::default()
        },
        back: wgpu::StencilFaceState::IGNORE,
        read_mask: 255,
        write_mask: 255,
    });
    scene.insert(NodeKind::Mesh(Mesh::new(geometry.clone(), Arc::new(mask))));
    let mut painted = Material::default();
    painted.properties_mut().color = Color::linear(1.0, 0.0, 0.0);
    painted.properties_mut().stencil_reference = 1;
    painted.properties_mut().stencil = Some(wgpu::StencilState {
        front: wgpu::StencilFaceState {
            compare: wgpu::CompareFunction::Equal,
            ..Default::default()
        },
        back: wgpu::StencilFaceState::IGNORE,
        read_mask: 255,
        write_mask: 0,
    });
    let paint = scene.insert(NodeKind::Mesh(Mesh::new(
        geometry.clone(),
        Arc::new(painted),
    )));
    scene.get_mut(paint).unwrap().render_order = 1;
    let target = RenderTarget::with_options(
        &renderer.device,
        32,
        32,
        RenderTargetOptions {
            stencil_buffer: true,
            ..Default::default()
        },
    )
    .unwrap();
    renderer.render(&mut scene, camera, &target).unwrap();
    let pixels = renderer.read_rgba(&target).unwrap();
    assert_eq!(pixels[(16 * 32 + 8) * 4], 0);
    assert_eq!(pixels[(16 * 32 + 24) * 4], 255);
    let mut additive = Material::default();
    additive.properties_mut().color = Color::linear(0.0, 0.25, 0.0);
    additive.properties_mut().blending = Some(wgpu::BlendState {
        color: wgpu::BlendComponent {
            src_factor: wgpu::BlendFactor::One,
            dst_factor: wgpu::BlendFactor::One,
            operation: wgpu::BlendOperation::Add,
        },
        alpha: wgpu::BlendComponent::REPLACE,
    });
    let add = scene.insert(NodeKind::Mesh(Mesh::new(geometry, Arc::new(additive))));
    scene.get_mut(add).unwrap().render_order = 2;
    renderer.render(&mut scene, camera, &target).unwrap();
    let pixels = renderer.read_rgba(&target).unwrap();
    assert_eq!(
        &pixels[(16 * 32 + 24) * 4..(16 * 32 + 24) * 4 + 3],
        &[255, 137, 0]
    );
}
