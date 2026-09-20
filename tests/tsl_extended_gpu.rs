use std::sync::Arc;
use three_rs_wasm::{
    camera::*,
    geometry::*,
    material::*,
    math::*,
    renderer::*,
    scene::*,
    tsl::{self, *},
};
#[test]
fn array_layer_selection_uses_resident_texture_and_rejects_wrong_dimensions() {
    let r = pollster::block_on(Renderer::new()).unwrap();
    let texture = r.device.create_texture(&wgpu::TextureDescriptor {
        label: None,
        size: wgpu::Extent3d {
            width: 1,
            height: 1,
            depth_or_array_layers: 3,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8Unorm,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        view_formats: &[],
    });
    r.queue.write_texture(
        wgpu::TexelCopyTextureInfo {
            texture: &texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
        },
        &[255, 0, 0, 255, 0, 255, 0, 255, 0, 0, 255, 255],
        wgpu::TexelCopyBufferLayout {
            offset: 0,
            bytes_per_row: Some(4),
            rows_per_image: Some(1),
        },
        texture.size(),
    );
    let view = texture.create_view(&wgpu::TextureViewDescriptor {
        dimension: Some(wgpu::TextureViewDimension::D2Array),
        ..Default::default()
    });
    let sampler = r.device.create_sampler(&Default::default());
    let graph =
        NodeMaterial::new(tsl::Texture::External(0).sample_array(uv(), uniform(0, Type::Float)));
    assert!(graph.wgsl(1).is_err());
    assert!(
        NodeMaterial::new(tsl::Texture::External(0).sample(uv()))
            .wgsl_with_texture_types(&[Type::TextureArray], &[])
            .is_err()
    );
    let material = pollster::block_on(
        graph.build_with_texture_types(&r, &[(&view, &sampler, Type::TextureArray)]),
    )
    .unwrap();
    let mut scene = Scene::new();
    let cam = scene.insert(NodeKind::Camera(Camera::Orthographic(OrthographicCamera {
        near: 0.0,
        far: 2.0,
        left: -1.0,
        right: 1.0,
        top: 1.0,
        bottom: -1.0,
        ..Default::default()
    })));
    scene.get_mut(cam).unwrap().position.z = 1.0;
    let h = scene.insert(NodeKind::Mesh(Mesh::new(
        Arc::new(PlaneGeometry::build(2.0, 2.0, 1, 1).unwrap()),
        Arc::new(Material::Shader(material)),
    )));
    let target = RenderTarget::new(&r.device, 8, 8).unwrap();
    for layer in [0, 1, 2, 0] {
        if let NodeKind::Mesh(m) = &mut scene.get_mut(h).unwrap().kind
            && let Material::Shader(m) = Arc::make_mut(&mut m.materials[0])
        {
            m.uniforms[0][0] = layer as f32;
        }
        r.render(&mut scene, cam, &target).unwrap();
        let bytes = r.read_rgba(&target).unwrap();
        let pixel = &bytes[(4 * 8 + 4) * 4..(4 * 8 + 4) * 4 + 4];
        let mut expected = [0, 0, 0, 255];
        expected[layer] = 255;
        assert_eq!(pixel, expected);
    }
}
#[test]
fn euler_gpu_deformation_matches_xyz_geometry_transform() {
    let r = pollster::block_on(Renderer::new()).unwrap();
    let mut scene = Scene::new();
    let cam = scene.insert(NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
        aspect: 1.0,
        near: 0.1,
        far: 10.0,
        ..Default::default()
    })));
    scene.get_mut(cam).unwrap().position.z = 3.0;
    let g = BoxGeometry::build(1.0, 0.7, 0.4).unwrap();
    let angles = Vector3::new(0.31, -0.47, 0.82);
    let graph = NodeMaterial {
        position: Some(rotate_euler(
            position_geometry(),
            vec3(
                float(angles.x as f32),
                float(angles.y as f32),
                float(angles.z as f32),
            ),
        )),
        color: vec3(float(0.7), float(0.2), float(0.1)),
    };
    let material = pollster::block_on(graph.build(&r, &[])).unwrap();
    let h = scene.insert(NodeKind::Mesh(Mesh::new(
        Arc::new(g.clone()),
        Arc::new(Material::Shader(material)),
    )));
    let target = RenderTarget::new(&r.device, 64, 64).unwrap();
    r.render(&mut scene, cam, &target).unwrap();
    let actual = r.read_rgba(&target).unwrap();
    let mut g = g;
    g.apply_matrix4(Matrix4::from_quat(Quaternion::from_euler(
        glam::EulerRot::XYZ,
        angles.x,
        angles.y,
        angles.z,
    )))
    .unwrap();
    let material = pollster::block_on(
        NodeMaterial::new(vec3(float(0.7), float(0.2), float(0.1))).build(&r, &[]),
    )
    .unwrap();
    scene.get_mut(h).unwrap().kind =
        NodeKind::Mesh(Mesh::new(Arc::new(g), Arc::new(Material::Shader(material))));
    r.render(&mut scene, cam, &target).unwrap();
    assert_eq!(actual, r.read_rgba(&target).unwrap());
}

