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
fn gpu_shadow_mask_can_remove_a_caster() {
    pollster::block_on(async {
        let r = Renderer::new().await.unwrap();
        let mut s = Scene::new();
        s.shadow_map_size = 128;
        let sun = s.insert(NodeKind::Light(Light::Sun {
            color: Color::WHITE,
            intensity: 2.,
        }));
        s.get_mut(sun).unwrap().position = Vector3::new(2., 4., 1.);
        s.get_mut(sun).unwrap().cast_shadow = true;
        s.get_mut(sun).unwrap().shadow.far = 20.;
        let camera = s.insert(NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            near: 0.1,
            far: 30.,
            ..Default::default()
        })));
        s.get_mut(camera).unwrap().position = Vector3::new(0., 5., 7.);
        s.look_at(camera, Vector3::ZERO).unwrap();
        let floor = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(PlaneGeometry::build(10., 10., 1, 1).unwrap()),
            Arc::new(Material::Lambert(MeshLambertMaterial::default())),
        )));
        s.get_mut(floor).unwrap().quaternion =
            Quaternion::from_rotation_x(-std::f64::consts::FRAC_PI_2);
        s.get_mut(floor).unwrap().receive_shadow = true;
        let mut material = MeshBasicMaterial::default();
        material.properties.shadow_program = Some(Arc::new(
            tsl::surface::shadow_program(
                &r,
                None,
                Some(uniform(0, Type::Float).greater_than(float(0.5))),
            )
            .await
            .unwrap(),
        ));
        let caster = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(BoxGeometry::build(1., 1., 1.).unwrap()),
            Arc::new(Material::Basic(material)),
        )));
        s.get_mut(caster).unwrap().position.y = 1.;
        s.get_mut(caster).unwrap().cast_shadow = true;
        let out = RenderTarget::new(&r.device, 128, 128).unwrap();
        r.render(&mut s, camera, &out).unwrap();
        let masked = r.read_rgba(&out).unwrap();
        s.get_mut(caster).unwrap().cast_shadow = false;
        r.render(&mut s, camera, &out).unwrap();
        let no_shadow = r.read_rgba(&out).unwrap();
        assert_eq!(masked, no_shadow);
        s.get_mut(caster).unwrap().cast_shadow = true;
        if let NodeKind::Mesh(m) = &mut s.get_mut(caster).unwrap().kind {
            Arc::make_mut(&mut m.materials[0])
                .properties_mut()
                .vertex_uniforms[0][0] = 1.;
        }
        r.render(&mut s, camera, &out).unwrap();
        let full = r.read_rgba(&out).unwrap();
        assert!(
            full.iter()
                .zip(masked)
                .filter(|(a, b)| a.abs_diff(*b) > 8)
                .count()
                > 20
        );
    });
}

#[test]
fn storage_instancing_uses_the_same_gpu_positions_in_color_and_shadow_passes() {
    use three_rs_wasm::{
        compute::{BufferAccess, GpuBuffer},
        tsl::surface::*,
    };
    pollster::block_on(async {
        let r = Renderer::new().await.unwrap();
        let mut s = Scene::new();
        s.shadow_map_size = 256;
        let light = s.insert(NodeKind::Light(Light::Directional {
            color: Color::WHITE,
            intensity: 2.,
            target: Vector3::ZERO,
        }));
        s.get_mut(light).unwrap().position = Vector3::new(2., 6., 3.);
        s.get_mut(light).unwrap().cast_shadow = true;
        let c = s.insert(NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            near: 0.1,
            far: 30.,
            ..Default::default()
        })));
        s.get_mut(c).unwrap().position = Vector3::new(0., 5., 8.);
        s.look_at(c, Vector3::ZERO).unwrap();
        let floor = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(PlaneGeometry::build(10., 10., 1, 1).unwrap()),
            Arc::new(Material::Lambert(MeshLambertMaterial::default())),
        )));
        s.get_mut(floor).unwrap().quaternion =
            Quaternion::from_rotation_x(-std::f64::consts::FRAC_PI_2);
        s.get_mut(floor).unwrap().receive_shadow = true;
        let offsets = [[-2f32, 1., 0., 0.], [0., 1., 0., 0.], [2., 1., 0., 0.]];
        let buffer =
            GpuBuffer::new(&r, bytemuck::cast_slice(&offsets), BufferAccess::Read).unwrap();
        let position = position_geometry() + storage_element(0, instance_index()).rgb();
        let material = Material::Basic(MeshBasicMaterial {
            properties: MaterialProperties {
                vertex_program: Some(Arc::new(
                    SurfaceNodes {
                        position: Some(position.clone()),
                        ..Default::default()
                    }
                    .build(&r, &[(&buffer, Type::Vec4)], &[])
                    .await
                    .unwrap(),
                )),
                shadow_program: Some(Arc::new(
                    shadow_program_with_storage(&r, Some(position), None, &[(&buffer, Type::Vec4)])
                        .await
                        .unwrap(),
                )),
                ..Default::default()
            },
        });
        let mut g = BoxGeometry::build(0.7, 1., 0.7).unwrap();
        g.instance_count = Some(3);
        let caster = s.insert(NodeKind::Mesh(Mesh::new(Arc::new(g), Arc::new(material))));
        s.get_mut(caster).unwrap().cast_shadow = true;
        let out = RenderTarget::new(&r.device, 160, 160).unwrap();
        r.render(&mut s, c, &out).unwrap();
        let actual = r.read_rgba(&out).unwrap();
        // Independent oracle: ordinary instance transforms, without custom shaders.
        if let NodeKind::Mesh(m) = &mut s.get_mut(caster).unwrap().kind {
            m.geometry = Arc::new(BoxGeometry::build(0.7, 1., 0.7).unwrap());
            m.materials[0] = Arc::new(Material::Basic(MeshBasicMaterial::default()));
        }
        s.get_mut(caster).unwrap().instances = offsets
            .iter()
            .map(|p| Instance {
                matrix: Matrix4::from_translation(Vector3::new(
                    p[0] as f64,
                    p[1] as f64,
                    p[2] as f64,
                )),
                ..Default::default()
            })
            .collect();
        r.render(&mut s, c, &out).unwrap();
        let expected = r.read_rgba(&out).unwrap();
        let bad = actual
            .iter()
            .zip(&expected)
            .filter(|(a, b)| a.abs_diff(**b) > 1)
            .count();
        assert!(
            bad == 0,
            "different channels {bad}, max {}",
            actual
                .iter()
                .zip(&expected)
                .map(|(a, b)| a.abs_diff(*b))
                .max()
                .unwrap()
        );
        s.get_mut(caster).unwrap().cast_shadow = false;
        r.render(&mut s, c, &out).unwrap();
        assert!(
            expected
                .iter()
                .zip(r.read_rgba(&out).unwrap())
                .filter(|(a, b)| a.abs_diff(*b) > 8)
                .count()
                > 50
        );
    });
}

