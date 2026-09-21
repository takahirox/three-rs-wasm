use std::sync::Arc;
use three_rs_wasm::{
    camera::*,
    geometry::*,
    material::*,
    math::*,
    renderer::*,
    scene::*,
    tsl::{self, lut::Lut3D, *},
};
fn target(r: &Renderer) -> RenderTarget {
    RenderTarget::with_options(
        &r.device,
        16,
        16,
        RenderTargetOptions {
            format: wgpu::TextureFormat::Rgba8Unorm,
            ..Default::default()
        },
    )
    .unwrap()
}
#[test]
fn lut_formats_and_gpu_trilinear_grading() {
    let cube = "LUT_3D_SIZE 2\n0 0 0\n1 0 0\n0 1 0\n1 1 0\n0 0 1\n1 0 1\n0 1 1\n1 1 1\n";
    let lut = Lut3D::from_cube(cube).unwrap();
    assert_eq!(lut.size, 2);
    assert_eq!(&lut.rgba[4..8], &[255, 0, 0, 255]);
    assert!(Lut3D::from_cube("LUT_3D_SIZE 2\n0 0 0").is_err());
    assert!(Lut3D::from_cube(&format!("DOMAIN_MIN -1 0 0\n{cube}")).is_err());
    assert!(Lut3D::from_3dl("0 1 3\n0 0 0").is_err());
    let three_dl =
        "0 1\n0 0 0\n0 0 256\n0 256 0\n0 256 256\n256 0 0\n256 0 256\n256 256 0\n256 256 256\n";
    assert_eq!(Lut3D::from_3dl(three_dl).unwrap().rgba, lut.rgba);
    let strip = Lut3D::from_strip(2, 4, &lut.rgba).unwrap();
    assert_eq!(strip.rgba, lut.rgba);
    let r = pollster::block_on(Renderer::new()).unwrap();
    let input = target(&r);
    let out = target(&r);
    let mut scene = Scene::new();
    scene.background = Color(Vector3::new(0.25, 0.5, 0.75));
    let cam = scene.insert(NodeKind::Camera(Camera::Perspective(Default::default())));
    r.render(&mut scene, cam, &input).unwrap();
    let inverted = Lut3D::new(
        2,
        lut.rgba
            .iter()
            .enumerate()
            .map(|(i, v)| if i % 4 == 3 { 255 } else { 255 - v })
            .collect(),
    )
    .unwrap();
    let mut pass =
        pollster::block_on(inverted.effect(&r, wgpu::TextureFormat::Rgba8Unorm, false)).unwrap();
    for (intensity, expected) in [
        (0.0, [64, 128, 191]),
        (1.0, [191, 127, 64]),
        (0.5, [128, 128, 128]),
    ] {
        pass.parameters[0][0] = intensity;
        pass.apply(&r, &input, None, &out).unwrap();
        let bytes = r.read_rgba(&out).unwrap();
        for c in 0..3 {
            assert!((bytes[c] as i32 - expected[c]).abs() <= 1);
        }
    }
    let encoded_pass =
        pollster::block_on(inverted.effect(&r, wgpu::TextureFormat::Rgba8Unorm, true)).unwrap();
    encoded_pass.apply(&r, &input, None, &out).unwrap();
    let bytes = r.read_rgba(&out).unwrap();
    for (i, expected) in [118, 67, 31].iter().enumerate() {
        assert!((bytes[i] as i32 - expected).abs() <= 1);
    }
}
#[test]
fn sobel_constant_and_vertical_edge() {
    let r = pollster::block_on(Renderer::new()).unwrap();
    let input = target(&r);
    let out = target(&r);
    let blank = target(&r);
    let paint = pollster::block_on(tsl::effect(
        &r,
        wgpu::TextureFormat::Rgba8Unorm,
        &uv()
            .x()
            .greater_than(float(0.5))
            .select(float(0.5), float(0.0)),
    ))
    .unwrap();
    paint.apply(&r, &blank, None, &input).unwrap();
    let pass = pollster::block_on(tsl::effect(
        &r,
        wgpu::TextureFormat::Rgba8Unorm,
        &tsl::display::sobel(
            tsl::Texture::Input,
            uv(),
            splat(float(1.0 / 16.0), Type::Vec2),
        ),
    ))
    .unwrap();
    pass.apply(&r, &input, None, &out).unwrap();
    let bytes = r.read_rgba(&out).unwrap();
    assert_eq!(bytes[(8 * 16 + 2) * 4], 0);
    assert_eq!(bytes[(8 * 16 + 7) * 4], 255);
    assert_eq!(bytes[(8 * 16 + 8) * 4], 255);
    assert_eq!(bytes[(8 * 16 + 12) * 4], 0);
}
#[test]
fn gpu_scene_capture_is_resident_and_restores_source_scene() {
    let r = pollster::block_on(Renderer::new()).unwrap();
    let mut source = Scene::new();
    let mut m = MeshBasicMaterial::default();
    m.properties.side = Side::Back;
    m.properties.color = Color(Vector3::new(0.2, 0.4, 0.8));
    source.insert(NodeKind::Mesh(Mesh::new(
        Arc::new(BoxGeometry::build(10.0, 10.0, 10.0).unwrap()),
        Arc::new(Material::Basic(m)),
    )));
    let before = source.handles().count();
    let env =
        three_rs_wasm::environment::EnvironmentMap::from_scene(&r, &mut source, 32, 0.04).unwrap();
    assert!(env.rgba.is_empty());
    assert_eq!(before, source.handles().count());
    assert!(
        three_rs_wasm::environment::EnvironmentMap::from_scene(&r, &mut source, 31, 0.04).is_err()
    );
    let mut scene = Scene::new();
    scene.environment = Some(Arc::new(env));
    scene.background_environment = true;
    let cam = scene.insert(NodeKind::Camera(Camera::Perspective(Default::default())));
    let out = target(&r);
    for blur in [0.0, 0.5, 1.0] {
        scene.background_blur = blur;
        r.render(&mut scene, cam, &out).unwrap();
        let bytes = r.read_rgba(&out).unwrap();
        for (c, want) in [51, 102, 204].iter().enumerate() {
            assert!((bytes[c] as i32 - want).abs() <= 2, "{bytes:?}");
        }
    }
    let before = r.resource_counts();
    r.render(&mut scene, cam, &out).unwrap();
    assert_eq!(before, r.resource_counts());
}
