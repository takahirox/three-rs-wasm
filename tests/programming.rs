use std::sync::Arc;
use three_rs_wasm::{
    camera::*, compute::*, geometry::*, material::*, postprocessing::*, renderer::*, scene::*,
    shader::ShaderProgram,
};

#[test]
fn compute_output_drives_custom_material_and_composable_effects() {
    let renderer = pollster::block_on(Renderer::new()).unwrap();
    let data = GpuBuffer::new(
        &renderer,
        bytemuck::cast_slice(&[0.0f32; 4]),
        BufferAccess::ReadWrite,
    )
    .unwrap();
    let kernel = pollster::block_on(ComputeKernel::new(
        &renderer,
        r#"
        @group(0) @binding(0) var<storage,read_write> data:array<f32>;
        @compute @workgroup_size(1) fn main(){data[0]+=0.25;}
    "#,
        &[&data],
    ))
    .unwrap();
    let program=pollster::block_on(ShaderProgram::new(&renderer,r#"
        @group(1) @binding(0) var<storage,read> data:array<f32>;
        fn deform(position:vec3<f32>,normal:vec3<f32>,uv:vec2<f32>)->vec3<f32>{return position;}
        fn shade(surface:VertexOut,base:vec4<f32>)->vec4<f32>{return vec4(data[0],u.custom[0].x,0.0,1.0);}
    "#,&[&data])).unwrap();
    let mut material = ShaderMaterial::new(Arc::new(program));
    material.uniforms[0][0] = 0.5;
    let mut scene = Scene::new();
    let camera = scene.insert(NodeKind::Camera(Camera::Perspective(
        PerspectiveCamera::default(),
    )));
    scene.get_mut(camera).unwrap().position.z = 2.0;
    scene.insert(NodeKind::Mesh(Mesh::new(
        Arc::new(PlaneGeometry::build(2.0, 2.0, 1, 1).unwrap()),
        Arc::new(Material::Shader(material)),
    )));
    let input = RenderTarget::new(&renderer.device, 16, 16).unwrap();
    kernel.dispatch(&renderer, [1, 1, 1]).unwrap();
    renderer.render(&mut scene, camera, &input).unwrap();
    let pixels = renderer.read_rgba(&input).unwrap();
    assert_eq!(
        &pixels[(8 * 16 + 8) * 4..(8 * 16 + 8) * 4 + 4],
        &[137, 188, 0, 255]
    );
    kernel.dispatch(&renderer, [1, 1, 1]).unwrap();
    renderer.render(&mut scene, camera, &input).unwrap();
    let values = data.read(&renderer).unwrap();
    assert_eq!(bytemuck::cast_slice::<u8, f32>(&values)[0], 0.5);
    let mut effect=pollster::block_on(Effect::new(&renderer,wgpu::TextureFormat::Rgba8UnormSrgb,r#"
        fn effect(uv:vec2<f32>)->vec4<f32>{let c=textureSample(input_texture,input_sampler,uv);return vec4(vec3(c.b,c.r,c.g)*params[0].x,c.a);}
    "#)).unwrap();
    effect.parameters[0][0] = 1.0;
    let output = RenderTarget::new(&renderer.device, 16, 16).unwrap();
    effect.apply(&renderer, &input, None, &output).unwrap();
    let pixels = renderer.read_rgba(&output).unwrap();
    assert_eq!(
        &pixels[(8 * 16 + 8) * 4..(8 * 16 + 8) * 4 + 4],
        &[0, 188, 188, 255]
    );
    effect.parameters[0][0] = 0.5;
    effect.apply(&renderer, &input, None, &output).unwrap();
    let pixels = renderer.read_rgba(&output).unwrap();
    assert_eq!(
        &pixels[(8 * 16 + 8) * 4..(8 * 16 + 8) * 4 + 4],
        &[0, 137, 137, 255]
    );
    assert!(effect.apply(&renderer, &input, None, &input).is_err());
    assert!(kernel.dispatch(&renderer, [u32::MAX, 1, 1]).is_err());
    assert!(data.write(&renderer, 1, &[0; 4]).is_err());
    assert!(pollster::block_on(ComputeKernel::new(&renderer, "invalid WGSL", &[])).is_err());
    assert!(pollster::block_on(ShaderProgram::new(&renderer, "invalid WGSL", &[])).is_err());
    assert!(
        pollster::block_on(Effect::new(
            &renderer,
            wgpu::TextureFormat::Rgba8Unorm,
            "invalid WGSL"
        ))
        .is_err()
    );
    for source in [GAUSSIAN_BLUR, BRIGHT_PASS, BLOOM_COMPOSITE] {
        let effect = pollster::block_on(Effect::new(
            &renderer,
            wgpu::TextureFormat::Rgba8UnormSrgb,
            source,
        ))
        .unwrap();
        effect
            .apply(&renderer, &input, Some(&input), &output)
            .unwrap();
    }
}
