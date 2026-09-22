use super::*;
pub(in crate::browser) struct ShadowMap {
    viewer: OrbitViewer,
    time: f64,
    torus: Object3D,
    group: Object3D,
    light: Object3D,
}
impl ShadowMap {
    pub async fn create(s: &mut Scene, c: Object3D, r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 45.,
            aspect,
            near: 1.,
            far: 1000.,
            ..Default::default()
        }));
        let center = Vector3::new(0., 2., 0.);
        let p = Vector3::new(0., 10., 20.) - center;
        let mut viewer = OrbitViewer::from_camera(center, p.length());
        viewer.fixture(0., (p.y / p.length()).asin(), 1.8);
        s.background = Color::from_hex(0x222244);
        s.fog = Some(Fog::Linear {
            color: s.background,
            near: 50.,
            far: 100.,
        });
        s.tone_mapping = ToneMapping::Aces;
        s.shadow_map_size = 2048;
        s.insert(NodeKind::Light(Light::Ambient {
            color: Color::from_hex(0x444444),
            intensity: 2.,
        }));
        let spot = s.insert(NodeKind::Light(Light::Spot {
            color: Color::from_hex(0xff8888),
            intensity: 400.,
            target: Vector3::ZERO,
            distance: 0.,
            decay: 2.,
            angle: std::f64::consts::PI / 5.,
            penumbra: 0.3,
        }));
        {
            let n = s.get_mut(spot)?;
            n.position = Vector3::new(8., 10., 5.);
            n.cast_shadow = true;
            n.shadow.near = 8.;
            n.shadow.far = 200.;
            n.shadow.radius = 4.;
        }
        let light = s.insert(NodeKind::Light(Light::Directional {
            color: Color::from_hex(0x8888ff),
            intensity: 3.,
            target: Vector3::ZERO,
        }));
        {
            let n = s.get_mut(light)?;
            n.position = Vector3::new(3., 12., 17.);
            n.cast_shadow = true;
            n.shadow.near = 0.1;
            n.shadow.far = 500.;
            n.shadow.extent = 17.;
            n.shadow.radius = 4.;
        }
        let group = s.insert(NodeKind::Group);
        s.add(group, light)?;
        let base = MeshPhongMaterial {
            properties: MaterialProperties {
                color: Color::from_hex(0x999999),
                ..Default::default()
            },
            shininess: 0.,
            specular: Color::from_hex(0x222222),
            ..Default::default()
        };
        let mask = tsl::materialx::fractal(
            position_local() * float(0.1),
            tsl::materialx::Dimension::D3,
            float(3.),
            float(2.),
            float(0.5),
        )
        .greater_than(float(0.));
        let mut material = base.clone();
        material.properties.transparent = true;
        material.properties.vertex_program = Some(Arc::new(
            SurfaceNodes {
                mask: Some(mask.clone()),
                ..Default::default()
            }
            .build(r, &[], &[])
            .await?,
        ));
        material.properties.shadow_program =
            Some(Arc::new(shadow_program(r, None, Some(mask)).await?));
        let torus = mesh(
            s,
            Arc::new(TorusKnotGeometry::build(25., 8., 75, 80, 2, 3)?),
            Material::Phong(material),
        );
        {
            let n = s.get_mut(torus)?;
            n.position.y = 3.;
            n.scale = Vector3::splat(1. / 18.);
            n.cast_shadow = true;
            n.receive_shadow = true;
        }
        let cylinder = Arc::new(CylinderGeometry::build(
            0.75,
            0.75,
            7.,
            32,
            1,
            false,
            0.,
            std::f64::consts::TAU,
        )?);
        for (x, z) in [(8., 8.), (8., -8.), (-8., 8.), (-8., -8.)] {
            let h = mesh(s, cylinder.clone(), Material::Phong(base.clone()));
            let n = s.get_mut(h)?;
            n.position = Vector3::new(x, 3.5, z);
            n.cast_shadow = true;
        }
        let noise = tsl::materialx::fractal_vec3(
            position_world() * float(2.),
            tsl::materialx::Dimension::D3,
            float(3.),
            float(2.),
            float(0.5),
        )
        .clamp(float(0.), float(1.));
        let mut material = base;
        material.specular = Color::from_hex(0x111111);
        material.properties.vertex_program = Some(Arc::new(
            SurfaceNodes {
                color: Some(noise.swizzle("zzz") * float(0.2) + float(0.5)),
                shadow_position: Some(
                    position_world() + vec3(noise.x(), float(0.), noise.swizzle("z")),
                ),
                ..Default::default()
            }
            .build(r, &[], &[])
            .await?,
        ));
        material.properties.shadow_program = Some(Arc::new(shadow_program(r, None, None).await?));
        let ground = mesh(
            s,
            Arc::new(PlaneGeometry::build(200., 200., 1, 1)?),
            Material::Phong(material),
        );
        {
            let n = s.get_mut(ground)?;
            n.quaternion = Quaternion::from_rotation_x(-std::f64::consts::FRAC_PI_2);
            n.scale = Vector3::splat(3.);
            n.cast_shadow = true;
            n.receive_shadow = true;
        }
        Ok(Self {
            viewer,
            time: 0.,
            torus,
            group,
            light,
        })
    }
    pub fn update(&mut self, s: &mut Scene, c: Object3D, d: f64, a: bool) -> Result<()> {
        if a {
            self.time += d;
        }
        self.viewer.update(s, c)?;
        if let NodeKind::Camera(Camera::Perspective(p)) = &mut s.get_mut(c)?.kind {
            p.near = 1.;
            p.far = 1000.;
        }
        s.get_mut(self.torus)?.quaternion = Euler {
            angles: Vector3::new(self.time * 0.25, self.time * 0.5, self.time),
            order: EulerOrder::XYZ,
        }
        .quaternion();
        s.get_mut(self.light)?.position.z = 17. + self.time.sin() * 5.;
        s.get_mut(self.group)?.quaternion = Quaternion::from_rotation_y(self.time * 0.7);
        Ok(())
    }
    pub fn seek(&mut self, t: f64) {
        self.time = t;
    }
    #[allow(clippy::too_many_arguments)]
    pub fn input(
        &mut self,
        s: &Scene,
        c: Object3D,
        dx: f64,
        dy: f64,
        w: f64,
        p: bool,
        h: f64,
    ) -> Result<()> {
        if p {
            self.viewer.pan_pixels(s, c, dx, dy, h)?;
        } else {
            self.viewer.orbit_pixels(dx, dy, w, h, 7., 40.);
        }
        Ok(())
    }
}
