use std::sync::Arc;
use three_rs_wasm::{
    camera::*, geometry::*, material::*, math::*, render_target::*, renderer::*, scene::*,
};

#[test]
fn compressed_mips_render_and_remain_resident() {
    let renderer = pollster::block_on(Renderer::new()).unwrap();
    for bytes in [
        include_bytes!("fixtures/ktx2/2d_etc1s.ktx2").as_slice(),
        include_bytes!("fixtures/ktx2/2d_uastc.ktx2").as_slice(),
    ] {
        let texture = Texture::from_basis_compressed(bytes.to_vec(), true).unwrap();
        assert!(texture.rgba.is_empty());
        assert_eq!((texture.width, texture.height), (40, 40));
        let mut scene = Scene::new();
        let camera = scene.insert(NodeKind::Camera(Camera::Perspective(
            PerspectiveCamera::default(),
        )));
        scene.get_mut(camera).unwrap().position.z = 3.0;
        let mut material = MeshBasicMaterial::default();
        material.properties.map = Some(Arc::new(texture));
        scene.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(BoxGeometry::build(1.0, 1.0, 1.0).unwrap()),
            Arc::new(Material::Basic(material)),
        )));
        let target = RenderTarget::new(&renderer.device, 32, 32).unwrap();
        renderer.render(&mut scene, camera, &target).unwrap();
        let image = renderer.read_rgba(&target).unwrap();
        assert!(image.chunks_exact(4).any(|p| p[0] > 128 && p[1] > 80));
        let counts = renderer.resource_counts();
        renderer.render(&mut scene, camera, &target).unwrap();
        assert_eq!(renderer.resource_counts(), counts);
    }
    assert!(Texture::from_basis_compressed(b"invalid".to_vec(), true).is_err());
}

#[test]
fn reinhard_maps_hdr_after_resolve_without_changing_legacy_aces_selection() {
    let renderer = pollster::block_on(Renderer::new()).unwrap();
    let mut scene = Scene::new();
    scene.background = Color::linear(4.0, 1.0, 0.25);
    let camera = scene.insert(NodeKind::Camera(Camera::Perspective(
        PerspectiveCamera::default(),
    )));
    let source = RenderTarget::with_options(
        &renderer.device,
        4,
        4,
        RenderTargetOptions {
            format: wgpu::TextureFormat::Rgba16Float,
            samples: 4,
            ..Default::default()
        },
    )
    .unwrap();
    let output = RenderTarget::new(&renderer.device, 4, 4).unwrap();
    renderer.render(&mut scene, camera, &source).unwrap();
    renderer.blit_with_tone_mapping(
        &source,
        &output.texture.create_view(&Default::default()),
        output.texture.format(),
        1.0,
        ToneMapping::Reinhard,
    );
    let pixels = renderer.read_rgba(&output).unwrap();
    for p in pixels.chunks_exact(4) {
        for (value, expected) in p[..3].iter().zip([231u8, 188, 124]) {
            assert!(value.abs_diff(expected) <= 1, "{p:?}");
        }
    }
    scene.aces_tone_mapping = true;
    assert_eq!(scene.output_tone_mapping(), ToneMapping::Aces);
    scene.tone_mapping = ToneMapping::Reinhard;
    assert_eq!(scene.output_tone_mapping(), ToneMapping::Reinhard);
}
