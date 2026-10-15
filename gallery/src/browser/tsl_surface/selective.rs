use super::*;
use crate::tsl::*;
impl Demo {
    pub(super) async fn selective(
        scene: &mut Scene,
        cam: Object3D,
        r: &Renderer,
        example: u32,
    ) -> Result<Self> {
        let phong = example == 69;
        let aspect = match scene.camera(cam)?.0 {
            Camera::Perspective(c) => c.aspect,
            _ => 1.0,
        };
        scene.get_mut(cam)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 50.0,
            aspect,
            near: 0.01,
            far: 100.0,
            ..Default::default()
        }));
        scene.get_mut(cam)?.position = Vector3::new(0.0, 0.0, 7.0);
        scene.look_at(cam, Vector3::ZERO)?;
        scene.background = Color::BLACK;
        scene.background_alpha = 0.0;
        scene.fog = Some(Fog::Linear {
            color: Color::from_hex(0xff00ff),
            near: 12.0,
            far: 30.0,
        });
        let mut images = Vec::new();
        for name in ["Water_1_M_Normal.jpg", "roughness_map.jpg"] {
            let mut image = super::super::gltf_viewer::decode_image(
                &super::super::gltf_viewer::fetch(&format!("/web/gallery/assets/{name}")).await?,
            )
            .await?;
            image.srgb = false;
            image.wrap_s = Wrapping::Repeat;
            image.wrap_t = Wrapping::Repeat;
            image.mipmap_filter = Some(Filter::Linear);
            images.push(Arc::new(image));
        }
        let alpha = r.upload_texture(&images[1])?;
        let normal = r.upload_texture(&images[0])?;
        let geometry = Arc::new(SphereGeometry::build(0.1, 16, 8)?);
        let mut lights = Vec::new();
        for hex in if phong {
            [0x0040ff, 0xffffff, 0x80ff80, 0xffaa00]
        } else {
            [0xff0040, 0x0040ff, 0x80ff80, 0xffaa00]
        } {
            let color = Color::from_hex(hex);
            let light = scene.insert(NodeKind::Light(Light::Point {
                color,
                intensity: 1700.0 / (4.0 * std::f64::consts::PI),
                distance: 100.0,
                decay: 2.0,
            }));
            let mut m = MeshBasicMaterial::default();
            m.properties.color = color;
            let marker = scene.insert(NodeKind::Mesh(Mesh::new(
                geometry.clone(),
                Arc::new(Material::Basic(m)),
            )));
            scene.add(light, marker)?;
            lights.push(light);
        }
        #[derive(serde::Deserialize)]
        struct AttributeData {
            size: usize,
            array: Vec<f32>,
        }
        #[derive(serde::Deserialize)]
        struct GeometryData {
            index: Vec<u32>,
            attributes: std::collections::HashMap<String, AttributeData>,
        }
        let data: GeometryData = serde_json::from_slice(
            &super::super::gltf_viewer::fetch("/web/gallery/assets/teapot-18.json").await?,
        )
        .map_err(|e| crate::Error::Asset(e.to_string()))?;
        let mut geometry = BufferGeometry::default();
        for (name, a) in data.attributes {
            geometry.set_attribute(
                &name,
                Attribute::F32(BufferAttribute::new(a.array, a.size, false)?),
            );
        }
        geometry.set_index(Some(data.index));
        let geometry = Arc::new(geometry);
        let sample =
            tsl::Texture::External(0).sample(vec2(tsl::uv().x(), float(1.0) - tsl::uv().y()));
        let mut objects = Vec::new();
        for i in 0..3 {
            let mut graph = SurfaceNodes::default();
            let mut material = if phong {
                let m = MeshPhongMaterial {
                    shininess: if i == 1 {
                        80.0
                    } else if i == 2 {
                        90.0
                    } else {
                        30.0
                    },
                    ..Default::default()
                };
                if i == 1 {
                    graph.normal = Some(tsl::surface::normal_map(
                        tsl::Texture::External(1)
                            .sample(vec2(tsl::uv().x(), float(1.0) - tsl::uv().y()))
                            .rgb(),
                        tsl::uv(),
                    ));
                } else {
                    graph.specular = Some(vec4(
                        if i == 0 {
                            sample.rgb()
                        } else {
                            mix(
                                vec3(float(0.0), float(0.0), float(1.0)),
                                vec3(float(1.0), float(0.0), float(0.0)),
                                checker(tsl::uv() * float(5.0)),
                            )
                        },
                        float(m.shininess as f32),
                    ));
                }
                Material::Phong(m)
            } else {
                let mut m = MeshStandardMaterial {
                    energy_conservation: true,
                    ..Default::default()
                };
                if i == 0 {
                    graph.roughness = Some(sample.x());
                } else if i == 1 {
                    graph.normal = Some(tsl::surface::normal_map(
                        tsl::Texture::External(1)
                            .sample(vec2(tsl::uv().x(), float(1.0) - tsl::uv().y()))
                            .rgb(),
                        tsl::uv(),
                    ));
                    m.metalness = 0.5;
                    m.roughness = 0.5;
                } else {
                    graph.metalness = Some(sample.x());
                }
                Material::Standard(m)
            };
            let p = material.properties_mut();
            p.color = Color::from_hex(0x555555);
            if i != 1 {
                p.lights = Some(vec![lights[if i == 0 { 0 } else { 1 }]]);
            }
            p.vertex_program = Some(Arc::new(
                graph
                    .build(
                        r,
                        &[],
                        &[
                            (&alpha.view, &alpha.sampler),
                            (&normal.view, &normal.sampler),
                        ],
                    )
                    .await?,
            ));
            let object = scene.insert(NodeKind::Mesh(Mesh::new(
                geometry.clone(),
                Arc::new(material),
            )));
            scene.get_mut(object)?.position = Vector3::new((i as f64 - 1.0) * 3.0, -1.0, 0.0);
            scene.get_mut(object)?.quaternion =
                Quaternion::from_rotation_y(-std::f64::consts::FRAC_PI_2);
            objects.push(object);
        }
        let mut viewer = OrbitViewer::from_camera(Vector3::ZERO, 7.0);
        viewer.fixture(0.0, 0.0, 1.8);
        let mut params = [[0.0; 4]; 16];
        params[1] = [0.5, 0.5, 0.0, 0.0];
        Ok(Self {
            example,
            time: 0.0,
            lights,
            objects,
            params,
            viewer,
            orbit: Vector2::ZERO,
            pan: Vector3::ZERO,
            mixer: None,
            depth: None,
            mrt_sampler: None,
            bloom: None,
            storage: None,
            jelly: None,
            mask: None,
        })
    }
}
