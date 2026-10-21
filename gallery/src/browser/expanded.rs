//! Individually ported additional official scenes; never executes upstream JavaScript.
use super::gltf_viewer::{OrbitViewer, decode_image, fetch, load_asset};
use crate::{Result, camera::*, geometry::*, material::*, math::*, scene::*};
use std::sync::Arc;

enum Content {
    /// A module's example ( gallery/src/browser/example.rs ).
    Module(Box<dyn super::example::Example>),
    Avif,
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
/// A module example's page content: its demo behind the gallery's camera
/// viewer and clip planes.
pub(super) fn module(
    viewer: OrbitViewer,
    near: f64,
    far: f64,
    content: Box<dyn super::example::Example>,
) -> super::PageContent {
    super::PageContent::Gallery(super::gallery_scenes::GalleryScene::Expanded(Demo {
        viewer,
        near,
        far,
        content: Content::Module(content),
        elapsed: 0.0,
    }))
}
impl Demo {
    #[cfg(feature = "example_tsl_procedural")]
    pub fn audio(&self) -> crate::Result<&super::tsl_procedural::audio::Audio> {
        if let Content::Module(demo) = &self.content
            && let Some(audio) = demo.on_audio()
        {
            audio
        } else {
            Err(crate::Error::Invalid("not an audio example"))
        }
    }