#[test]
fn occlusion_queries_follow_depth_and_reuse_readback() {
    let r = pollster::block_on(Renderer::new()).unwrap();
    let mut scene = Scene::new();
    let camera = scene.insert(NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
        fov: 50.0,
        aspect: 1.0,
        near: 0.01,
        far: 100.0,
        ..Default::default()
    })));
    scene.get_mut(camera).unwrap().position.z = 7.0;
    let material = Arc::new(Material::Basic(MeshBasicMaterial::default()));
    scene.insert(NodeKind::Mesh(Mesh::new(
        Arc::new(PlaneGeometry::build(2.0, 2.0, 1, 1).unwrap()),
        material.clone(),
    )));
    let sphere = scene.insert(NodeKind::Mesh(Mesh::new(
        Arc::new(SphereGeometry::build(0.5, 32, 16).unwrap()),
        material,
    )));
    scene.get_mut(sphere).unwrap().position.z = -1.0;
    let query = three_rs_wasm::occlusion::OcclusionQueries::new(&r, &[sphere], 1).unwrap();
    let target = RenderTarget::new(&r.device, 64, 64).unwrap();
    assert_eq!(query.is_occluded(sphere), None);
    for (x, hidden) in [(0.0, true), (2.0, false), (0.0, true)] {
        scene.get_mut(sphere).unwrap().position.x = x;
        r.render_with_occlusion(&mut scene, camera, &target, Some(&query))
            .unwrap();
        r.device.poll(wgpu::PollType::Wait).unwrap();
        assert_eq!(query.is_occluded(sphere), Some(hidden));
    }
}

#[test]
fn cube_mips_preserve_faces_and_explicit_levels() {
    let r = pollster::block_on(Renderer::new()).unwrap();
    let face = |size, rgba: [u8; 4]| {
        let mut t = three_rs_wasm::material::Texture::from_rgba(
            size,
            size,
            rgba.repeat((size * size) as usize),
            false,
        )
        .unwrap();
        t.srgb = false;
        t
    };
    let colors = [
        [255, 0, 0, 255],
        [0, 255, 0, 255],
        [0, 0, 255, 255],
        [255, 255, 0, 255],
        [255, 0, 255, 255],
        [0, 255, 255, 255],
    ];
    let base = std::array::from_fn(|i| face(4, colors[i]));
    let manual = GpuTexture::from_cube_mipmaps(
        &r,
        &[
            base.clone(),
            std::array::from_fn(|_| face(2, [128, 64, 32, 255])),
            std::array::from_fn(|_| face(1, [16, 32, 64, 255])),
        ],
    )
    .unwrap();
    let auto = GpuTexture::from_cube_rgba(&r, &base).unwrap();
    assert!(GpuTexture::from_cube_mipmaps(&r, &[]).is_err());
    assert!(GpuTexture::from_cube_mipmaps(&r, &[base.clone(), base]).is_err());
    let sample=WgslFn::new("cube_lod","fn cube_lod(t:texture_cube<f32>,s:sampler,d:vec3<f32>,level:f32)->vec4<f32>{return textureSampleLevel(t,s,d,level);}",&[Type::TextureCube,Type::Sampler,Type::Vec3,Type::Float],Type::Vec4).unwrap();
    for (cube, explicit) in [(&auto, false), (&manual, true)] {
        let graph = NodeMaterial::new(sample.call(&[
            tsl::Texture::External(0).node(),
            tsl::Texture::External(0).sampler(),
            uniform(0, Type::Vec3),
            uniform(1, Type::Float),
        ]));
        let material = pollster::block_on(
            graph.build_with_texture_types(&r, &[(&cube.view, &cube.sampler, Type::TextureCube)]),
        )
        .unwrap();
        let mut scene = Scene::new();
        let cam = scene.insert(NodeKind::Camera(Camera::Orthographic(OrthographicCamera {
            near: 0.0,
            far: 2.0,
            left: -1.0,
            right: 1.0,
            top: 1.0,
            bottom: -1.0,
            ..Default::default()
        })));
        scene.get_mut(cam).unwrap().position.z = 1.0;
        let object = scene.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(PlaneGeometry::build(2.0, 2.0, 1, 1).unwrap()),
            Arc::new(Material::Shader(material)),
        )));
        let target = RenderTarget::with_options(
            &r.device,
            4,
            4,
            RenderTargetOptions {
                format: wgpu::TextureFormat::Rgba8Unorm,
                ..Default::default()
            },
        )
        .unwrap();
        for (index, dir) in [
            [1., 0., 0., 0.],
            [-1., 0., 0., 0.],
            [0., 1., 0., 0.],
            [0., -1., 0., 0.],
            [0., 0., 1., 0.],
            [0., 0., -1., 0.],
        ]
        .iter()
        .enumerate()
        {
            for level in 0..3 {
                if let NodeKind::Mesh(m) = &mut scene.get_mut(object).unwrap().kind
                    && let Material::Shader(m) = Arc::make_mut(&mut m.materials[0])
                {
                    m.uniforms[0] = *dir;
                    m.uniforms[1][0] = level as f32;
                }
                r.render(&mut scene, cam, &target).unwrap();
                let data = r.read_rgba(&target).unwrap();
                let expected = if explicit && level > 0 {
                    if level == 1 {
                        [128, 64, 32, 255]
                    } else {
                        [16, 32, 64, 255]
                    }
                } else {
                    colors[index]
                };
                assert_eq!(&data[20..24], &expected);
            }
        }
    }
}