#[test]
fn per_light_shadow_resolution_survives_a_larger_atlas_neighbor() {
    let r = pollster::block_on(Renderer::new()).unwrap();
    for light in [
        Light::Directional {
            color: Color::WHITE,
            intensity: 2.,
            target: Vector3::ZERO,
        },
        Light::Sun {
            color: Color::WHITE,
            intensity: 2.,
        },
        Light::Spot {
            color: Color::WHITE,
            intensity: 150.,
            distance: 0.,
            angle: 0.8,
            penumbra: 0.2,
            decay: 2.,
            target: Vector3::ZERO,
        },
    ] {
        let mut s = Scene::new();
        s.shadow_map_size = 64;
        let c = s.insert(NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            near: 0.1,
            far: 20.,
            ..Default::default()
        })));
        s.get_mut(c).unwrap().position = Vector3::new(0., 4., 7.);
        s.look_at(c, Vector3::ZERO).unwrap();
        let key = s.insert(NodeKind::Light(light));
        {
            let n = s.get_mut(key).unwrap();
            n.position = Vector3::new(2., 4., 1.);
            n.cast_shadow = true;
            n.shadow.far = 20.;
            n.shadow.normal_bias = 0.01;
        }
        let floor = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(PlaneGeometry::build(10., 10., 1, 1).unwrap()),
            Arc::new(Material::Lambert(MeshLambertMaterial::default())),
        )));
        s.get_mut(floor).unwrap().quaternion =
            Quaternion::from_rotation_x(-std::f64::consts::FRAC_PI_2);
        s.get_mut(floor).unwrap().receive_shadow = true;
        let cube = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(BoxGeometry::build(1., 1., 1.).unwrap()),
            Arc::new(Material::Lambert(MeshLambertMaterial::default())),
        )));
        s.get_mut(cube).unwrap().position.y = 1.;
        s.get_mut(cube).unwrap().cast_shadow = true;
        let out = RenderTarget::new(&r.device, 96, 96).unwrap();
        r.render(&mut s, c, &out).unwrap();
        let expected = r.read_rgba(&out).unwrap();
        let neighbor = s.insert(NodeKind::Light(Light::Directional {
            color: Color::WHITE,
            intensity: 0.,
            target: Vector3::ZERO,
        }));
        {
            let n = s.get_mut(neighbor).unwrap();
            n.position = Vector3::new(-3., 5., 2.);
            n.cast_shadow = true;
            n.shadow.map_size = Some(256);
        }
        r.render(&mut s, c, &out).unwrap();
        let actual = r.read_rgba(&out).unwrap();
        assert_eq!(
            actual, expected,
            "per-light PCF texel size changed with the atlas"
        );
        s.get_mut(key).unwrap().cast_shadow = false;
        r.render(&mut s, c, &out).unwrap();
        assert_ne!(
            actual,
            r.read_rgba(&out).unwrap(),
            "oracle must contain visible shadows"
        );
        s.get_mut(key).unwrap().cast_shadow = true;
        s.get_mut(key).unwrap().shadow.map_size = Some(0);
        assert!(r.render(&mut s, c, &out).is_err());
    }
}
