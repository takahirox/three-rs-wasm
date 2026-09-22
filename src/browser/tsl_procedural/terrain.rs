use super::super::gltf_viewer::fetch;
use super::*;
pub(in crate::browser) struct Terrain {
    viewer: OrbitViewer,
    terrain: Object3D,
    water: Object3D,
    params: [f32; 12],
    offset: Vector2,
    pointer: Vector2,
    hit: Option<Vector3>,
    dragging: bool,
}
impl Terrain {
    pub async fn create(s: &mut Scene, c: Object3D, r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 35.,
            aspect,
            near: 0.1,
            far: 100.,
            ..Default::default()
        }));
        let p = Vector3::new(-10., 8., -2.2);
        let center = Vector3::new(0., -0.5, 0.);
        let d = p - center;
        let mut viewer = OrbitViewer::from_camera(center, d.length());
        viewer.fixture(d.x.atan2(d.z), (d.y / d.length()).asin(), 1.8);
        s.environment = Some(Arc::new(crate::environment::EnvironmentMap::from_hdr(
            &fetch("/web/gallery/assets/tsl-procedural/pedestrian_overpass_1k.hdr").await?,
        )?));
        s.background_environment = true;
        s.background_blur = 0.5;
        s.tone_mapping = ToneMapping::Aces;
        s.shadow_map_size = 1024;
        let sun = s.insert(NodeKind::Light(Light::Sun {
            color: Color::WHITE,
            intensity: 2.,
        }));
        {
            let n = s.get_mut(sun)?;
            n.position = Vector3::new(6.25, 3., 4.);
            n.cast_shadow = true;
            n.shadow.far = 30.;
            n.shadow.normal_bias = 0.05;
        }
        let kernel = WgslFn::new(
            "terrain_kernel",
            &format!(
                "{}\n{}",
                include_str!("../../tsl/materialx.wgsl"),
                r#"
fn terrain_kernel(p:vec2<f32>,snow:bool)->f32 {
 if snow {return mxn_perlin(vec3(p*25.0,0.0),2u,false).x*0.1+0.45;}
 var q=p+u.custom[9].xy;
 q+=mxn_perlin(vec3(q*u.custom[1].x*u.custom[3].x,0.0),2u,false).x*u.custom[4].x;
 var e=0.0;for(var i=1.0;i<=u.custom[0].x;i+=1.0){e+=mxn_perlin(vec3(q*u.custom[1].x*i*2.0+i*987.0,0.0),2u,false).x/((i+1.0)*2.0);}
 return pow(abs(e),2.0)*sign(e)*u.custom[2].x;
}"#
            ),
            &[Type::Vec2, Type::Bool],
            Type::Float,
        )?;
        let elevation = |p: tsl::Node| {
            kernel.call(&[
                vec2(p.x(), p.swizzle("z")),
                float(0.).greater_than(float(1.)),
            ])
        };
        let original = position_geometry();
        let p = original.clone() + vec3(float(0.), elevation(original.clone()), float(0.));
        let a = original.clone() + vec3(float(0.01), float(0.), float(0.));
        let b = original + vec3(float(0.), float(0.), float(-0.01));
        let a = a.clone() + vec3(float(0.), elevation(a), float(0.));
        let b = b.clone() + vec3(float(0.), elevation(b), float(0.));
        let normal = (a - p.clone())
            .normalize()
            .cross((b - p.clone()).normalize());
        let grass = position_local()
            .y()
            .less_than(float(-0.06))
            .select(float(0.), float(1.));
        let rock = normal_local()
            .y()
            .less_than(float(0.5))
            .select(grass.clone(), float(0.));
        let snow = kernel.call(&[
            vec2(position_local().x(), position_local().swizzle("z")) + uniform(9, Type::Vec2),
            float(1.).greater_than(float(0.)),
        ]);
        let color = mix(
            mix(uniform(5, Type::Vec3), uniform(6, Type::Vec3), grass),
            uniform(8, Type::Vec3),
            rock,
        );
        let color = position_local()
            .y()
            .less_than(snow)
            .select(color, uniform(7, Type::Vec3));
        let mut m = MeshStandardMaterial {
            roughness: 0.5,
            energy_conservation: true,
            ..Default::default()
        };
        m.properties.vertex_program = Some(Arc::new(
            SurfaceNodes {
                position: Some(p.clone()),
                vertex_normal: Some(normal),
                color: Some(color),
                ..Default::default()
            }
            .build(r, &[], &[])
            .await?,
        ));
        m.properties.shadow_program = Some(Arc::new(shadow_program(r, Some(p), None).await?));
        let mut g = PlaneGeometry::build(10., 10., 500, 500)?;
        g.apply_matrix4(Matrix4::from_rotation_x(-std::f64::consts::FRAC_PI_2))?;
        let terrain = mesh(s, Arc::new(g), Material::Standard(m));
        s.get_mut(terrain)?.cast_shadow = true;
        s.get_mut(terrain)?.receive_shadow = true;
        let mut m = MeshPhysicalMaterial {
            transmission: 1.,
            ior: 1.333,
            ..Default::default()
        };
        m.base.roughness = 0.5;
        m.base.energy_conservation = true;
        m.base.properties.color = Color::from_hex(0x4db2ff);
        let water = mesh(
            s,
            Arc::new(PlaneGeometry::build(10., 10., 1, 1)?),
            Material::Physical(m),
        );
        s.get_mut(water)?.quaternion = Quaternion::from_rotation_x(-std::f64::consts::FRAC_PI_2);
        s.get_mut(water)?.position.y = -0.1;
        Ok(Self {
            viewer,
            terrain,
            water,
            params: [
                3.,
                0.175,
                10.,
                6.,
                1.,
                0xffe894 as f32,
                0x85d534 as f32,
                0xffffff as f32,
                0xbfbd8d as f32,
                0.5,
                1.333,
                0x4db2ff as f32,
            ],
            offset: Vector2::ZERO,
            pointer: Vector2::ZERO,
            hit: None,
            dragging: false,
        })
    }
    pub fn update(&mut self, s: &mut Scene, c: Object3D, _d: f64, _a: bool) -> Result<()> {
        self.viewer.limit_pitch(
            std::f64::consts::PI * 0.05,
            std::f64::consts::FRAC_PI_2 - 1e-6,
        );
        self.viewer.update(s, c)?;
        if let NodeKind::Camera(Camera::Perspective(p)) = &mut s.get_mut(c)?.kind {
            p.near = 0.1;
            p.far = 100.;
        }
        self.drag_update(s, c)?;
        if let NodeKind::Mesh(m) = &mut s.get_mut(self.terrain)?.kind {
            let p = Arc::make_mut(&mut m.materials[0]).properties_mut();
            for i in 0..5 {
                p.vertex_uniforms[i][0] = self.params[i];
            }
            for i in 5..9 {
                let c = Color::from_hex(self.params[i] as u32).0;
                p.vertex_uniforms[i] = [c.x as f32, c.y as f32, c.z as f32, 0.];
            }
            p.vertex_uniforms[9] = [self.offset.x as f32, self.offset.y as f32, 0., 0.];
        }
        if let NodeKind::Mesh(m) = &mut s.get_mut(self.water)?.kind
            && let Material::Physical(m) = Arc::make_mut(&mut m.materials[0])
        {
            m.base.roughness = self.params[9] as f64;
            m.ior = self.params[10] as f64;
            m.base.properties.color = Color::from_hex(self.params[11] as u32);
        }
        Ok(())
    }
    pub fn pointer(&mut self, x: f64, y: f64) {
        self.pointer = Vector2::new(x, y);
    }
    pub fn dragging(&mut self, value: bool) {
        self.dragging = value && self.hit.is_some();
    }
    fn drag_update(&mut self, s: &mut Scene, c: Object3D) -> Result<()> {
        s.update_world_matrix(c, true, false)?;
        let (cam, world) = s.camera(c)?;
        let hit = cam
            .ray(self.pointer, world)?
            .intersect_plane(Plane {
                normal: Vector3::Y,
                constant: 0.,
            })
            .filter(|p| {
                p.x.abs() <= if self.dragging { 50. } else { 5. }
                    && p.z.abs() <= if self.dragging { 50. } else { 5. }
            });
        if self.dragging
            && let (Some(a), Some(b)) = (self.hit, hit)
        {
            self.offset += Vector2::new(a.x - b.x, a.z - b.z);
        }
        self.hit = hit;
        Ok(())
    }
    pub fn parameter(&mut self, i: usize, v: f32) -> Result<()> {
        if i >= 12 || !v.is_finite() {
            return Err(Error::Invalid("terrain parameter"));
        }
        self.params[i] = v;
        Ok(())
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
        if self.dragging {
            return Ok(());
        }
        if p {
            self.viewer.pan_pixels(s, c, dx, dy, h)?;
        } else {
            self.viewer.orbit_pixels(dx, dy, w, h, 0.1, 50.);
        }
        Ok(())
    }
}