#[test]
fn color_load_keeps_the_previous_viewport() {
    let r = pollster::block_on(Renderer::new()).unwrap();
    let mut s = Scene::new();
    let c = s.insert(NodeKind::Camera(Camera::Orthographic(OrthographicCamera {
        near: 0.0,
        far: 2.0,
        left: -1.0,
        right: 1.0,
        top: 1.0,
        bottom: -1.0,
        ..Default::default()
    })));
    s.get_mut(c).unwrap().position.z = 1.0;
    let m = pollster::block_on(NodeMaterial::new(uniform(0, Type::Vec3)).build(&r, &[])).unwrap();
    let h = s.insert(NodeKind::Mesh(Mesh::new(
        Arc::new(PlaneGeometry::build(2.0, 2.0, 1, 1).unwrap()),
        Arc::new(Material::Shader(m)),
    )));
    let mut target = RenderTarget::with_options(
        &r.device,
        8,
        4,
        RenderTargetOptions {
            format: wgpu::TextureFormat::Rgba8Unorm,
            ..Default::default()
        },
    )
    .unwrap();
    for (x, color, load) in [(0, [1., 0., 0., 0.], false), (4, [0., 1., 0., 0.], true)] {
        if let NodeKind::Mesh(m) = &mut s.get_mut(h).unwrap().kind
            && let Material::Shader(m) = Arc::make_mut(&mut m.materials[0])
        {
            m.uniforms[0] = color;
        }
        target.viewport = [x, 0, 4, 4];
        target.scissor = Some(target.viewport);
        target.set_load_color(load);
        r.render(&mut s, c, &target).unwrap();
    }
    let pixels = r.read_rgba(&target).unwrap();
    for y in 0..4 {
        for x in 0..8 {
            assert_eq!(
                &pixels[(y * 8 + x) * 4..(y * 8 + x) * 4 + 4],
                if x < 4 {
                    &[255, 0, 0, 255]
                } else {
                    &[0, 255, 0, 255]
                }
            );
        }
    }
}

#[test]
fn compute_writes_native_draw_indirect_without_geometry_uploads() {
    use three_rs_wasm::compute::{BufferAccess, ComputeKernel, GpuBuffer};
    let r = pollster::block_on(Renderer::new()).unwrap();
    let commands = GpuBuffer::zeroed(&r, 20, BufferAccess::ReadWrite).unwrap();
    let parameter = GpuBuffer::zeroed(&r, 16, BufferAccess::Uniform).unwrap();
    let kernel=pollster::block_on(ComputeKernel::new(&r,"struct Draw { count:u32, instances:u32, first:u32, base:i32, first_instance:u32 }; @group(0) @binding(0) var<storage,read_write> draw:Draw; @group(0) @binding(1) var<uniform> show:vec4<u32>; @compute @workgroup_size(1) fn main(){draw.count=6u;draw.instances=show.x;draw.first=0u;draw.base=0;draw.first_instance=0u;}",&[&commands,&parameter])).unwrap();
    let mut s = Scene::new();
    let c = s.insert(NodeKind::Camera(Camera::Orthographic(OrthographicCamera {
        near: 0.0,
        far: 2.0,
        left: -1.0,
        right: 1.0,
        top: 1.0,
        bottom: -1.0,
        ..Default::default()
    })));
    s.get_mut(c).unwrap().position.z = 1.0;
    let mut geometry = PlaneGeometry::build(2.0, 2.0, 1, 1).unwrap();
    geometry.gpu_indirect = Some(commands.buffer.clone());
    s.insert(NodeKind::Mesh(Mesh::new(
        Arc::new(geometry),
        Arc::new(Material::Basic(MeshBasicMaterial::default())),
    )));
    let target = RenderTarget::new(&r.device, 8, 8).unwrap();
    for show in [1u32, 0, 1] {
        parameter
            .write(&r, 0, bytemuck::cast_slice(&[show, 0, 0, 0]))
            .unwrap();
        kernel.dispatch(&r, [1, 1, 1]).unwrap();
        r.render(&mut s, c, &target).unwrap();
        let bytes = r.read_rgba(&target).unwrap();
        assert_eq!(bytes[4 * 36], if show == 1 { 255 } else { 0 });
    }
}
