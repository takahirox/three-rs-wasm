//! Individually ported additional official scenes; never executes upstream JavaScript.
use super::gltf_viewer::{OrbitViewer, decode_image, fetch, load_asset};
use crate::{Result, camera::*, geometry::*, material::*, math::*, scene::*};
use std::sync::Arc;

enum Content {
    Avif,
    Gltf(Box<super::gltf_examples::Demo>),
    Instancing,
    Triangles {
        objects: Vec<Object3D>,
        raw: bool,
    },
    Horse(super::expanded_morph_models::Horse),
    Sphere(super::expanded_morph_models::Sphere),
    Indexed(Object3D),
    ColorLines {
        objects: Vec<Object3D>,
        pointer: Vector2,
    },
    Geometries(Vec<Object3D>),
    Morph(Object3D),
    Dashed(Vec<Object3D>),
    AreaLights(Vec<Object3D>),
    VertexColors {
        pointer: Vector2,
    },
}
pub(super) struct Demo {
    viewer: OrbitViewer,
    near: f64,
    far: f64,
    content: Content,
    elapsed: f64,
}
impl Demo {
    pub async fn create(
        scene: &mut Scene,
        camera: Object3D,
        example: u32,
        renderer: &crate::renderer::Renderer,
    ) -> Result<Self> {
        let aspect = match scene.camera(camera)?.0 {
            Camera::Perspective(c) => c.aspect,
            _ => 1.0,
        };
        match example {
            29..=34 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.0),
                near: 0.1,
                far: 100.0,
                elapsed: 0.0,
                content: Content::Gltf(Box::new(
                    super::gltf_examples::Demo::create(scene, camera, example).await?,
                )),
            }),
            28 => {
                let (asset, buffers, images) =
                    load_asset("/web/models/AVIFTest/forest_house.glb").await?;
                crate::gltf::import_decoded(&asset, &buffers, &images)?.instantiate(scene)?;
                scene.background = Color::from_hex(0xf6eedc);
                scene.get_mut(camera)?.kind =
                    NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
                        fov: 45.0,
                        aspect,
                        near: 0.1,
                        far: 100.0,
                        ..Default::default()
                    }));
                let center = Vector3::new(0.0, 2.0, 0.0);
                let offset = Vector3::new(1.5, 4.0, 9.0) - center;
                let mut viewer = OrbitViewer::from_camera(center, offset.length());
                viewer.fixture(
                    offset.x.atan2(offset.z),
                    (offset.y / offset.length()).asin(),
                    1.8,
                );
                Ok(Self {
                    viewer,
                    near: 0.1,
                    far: 100.0,
                    content: Content::Avif,
                    elapsed: 0.0,
                })
            }
            26 | 27 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 2.0),
                near: 1.0,
                far: 3500.0,
                elapsed: 0.0,
                content: Content::Triangles {
                    objects: super::expanded_triangles::create(
                        scene,
                        camera,
                        aspect,
                        example == 27,
                        renderer,
                    )
                    .await?,
                    raw: example == 27,
                },
            }),
            16 => {
                let (asset, buffers, images) = load_asset(
                    "/web/models/DamagedHelmet/glTF-instancing/DamagedHelmetGpuInstancing.gltf",
                )
                .await?;
                crate::gltf::import_animated_decoded(&asset, &buffers, &images)?
                    .instantiate(scene)?;
                // r186 GLTFLoader applies assignFinalMaterial twice to InstancedMesh,
                // flipping derivative-tangent normalScale.y twice. Match this example's
                // actual material without changing the glTF importer's convention.
                for root in scene.roots().to_vec() {
                    for h in scene.traverse(root, true)? {
                        if let NodeKind::Mesh(mesh) = &mut scene.get_mut(h)?.kind {
                            for material in &mut mesh.materials {
                                if let Material::Standard(p) = Arc::make_mut(material) {
                                    p.normal_scale.y *= -1.0;
                                }
                            }
                        }
                    }
                }
                scene.environment = Some(Arc::new(crate::environment::EnvironmentMap::from_hdr(
                    &fetch("/web/environments/royal_esplanade_2k.hdr").await?,
                )?));
                scene.background_environment = true;
                scene.aces_tone_mapping = true;
                scene.get_mut(camera)?.kind =
                    NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
                        fov: 45.0,
                        aspect,
                        near: 0.25,
                        far: 20.0,
                        ..Default::default()
                    }));
                let center = Vector3::new(0.0, 0.25, 0.0);
                let offset = Vector3::new(-0.90, 0.41, -0.89) - center;
                let mut viewer = OrbitViewer::from_camera(center, offset.length());
                viewer.fixture(
                    offset.x.atan2(offset.z),
                    (offset.y / offset.length()).asin(),
                    1.8,
                );
                Ok(Self {
                    viewer,
                    near: 0.25,
                    far: 20.0,
                    content: Content::Instancing,
                    elapsed: 0.0,
                })
            }
            17 => {
                use std::f64::consts::{PI, TAU};
                scene.get_mut(camera)?.kind =
                    NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
                        fov: 45.0,
                        aspect,
                        near: 1.0,
                        far: 2000.0,
                        ..Default::default()
                    }));
                scene.insert(NodeKind::Light(Light::Ambient {
                    color: Color::from_hex(0xcccccc),
                    intensity: 1.5,
                }));
                let point = scene.insert(NodeKind::Light(Light::Point {
                    color: Color::WHITE,
                    intensity: 2.5,
                    distance: 0.0,
                    decay: 0.0,
                }));
                scene.add(camera, point)?;
                let mut map =
                    decode_image(&fetch("/web/gallery/assets/uv-grid.jpg").await?).await?;
                map.wrap_s = Wrapping::Repeat;
                map.wrap_t = Wrapping::Repeat;
                map.anisotropy = 16;
                map.srgb = true;
                map.mipmap_filter = Some(Filter::Linear);
                let mut material = MeshPhongMaterial::default();
                material.properties.map = Some(Arc::new(map));
                material.properties.side = Side::Double;
                let material = Arc::new(Material::Phong(material));
                let points = (0..50)
                    .map(|i| {
                        let i = i as f64;
                        Vector2::new(
                            (i * 0.2).sin() * (i * 0.1).sin() * 15.0 + 50.0,
                            (i - 5.0) * 2.0,
                        )
                    })
                    .collect::<Vec<_>>();
                let mut plane = ParametricGeometry::build(|u, v| Vector3::new(u, 0.0, v), 10, 10)?;
                plane.scale(Vector3::splat(100.0))?;
                plane.center()?;
                let klein = ParametricGeometry::build(
                    |v, u| {
                        let u = u * TAU;
                        let v = v * TAU;
                        let r = 2.0 * (1.0 - u.cos() / 2.0);
                        let (x, z) = if u < PI {
                            (
                                3.0 * u.cos() * (1.0 + u.sin()) + r * u.cos() * v.cos(),
                                -8.0 * u.sin() - r * u.sin() * v.cos(),
                            )
                        } else {
                            (
                                3.0 * u.cos() * (1.0 + u.sin()) + r * (v + PI).cos(),
                                -8.0 * u.sin(),
                            )
                        };
                        Vector3::new(x, -r * v.sin(), z)
                    },
                    20,
                    20,
                )?;
                let mobius = ParametricGeometry::build(
                    |u, t| {
                        let u = u - 0.5;
                        let v = t * TAU;
                        Vector3::new(
                            v.cos() * (2.0 + u * (v / 2.0).cos()),
                            v.sin() * (2.0 + u * (v / 2.0).cos()),
                            u * (v / 2.0).sin(),
                        )
                    },
                    20,
                    20,
                )?;
                let geometries = [
                    SphereGeometry::build(75.0, 20, 10)?,
                    IcosahedronGeometry::build(75.0, 0)?,
                    OctahedronGeometry::build(75.0, 0)?,
                    TetrahedronGeometry::build(75.0, 0)?,
                    PlaneGeometry::build(100.0, 100.0, 4, 4)?,
                    BoxGeometry::segmented(100.0, 100.0, 100.0, 4, 4, 4)?,
                    CircleGeometry::build(50.0, 20, 0.0, TAU)?,
                    RingGeometry::build(10.0, 50.0, 20, 5, 0.0, TAU)?,
                    CylinderGeometry::build(25.0, 75.0, 100.0, 40, 5, false, 0.0, TAU)?,
                    LatheGeometry::build(&points, 20, 0.0, TAU)?,
                    TorusGeometry::build(50.0, 20.0, 20, 20, TAU, 0.0, TAU)?,
                    TorusKnotGeometry::build(50.0, 10.0, 50, 20, 2, 3)?,
                    CapsuleGeometry::build(20.0, 50.0, 4, 8, 1)?,
                    plane,
                    klein,
                    mobius,
                ];
                let mut shapes = Vec::new();
                for (i, geometry) in geometries.into_iter().enumerate() {
                    let mesh = scene.insert(NodeKind::Mesh(Mesh::new(
                        Arc::new(geometry),
                        material.clone(),
                    )));
                    scene.get_mut(mesh)?.position = Vector3::new(
                        (i % 4) as f64 * 200.0 - 300.0,
                        0.0,
                        300.0 - (i / 4) as f64 * 200.0,
                    );
                    if i == 14 {
                        scene.get_mut(mesh)?.scale = Vector3::splat(5.0);
                    }
                    if i == 15 {
                        scene.get_mut(mesh)?.scale = Vector3::splat(30.0);
                    }
                    shapes.push(mesh);
                }
                Ok(Self {
                    viewer: OrbitViewer::from_camera(Vector3::ZERO, 800.0),
                    near: 1.0,
                    far: 2000.0,
                    content: Content::Geometries(shapes),
                    elapsed: 0.0,
                })
            }
            18 => {
                scene.background = Color::from_hex(0x8fbcd4);
                scene.get_mut(camera)?.kind =
                    NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
                        fov: 45.0,
                        aspect,
                        near: 1.0,
                        far: 20.0,
                        ..Default::default()
                    }));
                scene.insert(NodeKind::Light(Light::Ambient {
                    color: Color::from_hex(0x8fbcd4),
                    intensity: 1.5,
                }));
                let light = scene.insert(NodeKind::Light(Light::Point {
                    color: Color::WHITE,
                    intensity: 200.0,
                    distance: 0.0,
                    decay: 2.0,
                }));
                scene.add(camera, light)?;
                let mut geometry = BoxGeometry::segmented(2.0, 2.0, 2.0, 32, 32, 32)?;
                let mut sphere = Vec::new();
                let mut twist = Vec::new();
                for p in geometry.positions()? {
                    let Vector3 { x, y, z } = p;
                    sphere.extend(
                        [
                            x * (1.0 - y * y / 2.0 - z * z / 2.0 + y * y * z * z / 3.0).sqrt(),
                            y * (1.0 - z * z / 2.0 - x * x / 2.0 + z * z * x * x / 3.0).sqrt(),
                            z * (1.0 - x * x / 2.0 - y * y / 2.0 + x * x * y * y / 3.0).sqrt(),
                        ]
                        .map(|v| v as f32),
                    );
                    let q = Quaternion::from_axis_angle(Vector3::X, std::f64::consts::PI * x / 2.0);
                    twist.extend(
                        (q * Vector3::new(x * 2.0, y, z))
                            .to_array()
                            .map(|v| v as f32),
                    );
                }
                geometry.morph_attributes.insert(
                    "position".into(),
                    vec![
                        Attribute::F32(crate::attribute::BufferAttribute::new(sphere, 3, false)?),
                        Attribute::F32(crate::attribute::BufferAttribute::new(twist, 3, false)?),
                    ],
                );
                let mut material = MeshPhongMaterial::default();
                material.properties.color = Color::from_hex(0xff0000);
                material.properties.flat_shading = true;
                let mesh = scene.insert(NodeKind::Mesh(Mesh::new(
                    Arc::new(geometry),
                    Arc::new(Material::Phong(material)),
                )));
                scene.get_mut(mesh)?.morph_weights = vec![0.0, 0.0];
                Ok(Self {
                    viewer: OrbitViewer::from_camera(Vector3::ZERO, 10.0),
                    near: 1.0,
                    far: 20.0,
                    content: Content::Morph(mesh),
                    elapsed: 0.0,
                })
            }
            19 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 150.0),
                near: 1.0,
                far: 200.0,
                content: Content::Dashed(super::expanded_lines::dashed(scene, camera, aspect)?),
                elapsed: 0.0,
            }),
            20 => {
                scene.get_mut(camera)?.kind =
                    NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
                        fov: 45.0,
                        aspect,
                        near: 1.0,
                        far: 1000.0,
                        ..Default::default()
                    }));
                let center = Vector3::new(0.0, 5.5, 0.0);
                let offset = Vector3::new(0.0, 5.0, -15.0) - center;
                let mut viewer = OrbitViewer::from_camera(center, offset.length());
                viewer.fixture(
                    offset.x.atan2(offset.z),
                    (offset.y / offset.length()).asin(),
                    1.8,
                );
                Ok(Self {
                    viewer,
                    near: 1.0,
                    far: 1000.0,
                    content: Content::AreaLights(super::expanded_lights::rect_area(scene)?),
                    elapsed: 0.0,
                })
            }
            24 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 600.0),
                near: 1.0,
                far: 10000.0,
                content: Content::Horse(
                    super::expanded_morph_models::Horse::create(scene, camera, aspect).await?,
                ),
                elapsed: 0.0,
            }),
            25 => {
                let mut viewer = OrbitViewer::from_camera(Vector3::ZERO, 50.0_f64.sqrt());
                viewer.fixture(0.0, std::f64::consts::FRAC_PI_4, 1.8);
                Ok(Self {
                    viewer,
                    near: 0.2,
                    far: 100.0,
                    content: Content::Sphere(
                        super::expanded_morph_models::Sphere::create(scene, camera, aspect).await?,
                    ),
                    elapsed: 0.0,
                })
            }
            23 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1000.0),
                near: 1.0,
                far: 10000.0,
                content: Content::ColorLines {
                    objects: super::expanded_lines::colors(scene, camera, aspect)?,
                    pointer: Vector2::ZERO,
                },
                elapsed: 0.0,
            }),
            22 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 64.0),
                near: 1.0,
                far: 3500.0,
                content: Content::Indexed(super::expanded_indexed::create(scene, camera, aspect)?),
                elapsed: 0.0,
            }),
            21 => {
                super::expanded_geometry_colors::create(scene, camera, aspect)?;
                Ok(Self {
                    viewer: OrbitViewer::from_camera(Vector3::ZERO, 1800.0),
                    near: 1.0,
                    far: 10000.0,
                    content: Content::VertexColors {
                        pointer: Vector2::ZERO,
                    },
                    elapsed: 0.0,
                })
            }
            _ => Err(crate::Error::Invalid("expanded example id")),
        }
    }
    pub fn update(
        &mut self,
        scene: &mut Scene,
        camera: Object3D,
        delta: f64,
        animate: bool,
    ) -> Result<()> {
        if let Content::Gltf(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        if let Content::Triangles { objects, raw } = &self.content {
            if animate {
                self.elapsed += delta;
            }
            return super::expanded_triangles::update(scene, objects, self.elapsed, *raw);
        }
        if let Content::Horse(horse) = &mut self.content {
            if animate {
                self.elapsed += delta;
            }
            return horse.update(scene, camera, self.elapsed, animate);
        }
        if let Content::Sphere(sphere) = &self.content {
            if animate {
                self.elapsed += delta;
            }
            sphere.update(scene, self.elapsed)?;
        }
        if let Content::ColorLines { objects, pointer } = &self.content {
            if animate {
                self.elapsed += delta;
                let p = &mut scene.get_mut(camera)?.position;
                p.x += (pointer.x - p.x) * 0.05;
                p.y += (-pointer.y + 200.0 - p.y) * 0.05;
            }
            scene.look_at(camera, Vector3::ZERO)?;
            for (i, &object) in objects.iter().enumerate() {
                scene.get_mut(object)?.quaternion = Quaternion::from_rotation_y(
                    self.elapsed * 0.5 * if i % 2 == 1 { 1.0 } else { -1.0 },
                );
            }
            return Ok(());
        }
        if let Content::VertexColors { pointer } = &self.content {
            if animate {
                let p = &mut scene.get_mut(camera)?.position;
                p.x += (pointer.x - p.x) * 0.05;
                p.y += (-pointer.y - p.y) * 0.05;
            }
            scene.look_at(camera, Vector3::ZERO)?;
            return Ok(());
        }
        if let Content::Indexed(mesh) = self.content {
            if animate {
                self.elapsed += delta;
            }
            scene.get_mut(mesh)?.quaternion = Quaternion::from_euler(
                glam::EulerRot::XYZ,
                self.elapsed * 0.25,
                self.elapsed * 0.5,
                0.0,
            );
            return Ok(());
        }
        if let Content::Dashed(objects) = &self.content {
            if animate {
                self.elapsed += delta;
            }
            for &object in objects {
                scene.get_mut(object)?.quaternion = Quaternion::from_euler(
                    glam::EulerRot::XYZ,
                    self.elapsed * 0.25,
                    self.elapsed * 0.25,
                    0.0,
                );
            }
            return Ok(());
        }
        if let Content::Geometries(shapes) = &self.content {
            if animate {
                self.elapsed += delta;
            }
            let t = self.elapsed * 0.1;
            scene.get_mut(camera)?.position = Vector3::new(t.cos() * 800.0, 500.0, t.sin() * 800.0);
            scene.look_at(camera, Vector3::ZERO)?;
            for &mesh in shapes {
                scene.get_mut(mesh)?.quaternion =
                    Quaternion::from_euler(glam::EulerRot::XYZ, t * 5.0, t * 2.5, 0.0);
            }
            return Ok(());
        }
        if let Content::AreaLights(lights) = &self.content {
            if animate {
                self.elapsed += delta;
            }
            for (&light, speed) in lights.iter().zip([-1.0, 0.5, 1.0]) {
                scene.get_mut(light)?.quaternion =
                    Quaternion::from_rotation_y(self.elapsed * speed);
            }
        }
        self.viewer.update(scene, camera)?;
        if let NodeKind::Camera(Camera::Perspective(c)) = &mut scene.get_mut(camera)?.kind {
            c.near = self.near;
            c.far = self.far;
        }
        Ok(())
    }
    pub fn wireframe(&mut self, scene: &mut Scene, enabled: bool) -> Result<()> {
        if let Content::Indexed(mesh) = self.content {
            if let NodeKind::Mesh(mesh) = &mut scene.get_mut(mesh)?.kind {
                Arc::make_mut(&mut mesh.materials[0])
                    .properties_mut()
                    .wireframe = enabled;
            }
            Ok(())
        } else {
            Err(crate::Error::Invalid("indexed example"))
        }
    }
    pub fn morph(&mut self, scene: &mut Scene, weights: [f64; 2]) -> Result<()> {
        if let Content::Morph(mesh) = self.content {
            scene.get_mut(mesh)?.morph_weights.copy_from_slice(&weights);
            Ok(())
        } else {
            Err(crate::Error::Invalid("morph example"))
        }
    }
    pub fn pointer(&mut self, x: f64, y: f64) {
        if let Content::VertexColors { pointer } | Content::ColorLines { pointer, .. } =
            &mut self.content
        {
            *pointer = Vector2::new(x, y);
        }
    }
    pub fn dragging(&mut self, value: bool) {
        if let Content::Gltf(demo) = &mut self.content {
            demo.dragging(value);
        }
    }
    pub fn seek(&mut self, seconds: f64) {
        if let Content::Gltf(demo) = &mut self.content {
            demo.seek(seconds);
        }
        self.elapsed = seconds;
        if let Content::Horse(horse) = &mut self.content {
            horse.theta = seconds * 6.0;
        }
    }
    #[allow(clippy::too_many_arguments)]
    pub fn input(
        &mut self,
        scene: &Scene,
        camera: Object3D,
        dx: f64,
        dy: f64,
        wheel: f64,
        pan: bool,
        height: f64,
    ) -> Result<()> {
        if let Content::Gltf(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        if matches!(
            self.content,
            Content::Triangles { .. }
                | Content::Horse(_)
                | Content::ColorLines { .. }
                | Content::Indexed(_)
                | Content::Geometries(_)
                | Content::Dashed(_)
                | Content::VertexColors { .. }
        ) {
            return Ok(());
        }
        if pan {
            self.viewer.pan_pixels(scene, camera, dx, dy, height)?;
        } else {
            let (wheel, min, max) = if matches!(self.content, Content::Morph(_)) {
                (0.0, 0.0, f64::INFINITY)
            } else if matches!(self.content, Content::Sphere(_)) {
                (wheel, 1.0, 20.0)
            } else if matches!(self.content, Content::Instancing) {
                (wheel, 0.2, 10.0)
            } else {
                (wheel, 0.0, f64::INFINITY)
            };
            self.viewer.orbit_pixels(dx, dy, wheel, height, min, max);
        }
        Ok(())
    }
}