    async fn scene_28(
        scene: &mut Scene,
        camera: Object3D,
        example: u32,
        renderer: &crate::renderer::Renderer,
        aspect: f64,
    ) -> Result<Self> {
        let _ = (example, renderer);
        {
            // The page's DRACOLoader.
            super::draco_loader::install();
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
    }
    async fn scene_26_27(
        scene: &mut Scene,
        camera: Object3D,
        example: u32,
        renderer: &crate::renderer::Renderer,
        aspect: f64,
    ) -> Result<Self> {
        let _ = (example, renderer);
        Ok(Self {
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
        })
    }
    async fn scene_16(
        scene: &mut Scene,
        camera: Object3D,
        example: u32,
        renderer: &crate::renderer::Renderer,
        aspect: f64,
    ) -> Result<Self> {
        let _ = (example, renderer);
        {
            let (asset, buffers, images) = load_asset(
                "/web/models/DamagedHelmet/glTF-instancing/DamagedHelmetGpuInstancing.gltf",
            )
            .await?;
            crate::gltf::import_animated_decoded(&asset, &buffers, &images)?.instantiate(scene)?;
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
            scene.environment = Some(Arc::new(
                super::ultra_hdr::environment("/web/environments/royal_esplanade_2k.hdr.jpg")
                    .await?,
            ));
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
    }
    async fn scene_17(
        scene: &mut Scene,
        camera: Object3D,
        example: u32,
        renderer: &crate::renderer::Renderer,
        aspect: f64,
    ) -> Result<Self> {
        let _ = (example, renderer);
        {
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
            let mut map = decode_image(&fetch("/web/gallery/assets/uv-grid.jpg").await?).await?;
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
    }
    async fn scene_18(
        scene: &mut Scene,
        camera: Object3D,
        example: u32,
        renderer: &crate::renderer::Renderer,
        aspect: f64,
    ) -> Result<Self> {
        let _ = (example, renderer);
        {
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
    }
    async fn scene_19(
        scene: &mut Scene,
        camera: Object3D,
        example: u32,
        renderer: &crate::renderer::Renderer,
        aspect: f64,
    ) -> Result<Self> {
        let _ = (example, renderer);
        Ok(Self {
            viewer: OrbitViewer::from_camera(Vector3::ZERO, 150.0),
            near: 1.0,
            far: 200.0,
            content: Content::Dashed(super::expanded_lines::dashed(scene, camera, aspect)?),
            elapsed: 0.0,
        })
    }
    async fn scene_24(
        scene: &mut Scene,
        camera: Object3D,
        example: u32,
        renderer: &crate::renderer::Renderer,
        aspect: f64,
    ) -> Result<Self> {
        let _ = (example, renderer);
        Ok(Self {
            viewer: OrbitViewer::from_camera(Vector3::ZERO, 600.0),
            near: 1.0,
            far: 10000.0,
            content: Content::Horse(
                super::expanded_morph_models::Horse::create(scene, camera, aspect).await?,
            ),
            elapsed: 0.0,
        })
    }
    async fn scene_25(
        scene: &mut Scene,
        camera: Object3D,
        example: u32,
        renderer: &crate::renderer::Renderer,
        aspect: f64,
    ) -> Result<Self> {
        let _ = (example, renderer);
        {
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
    }
    async fn scene_23(
        scene: &mut Scene,
        camera: Object3D,
        example: u32,
        renderer: &crate::renderer::Renderer,
        aspect: f64,
    ) -> Result<Self> {
        let _ = (example, renderer);
        Ok(Self {
            viewer: OrbitViewer::from_camera(Vector3::ZERO, 1000.0),
            near: 1.0,
            far: 10000.0,
            content: Content::ColorLines {
                objects: super::expanded_lines::colors(scene, camera, aspect)?,
                pointer: Vector2::ZERO,
            },
            elapsed: 0.0,
        })
    }
    async fn scene_22(
        scene: &mut Scene,
        camera: Object3D,
        example: u32,
        renderer: &crate::renderer::Renderer,
        aspect: f64,
    ) -> Result<Self> {
        let _ = (example, renderer);
        Ok(Self {
            viewer: OrbitViewer::from_camera(Vector3::ZERO, 64.0),
            near: 1.0,
            far: 3500.0,
            content: Content::Indexed(super::expanded_indexed::create(scene, camera, aspect)?),
            elapsed: 0.0,
        })
    }
    async fn scene_21(
        scene: &mut Scene,
        camera: Object3D,
        example: u32,
        renderer: &crate::renderer::Renderer,
        aspect: f64,
    ) -> Result<Self> {
        let _ = (example, renderer);
        {
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
    }
    pub fn update(
        &mut self,
        scene: &mut Scene,
        camera: Object3D,
        delta: f64,
        animate: bool,
    ) -> Result<()> {
        if let Content::Module(demo) = &mut self.content
            && let Some(result) = demo.on_update(scene, camera, delta, animate)
        {
            return result;
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
    pub fn select(&mut self, scene: &mut Scene, cam: Object3D, x: f64, y: f64) -> Result<()> {
        if let Content::Module(demo) = &mut self.content
            && let Some(result) = demo.on_select(scene, cam, x, y)
        {
            return result;
        }
        Ok(())
    }
    pub fn gpu_pointer(
        &mut self,
        r: &crate::renderer::Renderer,
        scene: &mut Scene,
        cam: Object3D,
        x: f64,
        y: f64,
    ) -> Result<bool> {
        if let Content::Module(demo) = &mut self.content
            && let Some(result) = demo.on_gpu_pointer(r, scene, cam, x, y)
        {
            return result;
        }
        Ok(false)
    }
    pub fn pointer(&mut self, x: f64, y: f64) {
        if let Content::Module(demo) = &mut self.content {
            demo.on_pointer(x, y);
        }
        if let Content::VertexColors { pointer } | Content::ColorLines { pointer, .. } =
            &mut self.content
        {
            *pointer = Vector2::new(x, y);
        }
    }
    pub fn dragging(&mut self, value: bool) {
        if let Content::Module(demo) = &mut self.content {
            demo.on_dragging(value);
        }
    }
    pub fn render(
        &mut self,
        renderer: &crate::renderer::Renderer,
        scene: &mut Scene,
        camera: Object3D,
        target: &crate::renderer::RenderTarget,
    ) -> Result<bool> {
        if let Content::Module(demo) = &mut self.content
            && let Some(result) = demo.on_render(renderer, scene, camera, target)
        {
            return result;
        }
        Ok(false)
    }
    pub fn output_target(&self) -> Option<&crate::renderer::RenderTarget> {
        if let Content::Module(demo) = &self.content
            && let Some(result) = demo.on_output_target()
        {
            return result;
        }
        None
    }
    /// The target `output_target` returns, to render into the canvas ( see
    /// RenderTarget::alias_color ).
    pub fn output_target_mut(&mut self) -> Option<&mut crate::renderer::RenderTarget> {
        match &mut self.content {
            Content::Module(demo) => demo.on_output_target_mut().unwrap_or(None),
            _ => None,
        }
    }
    pub fn prepare(
        &mut self,
        renderer: &crate::renderer::Renderer,
        scene: &mut Scene,
        camera: Object3D,
        aspect: f64,
    ) -> Result<()> {
        if let Content::Module(demo) = &mut self.content {
            demo.on_prepare(renderer, scene, camera, aspect)?;
        }
        Ok(())
    }
    pub fn tsl_parameter(&mut self, index: usize, value: f32) -> Result<()> {
        if let Content::Module(demo) = &mut self.content
            && let Some(result) = demo.on_tsl_parameter(index, value)
        {
            return result;
        }
        Err(crate::Error::Invalid("not a TSL example"))
    }
    /// webaudio_timing's listener, sources and pending plays.
    pub fn audio_frame(
        &mut self,
        scene: &mut crate::scene::Scene,
        camera: crate::scene::Object3D,
    ) -> crate::Result<Vec<f32>> {
        match &mut self.content {
            Content::Module(demo) => demo
                .on_audio_frame(scene, camera)
                .unwrap_or(Err(crate::Error::Invalid("not a positional audio example"))),
            _ => Err(crate::Error::Invalid("not a positional audio example")),
        }
    }
    /// webaudio_visualizer's analyser bins.
    pub fn audio_data(&mut self, bytes: &[u8]) -> crate::Result<()> {
        match &mut self.content {
            Content::Module(demo) => demo
                .on_audio_data(bytes)
                .unwrap_or(Err(crate::Error::Invalid("not an analyser example"))),
            _ => Err(crate::Error::Invalid("not an analyser example")),
        }
    }
    /// A file produced by an exporter button, taken once.
    pub fn take_export(
        &mut self,
        renderer: &crate::renderer::Renderer,
    ) -> Option<(String, Vec<u8>)> {
        match &mut self.content {
            Content::Module(demo) => demo.on_take_export(renderer).unwrap_or(None),
            _ => None,
        }
    }
    pub fn status(&self) -> String {
        if let Content::Module(demo) = &self.content
            && let Some(result) = demo.on_status()
        {
            return result;
        }
        String::new()
    }
    pub fn key(&mut self, code: u32, down: bool) {
        if let Content::Module(demo) = &mut self.content {
            demo.on_key(code, down);
        }
    }
    pub fn slider(&mut self, x: f64) {
        if let Content::Module(demo) = &mut self.content {
            demo.on_slider(x);
        }
    }
    pub fn draw(&mut self, kind: u32, x: f64, y: f64) -> Result<()> {
        if let Content::Module(demo) = &mut self.content
            && let Some(result) = demo.on_draw(kind, x, y)
        {
            return result;
        }
        Err(crate::Error::Invalid("not a drawing example"))
    }
    pub fn seek(&mut self, seconds: f64) {
        if let Content::Module(demo) = &mut self.content {
            demo.on_seek(seconds);
        }
        self.elapsed = seconds;
        if let Content::Horse(horse) = &mut self.content {
            horse.theta = seconds * 6.0;
        }
    }
    #[allow(clippy::too_many_arguments)]
    pub fn input(
        &mut self,
        scene: &mut Scene,
        camera: Object3D,
        dx: f64,
        dy: f64,
        wheel: f64,
        pan: bool,
        height: f64,
    ) -> Result<()> {
        if let Content::Module(demo) = &mut self.content
            && let Some(result) = demo.on_input(scene, camera, dx, dy, wheel, pan, height)
        {
            return result;
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

impl Demo {
    pub fn viewport(&mut self, index: usize, rectangle: [f64; 4]) -> Result<()> {
        if let Content::Module(demo) = &mut self.content
            && let Some(result) = demo.on_viewport(index, rectangle)
        {
            return result;
        }
        Err(crate::Error::Invalid("multiple elements example"))
    }
}

impl Demo {
    /// After the frame is submitted, in the same task: effects that read the
    /// canvas back (AsciiEffect).
    pub fn presented(&mut self, canvas: &web_sys::HtmlCanvasElement) -> Result<()> {
        if let Content::Module(demo) = &mut self.content {
            demo.on_presented(canvas)?;
        }
        Ok(())
    }
    pub fn attach_canvases(
        &mut self,
        r: &crate::renderer::Renderer,
        canvases: js_sys::Array,
    ) -> Result<()> {
        if let Content::Module(demo) = &mut self.content
            && let Some(result) = demo.on_attach_canvases(r, canvases)
        {
            return result;
        }
        Err(crate::Error::Invalid("multiple canvases example"))
    }
}

/// The program of three.js examples/webgl_loader_gltf_avif.html: its renderer, its scene and its
/// frame loop.
#[wasm_bindgen::prelude::wasm_bindgen]
pub async fn webgl_loader_gltf_avif(
    canvas: web_sys::HtmlCanvasElement,
    animate: bool,
) -> std::result::Result<super::BrowserApp, wasm_bindgen::JsValue> {
    super::program(
        canvas,
        animate,
        super::PageOptions {
            encode_srgb: true,
            format: super::TargetFormat::Canvas,
            raw_output: true,
            on_demand: true,
            stop_when_paused: false,
            ..super::PageOptions::GALLERY
        },
        async |page| {
            let (scene, camera, renderer, aspect) = page.gallery_scene()?;
            Ok(super::PageContent::Gallery(
                super::gallery_scenes::GalleryScene::Expanded(
                    Demo::scene_28(scene, camera, 28, renderer, aspect).await?,
                ),
            ))
        },
    )
    .await
}

/// The program of three.js examples/webgl_buffergeometry.html: its renderer, its scene and its
/// frame loop.
#[wasm_bindgen::prelude::wasm_bindgen]
pub async fn webgl_buffergeometry(
    canvas: web_sys::HtmlCanvasElement,
    animate: bool,
) -> std::result::Result<super::BrowserApp, wasm_bindgen::JsValue> {
    super::program(
        canvas,
        animate,
        super::PageOptions {
            stop_when_paused: false,
            ..super::PageOptions::GALLERY
        },
        async |page| {
            let (scene, camera, renderer, aspect) = page.gallery_scene()?;
            Ok(super::PageContent::Gallery(
                super::gallery_scenes::GalleryScene::Expanded(
                    Demo::scene_26_27(scene, camera, 26, renderer, aspect).await?,
                ),
            ))
        },
    )
    .await
}

/// The program of three.js examples/webgl_buffergeometry_rawshader.html: its renderer, its scene
/// and its frame loop.
#[wasm_bindgen::prelude::wasm_bindgen]
pub async fn webgl_buffergeometry_rawshader(
    canvas: web_sys::HtmlCanvasElement,
    animate: bool,
) -> std::result::Result<super::BrowserApp, wasm_bindgen::JsValue> {
    super::program(
        canvas,
        animate,
        super::PageOptions {
            samples: 1,
            format: super::TargetFormat::Canvas,
            raw_output: true,
            stop_when_paused: false,
            ..super::PageOptions::GALLERY
        },
        async |page| {
            let (scene, camera, renderer, aspect) = page.gallery_scene()?;
            Ok(super::PageContent::Gallery(
                super::gallery_scenes::GalleryScene::Expanded(
                    Demo::scene_26_27(scene, camera, 27, renderer, aspect).await?,
                ),
            ))
        },
    )
    .await
}

/// The program of three.js examples/webgl_loader_gltf_instancing.html: its renderer, its scene and
/// its frame loop.
#[wasm_bindgen::prelude::wasm_bindgen]
pub async fn webgl_loader_gltf_instancing(
    canvas: web_sys::HtmlCanvasElement,
    animate: bool,
) -> std::result::Result<super::BrowserApp, wasm_bindgen::JsValue> {
    super::program(
        canvas,
        animate,
        super::PageOptions {
            encode_srgb: true,
            format: super::TargetFormat::Canvas,
            raw_output: true,
            on_demand: true,
            stop_when_paused: false,
            ..super::PageOptions::GALLERY
        },
        async |page| {
            let (scene, camera, renderer, aspect) = page.gallery_scene()?;
            Ok(super::PageContent::Gallery(
                super::gallery_scenes::GalleryScene::Expanded(
                    Demo::scene_16(scene, camera, 16, renderer, aspect).await?,
                ),
            ))
        },
    )
    .await
}

/// The program of three.js examples/webgl_geometries.html: its renderer, its scene and its frame
/// loop.
#[wasm_bindgen::prelude::wasm_bindgen]
pub async fn webgl_geometries(
    canvas: web_sys::HtmlCanvasElement,
    animate: bool,
) -> std::result::Result<super::BrowserApp, wasm_bindgen::JsValue> {
    super::program(
        canvas,
        animate,
        super::PageOptions {
            stop_when_paused: false,
            ..super::PageOptions::GALLERY
        },
        async |page| {
            let (scene, camera, renderer, aspect) = page.gallery_scene()?;
            Ok(super::PageContent::Gallery(
                super::gallery_scenes::GalleryScene::Expanded(
                    Demo::scene_17(scene, camera, 17, renderer, aspect).await?,
                ),
            ))
        },
    )
    .await
}

/// The program of three.js examples/webgpu_morphtargets.html: its renderer, its scene and its frame
/// loop.
#[wasm_bindgen::prelude::wasm_bindgen]
pub async fn webgpu_morphtargets(
    canvas: web_sys::HtmlCanvasElement,
    animate: bool,
) -> std::result::Result<super::BrowserApp, wasm_bindgen::JsValue> {
    super::program(
        canvas,
        animate,
        super::PageOptions {
            stop_when_paused: false,
            ..super::PageOptions::GALLERY
        },
        async |page| {
            let (scene, camera, renderer, aspect) = page.gallery_scene()?;
            Ok(super::PageContent::Gallery(
                super::gallery_scenes::GalleryScene::Expanded(
                    Demo::scene_18(scene, camera, 18, renderer, aspect).await?,
                ),
            ))
        },
    )
    .await
}

/// The program of three.js examples/webgl_lines_dashed.html: its renderer, its scene and its frame
/// loop.
#[wasm_bindgen::prelude::wasm_bindgen]
pub async fn webgl_lines_dashed(
    canvas: web_sys::HtmlCanvasElement,
    animate: bool,
) -> std::result::Result<super::BrowserApp, wasm_bindgen::JsValue> {
    super::program(
        canvas,
        animate,
        super::PageOptions {
            stop_when_paused: false,
            ..super::PageOptions::GALLERY
        },
        async |page| {
            let (scene, camera, renderer, aspect) = page.gallery_scene()?;
            Ok(super::PageContent::Gallery(
                super::gallery_scenes::GalleryScene::Expanded(
                    Demo::scene_19(scene, camera, 19, renderer, aspect).await?,
                ),
            ))
        },
    )
    .await
}

/// The program of three.js examples/webgl_morphtargets_horse.html: its renderer, its scene and its
/// frame loop.
#[wasm_bindgen::prelude::wasm_bindgen]
pub async fn webgl_morphtargets_horse(
    canvas: web_sys::HtmlCanvasElement,
    animate: bool,
) -> std::result::Result<super::BrowserApp, wasm_bindgen::JsValue> {
    super::program(
        canvas,
        animate,
        super::PageOptions {
            samples: 1,
            stop_when_paused: false,
            ..super::PageOptions::GALLERY
        },
        async |page| {
            let (scene, camera, renderer, aspect) = page.gallery_scene()?;
            Ok(super::PageContent::Gallery(
                super::gallery_scenes::GalleryScene::Expanded(
                    Demo::scene_24(scene, camera, 24, renderer, aspect).await?,
                ),
            ))
        },
    )
    .await
}

/// The program of three.js examples/webgl_morphtargets_sphere.html: its renderer, its scene and its
/// frame loop.
#[wasm_bindgen::prelude::wasm_bindgen]
pub async fn webgl_morphtargets_sphere(
    canvas: web_sys::HtmlCanvasElement,
    animate: bool,
) -> std::result::Result<super::BrowserApp, wasm_bindgen::JsValue> {
    super::program(
        canvas,
        animate,
        super::PageOptions {
            samples: 1,
            stop_when_paused: false,
            ..super::PageOptions::GALLERY
        },
        async |page| {
            let (scene, camera, renderer, aspect) = page.gallery_scene()?;
            Ok(super::PageContent::Gallery(
                super::gallery_scenes::GalleryScene::Expanded(
                    Demo::scene_25(scene, camera, 25, renderer, aspect).await?,
                ),
            ))
        },
    )
    .await
}

/// The program of three.js examples/webgl_lines_colors.html: its renderer, its scene and its frame
/// loop.
#[wasm_bindgen::prelude::wasm_bindgen]
pub async fn webgl_lines_colors(
    canvas: web_sys::HtmlCanvasElement,
    animate: bool,
) -> std::result::Result<super::BrowserApp, wasm_bindgen::JsValue> {
    super::program(
        canvas,
        animate,
        super::PageOptions {
            stop_when_paused: false,
            ..super::PageOptions::GALLERY
        },
        async |page| {
            let (scene, camera, renderer, aspect) = page.gallery_scene()?;
            Ok(super::PageContent::Gallery(
                super::gallery_scenes::GalleryScene::Expanded(
                    Demo::scene_23(scene, camera, 23, renderer, aspect).await?,
                ),
            ))
        },
    )
    .await
}

/// The program of three.js examples/webgl_buffergeometry_indexed.html: its renderer, its scene and
/// its frame loop.
#[wasm_bindgen::prelude::wasm_bindgen]
pub async fn webgl_buffergeometry_indexed(
    canvas: web_sys::HtmlCanvasElement,
    animate: bool,
) -> std::result::Result<super::BrowserApp, wasm_bindgen::JsValue> {
    super::program(
        canvas,
        animate,
        super::PageOptions {
            stop_when_paused: false,
            ..super::PageOptions::GALLERY
        },
        async |page| {
            let (scene, camera, renderer, aspect) = page.gallery_scene()?;
            Ok(super::PageContent::Gallery(
                super::gallery_scenes::GalleryScene::Expanded(
                    Demo::scene_22(scene, camera, 22, renderer, aspect).await?,
                ),
            ))
        },
    )
    .await
}

/// The program of three.js examples/webgl_geometry_colors.html: its renderer, its scene and its
/// frame loop.
#[wasm_bindgen::prelude::wasm_bindgen]
pub async fn webgl_geometry_colors(
    canvas: web_sys::HtmlCanvasElement,
    animate: bool,
) -> std::result::Result<super::BrowserApp, wasm_bindgen::JsValue> {
    super::program(
        canvas,
        animate,
        super::PageOptions {
            stop_when_paused: false,
            ..super::PageOptions::GALLERY
        },
        async |page| {
            let (scene, camera, renderer, aspect) = page.gallery_scene()?;
            Ok(super::PageContent::Gallery(
                super::gallery_scenes::GalleryScene::Expanded(
                    Demo::scene_21(scene, camera, 21, renderer, aspect).await?,
                ),
            ))
        },
    )
    .await
}
