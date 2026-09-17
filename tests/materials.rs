use std::sync::Arc;
use three_rs_wasm::{camera::*, geometry::*, material::*, math::*, renderer::*, scene::*};

#[test]
fn render_material_reference_cases() {
    let renderer = pollster::block_on(Renderer::new()).unwrap();
    std::fs::create_dir_all(".cache/core-materials").unwrap();
    for name in [
        "lambert",
        "phong",
        "flat",
        "normal",
        "physical",
        "clearcoat",
        "sheen",
        "anisotropy",
        "ior",
        "area",
        "toon",
        "matcap",
        "depth",
        "transmission",
        "iridescence",
        "physical_maps",
    ] {
        let mut material = match name {
            "toon" => Material::Toon(MeshToonMaterial::default()),
            "matcap" => Material::Matcap(MeshMatcapMaterial::default()),
            "depth" => Material::Depth(MeshDepthMaterial::default()),
            "lambert" => Material::Lambert(MeshLambertMaterial::default()),
            "phong" | "flat" => Material::Phong(MeshPhongMaterial::default()),
            "normal" => Material::Normal(MeshNormalMaterial::default()),
            _ => {
                let mut p = MeshPhysicalMaterial::default();
                p.base.roughness = 0.35;
                p.base.metalness = 0.1;
                match name {
                    "iridescence" => {
                        p.iridescence = 1.0;
                        p.base.metalness = 0.5;
                        p.iridescence_thickness_range = [100.0, 300.0];
                    }
                    "physical_maps" => {
                        let map = |rgba| {
                            let mut t = Texture::from_rgba(1, 1, rgba, false).unwrap();
                            t.flip_y = false;
                            Some(Arc::new(t))
                        };
                        p.clearcoat = 0.9;
                        p.clearcoat_roughness = 0.3;
                        p.sheen = 0.5;
                        p.sheen_color = Color::linear(0.3, 0.6, 0.8);
                        p.sheen_roughness = 0.7;
                        p.anisotropy = 0.4;
                        p.clearcoat_map = map(vec![180, 255, 255, 255]);
                        p.clearcoat_roughness_map = map(vec![255, 150, 255, 255]);
                        p.clearcoat_normal_map = map(vec![150, 110, 250, 255]);
                        p.sheen_color_map = map(vec![180, 100, 200, 255]);
                        p.sheen_roughness_map = map(vec![255, 255, 255, 180]);
                        p.anisotropy_map = map(vec![220, 170, 200, 255]);
                        p.specular_intensity_map = map(vec![255, 255, 255, 180]);
                        p.specular_color_map = map(vec![180, 220, 250, 255]);
                    }
                    "transmission" => {
                        p.base.roughness = 0.0;
                        p.base.metalness = 0.0;
                        p.transmission = 1.0;
                        p.thickness = 0.5;
                        p.attenuation_color = Color::linear(0.5, 0.7, 1.0);
                        p.attenuation_distance = 1.0;
                    }
                    "clearcoat" => {
                        p.clearcoat = 1.0;
                        p.clearcoat_roughness = 0.2;
                    }
                    "sheen" => {
                        p.sheen = 0.8;
                        p.sheen_color = Color::linear(0.1, 0.4, 0.8);
                        p.sheen_roughness = 0.4;
                    }
                    "anisotropy" => {
                        p.anisotropy = 0.8;
                        p.anisotropy_rotation = 0.4;
                    }
                    "ior" => {
                        p.ior = 2.0;
                        p.specular_intensity = 0.5;
                        p.specular_color = Color::linear(0.2, 0.8, 1.0);
                    }
                    _ => {}
                }
                Material::Physical(p)
            }
        };
        material.properties_mut().color = Color::linear(0.4, 0.12, 0.05);
        material.properties_mut().flat_shading = name == "flat";
        let mut scene = Scene::new();
        if name == "transmission" {
            scene.background = Color::linear(0.2, 0.4, 0.6);
        }
        let camera = scene.insert(NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 40.0,
            near: 0.1,
            far: 20.0,
            ..Default::default()
        })));
        scene.get_mut(camera).unwrap().position.z = 4.0;
        scene.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(SphereGeometry::build(1.0, 64, 32).unwrap()),
            Arc::new(material),
        )));
        scene.insert(NodeKind::Light(Light::Ambient {
            color: Color::WHITE,
            intensity: 0.2,
        }));
        let light = scene.insert(NodeKind::Light(if name == "area" {
            Light::RectArea {
                color: Color::WHITE,
                intensity: 3.0,
                width: 2.0,
                height: 2.0,
            }
        } else {
            Light::Directional {
                color: Color::WHITE,
                intensity: 3.0,
                target: Vector3::ZERO,
            }
        }));
        scene.get_mut(light).unwrap().position = Vector3::new(2.0, 2.0, 3.0);
        if name == "area" {
            scene.look_at(light, Vector3::ZERO).unwrap();
        }
        let target = RenderTarget::with_options(
            &renderer.device,
            256,
            256,
            RenderTargetOptions {
                format: wgpu::TextureFormat::Rgba16Float,
                samples: 4,
                ..Default::default()
            },
        )
        .unwrap();
        renderer.render(&mut scene, camera, &target).unwrap();
        let output = RenderTarget::new(&renderer.device, 256, 256).unwrap();
        renderer.blit_tone_mapped(
            &target,
            &output.texture.create_view(&Default::default()),
            wgpu::TextureFormat::Rgba8UnormSrgb,
            1.0,
            false,
        );
        let pixels = renderer.read_rgba(&output).unwrap();
        assert!(pixels.chunks_exact(4).filter(|p| p[0] > 20).count() > 1000);
        image::save_buffer(
            format!(".cache/core-materials/{name}.png"),
            &pixels,
            256,
            256,
            image::ColorType::Rgba8,
        )
        .unwrap();
    }
}
