use super::*;
impl Demo {
    fn view(
        &mut self,
        s: &mut Scene,
        c: Object3D,
        position: Vector3,
        target: Vector3,
    ) -> Result<()> {
        let p = position - target;
        self.viewer = OrbitViewer::from_camera(target, p.length());
        self.viewer
            .fixture(p.x.atan2(p.z), (p.y / p.length()).asin(), 1.8);
        self.viewer.update(s, c)
    }
    pub(super) async fn solids(&mut self, s: &mut Scene, c: Object3D) -> Result<()> {
        camera(s, c, 40., 0.1, 100.)?;
        self.view(s, c, Vector3::new(-6., 7., 14.), Vector3::new(0., 0.5, 0.))?;
        s.background = Color::from_hex(0x222222);
        let h = s.insert(NodeKind::Light(Light::Hemisphere {
            sky: Color::WHITE,
            ground: Color::from_hex(0x0e696c),
            intensity: 1.,
        }));
        s.get_mut(h)?.position.y = 1.;
        let light = s.insert(NodeKind::Light(Light::Directional {
            color: Color::WHITE,
            intensity: 6.,
            target: Vector3::ZERO,
        }));
        {
            let n = s.get_mut(light)?;
            n.position = Vector3::new(5., 10., 6.);
            n.cast_shadow = true;
            n.shadow.extent = 10.;
            n.shadow.far = 20.;
            n.shadow.map_size = Some(2048);
            n.shadow.radius = 10.;
        }
        #[derive(serde::Deserialize)]
        struct G {
            start: usize,
            count: usize,
            #[serde(rename = "materialIndex")]
            material_index: usize,
        }
        #[derive(serde::Deserialize)]
        struct Data {
            position: Vec<f32>,
            normal: Vec<f32>,
            groups: Vec<G>,
            #[serde(rename = "minY")]
            min_y: f64,
        }
        let data: Vec<Data> =
            serde_json::from_slice(&fetch(&format!("{ASSETS}/paper-models.json")).await?)
                .map_err(|e| Error::Asset(e.to_string()))?;
        let materials: Vec<_> = [0xe4002b, 0xff7f11, 0xffd100, 0x00a651, 0x0072ce, 0x8a2be2]
            .into_iter()
            .map(|color| {
                let mut m = MeshStandardMaterial {
                    roughness: 0.8,
                    ..Default::default()
                };
                m.properties.color = Color::from_hex(color);
                m.properties.side = Side::Double;
                m.energy_conservation = true;
                Arc::new(Material::Standard(m))
            })
            .collect();
        let mut geometries = Vec::new();
        let mut minimums = Vec::new();
        for d in data {
            let mut g = BufferGeometry::default();
            g.set_attribute(
                "position",
                Attribute::F32(BufferAttribute::new(d.position, 3, false)?),
            );
            g.set_attribute(
                "normal",
                Attribute::F32(BufferAttribute::new(d.normal, 3, false)?),
            );
            g.groups = d
                .groups
                .into_iter()
                .map(|x| Group {
                    start: x.start,
                    count: x.count,
                    material_index: x.material_index,
                })
                .collect();
            geometries.push(Arc::new(g));
            minimums.push(d.min_y);
        }
        let mut m = MeshStandardMaterial {
            roughness: 0.5,
            ..Default::default()
        };
        m.properties.color = Color::from_hex(0x0e696c);
        m.energy_conservation = true;
        let table = mesh(
            s,
            Arc::new(BoxGeometry::build(17., 0.5, 17.)?),
            Material::Standard(m),
        );
        s.get_mut(table)?.position.y = -0.25;
        s.get_mut(table)?.receive_shadow = true;
        for (i, j) in [0, 1, 2, 3, 3, 4, 0, 1, 1, 2, 3, 4, 4, 0, 1, 2]
            .into_iter()
            .enumerate()
        {
            let row = i / 4;
            let col = i % 4;
            let scale = 0.5 + row as f64 * 0.25;
            let mut m = Mesh::new(geometries[j].clone(), materials[0].clone());
            m.materials = materials.clone();
            let h = s.insert(NodeKind::Mesh(m));
            let n = s.get_mut(h)?;
            n.cast_shadow = true;
            n.scale = Vector3::splat(scale);
            n.position = Vector3::new(
                (col as f64 - 1.5) * 3.5,
                -minimums[j] * scale - 0.01,
                (1.5 - row as f64) * 3.5,
            );
        }
        Ok(())
    }
    pub(super) fn clipping(&mut self, s: &mut Scene, c: Object3D) -> Result<()> {
        camera(s, c, 36., 0.25, 16.)?;
        self.view(s, c, Vector3::new(0., 1.3, 3.), Vector3::new(0., 1., 0.))?;
        s.clipping_shadows = false;
        s.insert(NodeKind::Light(Light::Ambient {
            color: Color::from_hex(0xcccccc),
            intensity: 1.,
        }));
        let spot = s.insert(NodeKind::Light(Light::Spot {
            color: Color::WHITE,
            intensity: 60.,
            target: Vector3::ZERO,
            distance: 0.,
            decay: 2.,
            angle: std::f64::consts::PI / 5.,
            penumbra: 0.2,
        }));
        {
            let n = s.get_mut(spot)?;
            n.position = Vector3::new(2., 3., 3.);
            n.cast_shadow = true;
            n.shadow.near = 3.;
            n.shadow.far = 10.;
            n.shadow.map_size = Some(2048);
            n.shadow.radius = 4.;
        }
        let sun = s.insert(NodeKind::Light(Light::Sun {
            color: Color::from_hex(0x55505a),
            intensity: 3.,
        }));
        {
            let n = s.get_mut(sun)?;
            n.position = Vector3::new(0., 3., 0.);
            n.cast_shadow = true;
            n.shadow.far = 10.;
            n.shadow.map_size = Some(1024);
        }
        let mut m = MeshPhongMaterial {
            shininess: 0.,
            ..Default::default()
        };
        m.properties.color = Color::from_hex(0x80ee10);
        m.properties.side = Side::Double;
        m.properties.alpha_to_coverage = true;
        let knot = mesh(
            s,
            Arc::new(TorusKnotGeometry::build(0.4, 0.08, 95, 20, 2, 3)?),
            Material::Phong(m),
        );
        s.get_mut(knot)?.cast_shadow = true;
        let mut m = MeshPhongMaterial {
            shininess: 150.,
            ..Default::default()
        };
        m.properties.color = Color::from_hex(0xa0adaf);
        m.properties.alpha_to_coverage = true;
        let ground = mesh(
            s,
            Arc::new(PlaneGeometry::build(9., 9., 1, 1)?),
            Material::Phong(m),
        );
        let n = s.get_mut(ground)?;
        n.receive_shadow = true;
        n.quaternion = Quaternion::from_rotation_x(-std::f64::consts::FRAC_PI_2);
        self.objects = vec![knot, ground];
        self.update_clipping(s)
    }
    pub(super) fn update_clipping(&self, s: &mut Scene) -> Result<()> {
        s.clipping_planes = if self.params[5] > 0.5 {
            vec![Plane {
                normal: Vector3::NEG_X,
                constant: self.params[6] as f64,
            }]
        } else {
            vec![]
        };
        for (i, &h) in self.objects.iter().enumerate() {
            let n = s.get_mut(h)?;
            if i == 0 {
                n.position.y = 0.8;
                n.quaternion = Quaternion::from_euler(
                    glam::EulerRot::XYZ,
                    self.time * 0.5,
                    self.time * 0.2,
                    0.,
                );
                n.scale = Vector3::splat(self.time.cos() * 0.125 + 0.875);
            }
            if let NodeKind::Mesh(m) = &mut n.kind {
                let p = Arc::make_mut(&mut m.materials[0]).properties_mut();
                p.alpha_to_coverage = self.params[0] > 0.5;
                if i == 0 {
                    p.clip_shadows = self.params[2] > 0.5;
                    p.clip_intersection = self.params[3] > 0.5;
                    p.clipping_planes = if self.params[1] > 0.5 {
                        vec![
                            Plane {
                                normal: Vector3::NEG_Y,
                                constant: self.params[4] as f64,
                            },
                            Plane {
                                normal: Vector3::NEG_Z,
                                constant: 0.1,
                            },
                        ]
                    } else {
                        vec![]
                    };
                }
            }
        }
        Ok(())
    }
}
