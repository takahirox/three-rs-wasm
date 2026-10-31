//! LightProbeGrid's r187 bake: under a uniform background of radiance L, the
//! baked grid's irradiance is πL everywhere, so a white Lambert sphere inside
//! it shows L, the background's own color, from the face captures, the
//! solid-angle SH projection and the repack summing the face blocks.
use std::sync::Arc;
use three_rs_wasm::{
    camera::*,
    geometry::*,
    light_probe_grid::{self, BakeOptions, GridBaker, LightProbeGrid},
    material::*,
    math::*,
    renderer::*,
    scene::*,
};

fn sphere_and_background(size: u32, bounces: u32, intensity: f64) -> ([u8; 4], [u8; 4]) {
    let renderer = pollster::block_on(Renderer::new()).unwrap();
    let target = RenderTarget::new(&renderer.device, 64, 64).unwrap();
    let mut scene = Scene::new();
    scene.background = Color::linear(0.2, 0.3, 0.4);
    let camera = scene.insert(NodeKind::Camera(Camera::Orthographic(OrthographicCamera {
        left: -2.0,
        right: 2.0,
        top: 2.0,
        bottom: -2.0,
        near: 0.1,
        far: 20.0,
        ..Default::default()
    })));
    scene.get_mut(camera).unwrap().position.z = 5.0;
    let sphere = scene.insert(NodeKind::Mesh(Mesh::new(
        Arc::new(SphereGeometry::build(0.5, 32, 16).unwrap()),
        Arc::new(Material::Lambert(MeshLambertMaterial::default())),
    )));
    // Out of the captures: the probes see only the background.
    scene.get_mut(sphere).unwrap().visible = false;
    let grid = scene.insert(NodeKind::LightProbeGrid(Box::new(LightProbeGrid::new(
        2.0,
        2.0,
        2.0,
        Some([2, 2, 2]),
    ))));
    let mut baker = GridBaker::new(&renderer);
    light_probe_grid::bake(
        &mut baker,
        &renderer,
        &mut scene,
        grid,
        BakeOptions {
            cubemap_size: size,
            bounces,
            ..Default::default()
        },
    )
    .unwrap();
    scene.get_mut(sphere).unwrap().visible = true;
    if let NodeKind::LightProbeGrid(g) = &mut scene.get_mut(grid).unwrap().kind {
        g.intensity = intensity;
    }
    renderer.render(&mut scene, camera, &target).unwrap();
    let pixels = renderer.read_rgba(&target).unwrap();
    let at = |x: usize, y: usize| {
        let i = (y * 64 + x) * 4;
        [pixels[i], pixels[i + 1], pixels[i + 2], pixels[i + 3]]
    };
    (at(32, 32), at(2, 2))
}

#[test]
fn a_uniform_background_bakes_its_own_irradiance() {
    for (size, bounces) in [(8, 0), (16, 1), (32, 0)] {
        let (sphere, background) = sphere_and_background(size, bounces, 1.0);
        for c in 0..3 {
            assert!(
                (i32::from(sphere[c]) - i32::from(background[c])).abs() <= 2,
                "cubemapSize {size}: sphere {sphere:?}, background {background:?}"
            );
        }
    }
}

#[test]
fn the_grid_intensity_scales_the_irradiance() {
    // Half the irradiance: the sphere's linear color halves ( 0.2 → 0.1 in red ).
    let (sphere, background) = sphere_and_background(8, 0, 0.5);
    let linear = |v: u8| {
        let c = f64::from(v) / 255.0;
        if c <= 0.04045 {
            c / 12.92
        } else {
            ((c + 0.055) / 1.055).powf(2.4)
        }
    };
    for c in 0..3 {
        let ratio = linear(sphere[c]) / linear(background[c]);
        assert!((ratio - 0.5).abs() < 0.03, "{sphere:?} {background:?}");
    }
}

#[test]
fn probes_sit_at_their_cell_centers() {
    let g = LightProbeGrid::new(4.0, 2.0, 2.0, None);
    assert_eq!(g.resolution, [4, 2, 2]);
    let p = g.probe_position(Vector3::ZERO, [0, 1, 0]);
    assert_eq!(p, Vector3::new(-1.5, 0.5, -0.5));
}
