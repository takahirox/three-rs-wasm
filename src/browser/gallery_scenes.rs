//! Scene-specific ports. Keep upstream algorithms here rather than adding a second renderer.
use super::gltf_viewer::{OrbitViewer, decode_image, fetch};
use crate::raycast::Raycaster;
use crate::{
    Result, attribute::BufferAttribute, camera::*, geometry::*, material::*, math::*, scene::*,
};
use std::sync::Arc;

pub(super) enum GalleryScene {
    Expanded(super::expanded::Demo),
    Robot(super::robot::Robot),
    Calibration {
        viewer: OrbitViewer,
        light: Object3D,
    },
    Equirectangular {
        viewer: OrbitViewer,
        dragging: bool,
    },
    TextureRotation {
        mesh: Object3D,
        viewer: OrbitViewer,
    },
    MorphLine {
        object: Object3D,

        time: f64,
    },
    Snowflake {
        root: Object3D,
        time: f64,
    },
    Picking {
        objects: Vec<Object3D>,
        markers: Vec<Object3D>,
        pointer: Vector2,
        angle: f64,
        points: bool,
        elapsed: f64,
        marker: usize,
    },
    Panorama {
        lon: f64,
        lat: f64,
        dragging: bool,
    },
}
// A repeatable random sequence makes the original random examples comparable.
struct Random(u32);
impl Random {
    fn next(&mut self) -> f64 {
        self.0 = self.0.wrapping_mul(1664525).wrapping_add(1013904223);
        self.0 as f64 / 4294967296.0
    }
}
struct Snowflake {
    positions: Vec<Vector3>,
    colors: Vec<f32>,
    indices: Vec<u32>,
    random: Random,
}
impl Snowflake {
    fn vertex(&mut self, p: Vector3) {
        self.positions.push(p);
        self.colors.extend([
            (self.random.next() * 0.5 + 0.5) as f32,
            (self.random.next() * 0.5 + 0.5) as f32,
            1.0,
        ]);
    }
    fn segment(&mut self, p0: Vector3, p4: Vector3, depth: u32) {
        if depth == 0 {
            let i = self.positions.len() as u32 - 1;
            self.vertex(p4);
            self.indices.extend([i, i + 1]);
            return;
        }
        let v = p4 - p0;
        let tier = v / 3.0;
        let p1 = p0 + tier;
        let angle = v.y.atan2(v.x) + std::f64::consts::PI / 3.0;
        let p2 = p1 + Vector3::new(angle.cos(), angle.sin(), 0.0) * tier.length();
        let p3 = p0 + tier * 2.0;
        for (a, b) in [(p0, p1), (p1, p2), (p2, p3), (p3, p4)] {
            self.segment(a, b, depth - 1);
        }
    }
    fn curve(&mut self, mut points: Vec<Vector3>, closed: bool) {
        for depth in 0..4 {
            self.vertex(points[0]);
            for pair in points.windows(2) {
                self.segment(pair[0], pair[1], depth);
            }
            if closed {
                self.segment(*points.last().unwrap(), points[0], depth);
            }
            for p in &mut points {
                p.x += 600.0;
            }
        }
    }
}
impl GalleryScene {
    pub async fn create(
        scene: &mut Scene,
        camera: Object3D,
        placeholder: Object3D,
        example: u32,
        renderer: &crate::renderer::Renderer,
    ) -> Result<Self> {
        scene.dispose(placeholder)?;
        scene.background = Color::BLACK;
        let aspect = match &scene.get(camera)?.kind {
            NodeKind::Camera(Camera::Perspective(p)) => p.aspect,
            _ => 1.0,
        };
        if example >= 16 {
            Ok(Self::Expanded(
                super::expanded::Demo::create(scene, camera, example, renderer).await?,
            ))
        } else if example == 15 {
            Ok(Self::Robot(
                super::robot::Robot::create(scene, camera, aspect).await?,
            ))
        } else if example == 14 {
            scene.get_mut(camera)?.kind =
                NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
                    fov: 40.0,
                    aspect,
                    near: 1.0,
                    far: 30.0,
                    ..Default::default()
                }));
            scene.environment = Some(Arc::new(crate::environment::EnvironmentMap::from_hdr(
                &fetch("/web/gallery/assets/spot1Lux.hdr").await?,
            )?));
            scene.background_environment = true;
            scene.aces_tone_mapping = true;
            let geometry = Arc::new(SphereGeometry::build(0.4, 32, 32)?);
            for x in 0..=10 {
                for y in 0..=2 {
                    let mut material = MeshStandardMaterial {
                        roughness: x as f64 / 10.0,
                        metalness: if y < 1 { 1.0 } else { 0.0 },
                        ..Default::default()
                    };
                    material.properties.color = if y < 2 { Color::WHITE } else { Color::BLACK };
                    let mesh = scene.insert(NodeKind::Mesh(Mesh::new(
                        geometry.clone(),
                        Arc::new(Material::Standard(material)),
                    )));
                    scene.get_mut(mesh)?.position =
                        Vector3::new(x as f64 - 5.0, 1.0 - y as f64, 0.0);
                }
            }
            let theta = (597.0 + 0.5) * std::f64::consts::PI / 512.0;
            let phi = -(213.0 + 0.5) * std::f64::consts::PI / 512.0;
            let angle = std::f64::consts::FRAC_PI_2 - theta;
            let light = scene.insert(NodeKind::Light(Light::Directional {
                color: Color::WHITE,
                intensity: 0.0,
                target: Vector3::ZERO,
            }));
            scene.get_mut(light)?.position =
                Vector3::new(phi.sin() * angle.sin(), phi.cos(), phi.sin() * angle.cos()) * 100.0;
            Ok(Self::Calibration {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 16.0),
                light,
            })
        } else if example == 13 {
            scene.get_mut(camera)?.kind =
                NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
                    fov: 45.0,
                    aspect,
                    near: 0.25,
                    far: 20.0,
                    ..Default::default()
                }));
            let texture = decode_image(&fetch("/web/gallery/assets/panorama.jpg").await?).await?;
            scene.environment = Some(Arc::new(crate::environment::EnvironmentMap::from_texture(
                &texture,
            )?));
            scene.background_environment = true;
            scene.background_equirectangular = true;
            let mut viewer = OrbitViewer::from_camera(Vector3::ZERO, 1.0);
            viewer.fixture(std::f64::consts::FRAC_PI_2, 0.0, 1.8);
            Ok(Self::Equirectangular {
                viewer,
                dragging: false,
            })
        } else if example == 11 {
            scene.get_mut(camera)?.kind =
                NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
                    fov: 40.0,
                    aspect,
                    near: 1.0,
                    far: 1000.0,
                    ..Default::default()
                }));
            let position = Vector3::new(10.0, 15.0, 25.0);
            let mut viewer = OrbitViewer::from_camera(Vector3::ZERO, position.length());
            viewer.fixture(
                position.x.atan2(position.z),
                (position.y / position.length()).asin(),
                1.8,
            );
            let mut texture =
                decode_image(&fetch("/web/gallery/assets/uv-grid.jpg").await?).await?;
            texture.anisotropy = 16;
            texture.wrap_s = Wrapping::Repeat;
            texture.wrap_t = Wrapping::Repeat;
            texture.mipmap_filter = Some(Filter::Linear);
            texture.repeat = Vector2::splat(0.25);
            texture.center = Vector2::splat(0.5);
            texture.rotation = std::f64::consts::FRAC_PI_4;
            let mut material = MeshBasicMaterial::default();
            material.properties.map = Some(Arc::new(texture));
            let mesh = scene.insert(NodeKind::Mesh(Mesh::new(
                Arc::new(BoxGeometry::build(10.0, 10.0, 10.0)?),
                Arc::new(Material::Basic(material)),
            )));
            Ok(Self::TextureRotation { mesh, viewer })
        } else if example == 12 {
            scene.get_mut(camera)?.kind =
                NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
                    fov: 27.0,
                    aspect,
                    near: 1.0,
                    far: 4000.0,
                    ..Default::default()
                }));
            scene.get_mut(camera)?.position = Vector3::new(0.0, 0.0, 2750.0);
            let mut random = Random(1);
            let mut base = Vec::with_capacity(10000);
            let mut colors = Vec::with_capacity(30000);
            for _ in 0..10000 {
                let p = Vector3::new(random.next(), random.next(), random.next()) * 800.0
                    - Vector3::splat(400.0);
                base.push(p);
                colors.extend([
                    (p.x / 800.0 + 0.5) as f32,
                    (p.y / 800.0 + 0.5) as f32,
                    (p.z / 800.0 + 0.5) as f32,
                ]);
            }
            let target: Vec<Vector3> = (0..10000)
                .map(|_| {
                    Vector3::new(random.next(), random.next(), random.next()) * 800.0
                        - Vector3::splat(400.0)
                })
                .collect();
            // Morph attributes in the original are float32, including their base.
            for p in &mut base {
                *p = p.as_vec3().as_dvec3();
            }
            let target: Vec<Vector3> = target
                .into_iter()
                .map(|p: Vector3| p.as_vec3().as_dvec3())
                .collect();
            let mut geometry = BufferGeometry::default();
            geometry.set_from_points(&base)?;
            geometry.set_attribute(
                "color",
                Attribute::F32(BufferAttribute::new(colors, 3, false)?),
            );
            geometry.morph_attributes.insert(
                "position".into(),
                vec![Attribute::F32(BufferAttribute::new(
                    target.iter().flat_map(|p| p.as_vec3().to_array()).collect(),
                    3,
                    false,
                )?)],
            );
            let mut material = LineBasicMaterial::default();
            material.properties.vertex_colors = true;
            let object = scene.insert(NodeKind::Line(Line {
                geometry: Arc::new(geometry),
                material: Arc::new(Material::Line(material)),
                segments: false,
            }));
            Ok(Self::MorphLine { object, time: 0.0 })
        } else if example == 7 {
            scene.get_mut(camera)?.kind =
                NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
                    fov: 27.0,
                    aspect,
                    near: 1.0,
                    far: 10000.0,
                    ..Default::default()
                }));
            scene.get_mut(camera)?.position = Vector3::new(0.0, 0.0, 9000.0);
            let mut snow = Snowflake {
                positions: vec![],
                colors: vec![],
                indices: vec![],
                random: Random(1),
            };
            let points = |values: &[(f64, f64)]| {
                values
                    .iter()
                    .map(|&(x, y)| Vector3::new(x, y, 0.0))
                    .collect()
            };
            snow.curve(points(&[(0.0, 0.0), (500.0, 0.0)]), false);
            snow.curve(
                points(&[(0.0, 600.0), (250.0, 1000.0), (500.0, 600.0)]),
                true,
            );
            snow.curve(
                points(&[
                    (0.0, 1200.0),
                    (500.0, 1200.0),
                    (500.0, 1700.0),
                    (0.0, 1700.0),
                ]),
                true,
            );
            snow.curve(
                points(&[
                    (250.0, 2200.0),
                    (500.0, 2200.0),
                    (250.0, 2200.0),
                    (250.0, 2450.0),
                    (250.0, 2200.0),
                    (0.0, 2200.0),
                    (250.0, 2200.0),
                    (250.0, 1950.0),
                    (250.0, 2200.0),
                ]),
                false,
            );
            let mut geometry = BufferGeometry::default();
            geometry.set_from_points(&snow.positions)?;
            geometry.set_attribute(
                "color",
                Attribute::F32(BufferAttribute::new(snow.colors, 3, false)?),
            );
            geometry.set_index(Some(snow.indices));
            let mut material = LineBasicMaterial::default();
            material.properties.vertex_colors = true;
            let line = scene.insert(NodeKind::Line(Line {
                geometry: Arc::new(geometry),
                material: Arc::new(Material::Line(material)),
                segments: true,
            }));
            scene.get_mut(line)?.position = Vector3::new(-1200.0, -1200.0, 0.0);
            let root = scene.insert(NodeKind::Group);
            scene.add(root, line)?;
            Ok(Self::Snowflake { root, time: 0.0 })
        } else if example == 8 {
            scene.get_mut(camera)?.kind =
                NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
                    fov: 75.0,
                    aspect,
                    near: 1.0,
                    far: 1100.0,
                    ..Default::default()
                }));
            scene.get_mut(camera)?.position = Vector3::ZERO;
            let mut geometry = SphereGeometry::build(500.0, 60, 40)?;
            geometry.scale(Vector3::new(-1.0, 1.0, 1.0))?;
            let mut texture =
                decode_image(&fetch("/web/gallery/assets/panorama.jpg").await?).await?;
            texture.mipmap_filter = Some(Filter::Linear);
            let mut material = MeshBasicMaterial::default();
            material.properties.map = Some(Arc::new(texture));
            scene.insert(NodeKind::Mesh(Mesh::new(
                Arc::new(geometry),
                Arc::new(Material::Basic(material)),
            )));
            Ok(Self::Panorama {
                lon: 0.0,
                lat: 0.0,
                dragging: false,
            })
        } else {
            Self::picking(scene, camera, aspect, example == 10)
        }
    }
    pub fn update(
        &mut self,
        scene: &mut Scene,
        camera: Object3D,
        delta: f64,
        animate: bool,
    ) -> Result<()> {
        match self {
            Self::Expanded(demo) => demo.update(scene, camera, delta, animate)?,
            Self::Robot(robot) => robot.update(scene, camera, delta, animate)?,
            Self::Calibration { viewer, .. } => {
                viewer.update(scene, camera)?;
                if let NodeKind::Camera(Camera::Perspective(p)) = &mut scene.get_mut(camera)?.kind {
                    p.fov = 2.0
                        * ((20.0_f64.to_radians()).tan() / p.aspect)
                            .atan()
                            .to_degrees();
                }
            }
            Self::Equirectangular { viewer, dragging } => {
                if animate && !*dragging {
                    viewer.orbit(delta * std::f64::consts::TAU / 60.0 / 0.008, 0.0, 0.0);
                }
                viewer.update(scene, camera)?;
            }
            Self::TextureRotation { viewer, .. } => {
                viewer.update(scene, camera)?;
            }
            Self::MorphLine { object, time } => {
                if animate {
                    *time += delta;
                } else {
                    *time = 1.23;
                }
                let weight = (*time * 0.5).sin().abs();
                let n = scene.get_mut(*object)?;
                n.quaternion = Euler {
                    angles: Vector3::new(*time * 0.25, *time * 0.5, 0.0),
                    order: EulerOrder::XYZ,
                }
                .quaternion();
                n.morph_weights = vec![weight];
            }
            Self::Snowflake { root, time } => {
                if animate {
                    *time += delta;
                }
                scene.get_mut(*root)?.quaternion =
                    Quaternion::from_rotation_z(if animate { *time * 0.5 } else { 1.23 * 0.5 });
            }
            Self::Picking {
                objects,
                markers,
                pointer,
                angle,
                points,
                elapsed,
                marker,
            } => {
                if animate {
                    *angle += if *points { 0.005 } else { 0.1_f64.to_radians() };
                    *elapsed += delta;
                }
                let (radius, y) = if *points {
                    (200.0_f64.sqrt(), 10.0)
                } else {
                    (100.0, 100.0 * angle.sin())
                };
                let a = *angle
                    + if *points {
                        std::f64::consts::FRAC_PI_4
                    } else {
                        0.0
                    };
                scene.get_mut(camera)?.position =
                    Vector3::new(radius * a.sin(), y, radius * a.cos());
                scene.look_at(camera, Vector3::ZERO)?;
                scene.update()?;
                let mut ray = Raycaster::default();
                ray.params.line_threshold = 3.0;
                ray.params.points_threshold = 0.1;
                let (c, world) = scene.camera(camera)?;
                ray.set_from_camera(*pointer, c, world)?;
                let hits = ray.intersect_objects(scene, objects, true)?;
                if *points {
                    if *elapsed > 0.02
                        && let Some(hit) = hits.first()
                    {
                        let m = scene.get_mut(markers[*marker])?;
                        m.position = hit.point;
                        m.scale = Vector3::ONE;
                        *marker = (*marker + 1) % markers.len();
                        *elapsed = 0.0;
                    }
                    if animate {
                        for &m in markers.iter() {
                            let n = scene.get_mut(m)?;
                            n.scale = (n.scale * 0.98).clamp(Vector3::splat(0.01), Vector3::ONE);
                        }
                    }
                } else {
                    let m = scene.get_mut(markers[0])?;
                    m.visible = !hits.is_empty();
                    if let Some(hit) = hits.first() {
                        m.position = hit.point;
                    }
                }
            }
            Self::Panorama { lon, lat, dragging } => {
                if animate && !*dragging {
                    *lon += 0.1;
                }
                *lat = lat.clamp(-85.0, 85.0);
                let phi = (90.0 - *lat).to_radians();
                let theta = lon.to_radians();
                scene.look_at(
                    camera,
                    Vector3::new(phi.sin() * theta.cos(), phi.cos(), phi.sin() * theta.sin())
                        * 500.0,
                )?;
            }
        }
        Ok(())
    }
    pub fn pmrem(&mut self, scene: &mut Scene, enabled: bool) -> Result<()> {
        if let Self::Calibration { light, .. } = self {
            scene.environment_intensity = if enabled { 1.0 } else { 0.0 };
            if let NodeKind::Light(Light::Directional { intensity, .. }) =
                &mut scene.get_mut(*light)?.kind
            {
                *intensity = if enabled { 0.0 } else { 1.0 };
            }
        }
        Ok(())
    }
    pub fn texture_transform(&mut self, scene: &mut Scene, values: &[f64]) -> Result<()> {
        if values.len() != 7 || !values.iter().all(|v| v.is_finite()) {
            return Ok(());
        }
        if let Self::TextureRotation { mesh, .. } = self
            && let NodeKind::Mesh(m) = &mut scene.get_mut(*mesh)?.kind
        {
            let material = Arc::make_mut(&mut m.materials[0]);
            if let Some(map) = &mut material.properties_mut().map {
                let texture = Arc::make_mut(map);
                texture.offset = Vector2::new(values[0], values[1]);
                texture.repeat = Vector2::new(values[2], values[3]);
                texture.rotation = values[4];
                texture.center = Vector2::new(values[5], values[6]);
            }
        }
        Ok(())
    }
    pub fn pointer(&mut self, x: f64, y: f64) {
        if let Self::Picking { pointer, .. } = self {
            *pointer = Vector2::new(x, y);
        }
    }
    fn picking(scene: &mut Scene, camera: Object3D, aspect: f64, points: bool) -> Result<Self> {
        scene.get_mut(camera)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: if points { 45.0 } else { 70.0 },
            aspect,
            near: 1.0,
            far: 10000.0,
            ..Default::default()
        }));
        let mut objects = vec![];
        if points {
            for (i, color) in [Vector3::X, Vector3::Y, Vector3::new(0.0, 1.0, 1.0)]
                .into_iter()
                .enumerate()
            {
                let mut positions = vec![];
                let mut colors = vec![];
                for u in 0..80 {
                    for v in 0..160 {
                        let u = u as f64 / 80.0;
                        let v = v as f64 / 160.0;
                        let y = ((u * std::f64::consts::PI * 4.0).cos()
                            + (v * std::f64::consts::PI * 8.0).sin())
                            / 20.0;
                        positions.push(Vector3::new(u - 0.5, y, v - 0.5));
                        let c = color * ((y + 0.1) * 5.0);
                        colors.extend([c.x as f32, c.y as f32, c.z as f32]);
                    }
                }
                let mut geometry = BufferGeometry::default();
                geometry.set_from_points(&positions)?;
                geometry.set_attribute(
                    "color",
                    Attribute::F32(BufferAttribute::new(colors, 3, false)?),
                );
                if i > 0 {
                    geometry.set_index(Some((0..12800).collect()));
                }
                if i == 2 {
                    geometry.add_group(0, 12800, 0);
                }
                let mut material = PointsMaterial {
                    size: 0.05,
                    ..Default::default()
                };
                material.properties.vertex_colors = true;
                let object = scene.insert(NodeKind::Points(Points {
                    geometry: Arc::new(geometry),
                    material: Arc::new(Material::Points(material)),
                }));
                scene.get_mut(object)?.scale = Vector3::new(5.0, 10.0, 10.0);
                scene.get_mut(object)?.position.x = i as f64 * 5.0 - 5.0;
                objects.push(object);
            }
        } else {
            scene.background = Color::from_hex(0xf0f0f0);
            let mut random = Random(1);
            let mut position = Vector3::ZERO;
            let mut direction = Vector3::ZERO;
            let mut positions = vec![];
            for _ in 0..50 {
                direction += Vector3::new(
                    random.next() - 0.5,
                    random.next() - 0.5,
                    random.next() - 0.5,
                );
                direction = direction.normalize_or_zero() * 10.0;
                position += direction;
                positions.push(position);
            }
            let mut geometry = BufferGeometry::default();
            geometry.set_from_points(&positions)?;
            let geometry = Arc::new(geometry);
            fn transform(
                scene: &mut Scene,
                object: Object3D,
                random: &mut Random,
                spread: f64,
            ) -> Result<()> {
                let n = scene.get_mut(object)?;
                n.position = Vector3::new(
                    random.next() - 0.5,
                    random.next() - 0.5,
                    random.next() - 0.5,
                ) * spread;
                n.quaternion = Euler {
                    angles: Vector3::new(random.next(), random.next(), random.next())
                        * std::f64::consts::TAU,
                    order: EulerOrder::XYZ,
                }
                .quaternion();
                n.scale = Vector3::new(
                    random.next() + 0.5,
                    random.next() + 0.5,
                    random.next() + 0.5,
                );
                Ok(())
            }
            let parent = scene.insert(NodeKind::Group);
            transform(scene, parent, &mut random, 40.0)?;
            for _ in 0..50 {
                let mut material = LineBasicMaterial::default();
                material.properties.color =
                    Color::from_hex((random.next() * 0xffffff as f64) as u32);
                let segments = random.next() <= 0.5;
                let object = scene.insert(NodeKind::Line(Line {
                    geometry: geometry.clone(),
                    material: Arc::new(Material::Line(material)),
                    segments,
                }));
                transform(scene, object, &mut random, 400.0)?;
                scene.add(parent, object)?;
            }
            objects.push(parent);
        }
        let geometry = Arc::new(SphereGeometry::build(
            if points { 0.1 } else { 5.0 },
            32,
            32,
        )?);
        let mut material = MeshBasicMaterial::default();
        material.properties.color = Color::from_hex(0xff0000);
        let material = Arc::new(Material::Basic(material));
        let mut markers = vec![];
        for _ in 0..if points { 40 } else { 1 } {
            markers.push(scene.insert(NodeKind::Mesh(Mesh::new(
                geometry.clone(),
                material.clone(),
            ))));
        }
        Ok(Self::Picking {
            objects,
            markers,
            pointer: Vector2::ZERO,
            angle: 0.0,
            points,
            elapsed: 0.0,
            marker: 0,
        })
    }
    pub fn input(
        &mut self,
        scene: &mut Scene,
        camera: Object3D,
        dx: f64,
        dy: f64,
        wheel: f64,
        dragging: bool,
    ) -> Result<()> {
        if let Self::Robot(robot) = self {
            robot.viewer.orbit(dx, dy, wheel);
        }
        if let Self::Equirectangular {
            viewer,
            dragging: active,
        } = self
        {
            *active = dragging;
            viewer.orbit(-dx * 0.125, -dy * 0.125, wheel);
        }
        if let Self::TextureRotation { viewer, .. } | Self::Calibration { viewer, .. } = self {
            viewer.orbit(dx, dy, wheel);
        }
        if let Self::Panorama {
            lon,
            lat,
            dragging: active,
        } = self
        {
            *active = dragging;
            *lon -= dx * 0.1;
            *lat += dy * 0.1;
            if let NodeKind::Camera(Camera::Perspective(p)) = &mut scene.get_mut(camera)?.kind {
                p.fov = (p.fov + wheel * 0.05).clamp(10.0, 75.0);
            }
        }
        Ok(())
    }
}
